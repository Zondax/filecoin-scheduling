mod linearsolver;
use crate::solver::Solver;
use common::{Config, Error, TaskRequirements};
#[cfg(feature = "mip_solver")]
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
            job_id: map.job_id,
        };

        JobRequirements {
            jobs: vec![description],
            sequences: vec![],
        }
    }
}

// Remove later this option, Config will have a default value, use it
pub fn create_solver(_config: Option<&Config>) -> Result<Box<dyn Solver>, Error> {
    #[cfg(feature = "mip_solver")]
    Ok(Box::new(LinearSolverModel::new()))
}
