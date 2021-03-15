use chrono::offset::Utc;
use std::io;

use client::{
    register, schedule_one_of, spawn_scheduler_with_handler, ResourceAlloc, Task, TaskResult,
};
use std::time::Duration;

#[test]
fn test_schedule() {
    //let file_appender =
    //    RollingFileAppender::new(Rotation::HOURLY, "../client/tests", "test_schedule.log");
    //let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    //tracing_subscriber::fmt().with_writer(non_blocking).init();
    tracing_subscriber::fmt().with_writer(io::stdout).init();

    let handler = spawn_scheduler_with_handler("127.0.0.1:5000").unwrap();

    let mut index = 0;

    let mut joiner = vec![];
    for i in 0..5 {
        joiner.push(std::thread::spawn(move || {
            let client = register(i, i as u64).unwrap();
            let func = move |_alloc: &ResourceAlloc| -> TaskResult<String> {
                if index < 4 {
                    index += 1;
                    std::thread::sleep(Duration::from_secs(1));
                    return TaskResult::Continue;
                }
                tracing::info!("Client task {} Done!!! ", i);
                TaskResult::Done(Ok(format!("Task {} done!!!", i)))
            };
            let mut task = Task::default(func);

            let end;
            task.task_req.estimations.num_of_iter = 4;
            if i == 0 {
                end = Utc::now() + chrono::Duration::seconds(250 as _);
                task.task_req.estimations.time_per_iter = Duration::from_secs(60);
            } else {
                end = Utc::now() + chrono::Duration::seconds(20 as _);
            }
            task.task_req.deadline.1 = end;
            schedule_one_of(client, &mut task, Duration::from_secs(20))
        }));
        std::thread::sleep(Duration::from_secs(1));
    }
    for j in joiner.into_iter() {
        let res = j.join().unwrap();
        assert!(res.is_ok());
    }
    handler.close();
}
