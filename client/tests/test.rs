use client::{
    register, schedule_one_of, spawn_scheduler_with_handler, ResourceAlloc, Task, TaskResult,
};
use std::time::Duration;

use tracing_appender::rolling::{RollingFileAppender, Rotation};

#[test]
fn test_schedule() {
    let file_appender =
        RollingFileAppender::new(Rotation::HOURLY, "../client/tests", "test_schedule.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt().with_writer(non_blocking).init();

    let handler = spawn_scheduler_with_handler("127.0.0.1:5000").unwrap();

    let mut joiner = vec![];
    for i in 0..3 {
        joiner.push(std::thread::spawn(move || {
            let client = register(i, i as u64).unwrap();
            let func = move |_alloc: &[ResourceAlloc]| -> TaskResult<String> {
                std::thread::sleep(Duration::from_secs(1));
                tracing::info!("from client task {}", i);
                TaskResult::Done(Ok(format!("Hello World: {}", i)))
            };
            let task = Task::default(func);
            schedule_one_of(client, task, Duration::from_secs(15))
        }));
    }
    for j in joiner.into_iter() {
        let res = j.join().unwrap();
        assert!(res.is_ok());
    }
    // Closes the scheduler service
    handler.close();
}
