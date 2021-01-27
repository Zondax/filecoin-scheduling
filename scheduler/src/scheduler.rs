use std::collections::VecDeque;
use std::path::PathBuf;

use crate::handler::Handler;
use crate::requests::{RequestMethod, SchedulerRequest, SchedulerResponse};

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

    fn schedule(&self, task: String) -> SchedulerResponse {
        // A simple echo
        SchedulerResponse::Schedule(task)
    }
}

impl Handler for Scheduler {
    fn process_request(&self, request: SchedulerRequest) {
        let sender = request.sender;
        let response = match request.method {
            RequestMethod::Schedule(s) => self.schedule(s),
            RequestMethod::SchedulePreemptive(s) => SchedulerResponse::SchedulePreemptive(s),
        };
        let _ = sender.send(response);
    }
}
