use tui::widgets::TableState;

use crate::MonitorInfo;

const NUM_COLUMNS: usize = 7;

pub struct GpuTable {
    pub state: TableState,
    pub items: Vec<Vec<String>>,
}

impl GpuTable {
    pub fn new(num: usize) -> GpuTable {
        GpuTable {
            state: TableState::default(),
            items: vec![vec!["".to_string(); NUM_COLUMNS]; num],
        }
    }

    pub fn update(&mut self, info: &MonitorInfo) {
        use std::collections::HashMap;
        // gpu, name, memory, in_use, is_busy, num_jobs, current_job
        let mut root = vec![];
        let mut num_jobs = HashMap::new();
        for job in info.task_states.iter() {
            job.alloc.devices.iter().for_each(|id| {
                let counter = num_jobs.entry(id.to_string()).or_insert(0);
                *counter += 1;
            })
        }

        for resource in info.resources.iter() {
            let mut row = vec![];
            let njobs = format!(
                "{}",
                num_jobs.get(&resource.device_id.to_string()).unwrap_or(&0)
            );
            let mut current_job = "".to_string();
            for id in info.job_plan.iter() {
                if info
                    .task_states
                    .iter()
                    .filter(|job| job.id == *id)
                    .any(|job| {
                        job.alloc
                            .devices
                            .iter()
                            .any(|dev_id| *dev_id.to_string() == resource.device_id)
                    })
                {
                    current_job = format!("{}", id);
                    break;
                }
            }
            row.push(format!("{}", resource.device_id));
            row.push(resource.name.clone());
            row.push(format!("{}", resource.memory));
            row.push(format!("{}", resource.mem_usage));
            row.push(format!("{}", resource.is_busy));
            row.push(njobs.to_string());
            row.push(current_job);

            root.push(row);
        }
        if !root.is_empty() {
            self.items = root;
        } else {
            self.items = vec![vec!["".to_string(); NUM_COLUMNS]; NUM_COLUMNS];
        }
    }

    #[allow(dead_code)]
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    #[allow(dead_code)]
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
