use std::collections::{HashMap, VecDeque};

use crate::solver::{ResourceState, Resources, Solver, TaskState};
use common::{Error, ResourceAlloc, ResourceMemory, ResourceType, TaskRequirements};

pub struct GreedySolver;

impl Solver for GreedySolver {
    fn allocate_task(
        &mut self,
        resources: &Resources,
        requirements: &TaskRequirements,
    ) -> Option<(ResourceAlloc, Vec<ResourceState>)> {
        // Use heuristic criteria for picking up a resource depending on task requirements
        // basing on the current resource load or even a greedy approach. For now we just take the
        // first that match and return

        // Make a new resource state, that the caller will use for updating the main resource state
        let mut resources = resources.0.clone();
        //let mut resources: Vec<ResourceState> = resources.0.clone().iter().filter(|&r| r.is_exclusive == requirements.exclusive).into().collect();

        for req in requirements.req.iter() {
            let quantity = req.quantity;
            // check if the pool of devices have room for the requested allocations
            let mut selected_resources = resources
                .iter_mut()
                .enumerate()
                .filter(|(_, r)| r.is_exclusive == requirements.exclusive)
                .filter_map(|(index, device)| {
                    if let ResourceType::Gpu(ref mem) = req.resource {
                        match mem {
                            ResourceMemory::All => {
                                // All the memory means taking ownership of the device being also
                                // not preemptable. TODO: Should device be marked as no_shareable?
                                if device.mem_usage == 0 {
                                    Some(index)
                                } else {
                                    None
                                }
                            }
                            ResourceMemory::Mem(value) => {
                                if device.available_memory() >= *value {
                                    Some(index)
                                } else {
                                    None
                                }
                            }
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>();
            selected_resources.truncate(quantity as usize);
            if selected_resources.len() == quantity {
                // we need to update our resource increasing memory usage for the selected devices
                let resource_id = selected_resources
                    .into_iter()
                    .map(|index| {
                        let resource = &mut resources[index];
                        resource.update_memory_usage(&req.resource);
                        resource.dev.device_id()
                    })
                    .collect::<Vec<usize>>();
                return Some((
                    ResourceAlloc {
                        requirement: req.clone(),
                        resource_id,
                    },
                    resources,
                ));
            }
        }
        None
    }

    fn solve_job_schedule(
        &mut self,
        input: &HashMap<u32, TaskState>,
    ) -> Result<VecDeque<u32>, Error> {
        use priority_queue::PriorityQueue;
        use std::cmp::Reverse;
        //Criteria A: Use task end time as a priority indicator. The sooner the deadline the higher
        //the priority
        //
        //Criteria B: As a greedy algorithm, prioritize those that are consuming more resources in
        //term of memory and quantity
        //So that, we have a triplet (timestamp, memory_consumption, num_resources) as criteria for prioritazing

        // Use this handy crate that sort out our tasks
        let mut priority_queue = PriorityQueue::new();

        // iterate our tasks for making the triplet pushing it into the queue
        for (job_id, state) in input.iter() {
            // Intead of Reverse we can do something like deadline.end - chronos::now()?
            let condition = state
                .requirements
                .deadline
                .map_or(i64::MAX, |d| d.end_timestamp_secs());
            let finish_time = Reverse(condition);
            let mem_usage = match &state.allocation.requirement.resource {
                ResourceType::Gpu(mem) => match mem {
                    // This device is not preemptable so we put this at the end of our priority
                    // queue. Another option is removing it from the queue.
                    ResourceMemory::All => u64::MAX,
                    ResourceMemory::Mem(value) => {
                        value * state.allocation.requirement.quantity as u64
                    }
                },
                ResourceType::Cpu => unimplemented!("We handle just Gpu resources"),
            };
            let triplet = (
                finish_time,
                mem_usage,
                state.allocation.requirement.quantity,
            );
            priority_queue.push(job_id, triplet);
        }

        Ok(priority_queue
            .into_sorted_iter()
            .map(|(i, _)| *i)
            .collect::<VecDeque<u32>>())
    }
}
