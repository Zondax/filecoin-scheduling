use client::{
    list_all_resources, register, schedule_one_of, spawn_scheduler_with_handler, ClientError,
    ResourceAlloc, Task, TaskResult,
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

    let client = register(std::process::id(), 1);
    let func = |_alloc: &[ResourceAlloc]| -> TaskResult<String> {
        std::thread::sleep(Duration::from_secs(1));
        tracing::info!("from client task");
        TaskResult::Done(Ok("Hello World".to_string()))
    };
    let task = Task::default(func);
    let res = schedule_one_of(client, task, Duration::from_secs(15));
    // Closes the scheduler service
    handler.close();

    let num_devices = list_all_resources().gpu_devices().len();
    if num_devices == 0 && res.is_err() {
        // In the case that there are not GPUs on the system
        // we should have got and Timeout error
        assert_eq!(res.unwrap_err(), ClientError::Timeout);
    } else {
        assert!(res.is_ok());
    }
}
