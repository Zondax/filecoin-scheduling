use criterion::{criterion_group, criterion_main, Bencher, Criterion};

use std::time::Duration;

use client::{Client, Error, ResourceAlloc, TaskFunc, TaskResult};
use common::dummy_task_requirements;
const NUM_ITERATIONS: usize = 100;

#[derive(Copy, Clone)]
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
        let result = if self.index < NUM_ITERATIONS {
            self.index += 1;
            // Do some computations
            TaskResult::Continue
        } else {
            TaskResult::Done
        };

        Ok(result)
    }
}

fn call_schedule(b: &mut Bencher) {
    let client = Client::register::<Error>().unwrap();

    let task_req = dummy_task_requirements();
    let mut test_func = Test::new(client.token.pid as _);
    b.iter(|| {
        let _ = client
            .schedule_one_of(&mut test_func, task_req.clone(), Duration::from_secs(60))
            .unwrap();
    });
}

fn bench_schedule(c: &mut Criterion) {
    c.bench_function("Bench schedule_one_of for 1 client", |mut b| {
        call_schedule(&mut b)
    });
}

criterion_group!(benches, bench_schedule);
criterion_main!(benches);
