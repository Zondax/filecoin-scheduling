use std::io;

use client::{
    register, schedule_one_of, spawn_scheduler_with_handler, ResourceAlloc, Task, TaskFunc,
    TaskResult,
};
use std::time::Duration;

struct Test {
    index: usize,
    id: usize,
}

impl Test {
    fn new(id: usize) -> Self {
        Self { index: 0usize, id }
    }
}

impl TaskFunc for Test {
    type TaskOutput = String;

    fn task(&mut self, alloc: &ResourceAlloc) -> TaskResult<Self::TaskOutput> {
        if self.index < 4 {
            self.index += 1;
            std::thread::sleep(Duration::from_secs(1));
            return TaskResult::Continue;
        }
        tracing::info!("Client task {} Done!!! ", self.id);
        TaskResult::Done(Ok(format!("Task {} done!!!", self.id)))
    }
}

#[test]
fn test_schedule() {
    //let file_appender =
    //    RollingFileAppender::new(Rotation::HOURLY, "../client/tests", "test_schedule.log");
    //let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    //tracing_subscriber::fmt().with_writer(non_blocking).init();
    tracing_subscriber::fmt().with_writer(io::stdout).init();

    let handler = spawn_scheduler_with_handler("127.0.0.1:5000").unwrap();

    let mut joiner = vec![];
    for i in 0..5 {
        joiner.push(std::thread::spawn(move || {
            let client = register(i, i as u64).unwrap();
            let test_func = Test::new(i as _);
            let mut task = Task::default(test_func);
            if i == 0 {
                task.task_req.deadline = None;
            }
            schedule_one_of(client, task, Duration::from_secs(20))
        }));
        std::thread::sleep(Duration::from_secs(1));
    }
    for j in joiner.into_iter() {
        let res = j.join().unwrap();
        assert!(res.is_ok());
    }
    handler.close();
}
