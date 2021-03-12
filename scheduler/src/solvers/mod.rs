#[cfg(feature = "greedy_solver")]
mod greedy;
#[cfg(feature = "mip_solver")]
mod linearsolver;
#[cfg(feature = "greedy_solver")]
pub use greedy::GreedySolver;
#[cfg(feature = "mip_solver")]
pub use linearsolver::{
    JobAllocation, JobConstraint, JobDescription, JobPlan, JobRequirements, LinearSolverModel,
};

use crate::solver::Solver;
#[cfg(feature = "mip_solver")]
use common::TaskRequirements;
use common::{Config, Error};

/// Wrapper struct for converting from TaskRequirements to
/// JobRequirements
#[cfg(feature = "mip_solver")]
pub struct RequirementsMap {
    pub reqs: TaskRequirements,
    // the available resources to use
    pub resources: Vec<u32>,
    pub job_id: usize,
    pub preemptive: Option<usize>,
    pub has_started: Option<(usize, usize)>,
}

#[cfg(feature = "mip_solver")]
impl From<RequirementsMap> for JobRequirements {
    fn from(map: RequirementsMap) -> Self {
        let options = map
            .resources
            .iter()
            .map(|id| JobConstraint {
                machine: *id as usize,
                duration: map.reqs.exec_time.as_secs() as usize,
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
            is_support: false,
            job_id: map.job_id,
        };

        JobRequirements {
            jobs: vec![description],
            sequences: vec![],
            supports: vec![],
        }
    }
}

// Remove later this option, Config will have a default value, use it
#[cfg(feature = "mip_solver")]
pub fn create_solver(_config: Option<&Config>) -> Result<Box<dyn Solver>, Error> {
    Ok(Box::new(LinearSolverModel::new()))
}

#[cfg(feature = "greedy_solver")]
pub fn create_solver(_config: Option<&Config>) -> Result<Box<dyn Solver>, Error> {
    Ok(Box::new(GreedySolver))
}
