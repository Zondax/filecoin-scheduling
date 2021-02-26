#![allow(dead_code)]
use array_tool::vec::Intersect;
use coin_cbc::{raw::Status, Col, Model, Sense};
use common::Error;

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
            for (j, job_j) in allocs.iter().enumerate() {
                if i != j
                    && job_i.machine == job_j.machine
                    && (job_i.starting_time <= job_j.starting_time
                        && job_j.starting_time < job_i.end_time)
                {
                    return false;
                }
            }
        }
        true
    }
}

fn find_upper_bounds(input: &JobRequirements) -> (usize, f64) {
    let input_data = input.jobs.clone();
    let mut num_machines: usize = 0;
    let mut big_num: f64 = 0.;
    for job in input_data.iter() {
        for constraint in job.options.clone() {
            big_num += constraint.duration as f64;
            if constraint.machine > num_machines {
                num_machines = constraint.machine;
            }
        }
    }
    num_machines += 1;
    (num_machines, big_num)
}

fn add_dummy_jobs(
    jobs_data: &mut Vec<JobDescription>,
    num_machines: usize,
    setup_time: usize,
    finish_time: usize,
) {
    for machine in (0..num_machines).rev() {
        jobs_data.insert(
            0,
            JobDescription {
                options: vec![JobConstraint {
                    machine: machine as usize,
                    duration: setup_time,
                }],
                deadline: None,
            },
        );
    }
    for machine in 0..num_machines {
        jobs_data.push(JobDescription {
            options: vec![JobConstraint {
                machine: machine as usize,
                duration: finish_time,
            }],
            deadline: None,
        });
    }
}

fn add_constraints_per_job(
    m: &mut Model,
    columns: &mut Vec<Col>,
    jobs_data: &Vec<JobDescription>,
    num_machines: usize,
) -> (Vec<usize>, Vec<usize>) {
    let mut indexes_sv: Vec<usize> = vec![];
    let mut indexes_ev: Vec<usize> = vec![];

    for (i, job) in jobs_data.iter().enumerate() {
        let index = m.num_cols() as usize;
        for _ in 0..num_machines {
            columns.push(m.add_binary());
        }
        let index_sv = m.num_cols() as usize;
        indexes_sv.push(index_sv);
        columns.push(m.add_integer()); //s_v
        let index_ev = m.num_cols() as usize;
        indexes_ev.push(index_ev);
        columns.push(m.add_integer()); //e_v
        let index_pv = m.num_cols() as usize;
        columns.push(m.add_integer()); //p_v

        let row_onemachine = m.add_row();
        let row_onetime = m.add_row();
        //sum_{k} x_{k,w} == 1
        //sum_{k} x_{k,w} * dur - pv_w == 0
        for machine in 0..num_machines {
            let options = job.options.clone();
            for option in options {
                if option.machine == machine {
                    m.set_weight(row_onemachine, columns[index + machine], 1.0);
                    m.set_weight(
                        row_onetime,
                        columns[index + machine],
                        option.duration as f64,
                    );
                }
            }
        }

        m.set_row_lower(row_onemachine, 1.0);
        m.set_row_upper(row_onemachine, 1.0);

        m.set_weight(row_onetime, columns[index_pv], -1.0);
        m.set_row_lower(row_onetime, 0.0);
        m.set_row_upper(row_onetime, 0.0);

        //s_v >= 0
        let mut row = m.add_row();
        m.set_row_lower(row, 0.0);
        m.set_weight(row, columns[index_sv], 1.0);
        if i < num_machines {
            m.set_row_upper(row, 0.0);
        }

        //e_v >= 0
        row = m.add_row();
        m.set_row_lower(row, 0.0);
        m.set_weight(row, columns[index_ev], 1.0);

        //p_v >= 0
        row = m.add_row();
        m.set_row_lower(row, 0.0);
        m.set_weight(row, columns[index_pv], 1.0);

        //-ev + sv + pv == 0
        row = m.add_row();
        m.set_row_lower(row, 0.0);
        m.set_row_upper(row, 0.0);
        m.set_weight(row, columns[index_ev], -1.0);
        m.set_weight(row, columns[index_sv], 1.0);
        m.set_weight(row, columns[index_pv], 1.0);

        //e_v - makespan <= 0
        row = m.add_row();
        m.set_row_upper(row, 0.0);
        m.set_weight(row, columns[0], -1.0);
        m.set_weight(row, columns[index_ev], 1.0);

        //e_v <= deadline
        if job.deadline.is_some() {
            row = m.add_row();
            m.set_row_upper(row, job.deadline.unwrap() as f64);
            m.set_weight(row, columns[index_ev], 1.0);
        }
    }
    (indexes_sv, indexes_ev)
}

fn add_sequential_constraints(
    m: &mut Model,
    columns: &Vec<Col>,
    sequences: &Vec<(JobIndex, JobIndex)>,
    indexes_sv: &Vec<usize>,
    indexes_ev: &Vec<usize>,
    num_machines: usize,
) {
    for (i, j) in sequences.iter() {
        let row = m.add_row();
        m.set_row_upper(row, 0.0);
        let index_i = num_machines + (*i);
        let index_j = num_machines + (*j);
        m.set_weight(row, columns[indexes_ev[index_i]], 1.0);
        m.set_weight(row, columns[indexes_sv[index_j]], -1.0);
    }
}

fn add_machine_constraints(
    m: &mut Model,
    columns: &mut Vec<Col>,
    jobs_data: &Vec<JobDescription>,
    indexes_sv: &Vec<usize>,
    indexes_ev: &Vec<usize>,
    num_machines: usize,
    big_num: f64,
) {
    //
    let num_jobs = jobs_data.len();
    let index = m.num_cols() as usize;
    for _ in 0..(num_machines * num_jobs * num_jobs) {
        columns.push(m.add_binary());
    }
    //w has a predecessor on machine k
    let mut row;
    for k in 0..num_machines {
        for w in num_machines..num_jobs {
            if jobs_data[w].options.iter().any(
                |JobConstraint {
                     machine: i,
                     duration: _,
                 }| i == &k,
            ) {
                row = m.add_row();
                //\sum_{v | v \neq w} y_{k,v,w} - x_{k,w} == 0
                for v in 0..num_jobs {
                    if v != w
                        && jobs_data[v].options.iter().any(
                            |JobConstraint {
                                 machine: i,
                                 duration: _,
                             }| i == &k,
                        )
                    {
                        //&& jobs_data[v].0.intersect(jobs_data[w].0.clone()).len() > 0
                        m.set_weight(
                            row,
                            columns[index + k * num_jobs * num_jobs + v * num_jobs + w],
                            1.0,
                        );
                    }
                }
                m.set_weight(row, columns[1 + w * (num_machines + 3) + k], -1.);
                m.set_row_lower(row, 0.0);
                m.set_row_upper(row, 0.0);
            }
        }
        //v has a successor on machine k
        for v in 0..num_jobs - num_machines {
            if jobs_data[v].options.iter().any(
                |JobConstraint {
                     machine: i,
                     duration: _,
                 }| i == &k,
            ) {
                row = m.add_row();
                for w in 0..num_jobs {
                    //\sum_{w | w \neq v} y_{k,v,w} - x_{k,v} == 0
                    if v != w
                        && jobs_data[w].options.iter().any(
                            |JobConstraint {
                                 machine: i,
                                 duration: _,
                             }| i == &k,
                        )
                    {
                        m.set_weight(
                            row,
                            columns[index + k * num_jobs * num_jobs + v * num_jobs + w],
                            1.0,
                        );
                    }
                }
                m.set_weight(row, columns[1 + v * (num_machines + 3) + k], -1.);
                m.set_row_lower(row, 0.0);
                m.set_row_upper(row, 0.0);
            }
        }
    }

    for v in 0..num_jobs {
        for w in 0..num_jobs {
            if v != w
                && !jobs_data[v]
                    .options
                    .intersect_if(jobs_data[w].options.clone(), |l, r| l.machine == r.machine)
                    .is_empty()
            {
                //L*sum_{k : job_v(k) && job_w(k)} y_{k,v,w} + e_v - s_w <= L
                row = m.add_row();
                let machines = jobs_data[v]
                    .options
                    .intersect_if(jobs_data[w].options.clone(), |l, r| l.machine == r.machine);
                //e_v
                m.set_weight(row, columns[indexes_ev[v]], 1.0);
                //-s_w
                m.set_weight(row, columns[indexes_sv[w]], -1.0);
                for machine_index in machines {
                    m.set_weight(
                        row,
                        columns[index
                            + machine_index.machine * num_jobs * num_jobs
                            + v * num_jobs
                            + w],
                        big_num as f64,
                    );
                }
                m.set_row_upper(row, big_num as f64);
            }
        }
    }
}

pub fn solve_jobschedule(
    input: &JobRequirements,
    setup_time: usize,
    finish_time: usize,
) -> Result<JobPlan, Error> {
    let (num_machines, big_num) = find_upper_bounds(input);

    let mut jobs_data = input.jobs.clone();

    add_dummy_jobs(&mut jobs_data, num_machines, setup_time, finish_time);

    let num_jobs = jobs_data.len();
    let mut m = Model::default();

    let mut columns = vec![];

    //makespan = col[0] >= 0
    columns.push(m.add_integer());
    let row = m.add_row();
    m.set_row_lower(row, 0.0);
    m.set_weight(row, columns[0], 1.0);

    let (indexes_sv, indexes_ev) =
        add_constraints_per_job(&mut m, &mut columns, &jobs_data, num_machines);
    //
    assert_eq!(m.num_cols() as usize, 1 + num_jobs * (num_machines + 3));

    add_sequential_constraints(
        &mut m,
        &columns,
        &input.sequences,
        &indexes_sv,
        &indexes_ev,
        num_machines,
    );

    add_machine_constraints(
        &mut m,
        &mut columns,
        &jobs_data,
        &indexes_sv,
        &indexes_ev,
        num_machines,
        big_num,
    );
    //

    m.set_obj_coeff(columns[0], 1.);
    // Set objective sense.
    m.set_obj_sense(Sense::Minimize);

    // Solve the problem. Returns the solution
    let sol = m.solve();

    if Status::Finished != sol.raw().status() {
        return Err(Error::Other(
            "Solver did not find solution (conflicting constraints?)".to_string(),
        ));
    }

    let solution = sol.raw().obj_value() as usize;
    if solution > big_num as usize {
        return Err(Error::Other(
            "Solver did not find correct solution (conflicting constraints?)".to_string(),
        ));
    }

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
    Ok(JobPlan {
        makespan: solution,
        plan: allocs,
        machine_finish_times: finishtimes,
        idletime: total_idle_time,
    })
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
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: None,
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
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: None,
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
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: None,
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
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
                deadline: None,
            },
        ];
        let reqs = JobRequirements {
            jobs: jobs_data4.clone(),
            sequences: vec![],
        };

        let result = solve_jobschedule(&reqs, 3, 0);
        assert!(!result.is_ok()); //cannot meet deadline i if setup_time >= deadline
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
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 1,
                    duration: 2,
                }],
                deadline: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 2,
                    duration: 2,
                }],
                deadline: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 2,
                }],
                deadline: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 2,
                    duration: 1,
                }],
                deadline: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 1,
                    duration: 4,
                }],
                deadline: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 1,
                    duration: 4,
                }],
                deadline: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 2,
                    duration: 3,
                }],
                deadline: None,
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
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 1,
                    duration: 2,
                }],
                deadline: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 2,
                    duration: 2,
                }],
                deadline: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 2,
                }],
                deadline: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 2,
                    duration: 1,
                }],
                deadline: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 1,
                    duration: 4,
                }],
                deadline: Some(4), //this should move this job to the start
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 1,
                    duration: 4,
                }],
                deadline: None,
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 2,
                    duration: 3,
                }],
                deadline: None,
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
        assert_eq!(plan.plan[5].starting_time, 0);
        assert!(plan.is_valid(&reqs));

        let reqs_with_sequence = JobRequirements {
            jobs: jobs_data.clone(),
            sequences: vec![(5, 6), (6, 7), (7, 0)],
        };
        let result = solve_jobschedule(&reqs_with_sequence, 0, 0);
        assert!(result.is_ok());
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
