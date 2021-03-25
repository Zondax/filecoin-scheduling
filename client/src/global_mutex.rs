use std::fs::{File, OpenOptions};
use std::path::PathBuf;

use tracing::debug;

use fs2::FileExt;

use common::Error as ClientError;

// The path to were the shared_mem file-link would be stored
const SHARED_MEM_PATH: &str = "scheduler_shm";
#[cfg(test)]
const IPC_PATH: &str = "ipc_buffer";

pub struct GlobalMutex(File);

impl GlobalMutex {
    pub fn new() -> Result<Self, ClientError> {
        Self::_new(None)
    }

    #[allow(dead_code)]
    pub fn new_with_name(name: &str) -> Result<Self, ClientError> {
        Self::_new(Some(name))
    }

    fn tmp_path(filename: &str) -> PathBuf {
        let mut p = std::env::temp_dir();
        p.push(filename);
        p
    }

    fn _new<'a>(name: Option<&'a str>) -> Result<Self, ClientError> {
        let path = if let Some(suffix) = name {
            Self::tmp_path(&format!("{}_{}", SHARED_MEM_PATH, suffix))
        } else {
            Self::tmp_path(SHARED_MEM_PATH)
        };
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
            .unwrap();
        Ok(Self(file))
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub fn try_lock(&self) -> Result<() /*LockGuard<'_>*/, ClientError> {
        debug!(
            "Trying to acquire the mutex - process id: {}",
            std::process::id()
        );

        self.0
            .try_lock_exclusive()
            .map_err(|e| ClientError::GlobalMutexError(e.to_string()))
    }

    pub fn release(&self) -> Result<(), ClientError> {
        self.0
            .unlock()
            .map_err(|e| ClientError::GlobalMutexError(e.to_string()))
    }
}

impl Drop for GlobalMutex {
    fn drop(&mut self) {
        let _ = self.release();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUM_THREADS: usize = 4;

    // We will not use this for anything except hosting a global mutex
    const SHARED_MEM_SIZE: usize = 2048;

    #[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
    #[repr(u8)]
    enum MutexState {
        //Only one thread should report Owned
        Owned,
        // Other threads should report that the mutex is locked
        Locked,
        Error,
    }

    #[test]
    fn test_mutex_contention() {
        mutex_contention_threads();
        #[cfg(target_os = "linux")]
        mutex_contention_processes();
    }

    fn mutex_contention_threads() {
        use std::sync::mpsc;

        let (tx, rx) = mpsc::channel::<MutexState>();
        let mut handlers = vec![];
        // Lets run 4 threads instead of processes
        for i in 0..NUM_THREADS {
            let sender = tx.clone();
            let handler = std::thread::Builder::new()
                .name(i.to_string())
                .spawn(move || {
                    // Pass a name to the mutex because so that it is exclusive to this test
                    let mutex = GlobalMutex::new_with_name("threads").unwrap();
                    let guard = mutex.try_lock();
                    if let Ok(_) = guard {
                        sender.send(MutexState::Owned).unwrap();
                        // Ensures that this threads owns the mutex along the test
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    } else {
                        sender.send(MutexState::Locked).unwrap();
                    }
                });
            handlers.push(handler.unwrap());
        }

        for h in handlers.drain(..) {
            h.join().unwrap();
        }

        let mut res = vec![];
        for _ in 0..NUM_THREADS {
            if let Ok(state) = rx.recv() {
                res.push(state);
            }
        }

        assert_eq!(res.len(), NUM_THREADS);
        // At least one thread should have owned the mutex
        assert_eq!(1, res.iter().filter(|s| **s == MutexState::Owned).count());
    }

    #[cfg(target_os = "linux")]
    fn mutex_contention_processes() {
        use ipmpsc::{Receiver, Sender, SharedRingBuffer};
        use nix::unistd::{fork, ForkResult};
        use std::path::Path;

        if Path::new(IPC_PATH).exists() {
            std::fs::remove_file(IPC_PATH).unwrap();
        }

        //Lets run 4 threads instead of processes
        match unsafe { fork() } {
            Ok(ForkResult::Parent { .. }) => {
                // checks if the file buffer exists
                if !Path::new(IPC_PATH).exists() {
                    SharedRingBuffer::create(IPC_PATH, SHARED_MEM_SIZE as _).unwrap();
                }
                let shared = SharedRingBuffer::open(IPC_PATH).unwrap();
                let sender = Sender::new(shared);
                let mutex = if let Ok(mutex) = GlobalMutex::new_with_name("process") {
                    mutex
                } else {
                    sender.send(&MutexState::Error).unwrap();
                    return;
                };
                let guard = mutex.try_lock();
                if let Ok(_) = guard {
                    sender.send(&MutexState::Owned).unwrap();
                    // Ensures that this threads owns the mutex along the test
                    std::thread::sleep(std::time::Duration::from_millis(300));
                } else {
                    sender.send(&MutexState::Locked).unwrap();
                }
                drop(guard);

                let mut res: Vec<MutexState> = vec![];
                let shared = SharedRingBuffer::open(IPC_PATH).unwrap();
                let rx = Receiver::new(shared);
                for _ in 0..2 {
                    if let Ok(state) = rx.recv::<MutexState>() {
                        if state == MutexState::Error {
                            break;
                        }
                        res.push(state);
                    }
                }

                assert_eq!(res.len(), 2);
                // At least one thread should have owned the mutex
                assert_eq!(1, res.iter().filter(|s| **s == MutexState::Owned).count());
            }

            Ok(ForkResult::Child) => {
                // checks if the file buffer exists
                if !Path::new(IPC_PATH).exists() {
                    SharedRingBuffer::create(IPC_PATH, SHARED_MEM_SIZE as _).unwrap();
                }
                let shared = SharedRingBuffer::open(IPC_PATH).unwrap();
                let sender = Sender::new(shared);
                let mutex = if let Ok(mutex) = GlobalMutex::new_with_name("process") {
                    mutex
                } else {
                    sender.send(&MutexState::Error).unwrap();
                    return;
                };
                let guard = mutex.try_lock();
                if let Ok(_) = guard {
                    sender.send(&MutexState::Owned).unwrap();
                    // Ensures that this threads owns the mutex along the test
                    std::thread::sleep(std::time::Duration::from_millis(300));
                } else {
                    sender.send(&MutexState::Locked).unwrap();
                }
                drop(guard);
            }
            Err(e) => panic!("{}", e.to_string()),
        }
    }
}
