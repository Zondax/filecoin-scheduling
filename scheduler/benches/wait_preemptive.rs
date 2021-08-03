use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use std::collections::HashSet;

use common::{dummy_task_requirements, list_devices, ClientToken, PreemptionResponse};
use rand::seq::IteratorRandom;
use scheduler::{Scheduler, Settings};

fn wait_preemptive_one_job(c: &mut Criterion) {
    let client = ClientToken {
        pid: 0,
        name: "one_client".to_string(),
    };
    let mut clients = std::iter::once(client).collect::<HashSet<_>>();
    c.bench_function("Bench wait_preemptive for one client", |mut b| {
        preemption(&mut b, &mut clients)
    });
}

fn wait_preemptive_1000_jobs(c: &mut Criterion) {
    let mut clients = (0..1000)
        .map(|id| {
            let client = ClientToken {
                pid: id,
                name: id.to_string(),
            };
            client
        })
        .collect::<HashSet<_>>();
    c.bench_function("Bench wait_preemptive for 1000 clients", |mut b| {
        preemption(&mut b, &mut clients)
    });
}

fn preemption(bench: &mut Bencher, clients: &mut HashSet<ClientToken>) {
    let settings = Settings::default();
    let devices = list_devices();
    let task_req = dummy_task_requirements();
    let scheduler = Scheduler::new(settings, devices, None);
    let mut rng = rand::thread_rng();
    for client in clients.iter() {
        scheduler.schedule(client.clone(), task_req.clone(), "".to_string());
    }
    let mut ids = Vec::with_capacity(clients.len());
    bench.iter(|| {
        while clients.len() > 0 {
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
