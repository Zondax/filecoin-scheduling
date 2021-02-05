use std::collections::VecDeque;
use std::path::PathBuf;

use crate::handler::Handler;
use crate::requests::{SchedulerRequest, SchedulerResponse};
use common::{Error, RequestMethod, ResourceAlloc, TaskRequirements};

pub(crate) struct Scheduler {
    _task_queue: VecDeque<SchedulerRequest>,
    _state: PathBuf,
}

impl Scheduler {
    pub fn new<T: Into<PathBuf>>(state: T) -> Self {
        let _task_queue = VecDeque::new();
        Self {
            _task_queue,
            _state: state.into(),
        }
    }

    fn schedule(&self, requirements: TaskRequirements) -> SchedulerResponse {
        if requirements.req.is_empty() {
            return SchedulerResponse::Schedule(Err(Error::ResourceReqEmpty));
        }
        let alloc = ResourceAlloc {
            // For now just use the first req
            resource: requirements.req[0].clone(),
            resource_id: 0,
        };
        // A simple echo
        SchedulerResponse::Schedule(Ok(alloc))
    }
}

impl Handler for Scheduler {
    fn process_request(&self, request: SchedulerRequest) {
        let sender = request.sender;
        let response = match request.method {
            RequestMethod::Schedule(s) => self.schedule(s),
            RequestMethod::SchedulePreemptive(s) => SchedulerResponse::SchedulePreemptive(s),
            RequestMethod::WaitPreemptive(client, timeout) => self.wait_preemptive(client, timeout),
        };
        let _ = sender.send(response);
    }
}
