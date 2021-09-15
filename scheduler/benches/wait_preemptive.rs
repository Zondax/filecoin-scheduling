use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use std::collections::HashSet;

use rand::seq::IteratorRandom;
use scheduler::{dummy_task_requirements, ClientToken, PreemptionResponse, Scheduler};

mod common;

fn wait_preemptive_one_job(c: &mut Criterion) {
    let client = ClientToken {
        pid: 0,
        name: "one_client".to_string(),
        context: "".to_string(),
    };
    let mut clients = std::iter::once(client).collect::<HashSet<_>>();
    let scheduler = common::create_scheduler("preemption_1_job").unwrap();
    c.bench_function("Bench wait_preemptive for one client", |mut b| {
        preemption(&mut b, &mut clients, &scheduler)
    });
}

fn wait_preemptive_1000_jobs(c: &mut Criterion) {
    let mut clients = (0..1000)
        .map(|id| ClientToken {
            pid: id,
            name: id.to_string(),
            context: "".to_string(),
        })
        .collect::<HashSet<_>>();
    let scheduler = common::create_scheduler("bench_preemption_1000_jobs").unwrap();
    c.bench_function("Bench wait_preemptive for 1000 clients", |mut b| {
        preemption(&mut b, &mut clients, &scheduler)
    });
}

fn preemption(bench: &mut Bencher, clients: &mut HashSet<ClientToken>, scheduler: &Scheduler) {
    let task_req = dummy_task_requirements();
    let mut rng = rand::thread_rng();
    for client in clients.iter() {
        scheduler
            .schedule(client.clone(), task_req.clone())
            .unwrap();
    }
    let mut ids = Vec::with_capacity(clients.len());
    bench.iter(|| {
        while !clients.is_empty() {
            // simulates random netwirk requests
            for client in clients
                .iter()
                .choose_multiple(&mut rng, clients.len())
                .into_iter()
            {
                let res = scheduler.wait_preemptive(client.clone()).unwrap();
                if matches!(res, PreemptionResponse::Execute) {
                    ids.push(client.clone());
                }
            }
            // do just one iteration per clients
            for client in ids.iter() {
                scheduler.release_preemptive(client.clone());
                scheduler.release(client.clone());
                clients.remove(client);
            }
            ids.clear();
        }
    });
}

criterion_group!(benches, wait_preemptive_one_job, wait_preemptive_1000_jobs);

criterion_main!(benches);
