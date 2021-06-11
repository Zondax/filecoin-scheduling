use std::collections::{HashMap, VecDeque};

use crate::config::Settings;
use crate::scheduler::task_is_stalled;
use crate::solver::{ResourceState, Resources, Solver, TaskState};
use crate::Error;
use common::{ResourceAlloc, TaskRequirements};

use priority_queue::PriorityQueue;
use rust_gpu_tools::opencl::GPUSelector;
use std::cmp::Reverse;

pub struct GreedySolver;
use std::sync::atomic::Ordering;

pub fn find_idle_gpus(resources: &'_ Resources) -> impl Iterator<Item = GPUSelector> + Clone + '_ {
    resources
        .0
        .iter()
        .filter(|(_, res)| !res.is_busy())
        .map(|(id, _)| *id)
}

impl Solver for GreedySolver {
    fn allocate_task(
        &mut self,
        resources: &Resources,
        requirements: &TaskRequirements,
        restrictions: &Option<Vec<GPUSelector>>,
    ) -> Option<(ResourceAlloc, HashMap<GPUSelector, ResourceState>)> {
        // Use heuristic criteria for picking up a resource depending on task requirements
        // basing on the current resource load or even a greedy approach. For now we just take the
        // first that match and return

        let device_restrictions = restrictions
            .clone()
            .unwrap_or_else(|| resources.0.keys().copied().collect::<Vec<GPUSelector>>());

        let idle_gpus_iter = find_idle_gpus(resources);
        let mut options = vec![];
        for req in requirements.req.iter() {
            let quantity = req.quantity;
            // check if the pool of devices have room for the requested allocations
            let optional_resources = resources
                .get_devices_with_requirements(req)
                .filter(|b| device_restrictions.iter().any(|x| x == b))
                .collect::<Vec<GPUSelector>>();
            let idle_gpus_available = optional_resources
                .iter()
                .cloned()
                .filter(|b| idle_gpus_iter.clone().any(|x| x == *b))
                .collect::<Vec<GPUSelector>>();

            //if there's enough idle gpus just offer those directly
            if idle_gpus_available.len() >= quantity {
                options = vec![(idle_gpus_available, req.clone())];
                break;
            } else if optional_resources.len() >= quantity {
                //otherwise fallback to non-idle resources
                options.push((optional_resources, req.clone()));
                break;
            }
        }

        // Make a new resource state, that the caller will use for updating the main resource state
        let mut resources = resources.0.clone();
        if !options.is_empty() {
            let selected_req = options[0].1.clone();
            // it is here where we can use some heuristic approach to select the best devices
            let selected_resources = options[0]
                .0
                .iter()
                .cloned()
                .take(selected_req.quantity as usize)
                .collect::<Vec<_>>();
            selected_resources.iter().for_each(|id| {
                let _ = resources
                    .get_mut(id)
                    //allocate memory
                    .map(|dev| dev.update_memory_usage(&selected_req.resource));
            });
            return Some((
                ResourceAlloc {
                    requirement: selected_req,
                    devices: selected_resources,
                },
                resources,
            ));
        }
        None
    }

    fn solve_job_schedule(
        &mut self,
        input: &HashMap<u32, TaskState>,
        scheduler_settings: &Settings,
    ) -> Result<VecDeque<u32>, Error> {
        // Criterion A; If the job is marked as stalled, it will be moved at the end of the queue.
        //
        // Criterion B: Use task deadline as a priority indicator. The sooner the deadline the higher
        //      the priority
        //
        // Criterion C: to avoid ties, we also use the time at which the task was created in the
        //      scheduler. to ensure a partial ordering among task that might have the same deadline.

        let mut priority_queue = PriorityQueue::new();

        // iterate our tasks for making the triplet pushing it into the queue
        for (job_id, state) in input.iter() {
            // stalled jobs are moved to the back of the queue, but the resource(s) it is using
            // will remain marked as busy, blocking other task from using the resource(s) and continue,
            // which will lead to a kind of deadlock.
            let is_stalled = Reverse(task_is_stalled(
                state.last_seen.load(Ordering::Relaxed),
                state.requirements.task_type,
                scheduler_settings,
            ));

            // get the jobs deadline or fake a new one.
            let deadline = state.requirements.deadline.map_or(i64::MAX, |d| {
                let start = d.start_timestamp_secs();
                let end = d.end_timestamp_secs();
                end.checked_sub(start).unwrap_or(i64::MAX)
            });
            let conditions = (is_stalled, Reverse(deadline), Reverse(state.creation_time));
            priority_queue.push(job_id, conditions);
        }

        Ok(priority_queue
            .into_sorted_iter()
            .map(|(i, _)| *i)
            .collect::<VecDeque<u32>>())
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
            let is_stalled = Reverse(false);
            let creation_time: u64 = chrono::Utc::now().timestamp() as u64;
            let conditions = (is_stalled, finish_time, Reverse(creation_time));
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
    fn test_is_stalled_priority() {
        let mut priority_queue = PriorityQueue::new();
        let mut res = vec![];
        for i in 0..5 {
            res.push(i);
            let finish_time = Reverse(10);
            // job 1 is stalled so it should be at the end of the queue
            let is_stalled = Reverse(i == 1);
            let creation_time: u64 = chrono::Utc::now().timestamp() as u64;
            let conditions = (is_stalled, finish_time, Reverse(creation_time));
            priority_queue.push(i, conditions);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        let queue = priority_queue
            .into_sorted_iter()
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        assert_eq!(res[1], queue[queue.len() - 1]);
    }

    #[test]
    fn test_finish_time_priority() {
        let mut priority_queue = PriorityQueue::new();
        let mut res = vec![];
        for i in 0..5 {
            res.push(i);
            let finish_time = Reverse(5 - i);
            let is_stalled = Reverse(false);
            let creation_time: u64 = chrono::Utc::now().timestamp() as u64;
            let conditions = (is_stalled, finish_time, Reverse(creation_time));
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
