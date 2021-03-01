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
}

#[derive(Clone, Debug, PartialEq)]
pub struct JobDescription {
    pub options: Vec<JobConstraint>,
    pub starttime: Option<usize>,
    pub deadline: Option<usize>,
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

            if reqs.jobs[i].starttime.is_some() && job_i.starting_time < reqs.jobs[i].starttime.unwrap() {
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
                if reqs.sequences.iter().any(
                    |(k,l)| &i == k && &j == l
                ) && job_i.end_time > job_j.starting_time {
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
}

impl LinearSolverModel {
    /// This function initializes the Linear Model solver
    /// It sets the target for the makespan (the total execution time)
    /// The objective is to minimize that
    pub fn initialize(input: &[JobDescription]) -> LinearSolverModel {
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
        self.big_num = big_num;
    }

    /// This function adds dummy starting and dummy finishing jobs
    /// We need this mostly for the solver
    /// But we can also set setup times / finish times if needed (both can be 0)
    pub fn add_dummy_jobs(&mut self, setup_time: usize, finish_time: usize) {
        let num_machines = self.num_machines;
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
                },
            );
        }
        for machine in 0..num_machines {
            self.jobs_data.push(JobDescription {
                options: vec![JobConstraint {
                    machine: machine as usize,
                    duration: finish_time,
                }],
                deadline: None,
                starttime: None,
            });
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
        for (i, job) in self.jobs_data.iter().enumerate() {
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
            let row_onetime = self.m.add_row();
            //sum_{k} x_{k,w} == 1
            //sum_{k} x_{k,w} * dur - pv_w == 0
            for machine in 0..num_machines {
                let options = job.options.clone();
                for option in options {
                    if option.machine == machine {
                        self.m
                            .set_weight(row_onemachine, self.columns[index + machine], 1.0);
                        self.m.set_weight(
                            row_onetime,
                            self.columns[index + machine],
                            option.duration as f64,
                        );
                    }
                }
            }

            self.m.set_row_equal(row_onemachine, 1.0);

            self.m.set_weight(row_onetime, self.columns[index_pv], -1.0);
            self.m.set_row_equal(row_onetime, 0.0);

            //s_v >= 0
            let mut row = self.m.add_row();
            self.m.set_row_lower(row, 0.0);
            self.m.set_weight(row, self.columns[index_sv], 1.0);
            if i < num_machines {
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
                            self.big_num as f64,
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
                    processtimes[k] += endtime - starttime;
                    if endtime > finishtimes[k] {
                        finishtimes[k] = endtime;
                    }
                    allocs.push(JobAllocation {
                        machine: k,
                        starting_time: starttime,
                        end_time: endtime,
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
) -> Result<JobPlan, Error> {
    let mut model: LinearSolverModel = LinearSolverModel::initialize(&input.jobs);

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
    fn test_infeasible_constraints() {
        let jobs_data1 = vec![
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: Some(2),
                starttime: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: None,
                starttime: None,
            },
        ];
        let reqs = JobRequirements {
            jobs: jobs_data1.clone(),
            sequences: vec![],
        };

        let result = solve_jobschedule(&reqs, 0, 0);
        assert!(!result.is_ok()); //deadline cannot be shorter than duration

        let jobs_data2 = vec![
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: None,
                starttime: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: None,
                starttime: None,
            },
        ];
        let reqs = JobRequirements {
            jobs: jobs_data2.clone(),
            sequences: vec![(0, 1), (1, 0)],
        };

        let result = solve_jobschedule(&reqs, 0, 0);
        assert!(!result.is_ok()); //cannot handle i before j AND j before i simultaneously

        let jobs_data3 = vec![
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: Some(3),
                starttime: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: None,
                starttime: None,
            },
        ];
        let reqs = JobRequirements {
            jobs: jobs_data3.clone(),
            sequences: vec![(1, 0)],
        };

        let result = solve_jobschedule(&reqs, 0, 0);
        assert!(!result.is_ok()); //cannot meet deadline i if i processed after j and duration process j > deadline i

        let jobs_data4 = vec![
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: Some(3),
                starttime: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: None,
                starttime: None,
            },
        ];
        let reqs = JobRequirements {
            jobs: jobs_data4.clone(),
            sequences: vec![],
        };

        let result = solve_jobschedule(&reqs, 3, 0);
        assert!(!result.is_ok()); //cannot meet deadline i if setup_time >= deadline

        let jobs_data5 = vec![
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: Some(3),
                starttime: Some(1),
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: None,
                starttime: None,
            },
        ];
        let reqs = JobRequirements {
            jobs: jobs_data5.clone(),
            sequences: vec![],
        };

        let result = solve_jobschedule(&reqs, 0, 0);
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
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 1,
                    duration: 2,
                }],
                deadline: None,
                starttime: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 2,
                    duration: 2,
                }],
                deadline: None,
                starttime: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 2,
                }],
                deadline: None,
                starttime: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 2,
                    duration: 1,
                }],
                deadline: None,
                starttime: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 1,
                    duration: 4,
                }],
                deadline: None,
                starttime: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 1,
                    duration: 4,
                }],
                deadline: None,
                starttime: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 2,
                    duration: 3,
                }],
                deadline: None,
                starttime: None,
            },
        ];

        let reqs = JobRequirements {
            jobs: jobs_data.clone(),
            sequences: vec![],
        };

        let result = solve_jobschedule(&reqs, 0, 0);
        assert!(result.is_ok());
        let plan = result.unwrap();
        assert_eq!(plan.plan.len(), jobs_data.len());
        assert_eq!(plan.makespan, 10);
        assert_eq!(plan.idletime, 0);
        assert!(plan.is_valid(&reqs));

        let reqs_with_sequence = JobRequirements {
            jobs: jobs_data.clone(),
            sequences: vec![(0, 1), (1, 2), (3, 4), (4, 5), (6, 7)],
        };
        let result = solve_jobschedule(&reqs_with_sequence, 0, 0);
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
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 1,
                    duration: 2,
                }],
                deadline: None,
                starttime: Some(5),
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 2,
                    duration: 2,
                }],
                deadline: None,
                starttime: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 2,
                }],
                deadline: None,
                starttime: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 2,
                    duration: 1,
                }],
                deadline: None,
                starttime: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 1,
                    duration: 4,
                }],
                deadline: Some(4), //this should move this job to the start
                starttime: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 1,
                    duration: 4,
                }],
                deadline: None,
                starttime: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 2,
                    duration: 3,
                }],
                deadline: None,
                starttime: None,
            },
        ];

        let reqs = JobRequirements {
            jobs: jobs_data.clone(),
            sequences: vec![],
        };

        let result = solve_jobschedule(&reqs, 0, 0);
        assert!(result.is_ok());
        let plan = result.unwrap();
        assert_eq!(plan.plan.len(), jobs_data.len());
        assert_eq!(plan.makespan, 10);
        assert_le!(plan.plan[5].end_time, 4);     //did we make the deadline?
        assert_ge!(plan.plan[1].starting_time, 5);      //did this job start late enough?
        assert!(plan.is_valid(&reqs));

        let reqs_with_sequence = JobRequirements {
            jobs: jobs_data.clone(),
            sequences: vec![(5, 6), (6, 7), (7, 0)],
        };
        let result = solve_jobschedule(&reqs_with_sequence, 0, 0);
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
