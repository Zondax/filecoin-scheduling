use std::collections::{HashMap, VecDeque};

use crate::{
    config::Settings,
    solver::{Resources, Solver, TaskState},
    Result,
};
use common::{DeviceId, Pid, ResourceAlloc, TaskRequirements};

use priority_queue::PriorityQueue;
use std::cmp::Reverse;

pub struct GreedySolver;

fn get_by_resource_load(
    resources: &Resources,
    tasks_state: &HashMap<Pid, TaskState>,
) -> Vec<DeviceId> {
    let mut map = HashMap::new();
    // get the load of each device
    resources.0.iter().for_each(|(id, _)| {
        map.insert(id, 0usize);
    });
    for (id, counter) in map.iter_mut() {
        if tasks_state
            .iter()
            .any(|(_, state)| state.allocation.devices.iter().any(|dev| dev == *id))
        {
            *counter += 1;
        }
    }
    let mut resource_load_queue = PriorityQueue::new();

    // here we order the resources according to the number of jobs that are using it
    // so we can select those with lower load
    map.into_iter().for_each(|(key, val)| {
        resource_load_queue.push(key, Reverse(val));
    });

    resource_load_queue
        .into_sorted_iter()
        .map(|(i, _)| i.clone())
        .collect::<Vec<_>>()
}

impl Solver for GreedySolver {
    fn allocate_task(
        &mut self,
        resources: &Resources,
        requirements: &TaskRequirements,
        restrictions: &Option<Vec<DeviceId>>,
        tasks_state: &HashMap<Pid, TaskState>,
    ) -> Option<ResourceAlloc> {
        let device_restrictions = restrictions
            .clone()
            .unwrap_or_else(|| resources.0.keys().cloned().collect::<Vec<DeviceId>>());

        let mut options = vec![];

        for req in requirements.req.iter() {
            let mut quantity = req.quantity;
            // we are bounded by the number of resources the user has assigned to this task
            if quantity > device_restrictions.len() {
                quantity = device_restrictions.len();
            }
            // check if the pool of devices have room for the requested allocations
            let mut optional_resources = resources
                .get_devices_with_requirements(req)
                .filter(|b| device_restrictions.iter().any(|x| x == b))
                .collect::<Vec<DeviceId>>();

            if optional_resources.len() >= quantity {
                if resources.0.len() > 1 {
                    let ordered = get_by_resource_load(resources, tasks_state);
                    let filtered = ordered
                        .iter()
                        .filter(|id| optional_resources.iter().any(|optional| optional == *id))
                        .take(quantity)
                        .cloned()
                        .collect::<Vec<_>>();
                    options.push((filtered, req.clone()));
                } else {
                    optional_resources.truncate(quantity);
                    options.push((optional_resources, req.clone()));
                }
            }
        }

        if !options.is_empty() {
            // it is here where we can use some heuristic approach to select the best devices
            // but maybe for this we need a more advance scheduler algorithm
            let requirement = options[0].1.clone();
            let devices = options[0].0.clone();
            return Some(ResourceAlloc {
                requirement,
                devices,
            });
        }
        None
    }

    fn solve_job_schedule(
        &mut self,
        current_state: &HashMap<Pid, TaskState>,
        _scheduler_settings: &Settings,
    ) -> Result<VecDeque<Pid>> {
        // Criterion A: Use task deadline as a priority indicator. The sooner the deadline the higher
        //      the priority
        //
        // Criterion B: to avoid ties, we also use the time at which the task was created in the
        //      scheduler. to ensure a partial ordering among task that might have the same deadline.

        let mut priority_queue = PriorityQueue::new();

        // iterate our tasks for making the triplet pushing it into the queue
        for (job_id, state) in current_state.iter() {
            // get the jobs deadline or fake a new one.
            let deadline = state.requirements.deadline.map_or(u64::MAX, |d| {
                d.as_duration().map(|d| d.as_secs()).unwrap_or(u64::MAX)
            });
            let conditions = (Reverse(deadline), Reverse(state.creation_time));
            priority_queue.push(job_id, conditions);
        }

        Ok(priority_queue
            .into_sorted_iter()
            .map(|(i, _)| *i)
            .collect::<VecDeque<Pid>>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn test_creation_time_priority() {
        let mut priority_queue = PriorityQueue::new();
        let mut res = vec![];
        for i in 0..5 {
            res.push(i);
            let finish_time = Reverse(10);
            let creation_time: u64 = chrono::Utc::now().timestamp() as u64;
            let conditions = (finish_time, Reverse(creation_time));
            priority_queue.push(i, conditions);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        // using only the creation time as a result we have a FIFO queue
        let mut queue = priority_queue
            .into_sorted_iter()
            .map(|(i, _)| i)
            .collect::<VecDeque<_>>();
        for i in res.into_iter() {
            assert_eq!(queue.pop_front().unwrap(), i);
        }
    }

    #[test]
    fn test_finish_time_priority() {
        let mut priority_queue = PriorityQueue::new();
        let mut res = vec![];
        for i in 0..5 {
            res.push(i);
            let finish_time = Reverse(5 - i);
            let creation_time: u64 = chrono::Utc::now().timestamp() as u64;
            let conditions = (finish_time, Reverse(creation_time));
            priority_queue.push(i, conditions);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        let queue = priority_queue
            .into_sorted_iter()
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        let queue = queue.into_iter().rev().collect::<Vec<_>>();
        assert_eq!(res, queue);
    }
}
