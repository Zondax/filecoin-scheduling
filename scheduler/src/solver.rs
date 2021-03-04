use crate::solvers::{JobPlan, JobRequirements};
use common::Error;

// Trait that is implemented by any object that can be used as a solver
pub trait Solver {
    fn solve_job_schedule(
        &mut self,
        input: JobRequirements,
        setup_time: usize,
        finish_time: usize,
        swapping_costs: Option<f64>,
    ) -> Result<JobPlan, Error>;
}
