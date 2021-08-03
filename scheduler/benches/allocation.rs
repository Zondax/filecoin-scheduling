use criterion::{criterion_group, criterion_main, Bencher, Criterion};

use common::{dummy_task_requirements, list_devices, ClientToken};
use scheduler::{Scheduler, Settings};

fn schedule_one_job_bench(c: &mut Criterion) {
    let client = ClientToken {
        pid: 0,
        name: "one_client".to_string(),
    };
    let clients = vec![client];
    c.bench_function("Bench allocation for one client", |mut b| {
        schedule(&mut b, clients.as_slice())
    });
}

fn schedule_500_jobs_bench(c: &mut Criterion) {
    let clients = (0..500)
        .map(|id| {
            let client = ClientToken {
                pid: id,
                name: id.to_string(),
            };
            client
        })
        .collect::<Vec<_>>();
    c.bench_function("Bench allocation for 500 clients", |mut b| {
        schedule(&mut b, clients.as_slice())
    });
}

fn schedule(bench: &mut Bencher, clients: &[ClientToken]) {
    let settings = Settings::default();
    let devices = list_devices();
    let scheduler = Scheduler::new(settings, devices, None);
    let task_req = dummy_task_requirements();
    bench.iter(|| {
        for client in clients.iter() {
            scheduler.schedule(client.clone(), task_req.clone(), "".to_string());
        }
    });
}

criterion_group!(benches, schedule_one_job_bench, schedule_500_jobs_bench);

criterion_main!(benches);
