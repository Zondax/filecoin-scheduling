use criterion::{criterion_group, criterion_main, Bencher, Criterion};

use scheduler::{dummy_task_requirements, ClientToken, Scheduler};
mod common;

fn schedule_one_job_bench(c: &mut Criterion) {
    let client = ClientToken {
        pid: 0,
        name: "one_client".to_string(),
        context: "".to_string(),
    };
    let clients = vec![client];
    let scheduler = common::create_scheduler("bench_1_job").unwrap();
    c.bench_function("Bench allocation for one client", |mut b| {
        schedule(&mut b, clients.as_slice(), &scheduler)
    });
}

fn schedule_500_jobs_bench(c: &mut Criterion) {
    let clients = (0..500)
        .map(|id| ClientToken {
            pid: id,
            name: id.to_string(),
            context: "".to_string(),
        })
        .collect::<Vec<_>>();
    let scheduler = common::create_scheduler("bench_500_allocation").unwrap();
    c.bench_function("Bench allocation for 500 clients", |mut b| {
        schedule(&mut b, clients.as_slice(), &scheduler)
    });
}

fn schedule(bench: &mut Bencher, clients: &[ClientToken], scheduler: &Scheduler) {
    let task_req = dummy_task_requirements();
    bench.iter(|| {
        for client in clients.iter() {
            scheduler
                .schedule(client.clone(), task_req.clone())
                .unwrap();
        }
    });
}

criterion_group!(benches, schedule_one_job_bench, schedule_500_jobs_bench);

criterion_main!(benches);
