mod linearsolver;
use crate::solver::Solver;
use common::{Config, Error, TaskRequirements};
pub use linearsolver::{
    JobAllocation, JobConstraint, JobDescription, JobPlan, JobRequirements, LinearSolverModel,
};

/// Wrapper struct for converting from TaskRequirements to
/// JobRequirements
pub struct RequirementsMap {
    pub reqs: TaskRequirements,
    // the available resources to use
    pub resources: Vec<u32>,
    pub job_id: usize,
    pub preemptive: Option<usize>,
    pub has_started: Option<(usize, usize)>,
}

#[allow(clippy::from_over_into)]
impl Into<JobRequirements> for RequirementsMap {
    fn into(self) -> JobRequirements {
        let options = self
            .resources
            .iter()
            .map(|id| JobConstraint {
                machine: *id as usize,
                duration: self.reqs.exec_time.as_secs() as usize,
            })
            .collect::<_>();
        let description = JobDescription {
            options,
            // We have a deadline wich contains the start/end times but need to
            // check corner cases or how they are going to be interpreted by the solver
            starttime: None,
            deadline: None,
            preemptive: None,
            has_started: None,
            job_id: self.job_id,
        };

        JobRequirements {
            jobs: vec![description],
            sequences: vec![],
        }
    }
}

// Remove later this option, Config will have a default value, use it
pub fn create_solver(_config: Option<&Config>) -> Result<Box<dyn Solver>, Error> {
    Ok(Box::new(LinearSolverModel::new()))
}
