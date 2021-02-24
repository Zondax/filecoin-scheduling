#![allow(dead_code)]
use array_tool::vec::Intersect;
use coin_cbc::{raw::Status, Model, Sense};

const ERROR_MARGIN: f64 = 1e-20;

#[derive(Clone, Debug, PartialEq)]
pub struct JobConstraint {
    pub machine: usize,
    pub duration: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JobAllocation {
    pub machine: usize,
    pub starting_time: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JobDescription {
    pub options: Vec<JobConstraint>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JobRequirements {
    pub jobs: Vec<JobDescription>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JobPlan {
    pub makespan: usize, //total execution time
    pub plan: Vec<JobAllocation>,
}

pub fn solve_jobschedule(input: JobRequirements, setup_time: usize, finish_time: usize) -> JobPlan {
    let input_data = input.jobs;
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

    let mut jobs_data = input_data;
    for machine in (0..num_machines).rev() {
        jobs_data.insert(
            0,
            JobDescription {
                options: vec![JobConstraint {
                    machine: machine as usize,
                    duration: setup_time,
                }],
            },
        );
    }
    for machine in 0..num_machines {
        jobs_data.push(JobDescription {
            options: vec![JobConstraint {
                machine: machine as usize,
                duration: finish_time,
            }],
        });
    }
    let num_jobs = jobs_data.len();
    let mut m = Model::default();

    let mut columns = vec![];

    //makespan = col[0] >= 0
    columns.push(m.add_integer());
    let mut row = m.add_row();
    m.set_row_lower(row, 0.0);
    m.set_weight(row, columns[0], 1.0);

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
        row = m.add_row();
        m.set_row_lower(row, 0.0);
        m.set_weight(row, columns[index_sv], 1.0);
        if i < num_machines {
            m.set_row_upper(row, 0.0);
        }
        if i >= num_jobs - num_machines {
            m.set_weight(row, columns[0], -1.0);
        }

        //-ev + sv + pv == 0
        if i >= num_machines && i < num_jobs - num_machines {
            row = m.add_row();
            m.set_row_lower(row, 0.0);
            m.set_row_upper(row, 0.0);
            m.set_weight(row, columns[index_ev], -1.0);
            m.set_weight(row, columns[index_sv], 1.0);
            m.set_weight(row, columns[index_pv], 1.0);
        }
        //FIXME: we need some way to tell if jobs have to be processed sequentially
        /*
        if i >= 7 && i < 9 {
            row = m.add_row();
            m.set_row_upper(row, 0.0);
            m.set_weight(row, columns[indexes_ev[i - 1]], 1.0);
            m.set_weight(row, columns[index_sv], -1.0);
        }

        if i >= 10 && i < 12 {
            row = m.add_row();
            m.set_row_upper(row, 0.0);
            m.set_weight(row, columns[indexes_ev[i - 1]], 1.0);
            m.set_weight(row, columns[index_sv], -1.0);
        }

        if i == 13 {
            row = m.add_row();
            m.set_row_upper(row, 0.0);
            m.set_weight(row, columns[indexes_ev[i - 1]], 1.0);
            m.set_weight(row, columns[index_sv], -1.0);
        }
         */
        //e_v - makespan <= 0
        row = m.add_row();
        m.set_row_upper(row, 0.0);
        m.set_weight(row, columns[0], -1.0);
        m.set_weight(row, columns[index_ev], 1.0);
    }
    //
    assert_eq!(m.num_cols() as usize, 1 + num_jobs * (num_machines + 3));
    //
    let index = m.num_cols() as usize;
    for _ in 0..(num_machines * num_jobs * num_jobs) {
        columns.push(m.add_binary());
    }
    //w has a predecessor on machine k
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

    m.set_obj_coeff(columns[0], 1.);
    // Set objective sense.
    m.set_obj_sense(Sense::Minimize);

    // Solve the problem. Returns the solution
    let sol = m.solve();

    assert_eq!(Status::Finished, sol.raw().status());

    let solution = sol.raw().obj_value() as usize;

    let mut allocs = vec![];
    let start_index = 1 + num_machines * (num_machines + 3);
    for i in 0..num_jobs {
        for k in 0..num_machines {
            let index = start_index + i * (num_machines + 3);
            if (sol.col(columns[index + k]).round() - 1.0).abs() < ERROR_MARGIN {
                let starttime = sol.col(columns[indexes_sv[i]]).round() as usize;
                allocs.push(JobAllocation {
                    machine: k,
                    starting_time: starttime,
                });
            }
        }
    }
    JobPlan {
        makespan: solution,
        plan: allocs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use array_tool::vec::Intersect;
    use coin_cbc::{raw::Status, Model, Sense};
    use itertools::Itertools;

    #[test]
    fn test_solverfunction() {
        /*let jobs_data = vec![JobWithID(0,3,0),JobWithID(1,3,0), JobWithID(2,3,0) JobWithID(1, 2, 1), JobWithID(2, 2, 2),
        JobWithID(0, 2, 3), JobWithID(2, 1, 4), JobWithID(1, 4, 5),
        JobWithID(1, 4, 6), JobWithID(2, 3, 7)];
         */

        let jobs_data = vec![
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 3,
                }],
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 1,
                    duration: 2,
                }],
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 2,
                    duration: 2,
                }],
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 0,
                    duration: 2,
                }],
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 2,
                    duration: 1,
                }],
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 1,
                    duration: 4,
                }],
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 1,
                    duration: 4,
                }],
            },
            JobDescription {
                options: vec![JobConstraint {
                    machine: 2,
                    duration: 3,
                }],
            },
        ];

        let reqs = JobRequirements { jobs: jobs_data };

        let plan: JobPlan = solve_jobschedule(reqs, 0, 0);
        assert_eq!(plan.makespan, 10);
    }

    /*
    Job shop problem.

    Model each job starting time as c_{ij} for job j at machine i
    Let each job j have duration dur_j
    To enforce one job per machine at any time, define binary variables b_{ijk}
    to denote job j starting before job k on machine i.

    Let bignum be a big number.
    Let makespan be the time the last job finishes. Then:

    - The objective function is simply:
    \min makespan

    Then constraints:
        c_{ij} + dur_j <= makespan for all j
        c_{ij} + dur_j <= c_{ik} + b_{i,j,k} * M for all i,j,k
        sum_{j,k} b_{i,j,k} == 1 for all i
     */

    /*
    Not covered in this example: a job that can be executed by one of the machines:
    Say job j is executable on machine 1 or 2 or 3, then make a copy of job j and add constraints like:

    bin{j,1}, bin{j,2}, bin{j,3} binary variable
    sum == 1

    c_{1j} <= bin{j,1}*M
    c_{2j} <= bin{j,2}*M

    c_{1j} + dur_j * bin{1,2} <= makespan
    c_{2j} + dur_j * (1-bin{1,2}) <= makespan

    c_{1j} dur_j*bin{j,k} == c_{j} + b_{

    c_{1j} + dur_j*bin{1,2} <= c_{ik} + b_{i,j,k} * M for all i,j,k
    c_{2j} + dur_j*(1-bin{1,2}) <= c_{ik} + b_{i,j,k} * M for all i,j,k
     */

    //machines, duration, id
    #[derive(Clone, Debug)]
    pub struct JobWithID(u32, i32, u32);

    #[derive(Clone, Debug)]
    pub struct Job(u32, i32);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Jobj(Vec<usize>, Vec<i32>);

    pub const BIG_NUM: f64 = 10000 as f64;

    #[test]
    fn test_jobswithmultiplemachines() {
        /*let jobs_data = vec![JobWithID(0,3,0),JobWithID(1,3,0), JobWithID(2,3,0) JobWithID(1, 2, 1), JobWithID(2, 2, 2),
        JobWithID(0, 2, 3), JobWithID(2, 1, 4), JobWithID(1, 4, 5),
        JobWithID(1, 4, 6), JobWithID(2, 3, 7)];
         */

        let jobs_data = vec![
            Jobj(vec![0], vec![0]),
            Jobj(vec![1], vec![0]),
            Jobj(vec![2], vec![0]),
            Jobj(vec![0], vec![0]),
            Jobj(vec![1], vec![0]),
            Jobj(vec![2], vec![0]),
            Jobj(vec![0], vec![3]),
            Jobj(vec![1], vec![2]),
            Jobj(vec![2], vec![2]),
            Jobj(vec![0], vec![2]),
            Jobj(vec![2], vec![1]),
            Jobj(vec![1], vec![4]),
            Jobj(vec![1], vec![4]),
            Jobj(vec![2], vec![3]),
        ];

        const num_machines: usize = 3;
        const num_jobs: usize = 14;

        let mut m = Model::default();

        let mut columns = vec![];

        //makespan = col[0] >= 0
        columns.push(m.add_integer());
        let mut row = m.add_row();
        m.set_row_lower(row, 0.0);
        m.set_weight(row, columns[0], 1.0);

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

            for machine in 0..num_machines {
                for i in 0..job.0.len() {
                    if job.0[i] == machine {
                        m.set_weight(row_onemachine, columns[index + machine], 1.0);
                        m.set_weight(row_onetime, columns[index + machine], job.1[i] as f64);
                    }
                }
            }

            m.set_row_lower(row_onemachine, 1.0);
            m.set_row_upper(row_onemachine, 1.0);

            m.set_weight(row_onetime, columns[index_pv], -1.0);
            m.set_row_lower(row_onetime, 0.0);
            m.set_row_upper(row_onetime, 0.0);

            //s_v >= 0
            row = m.add_row();
            m.set_row_lower(row, 0.0);
            m.set_weight(row, columns[index_sv], 1.0);
            if i < 6 {
                m.set_row_upper(row, 0.0);
            }
            if i > 2 && i < 6 {
                m.set_weight(row, columns[0], -1.0);
            }

            //-ev + sv + pv == 0
            if i >= 6 {
                row = m.add_row();
                m.set_row_lower(row, 0.0);
                m.set_row_upper(row, 0.0);
                m.set_weight(row, columns[index_ev], -1.0);
                m.set_weight(row, columns[index_sv], 1.0);
                m.set_weight(row, columns[index_pv], 1.0);
            }

            if i >= 7 && i < 9 {
                row = m.add_row();
                m.set_row_upper(row, 0.0);
                m.set_weight(row, columns[indexes_ev[i - 1]], 1.0);
                m.set_weight(row, columns[index_sv], -1.0);
            }

            if i >= 10 && i < 12 {
                row = m.add_row();
                m.set_row_upper(row, 0.0);
                m.set_weight(row, columns[indexes_ev[i - 1]], 1.0);
                m.set_weight(row, columns[index_sv], -1.0);
            }

            if i == 13 {
                row = m.add_row();
                m.set_row_upper(row, 0.0);
                m.set_weight(row, columns[indexes_ev[i - 1]], 1.0);
                m.set_weight(row, columns[index_sv], -1.0);
            }
            //e_v - makespan <= 0
            row = m.add_row();
            m.set_row_upper(row, 0.0);
            m.set_weight(row, columns[0], -1.0);
            m.set_weight(row, columns[index_ev], 1.0);
        }

        assert_eq!(m.num_cols() as usize, 1 + num_jobs * (num_machines + 3));

        let index = m.num_cols() as usize;
        for _ in 0..(num_machines * num_jobs * num_jobs) {
            columns.push(m.add_binary());
        }
        //w has a predecessor on machine k
        for k in 0..num_machines {
            for w in 3..num_jobs {
                if jobs_data[w].0.iter().any(|&i| i == k) {
                    row = m.add_row();
                    //\sum_{v | v \neq w} y_{k,v,w} - x_{k,w} == 0
                    for v in 0..num_jobs {
                        if v != w && jobs_data[v].0.iter().any(|&i| i == k) {
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
            for v in 0..num_jobs {
                if v >= 3 && v < 6 {
                    continue;
                }
                if jobs_data[v].0.iter().any(|&i| i == k) {
                    row = m.add_row();
                    for w in 0..num_jobs {
                        //\sum_{w | w \neq v} y_{k,v,w} - x_{k,v} == 0
                        if v != w && jobs_data[w].0.iter().any(|&i| i == k) {
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
                if v != w && jobs_data[v].0.intersect(jobs_data[w].0.clone()).len() > 0 {
                    //L*sum_{k : job_v(k) && job_w(k)} y_{k,v,w} + e_v - s_w <= L
                    row = m.add_row();
                    let machines = jobs_data[v].0.intersect(jobs_data[w].0.clone());
                    //e_v
                    m.set_weight(row, columns[indexes_ev[v]], 1.0);
                    //-s_w
                    m.set_weight(row, columns[indexes_sv[w]], -1.0);
                    for machine_index in machines.iter() {
                        m.set_weight(
                            row,
                            columns[index + machine_index * num_jobs * num_jobs + v * num_jobs + w],
                            BIG_NUM,
                        );
                    }
                    m.set_row_upper(row, BIG_NUM);
                }
            }
        }

        m.set_obj_coeff(columns[0], 1.);
        // Set objective sense.
        m.set_obj_sense(Sense::Minimize);

        // Solve the problem. Returns the solution
        let sol = m.solve();

        assert_eq!(Status::Finished, sol.raw().status());

        //Optimal Schedule Length: 11
        assert_eq!(11., sol.raw().obj_value());
        let end_time = sol.raw().obj_value() as f64;
        //sv,ev,pv

        // Optimal Schedule

        assert_eq!(sol.col(columns[1 + num_machines]), 0.);
        assert_eq!(sol.col(columns[1 + (num_machines + 3) + num_machines]), 0.);
        assert_eq!(
            sol.col(columns[1 + 2 * (num_machines + 3) + num_machines]),
            0.
        );

        assert_eq!(
            sol.col(columns[1 + 3 * (num_machines + 3) + num_machines]),
            end_time
        );
        assert_eq!(
            sol.col(columns[1 + 4 * (num_machines + 3) + num_machines]),
            end_time
        );
        assert_eq!(
            sol.col(columns[1 + 5 * (num_machines + 3) + num_machines]),
            end_time
        );

        assert_eq!(sol.col(columns[1 + 6 * (num_machines + 3)]).round(), 1.);
        assert_eq!(sol.col(columns[1 + 7 * (num_machines + 3) + 1]).round(), 1.);
        assert_eq!(sol.col(columns[1 + 8 * (num_machines + 3) + 2]).round(), 1.);

        // Machine 0: job_0_0   job_1_0
        // Machine 1: job_2_0   job_0_1   job_1_2
        // Machine 2: job_1_1   job_2_1   job_0_2  //job_2_1   job_0_2 swapped with jobshop example below

        // Machine 0: [0,3]     [3,5]
        // Machine 1: [0,4]     [4,6]     [7,11]
        // Machine 2: [5,6]     [6,9]     [9,11]
    }

    pub fn flatten_jobs(input_data: Vec<Vec<Job>>) -> Vec<(u32, i32, usize)> {
        let mut result = vec![];
        let mut index: usize = 1;
        for joblist in input_data.iter() {
            for job in joblist.iter() {
                result.push((job.0, job.1, index));
                index += 1;
            }
        }
        result
    }

    #[test]
    fn test_jobshop() {
        let jobs_data1 = vec![Job(0, 3), Job(1, 2), Job(2, 2)];

        let jobs_data2 = vec![Job(0, 2), Job(2, 1), Job(1, 4)];

        let jobs_data3 = vec![Job(1, 4), Job(2, 3)];

        let jobs_data = [jobs_data1, jobs_data2, jobs_data3];

        let mut m = Model::default();

        let mut columns = vec![];

        //makespan = col[0] >= 0
        columns.push(m.add_integer());
        let mut row = m.add_row();
        m.set_row_lower(row, 0.0);
        m.set_weight(row, columns[0], 1.0);

        for joblist in jobs_data.iter() {
            let n = columns.len();
            for _ in joblist {
                columns.push(m.add_integer());
            }
            for (i, job) in joblist.iter().enumerate() {
                //c_{ij} + d_j <= makespan
                let duration: f64 = -job.1 as f64;
                row = m.add_row();
                m.set_row_upper(row, duration);
                m.set_weight(row, columns[0], -1.0);
                m.set_weight(row, columns[n + i], 1.0);

                //c_{ij} >= 0
                row = m.add_row();
                m.set_row_lower(row, 0.0);
                m.set_weight(row, columns[n + i], 1.0);

                if i < joblist.len() - 1 {
                    //c_{ij} + d_j <= c_{i,j+1}
                    row = m.add_row();
                    m.set_row_upper(row, duration);
                    m.set_weight(row, columns[n + i + 1], -1.0);
                    m.set_weight(row, columns[n + i], 1.0);
                }
            }
        }

        let flattenjobs = flatten_jobs(jobs_data.to_vec());

        for (job_i, job_j) in flattenjobs.iter().tuple_combinations() {
            if job_i.0 == job_j.0 {
                let n = columns.len();
                columns.push(m.add_binary());

                //c_ki + d_i \le c_kj + bin*M
                let duration_i: f64 = -job_i.1 as f64;
                row = m.add_row();
                m.set_row_upper(row, duration_i);
                m.set_weight(row, columns[job_i.2], 1.0);
                m.set_weight(row, columns[job_j.2], -1.0);
                m.set_weight(row, columns[n], -BIG_NUM);

                //c_kj + d_j \le c_ki + (1-bin)*M
                let duration_j: f64 = -job_j.1 as f64;
                row = m.add_row();
                m.set_row_upper(row, duration_j + BIG_NUM);
                m.set_weight(row, columns[job_i.2], -1.0);
                m.set_weight(row, columns[job_j.2], 1.0);
                m.set_weight(row, columns[n], BIG_NUM);
            }
        }

        assert_eq!(m.num_cols(), 16);
        assert_eq!(m.num_rows(), 36);

        m.set_obj_coeff(columns[0], 1.);
        // Set objective sense.
        m.set_obj_sense(Sense::Minimize);

        // Solve the problem. Returns the solution
        let sol = m.solve();

        assert_eq!(Status::Finished, sol.raw().status());

        //Optimal Schedule Length: 11

        assert_eq!(11., sol.raw().obj_value());

        // Optimal Schedule
        //
        // Machine 0: job_0_0   job_1_0
        // Machine 1: job_2_0   job_0_1   job_1_2
        // Machine 2: job_1_1   job_0_2   job_2_1
        //
        // Task Time Intervals
        //
        // Machine 0: [0,3]     [3,5]
        // Machine 1: [0,4]     [4,6]     [7,11]    //Wrong in example, this could just be [6,10]!!
        // Machine 2: [5,6]     [6,8]     [8,11]

        assert_eq!(sol.col(columns[1]), 0.);
        assert_eq!(sol.col(columns[2]), 4.);
        assert_eq!(sol.col(columns[3]), 6.);

        assert_eq!(sol.col(columns[4]), 3.);
        assert_eq!(sol.col(columns[5]), 5.);
        assert_eq!(sol.col(columns[6]), 6.);

        assert_eq!(sol.col(columns[7]), 0.);
        assert_eq!(sol.col(columns[8]), 8.);
    }
}
