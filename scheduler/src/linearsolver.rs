#![allow(dead_code, unused_imports)]
use array_tool::vec::Intersect;
use coin_cbc::{raw::Status, Col, Model, Sense, Solution};
use common::Error;
use more_asserts::*;

const ERROR_MARGIN: f64 = 1e-20;

pub type JobIndex = usize;

#[derive(Clone, Debug, PartialEq)]
pub struct JobConstraint {
    pub machine: usize,
    pub duration: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JobAllocation {
    pub machine: usize,
    pub starting_time: usize,
    pub end_time: usize,
    pub job_id: usize,
}

/* Notes:
* if a job is preemtive, all jobs take same time on all possible machines, so we take options[0].duration as total duration
* if a job is preemtive, then the number inside the option denotes the number of pieces ( >= 2)
* if has_started is some: the numbers inside it is the allocated machine and the minimal number of time it should continue before interupt (can be 0)
 */

#[derive(Clone, Debug, PartialEq)]
pub struct JobDescription {
    pub options: Vec<JobConstraint>,
    pub starttime: Option<usize>,
    pub deadline: Option<usize>,
    pub preemtive: Option<usize>,
    pub has_started: Option<(usize, usize)>,
    pub job_id: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JobRequirements {
    pub jobs: Vec<JobDescription>,
    pub sequences: Vec<(JobIndex, JobIndex)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JobPlan {
    pub makespan: usize,                  //total execution time
    pub idletime: usize,                  //todal idle time
    pub machine_finish_times: Vec<usize>, //finishing time per machine
    pub plan: Vec<JobAllocation>,
}

impl JobPlan {
    #[allow(clippy::suspicious_operation_groupings)]
    pub fn is_valid(&self, reqs: &JobRequirements) -> bool {
        let n = self.plan.len();
        let allocs = self.plan.clone();
        for i in 0..n {
            let job_i = allocs[i].clone();
            if job_i.end_time > self.makespan {
                return false;
            }
            if reqs.jobs[i].deadline.is_some() && job_i.end_time > reqs.jobs[i].deadline.unwrap() {
                return false;
            }

            if reqs.jobs[i].starttime.is_some()
                && job_i.starting_time < reqs.jobs[i].starttime.unwrap()
            {
                return false;
            }
            for (j, job_j) in allocs.iter().enumerate() {
                if i != j
                    && job_i.machine == job_j.machine
                    && (job_i.starting_time <= job_j.starting_time
                        && job_j.starting_time < job_i.end_time)
                {
                    return false;
                }
                if reqs.sequences.iter().any(|(k, l)| &i == k && &j == l)
                    && job_i.end_time > job_j.starting_time
                {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(Default)]
struct LinearSolverModel {
    pub m: Model,
    pub jobs_data: Vec<JobDescription>,
    pub columns: Vec<Col>,
    pub big_num: f64,
    pub num_machines: usize,
    pub indexes_sv: Vec<usize>,
    pub indexes_ev: Vec<usize>,

    pub swapping_costs: f64,
}

impl LinearSolverModel {
    /// This function initializes the Linear Model solver
    /// It sets the target for the makespan (the total execution time)
    /// The objective is to minimize that
    pub fn initialize(input: &[JobDescription], swapping_costs: f64) -> LinearSolverModel {
        let mut m = Model::default();
        let mut columns = vec![];

        //makespan = col[0] >= 0
        columns.push(m.add_integer());
        let row = m.add_row();
        m.set_row_lower(row, 0.0);
        m.set_weight(row, columns[0], 1.0);
        m.set_obj_coeff(columns[0], 1.);
        // Set objective sense.
        m.set_obj_sense(Sense::Minimize);
        LinearSolverModel {
            m,
            jobs_data: input.to_owned(),
            columns,
            big_num: 0.0,
            num_machines: 0,
            indexes_sv: vec![],
            indexes_ev: vec![],
            swapping_costs,
        }
    }

    /// This function calculates some upper bounds we need a lot later
    /// The number of machines available
    /// A big upper bound on all execution times we need for the linear solver later
    pub fn find_upper_bounds(&mut self) {
        let mut num_machines: usize = 0;
        let mut big_num: f64 = 0.;
        for job in self.jobs_data.iter() {
            for constraint in job.options.clone() {
                big_num += constraint.duration as f64;
                if constraint.machine > num_machines {
                    num_machines = constraint.machine;
                }
            }
        }
        num_machines += 1;
        self.num_machines = num_machines;
        self.big_num = big_num + 100000.;
    }

    /// This function adds dummy starting and dummy finishing jobs
    /// We need this mostly for the solver
    /// But we can also set setup times / finish times if needed (both can be 0)
    pub fn add_dummy_jobs(&mut self, setup_time: usize, finish_time: usize) {
        let num_machines = self.num_machines;
        let mut index: usize = 0;
        for machine in (0..num_machines).rev() {
            self.jobs_data.insert(
                0,
                JobDescription {
                    options: vec![JobConstraint {
                        machine: machine as usize,
                        duration: setup_time,
                    }],
                    deadline: None,
                    starttime: None,
                    preemtive: None,
                    has_started: None,
                    job_id: 100 + index,
                },
            );
            index += 1;
        }
        for machine in 0..num_machines {
            self.jobs_data.push(JobDescription {
                options: vec![JobConstraint {
                    machine: machine as usize,
                    duration: finish_time,
                }],
                deadline: None,
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 100 + index,
            });
            index += 1;
        }
    }

    pub fn add_general_job_constraints(
        &mut self,
        job: &JobDescription,
        dummy_start: bool,
        preemtive: bool,
    ) -> Option<usize> {
        let num_machines = self.num_machines;
        let index = self.m.num_cols() as usize;
        for _ in 0..num_machines {
            self.columns.push(self.m.add_binary());
        }
        let index_sv = self.m.num_cols() as usize;
        self.indexes_sv.push(index_sv);
        self.columns.push(self.m.add_integer()); //s_v
        let index_ev = self.m.num_cols() as usize;
        self.indexes_ev.push(index_ev);
        self.columns.push(self.m.add_integer()); //e_v
        let index_pv = self.m.num_cols() as usize;
        self.columns.push(self.m.add_integer()); //p_v

        let row_onemachine = self.m.add_row();

        //sum_{k} x_{k,w} == 1
        //sum_{k} x_{k,w} * dur - pv_w == 0
        for machine in 0..num_machines {
            let options = job.options.clone();
            for option in options {
                if option.machine == machine {
                    self.m
                        .set_weight(row_onemachine, self.columns[index + machine], 1.0);
                }
            }
        }

        self.m.set_row_equal(row_onemachine, 1.0);

        //s_v >= 0
        let mut row = self.m.add_row();
        self.m.set_row_lower(row, 0.0);
        self.m.set_weight(row, self.columns[index_sv], 1.0);
        if dummy_start {
            self.m.set_row_upper(row, 0.0);
        }

        //e_v >= 0
        row = self.m.add_row();
        self.m.set_row_lower(row, 0.0);
        self.m.set_weight(row, self.columns[index_ev], 1.0);

        //p_v >= 0
        row = self.m.add_row();
        self.m.set_row_lower(row, 0.0);
        self.m.set_weight(row, self.columns[index_pv], 1.0);

        //-ev + sv + pv == 0
        row = self.m.add_row();
        self.m.set_row_lower(row, 0.0);
        self.m.set_row_upper(row, 0.0);
        self.m.set_weight(row, self.columns[index_ev], -1.0);
        self.m.set_weight(row, self.columns[index_sv], 1.0);
        self.m.set_weight(row, self.columns[index_pv], 1.0);

        //e_v - makespan <= 0
        row = self.m.add_row();
        self.m.set_row_upper(row, 0.0);
        self.m.set_weight(row, self.columns[0], -1.0);
        self.m.set_weight(row, self.columns[index_ev], 1.0);

        //e_v <= deadline
        if job.deadline.is_some() {
            row = self.m.add_row();
            self.m.set_row_upper(row, job.deadline.unwrap() as f64);
            self.m.set_weight(row, self.columns[index_ev], 1.0);
        }

        //s_v >= starttime
        if job.starttime.is_some() {
            row = self.m.add_row();
            self.m.set_row_lower(row, job.starttime.unwrap() as f64);
            self.m.set_weight(row, self.columns[index_sv], 1.0);
        }

        if !preemtive {
            let row_onetime = self.m.add_row();
            for machine in 0..num_machines {
                let options = job.options.clone();
                for option in options {
                    if option.machine == machine {
                        self.m.set_weight(
                            row_onetime,
                            self.columns[index + machine],
                            option.duration as f64,
                        );
                    }
                }
            }
            self.m.set_weight(row_onetime, self.columns[index_pv], -1.0);
            self.m.set_row_equal(row_onetime, 0.0);

            if job.has_started.is_some() {
                //start first part at time 0
                row = self.m.add_row();
                self.m.set_row_equal(row, 0.0);
                self.m.set_weight(row, self.columns[index_sv], 1.0);

                let (machine, min_time) = job.has_started.unwrap();

                //start first part at time 0
                row = self.m.add_row();
                self.m.set_row_lower(row, min_time as f64);
                self.m.set_weight(row, self.columns[index_ev], 1.0);

                //set first part machine at machine k
                row = self.m.add_row();
                self.m.set_row_equal(row, 1.0);
                self.m.set_weight(row, self.columns[index + machine], 1.0);
            }
        }

        if preemtive {
            Some(index_pv)
        } else {
            None
        }
    }

    /// In this function we set some constraints per job
    /// Mainly:
    /// - A job is allocated to one machine only
    /// - A job is only allocated to machines it is allowed to be processed
    /// - Starting time + processing time == ending time
    /// - If a job has a deadline, then ending_time <= deadline
    pub fn add_constraints_per_job(&mut self) {
        let num_machines = self.num_machines;
        let jobdata = self.jobs_data.clone();
        let mut preemt_job_index = 0;
        for (i, job) in jobdata.iter().enumerate() {
            if job.preemtive.is_some() {
                let num_preemtive = job.preemtive.unwrap();
                let indexes_index = self.indexes_ev.len();

                let mut indexes_preemt_pv = vec![];
                for preempt_index in 0..num_preemtive {
                    self.jobs_data.insert(
                        preemt_job_index + i + 1,
                        JobDescription {
                            options: job.options.clone(),
                            starttime: None,
                            deadline: None,
                            preemtive: None,
                            has_started: None,
                            job_id: job.job_id,
                        },
                    );
                    let index = self.m.num_cols() as usize;
                    let index_pv = self.add_general_job_constraints(job, false, true).unwrap();
                    indexes_preemt_pv.push(index_pv);
                    let mut row;
                    if preempt_index == 0 && job.has_started.is_some() {
                        //start first part at time 0
                        row = self.m.add_row();
                        self.m.set_row_equal(row, 0.0);
                        self.m.set_weight(
                            row,
                            self.columns[self.indexes_sv[indexes_index + preempt_index]],
                            1.0,
                        );

                        let (machine, min_time) = job.has_started.unwrap();

                        //start first part at time 0
                        row = self.m.add_row();
                        self.m.set_row_lower(row, min_time as f64);
                        self.m.set_weight(
                            row,
                            self.columns[self.indexes_ev[indexes_index + preempt_index]],
                            1.0,
                        );

                        //set first part machine at machine k
                        row = self.m.add_row();
                        self.m.set_row_equal(row, 1.0);
                        self.m.set_weight(row, self.columns[index + machine], 1.0);
                    }

                    if preempt_index > 0 {
                        //s_v_i <= s_v_{i+1}
                        row = self.m.add_row();
                        self.m.set_row_upper(row, 0.0);
                        self.m.set_weight(
                            row,
                            self.columns[self.indexes_sv[indexes_index + preempt_index - 1]],
                            1.0,
                        );
                        self.m.set_weight(
                            row,
                            self.columns[self.indexes_sv[indexes_index + preempt_index]],
                            -1.0,
                        );

                        //e_v_i <= e_v_{i+1}
                        row = self.m.add_row();
                        self.m.set_row_upper(row, 0.0);
                        self.m.set_weight(
                            row,
                            self.columns[self.indexes_ev[indexes_index + preempt_index - 1]],
                            1.0,
                        );
                        self.m.set_weight(
                            row,
                            self.columns[self.indexes_ev[indexes_index + preempt_index]],
                            -1.0,
                        );

                        //e_v_i <= s_v_{i+1}
                        row = self.m.add_row();
                        self.m.set_row_upper(row, 0.0);
                        self.m.set_weight(
                            row,
                            self.columns[self.indexes_ev[indexes_index + preempt_index - 1]],
                            1.0,
                        );
                        self.m.set_weight(
                            row,
                            self.columns[self.indexes_sv[indexes_index + preempt_index]],
                            -1.0,
                        );
                    }
                }
                //p1 + p2 + ... = p
                let row = self.m.add_row();
                self.m.set_row_equal(row, job.options[0].duration as f64);
                for preemt_index in indexes_preemt_pv {
                    self.m.set_weight(row, self.columns[preemt_index], 1.0);
                }
                self.jobs_data.remove(preemt_job_index + i);
                preemt_job_index += num_preemtive - 1;
            //               assert_eq!(self.jobs_data, jobdata);
            } else {
                let b = i < num_machines;
                self.add_general_job_constraints(job, b, false);
            }
        }
    }

    /// In this function we set the constraints for sequential jobs
    /// On input of a vector of indices (a,b), we constraint the model to process a before b
    /// Meaning: ending time of a <= starting time of b
    fn add_sequential_constraints(&mut self, sequences: &[(JobIndex, JobIndex)]) {
        let num_machines = self.num_machines;
        let columns = &self.columns;
        for (i, j) in sequences.iter() {
            let row = self.m.add_row();
            self.m.set_row_upper(row, 0.0);
            let index_i = num_machines + (*i);
            let index_j = num_machines + (*j);
            self.m
                .set_weight(row, columns[self.indexes_ev[index_i]], 1.0);
            self.m
                .set_weight(row, columns[self.indexes_sv[index_j]], -1.0);
        }
    }

    /// In this function we set the constraints per machine
    /// That is :
    /// - No more than one job at any time on a given machine
    /// - Every job has another job before it (except the dummy starting jobs)
    /// - Evey job has another job after it (except the dummy finishing jobs)
    pub fn add_constraints_per_machine(&mut self) {
        //
        let num_machines = self.num_machines;
        let num_jobs = self.jobs_data.len();
        let index = self.m.num_cols() as usize;
        for _ in 0..(num_machines * num_jobs * num_jobs) {
            self.columns.push(self.m.add_binary());
        }
        //w has a predecessor on machine k
        let mut row;
        for k in 0..num_machines {
            for w in num_machines..num_jobs {
                if self.jobs_data[w].options.iter().any(
                    |JobConstraint {
                         machine: i,
                         duration: _,
                     }| i == &k,
                ) {
                    row = self.m.add_row();
                    //\sum_{v | v \neq w} y_{k,v,w} - x_{k,w} == 0
                    for v in 0..num_jobs {
                        if v != w
                            && self.jobs_data[v].options.iter().any(
                                |JobConstraint {
                                     machine: i,
                                     duration: _,
                                 }| i == &k,
                            )
                        {
                            //&& jobs_data[v].0.intersect(jobs_data[w].0.clone()).len() > 0
                            self.m.set_weight(
                                row,
                                self.columns[index + k * num_jobs * num_jobs + v * num_jobs + w],
                                1.0,
                            );
                        }
                    }
                    self.m
                        .set_weight(row, self.columns[1 + w * (num_machines + 3) + k], -1.);
                    self.m.set_row_equal(row, 0.0);
                }
            }
            //v has a successor on machine k
            for v in 0..num_jobs - num_machines {
                if self.jobs_data[v].options.iter().any(
                    |JobConstraint {
                         machine: i,
                         duration: _,
                     }| i == &k,
                ) {
                    row = self.m.add_row();
                    for w in 0..num_jobs {
                        //\sum_{w | w \neq v} y_{k,v,w} - x_{k,v} == 0
                        if v != w
                            && self.jobs_data[w].options.iter().any(
                                |JobConstraint {
                                     machine: i,
                                     duration: _,
                                 }| i == &k,
                            )
                        {
                            self.m.set_weight(
                                row,
                                self.columns[index + k * num_jobs * num_jobs + v * num_jobs + w],
                                1.0,
                            );
                        }
                    }
                    self.m
                        .set_weight(row, self.columns[1 + v * (num_machines + 3) + k], -1.);
                    self.m.set_row_lower(row, 0.0);
                    self.m.set_row_upper(row, 0.0);
                }
            }
        }

        //if v and w processed on machine k, then they are not processed at the same times
        for v in 0..num_jobs {
            for w in 0..num_jobs {
                let machines = self.jobs_data[v]
                    .options
                    .intersect_if(self.jobs_data[w].options.clone(), |l, r| {
                        l.machine == r.machine
                    });
                let costs;
                if (v < num_machines || v >= num_jobs - num_machines)
                    || (w < num_machines || w >= num_jobs - num_machines)
                    || (self.jobs_data[v].job_id == self.jobs_data[w].job_id)
                {
                    costs = 0.0
                } else {
                    costs = self.swapping_costs;
                }
                if v != w && !machines.is_empty() {
                    //L*sum_{k : job_v(k) && job_w(k)} y_{k,v,w} + e_v - s_w <= L
                    row = self.m.add_row();
                    //e_v
                    self.m
                        .set_weight(row, self.columns[self.indexes_ev[v]], 1.0);
                    //-s_w
                    self.m
                        .set_weight(row, self.columns[self.indexes_sv[w]], -1.0);
                    for machine_index in machines {
                        self.m.set_weight(
                            row,
                            self.columns[index
                                + machine_index.machine * num_jobs * num_jobs
                                + v * num_jobs
                                + w],
                            self.big_num as f64 + costs,
                        );
                    }
                    self.m.set_row_upper(row, self.big_num as f64);
                }
            }
        }
    }

    ///This function executes the solver and checks for feasibility
    pub fn solve_and_check(&self) -> Result<Solution, Error> {
        let sol = self.m.solve();

        if Status::Finished != sol.raw().status() {
            return Err(Error::Other(
                "Solver did not find solution (conflicting constraints?)".to_string(),
            ));
        }

        let solution = sol.raw().obj_value() as usize;
        if solution > self.big_num as usize {
            return Err(Error::Other(
                "Solver did not find correct solution (conflicting constraints?)".to_string(),
            ));
        }
        Ok(sol)
    }

    ///This function transforms the solution into readable output (a Jobplan)
    pub fn make_jobplan(&self, sol: Solution) -> JobPlan {
        let num_jobs = self.jobs_data.len();
        let num_machines = self.num_machines;
        let indexes_sv = &self.indexes_sv;
        let indexes_ev = &self.indexes_ev;
        let columns = &self.columns;
        let solution = sol.raw().obj_value() as usize;
        let mut allocs = vec![];
        let mut processtimes: Vec<usize> = vec![0; num_machines];
        let mut finishtimes: Vec<usize> = vec![0; num_machines];
        for i in num_machines..(num_jobs - num_machines) {
            for k in 0..num_machines {
                let index = 1 + i * (num_machines + 3);
                if (sol.col(columns[index + k]).round() - 1.0).abs() < ERROR_MARGIN {
                    let starttime = sol.col(columns[indexes_sv[i]]).round() as usize;
                    let endtime = sol.col(columns[indexes_ev[i]]).round() as usize;
                    if endtime - starttime == 0 {
                        continue;
                    }
                    processtimes[k] += endtime - starttime;
                    if endtime > finishtimes[k] {
                        finishtimes[k] = endtime;
                    }
                    allocs.push(JobAllocation {
                        machine: k,
                        starting_time: starttime,
                        end_time: endtime,
                        job_id: self.jobs_data[i].job_id,
                    });
                }
            }
        }
        let mut total_idle_time = 0;
        for k in 0..num_machines {
            total_idle_time += finishtimes[k] - processtimes[k];
        }

        JobPlan {
            makespan: solution,
            plan: allocs,
            machine_finish_times: finishtimes,
            idletime: total_idle_time,
        }
    }
}

pub fn solve_jobschedule(
    input: &JobRequirements,
    setup_time: usize,
    finish_time: usize,
    swapping_costs: Option<f64>,
) -> Result<JobPlan, Error> {
    let costs = swapping_costs.unwrap_or(0.0);
    let mut model: LinearSolverModel = LinearSolverModel::initialize(&input.jobs, costs);

    model.find_upper_bounds();

    model.add_dummy_jobs(setup_time, finish_time);

    model.add_constraints_per_job();

    model.add_sequential_constraints(&input.sequences);

    model.add_constraints_per_machine();

    // Solve the problem. Returns the solution if ok
    let sol = model.solve_and_check()?;

    Ok(model.make_jobplan(sol))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preemtive_jobs() {
        let jobs_data1 = vec![
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: Some(8),
                starttime: Some(5),
                preemtive: None,
                has_started: None,
                job_id: 0,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 20,
                }],
                deadline: None,
                starttime: None,
                preemtive: Some(3),
                has_started: Some((0, 2)),
                job_id: 1,
            },
        ];
        let reqs = JobRequirements {
            jobs: jobs_data1.clone(),
            sequences: vec![],
        };

        let result = solve_jobschedule(&reqs, 0, 0, Some(1.0));
        assert!(result.is_ok());
        let plan = result.unwrap();
        assert_eq!(plan.makespan, 25);
        assert_eq!(plan.plan[1].end_time, 4);
    }

    #[test]
    fn test_scenario() {
        //The time it takes to remove one job and load another is a system constant
        //We set this to 2 minutes

        //A long job of one hour comes in, with a deadline in 4 hours
        //No specific starttime, but it is preemtible

        let jobs_data_t0 = vec![JobDescription {
            options: vec![JobConstraint {
                machine: 0,
                duration: 60,
            }],
            deadline: Some(240),
            starttime: None,
            preemtive: None,
            has_started: None,
            job_id: 0,
        }];
        let reqs = JobRequirements {
            jobs: jobs_data_t0.clone(),
            sequences: vec![],
        };

        let result = solve_jobschedule(&reqs, 0, 0, Some(2.0));
        assert!(result.is_ok());
        let plan = result.unwrap();
        assert_eq!(plan.makespan, 60);
        assert_eq!(plan.plan[0].end_time, 60);

        //The solver should just allocate so the execution end time is at time=60, when the job is finished.

        //A new job comes in at time t1 = 20, not so urgent, taking 5 minutes. Deadline is in 2 hours
        //So the first job still has 40 minutes to go, and has deadline in 240-20 = 220 minutes
        //it is preemtible, but already started on one of the machines
        //So the new jobs data at t1 will be

        let jobs_data_t1 = vec![
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 40,
                }],
                deadline: Some(220),
                starttime: None,
                preemtive: Some(2),
                has_started: Some((0, 0)),
                job_id: 0,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 5,
                }],
                deadline: Some(120),
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 1,
            },
        ];

        let reqs = JobRequirements {
            jobs: jobs_data_t1.clone(),
            sequences: vec![],
        };

        let result = solve_jobschedule(&reqs, 0, 0, Some(2.0));
        assert!(result.is_ok());
        let plan = result.unwrap();
        assert_eq!(plan.makespan, 47);
        assert_eq!(plan.plan[1].end_time, 47);

        //The solver should just put the job in the end, as there is no need to put it earlier because of a deadline
        //This would invoke a swapping cost of 2 * 2 = 4
        //However, we need the additional time to load the new process in the end too, taking 2 minutes
        //So this is all finished when t = 40 + 5 + 2 = 47

        //Again time t2 = 20 minutes later, an urgent job comes in
        //deadline in 10 minutes, taking 5 minutes
        //job 0 is still busy for 20 minutes, job 1 not even started

        let jobs_data_t2 = vec![
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 20,
                }],
                deadline: Some(200),
                starttime: None,
                preemtive: Some(2),
                has_started: Some((0, 0)),
                job_id: 0,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 5,
                }],
                deadline: Some(120),
                starttime: None,
                preemtive: Some(2),
                has_started: None,
                job_id: 1,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 5,
                }],
                deadline: Some(10),
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 2,
            },
        ];

        let reqs = JobRequirements {
            jobs: jobs_data_t2.clone(),
            sequences: vec![],
        };

        let result = solve_jobschedule(&reqs, 0, 0, Some(2.0));
        assert!(result.is_ok());
        let plan = result.unwrap();
        assert_eq!(plan.makespan, 36);

        //The solver should just put job 1 in the end, as there is no need to put it earlier because of a deadline
        //However, job 2 will be put earlier because of the deadline
        //The solver puts job 0 until time t = 3, starts job 2 at time t = 5, ending at time t = 10, before the deadline
        //it will then continue with job 3, and finish with job 0 again. the last two can of course be swapped
        //This takes 3 (job0) + 2 (swap) + 5 (job2) + 2(swap) + 5 (job2) + 2(swap) + 17(job0) = 36
    }

    #[test]
    fn test_infeasible_constraints() {
        let jobs_data1 = vec![
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: Some(2),
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 0,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: None,
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 1,
            },
        ];
        let reqs = JobRequirements {
            jobs: jobs_data1.clone(),
            sequences: vec![],
        };

        let result = solve_jobschedule(&reqs, 0, 0, None);
        assert!(!result.is_ok()); //deadline cannot be shorter than duration

        let jobs_data2 = vec![
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: None,
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 0,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: None,
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 1,
            },
        ];
        let reqs = JobRequirements {
            jobs: jobs_data2.clone(),
            sequences: vec![(0, 1), (1, 0)],
        };

        let result = solve_jobschedule(&reqs, 0, 0, None);
        assert!(!result.is_ok()); //cannot handle i before j AND j before i simultaneously

        let jobs_data3 = vec![
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: Some(3),
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 0,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: None,
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 1,
            },
        ];
        let reqs = JobRequirements {
            jobs: jobs_data3.clone(),
            sequences: vec![(1, 0)],
        };

        let result = solve_jobschedule(&reqs, 0, 0, None);
        assert!(!result.is_ok()); //cannot meet deadline i if i processed after j and duration process j > deadline i

        let jobs_data4 = vec![
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: Some(3),
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 0,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: None,
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 1,
            },
        ];
        let reqs = JobRequirements {
            jobs: jobs_data4.clone(),
            sequences: vec![],
        };

        let result = solve_jobschedule(&reqs, 3, 0, None);
        assert!(!result.is_ok()); //cannot meet deadline i if setup_time >= deadline

        let jobs_data5 = vec![
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: Some(3),
                starttime: Some(1),
                preemtive: None,
                has_started: None,
                job_id: 0,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: None,
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 1,
            },
        ];
        let reqs = JobRequirements {
            jobs: jobs_data5.clone(),
            sequences: vec![],
        };

        let result = solve_jobschedule(&reqs, 0, 0, None);
        assert!(!result.is_ok()); //cannot meet deadline starting time is too late
    }

    #[test]
    fn test_solverfunction() {
        let jobs_data = vec![
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: None,
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 0,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 1,
                    duration: 2,
                }],
                deadline: None,
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 1,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 2,
                    duration: 2,
                }],
                deadline: None,
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 2,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 2,
                }],
                deadline: None,
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 3,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 2,
                    duration: 1,
                }],
                deadline: None,
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 4,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 1,
                    duration: 4,
                }],
                deadline: None,
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 5,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 1,
                    duration: 4,
                }],
                deadline: None,
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 6,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 2,
                    duration: 3,
                }],
                deadline: None,
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 7,
            },
        ];

        let reqs = JobRequirements {
            jobs: jobs_data.clone(),
            sequences: vec![],
        };

        let result = solve_jobschedule(&reqs, 0, 0, None);
        assert!(result.is_ok());
        let plan = result.unwrap();
        assert_eq!(plan.plan.len(), jobs_data.len());
        assert_eq!(plan.makespan, 10);
        //assert_eq!(plan.idletime, 0);
        assert!(plan.is_valid(&reqs));

        let reqs_with_sequence = JobRequirements {
            jobs: jobs_data.clone(),
            sequences: vec![(0, 1), (1, 2), (3, 4), (4, 5), (6, 7)],
        };
        let result = solve_jobschedule(&reqs_with_sequence, 0, 0, None);
        assert!(result.is_ok());
        let plan = result.unwrap();
        assert_eq!(plan.plan.len(), jobs_data.len());
        assert_eq!(plan.makespan, 11);
        assert!(plan.is_valid(&reqs_with_sequence));
        // assert_eq!(
        //     plan.plan[0],
        //     JobAllocation {
        //         machine: 0,
        //         starting_time: 0,
        //         end_time: 3
        //     }
        // );
        // assert_eq!(
        //     plan.plan[1],
        //     JobAllocation {
        //         machine: 1,
        //         starting_time: 4,
        //         end_time: 6
        //     }
        // );
        // assert_eq!(
        //     plan.plan[2],
        //     JobAllocation {
        //         machine: 2,
        //         starting_time: 0,
        //         end_time: 2
        //     }
        // );
        // assert_eq!(
        //     plan.plan[3],
        //     JobAllocation {
        //         machine: 0,
        //         starting_time: 3,
        //         end_time: 5
        //     }
        // );
        // assert_eq!(
        //     plan.plan[4],
        //     JobAllocation {
        //         machine: 2,
        //         starting_time: 2,
        //         end_time: 3
        //     }
        // );
        // assert_eq!(
        //     plan.plan[5],
        //     JobAllocation {
        //         machine: 1,
        //         starting_time: 6,
        //         end_time: 10
        //     }
        // );
        // assert_eq!(
        //     plan.plan[6],
        //     JobAllocation {
        //         machine: 1,
        //         starting_time: 0,
        //         end_time: 4
        //     }
        // );
        // assert_eq!(
        //     plan.plan[7],
        //     JobAllocation {
        //         machine: 2,
        //         starting_time: 3,
        //         end_time: 6
        //     }
        // );
    }

    #[test]
    fn test_withdeadlines() {
        let jobs_data = vec![
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: None,
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 0,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 1,
                    duration: 2,
                }],
                deadline: None,
                starttime: Some(5),
                preemtive: None,
                has_started: None,
                job_id: 1,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 2,
                    duration: 2,
                }],
                deadline: None,
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 2,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 2,
                }],
                deadline: None,
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 3,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 2,
                    duration: 1,
                }],
                deadline: None,
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 4,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 1,
                    duration: 4,
                }],
                deadline: Some(4), //this should move this job to the start
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 5,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 1,
                    duration: 4,
                }],
                deadline: None,
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 6,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 2,
                    duration: 3,
                }],
                deadline: None,
                starttime: None,
                preemtive: None,
                has_started: None,
                job_id: 7,
            },
        ];

        let reqs = JobRequirements {
            jobs: jobs_data.clone(),
            sequences: vec![],
        };

        let result = solve_jobschedule(&reqs, 0, 0, None);
        assert!(result.is_ok());
        let plan = result.unwrap();
        assert_eq!(plan.plan.len(), jobs_data.len());
        assert_eq!(plan.makespan, 10);
        assert_le!(plan.plan[5].end_time, 4); //did we make the deadline?
        assert_ge!(plan.plan[1].starting_time, 5); //did this job start late enough?
        assert!(plan.is_valid(&reqs));

        let reqs_with_sequence = JobRequirements {
            jobs: jobs_data.clone(),
            sequences: vec![(5, 6), (6, 7), (7, 0)],
        };
        let result = solve_jobschedule(&reqs_with_sequence, 0, 0, None);
        assert!(result.is_ok());
        assert_ge!(plan.plan[1].starting_time, 5);
        let plan = result.unwrap();

        // assert_eq!(
        //     plan.plan[0],
        //     JobAllocation {
        //         machine: 0,
        //         starting_time: 11,
        //         end_time: 14
        //     }
        // );
        // assert_eq!(
        //     plan.plan[1],
        //     JobAllocation {
        //         machine: 1,
        //         starting_time: 8,
        //         end_time: 10
        //     }
        // );
        // assert_eq!(
        //     plan.plan[2],
        //     JobAllocation {
        //         machine: 2,
        //         starting_time: 1,
        //         end_time: 3
        //     }
        // );
        // assert_eq!(
        //     plan.plan[3],
        //     JobAllocation {
        //         machine: 0,
        //         starting_time: 0,
        //         end_time: 2
        //     }
        // );
        // assert_eq!(
        //     plan.plan[4],
        //     JobAllocation {
        //         machine: 2,
        //         starting_time: 0,
        //         end_time: 1
        //     }
        // );
        // assert_eq!(
        //     plan.plan[5],
        //     JobAllocation {
        //         machine: 1,
        //         starting_time: 0,
        //         end_time: 4
        //     }
        // );
        // assert_eq!(
        //     plan.plan[6],
        //     JobAllocation {
        //         machine: 1,
        //         starting_time: 4,
        //         end_time: 8
        //     }
        // );
        // assert_eq!(
        //     plan.plan[7],
        //     JobAllocation {
        //         machine: 2,
        //         starting_time: 8,
        //         end_time: 11
        //     }
        // );

        assert_eq!(plan.plan.len(), jobs_data.len());
        assert_eq!(plan.makespan, 14);
        assert!(plan.is_valid(&reqs_with_sequence));
    }
}
