use std::sync::atomic::{AtomicU8, Ordering};
use std::time::Duration;

use tracing::{debug, error};

use raw_sync::locks::*;
use raw_sync::Timeout;
use shared_memory::*;

use common::Error as ClientError;

// We will not use this for anything except hosting a global mutex
const SHARED_MEM_SIZE: usize = 2048;

// The path to were the shared_mem file-link would be stored
const SHARED_MEM_PATH: &str = concat!(env!("OUT_DIR"), "/scheduler_shm");
#[cfg(test)]
const IPC_PATH: &str = concat!(env!("OUT_DIR"), "/ipc_buffer");

// The time in milliseconds to wait until an error is returned when trying to lock a global mutex
const TIMEOUT: u64 = 10;

pub struct GlobalMutex {
    #[allow(dead_code)]
    mem: Shmem,
    mutex: Box<dyn LockImpl>,
}

impl GlobalMutex {
    pub fn new() -> Result<Self, ClientError> {
        Self::_new(None)
    }

    pub fn new_with_name<'a>(name: &'a str) -> Result<Self, ClientError> {
        Self::_new(Some(name))
    }

    #[tracing::instrument(level = "debug")]
    fn _new<'a>(name: Option<&'a str>) -> Result<Self, ClientError> {
        let path = if let Some(suffix) = name {
            format!("{}_{}", SHARED_MEM_PATH, suffix)
        } else {
            SHARED_MEM_PATH.to_string()
        };
        // Create or open the shared memory mapping
        let shmem = match ShmemConf::new().size(SHARED_MEM_SIZE).flink(&path).create() {
            Ok(m) => m,
            Err(ShmemError::LinkExists) => {
                let share = ShmemConf::new().flink(&path).open();
                if let Err(e) = share {
                    error!("Shared memory exist but can not be opened");
                    return Err(ClientError::GlobalMutexError(e.to_string()));
                }
                share.unwrap()
            }
            Err(e) => {
                error!(
                    "Unable to create or open shmem link for global mutex {} : {}",
                    path, e
                );
                return Err(ClientError::GlobalMutexError(e.to_string()));
            }
        };

        let mut raw_ptr = shmem.as_ptr();
        let is_init: &mut AtomicU8;
        let mutex: Box<dyn LockImpl>;

        unsafe {
            is_init = &mut *(raw_ptr as *mut u8 as *mut AtomicU8);
            raw_ptr = raw_ptr.add(8);
        };

        // Initialize or wait for initialized mutex
        mutex = if shmem.is_owner() {
            debug!("Process {} owns the shared memory", std::process::id());
            is_init.store(0, Ordering::Relaxed);
            // Initialize the mutex
            let (lock, _bytes_used) = unsafe {
                Mutex::new(
                    // Base address of Mutex
                    raw_ptr,
                    // Address of data protected by mutex
                    raw_ptr.add(Mutex::size_of(Some(raw_ptr))),
                )
                .map_err(|e| ClientError::GlobalMutexError(e.to_string()))?
            };
            is_init.store(1, Ordering::Relaxed);
            lock
        } else {
            // We are not the owner of the share memory so we need to why until the process that
            // owns it has already initialized it.
            while is_init.load(Ordering::Relaxed) != 1 {}
            // Load existing mutex
            let (lock, _bytes_used) = unsafe {
                Mutex::from_existing(raw_ptr, raw_ptr.add(Mutex::size_of(Some(raw_ptr))))
                    .map_err(|e| ClientError::GlobalMutexError(e.to_string()))?
            };
            lock
        };

        Ok(Self { mem: shmem, mutex })
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub fn try_lock(&self) -> Result<LockGuard<'_>, ClientError> {
        debug!(
            "Trying to acquire the mutex - process id: {}",
            std::process::id()
        );

        self.mutex
            .try_lock(Timeout::Val(Duration::from_millis(TIMEOUT)))
            .map_err(|e| ClientError::GlobalMutexError(e.to_string()))
    }

    pub fn release(&self) -> Result<(), ClientError> {
        self.mutex
            .release()
            .map_err(|e| ClientError::GlobalMutexError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUM_THREADS: usize = 4;

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
            Err(e) => panic!(e.to_string()),
        }
    }
}
