use std::io;
use tracing_appender::rolling::{RollingFileAppender, Rotation};

use client::{
    register, schedule_one_of, spawn_scheduler_with_handler, Deadline, Error, ResourceAlloc,
    ResourceMemory, ResourceReq, ResourceType, TaskFunc, TaskReqBuilder, TaskRequirements,
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
    type Output = String;
    type Error = Error;

    fn end(&mut self, _: Option<&ResourceAlloc>) -> Result<Self::Output, Self::Error> {
        Ok(format!("Task {} done!!!", self.id))
    }

    fn task(&mut self, _alloc: Option<&ResourceAlloc>) -> Result<TaskResult, Self::Error> {
        if self.index < 4 {
            self.index += 1;
            tracing::info!("Task {} Running!!! ", self.id);
            std::thread::sleep(Duration::from_secs(2));
            tracing::info!("Task {} returning!!! ", self.id);
            return Ok(TaskResult::Continue);
        }
        tracing::info!("Task {} Done!!! ", self.id);
        Ok(TaskResult::Done)
    }
}

fn task_requirements() -> TaskRequirements {
    let start = chrono::Utc::now();
    let end = start + chrono::Duration::seconds(30);
    let deadline = Deadline::new(start, end);
    TaskReqBuilder::new()
        .resource_req(ResourceReq {
            resource: ResourceType::Gpu(ResourceMemory::Mem(2)),
            quantity: 1,
            preemptible: true,
        })
        .with_time_estimations(Duration::from_millis(500), 1, Duration::from_millis(3000))
        .with_deadline(Some(deadline))
        .build()
}

#[test]
fn test_schedule() {
    //let file_appender =
    //    RollingFileAppender::new(Rotation::HOURLY, "../client/tests", "test_schedule.log");
    //let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    //tracing_subscriber::fmt().with_writer(non_blocking).init();
    tracing_subscriber::fmt().with_writer(io::stdout).init();

    let handler = if let Ok(handle) = spawn_scheduler_with_handler("127.0.0.1:5000") {
        Some(handle)
    } else {
        None
    };

    let mut joiner = vec![];
    for i in 0..5 {
        joiner.push(std::thread::spawn(move || {
            let client = register(i, i as u64).unwrap();
            let mut test_func = Test::new(i as _);
            let mut task_req = task_requirements();
            if i == 0 {
                task_req.deadline = None;
            }
            schedule_one_of(
                client,
                &mut test_func,
                Some(task_req),
                Duration::from_secs(20),
            )
        }));
        std::thread::sleep(Duration::from_secs(2));
    }
    for j in joiner.into_iter() {
        let res = j.join().unwrap();
        assert!(res.is_ok());
    }

    if let Some(h) = handler {
        h.close();
    }
}
