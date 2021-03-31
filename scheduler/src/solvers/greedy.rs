use std::collections::{HashMap, VecDeque};

use crate::solver::{ResourceState, Resources, Solver, TaskState};
use common::{Error, ResourceAlloc, ResourceMemory, ResourceType, TaskRequirements};

pub struct GreedySolver;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
//
// #[derive(Clone, Debug)]
// pub struct TaskState {
//     pub requirements: TaskRequirements,
//     pub current_iteration: u16,
//     // The list of jobs associates with this task, each job is a requirement plus the resource
//     // assigned to it accordingly.
//     pub allocation: ResourceAlloc,
// }pub resource_id: Vec<u32>,
//
// pub struct ResourceState {
//     /// Index that points to the Device.
//     pub dev: Device,
//     /// Current memory in use
//     pub mem_usage: u64,
//     /// Mark device as exclusive
//     pub is_exclusive: bool,
//     /// Using resource?
//     pub is_busy: bool,
// }

pub fn find_idle_gpus(resources: &Resources) -> Vec<u32>{
    resources.0.iter().filter(|res|res.is_busy == false).map(|res|res.dev.bus_id()).collect::<Vec<u32>>()
}
//
// pub fn find_device_allocs(input: &HashMap<u32, TaskState>) -> Vec<u32>{
//     resources.0.iter().filter(|res|res.is_busy == false).map(|res|res.dev.bus_id()).collect::<Vec<u32>>()
// }

impl Solver for GreedySolver {
    fn allocate_task(
        &mut self,
        input: &HashMap<u32, TaskState>,
        resources: &Resources,
        requirements: &TaskRequirements,
    ) -> Option<(
        ResourceAlloc,
        std::collections::HashMap<usize, ResourceState>,
    )> {
        // Use heuristic criteria for picking up a resource depending on task requirements
        // basing on the current resource load or even a greedy approach. For now we just take the
        // first that match and return

        let idle_gpus = find_idle_gpus(resources);
        // Make a new resource state, that the caller will use for updating the main resource state
        let mut resources = resources.0.clone();
        //let mut resources: Vec<ResourceState> = resources.0.clone().iter().filter(|&r| r.is_exclusive == requirements.exclusive).into().collect();
        let mut priority_queue = PriorityQueue::new();
        for req in requirements.req.iter() {
            let quantity = req.quantity;
            // check if the pool of devices have room for the requested allocations
            let mut selected_resources = resources
                .iter_mut()
                .filter(|(_, r)| r.is_exclusive == requirements.exclusive)
                .filter_map(|(index, device)| {
                    if let ResourceType::Gpu(ref mem) = req.resource {
                        match mem {
                            ResourceMemory::All => {
                                // All the memory means taking ownership of the device being also
                                // not preemptable. TODO: Should device be marked as no_shareable?
                                if device.mem_usage == 0 {
                                    Some((index, device.dev.bus_id()))
                                } else {
                                    None
                                }
                            }
                            ResourceMemory::Mem(value) => {
                                if device.available_memory() >= *value {
                                    Some((index,device.dev.bus_id()))
                                } else {
                                    None
                                }
                            }
                        }
                    } else {
                        None
                    }
                }).collect::<Vec<(usize,u32)>>();;
            let mut idle_gpus_available = selected_resources.clone().iter().filter(|(_,b)| idle_gpus.iter().any(|x| x==b)).map(|(i,_)|*i).collect::<Vec<usize>>();
            idle_gpus_available.truncate(quantity as usize);
            if idle_gpus_available.len() == quantity {
                // we need to update our resource increasing memory usage for the selected devices
                let resource_id = idle_gpus_available
                    .into_iter()
                    .map(|index| {
                        let resource = &mut resources[index];
                        resource.update_memory_usage(&req.resource);
                        resource.dev.bus_id()
                    })
                    .collect::<Vec<u32>>();
                return Some((
                    ResourceAlloc {
                        requirement: req.clone(),
                        resource_id: selected_resources,
                    },
                    resources,
                ));
            }else if selected_resources.len() >= quantity {
                priority_queue.push(selected_resources,0);
            }
        }
        if !priority_queue.is_empty(){

        }
        None
    }

    // fn allocate_task(
    //     &mut self,
    //     resources: &Resources,
    //     requirements: &TaskRequirements,
    // ) -> Option<(ResourceAlloc, Vec<ResourceState>)> {
    //     // Use heuristic criteria for picking up a resource depending on task requirements
    //     // basing on the current resource load or even a greedy approach. For now we just take the
    //     // first that match and return
    //
    //     // Make a new resource state, that the caller will use for updating the main resource state
    //     let mut resources = resources.0.clone();
    //     //let mut resources: Vec<ResourceState> = resources.0.clone().iter().filter(|&r| r.is_exclusive == requirements.exclusive).into().collect();
    //
    //     for req in requirements.req.iter() {
    //         let quantity = req.quantity;
    //         // check if the pool of devices have room for the requested allocations
    //         let mut selected_resources = resources
    //             .iter_mut()
    //             .enumerate()
    //             .filter(|(_, r)| r.is_exclusive == requirements.exclusive)
    //             .filter_map(|(index, device)| {
    //                 if let ResourceType::Gpu(ref mem) = req.resource {
    //                     match mem {
    //                         ResourceMemory::All => {
    //                             // All the memory means taking ownership of the device being also
    //                             // not preemptable. TODO: Should device be marked as no_shareable?
    //                             if device.mem_usage == 0 {
    //                                 Some(index)
    //                             } else {
    //                                 None
    //                             }
    //                         }
    //                         ResourceMemory::Mem(value) => {
    //                             if device.available_memory() >= *value {
    //                                 Some(index)
    //                             } else {
    //                                 None
    //                             }
    //                         }
    //                     }
    //                 } else {
    //                     None
    //                 }
    //             })
    //             .collect::<Vec<usize>>();
    //         selected_resources.truncate(quantity as usize);
    //         if selected_resources.len() == quantity {
    //             // we need to update our resource increasing memory usage for the selected devices
    //             let resource_id = selected_resources
    //                 .into_iter()
    //                 .map(|index| {
    //                     let resource = &mut resources[index];
    //                     resource.update_memory_usage(&req.resource);
    //                     resource.dev.bus_id()
    //                 })
    //                 .collect::<Vec<u32>>();
    //             return Some((
    //                 ResourceAlloc {
    //                     requirement: req.clone(),
    //                     resource_id,
    //                 },
    //                 resources,
    //             ));
    //         }
    //     }
    //     None
    // }

    fn solve_job_schedule(
        &mut self,
        input: &HashMap<u32, TaskState>,
    ) -> Result<VecDeque<u32>, Error> {
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
