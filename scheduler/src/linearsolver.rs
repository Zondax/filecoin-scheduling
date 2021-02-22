#[cfg(test)]
mod tests {
    use coin_cbc::{raw::Status, Model, Sense};
    use itertools::Itertools;

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

    #[derive(Clone, Debug)]
    pub struct Job(u32, i32);

    pub const BIG_NUM: f64 = 10000 as f64;

    pub fn flatten_jobs(input_data: Vec<Vec<Job>>) -> Vec<(u32, i32, usize)> {
        let mut result = vec![];
        let mut index: usize = 1;
        for joblist in input_data.iter() {
            for job in joblist.iter() {
                result.push((job.0, job.1, index.clone()));
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

        //c_{ij} + d_j <= c_{i,j+1}

        for joblist in jobs_data.iter() {
            let n = columns.len();
            for _ in joblist {
                columns.push(m.add_integer());
            }
            for (i, job) in joblist.iter().enumerate() {
                //c_{ij} + d_j <= makespan
                let duration: f64 = -job.1 as f64;
                row = m.add_row();
                m.set_row_upper(row, duration.clone());
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
