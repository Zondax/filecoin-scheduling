use crate::requests::{RequestMethod, SchedulerRequest, SchedulerResponse};
use crate::scheduler::Scheduler;
use tracing::warn;

pub trait Handler: Send + Sync + 'static {
    fn process_request(&self, request: SchedulerRequest);

    // Perform  a maintenance iteration on this handler instance.
    // returns True if the maintenance cycle should continue, otherwise it will be closed
    fn maintenance(&self) -> bool {
        false
    }
}

impl Handler for Scheduler {
    fn process_request(&self, request: SchedulerRequest) {
        // TODO: Analyze if spawning a thread is worth considering that doing so the handler's
        // Executor doesnt get blocked by this intensive operation
        let sender = request.sender;
        let response = match request.method {
            RequestMethod::Schedule(client, req) => {
                SchedulerResponse::Schedule(self.schedule(client, req))
            }
            RequestMethod::ListAllocations => self.list_allocations(),
            RequestMethod::WaitPreemptive(client) => {
                SchedulerResponse::SchedulerWaitPreemptive(self.wait_preemptive(client))
            }
            RequestMethod::Release(client) => {
                self.release(client);
                SchedulerResponse::Release
            }
            RequestMethod::ReleasePreemptive(client) => {
                self.release_preemptive(client);
                SchedulerResponse::ReleasePreemptive
            }
            RequestMethod::Abort(client_id) => SchedulerResponse::Abort(self.abort(client_id)),
            RequestMethod::RemoveStalled(client_id) => {
                SchedulerResponse::RemoveStalled(self.remove_stalled(client_id))
            }
            RequestMethod::Monitoring => SchedulerResponse::Monitoring(self.monitor()),
            RequestMethod::CheckService => SchedulerResponse::CheckService(self.pid),
        };
        let _ = sender.send(response);
    }

    fn maintenance(&self) -> bool {
        let mut _continue = true;
        // remove jobs that no longer exist in the system.
        let mut to_remove = vec![];
        for id in self.jobs_queue.read().iter() {
            if !self.check_process_exist(*id) {
                warn!("Removing job {}. Parent process does not exist", id);
                to_remove.push(*id);
            }
        }

        for id in to_remove.into_iter() {
            self.remove_job(id);
        }

        if let Some(shutdown_timeout) = self.settings.service.shutdown_timeout {
            if self.shutdown_tracker.read().elapsed().as_secs() > shutdown_timeout {
                warn!("Closing service after {}s of inactivity", shutdown_timeout);
                _continue = false;
            }
        }
        _continue
    }
}
