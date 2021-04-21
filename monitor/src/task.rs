use chrono::{DateTime, NaiveDateTime, Utc};
use tui::widgets::TableState;

use crate::MonitorInfo;

pub const TASK_NUM_COLUMNS: usize = 4;

pub struct TaskTable {
    pub state: TableState,
    pub items: Vec<Vec<String>>,
}

impl TaskTable {
    pub fn new() -> TaskTable {
        TaskTable {
            state: TableState::default(),
            items: vec![vec!["".to_string(); TASK_NUM_COLUMNS]; TASK_NUM_COLUMNS],
        }
    }

    pub fn update(&mut self, info: &MonitorInfo) {
        // job, gpus, end, last_seen
        let mut root = vec![];

        for job_id in info.job_plan.iter() {
            let mut row = vec![];

            info.task_states
                .iter()
                .filter(|job| job.id == *job_id)
                .for_each(|job| {
                    if job.stalled {
                        row.push(format!("{} **", job.id));
                    } else {
                        row.push(format!("{}", job.id));
                    }
                    row.push(format!("{:?}", job.alloc.resource_id.as_slice()));
                    if let Some(d) = job.deadline {
                        row.push(d.1.format("%H:%M:%S").to_string());
                    } else {
                        row.push("".to_string());
                    }
                    let dt = DateTime::<Utc>::from_utc(
                        NaiveDateTime::from_timestamp(job.last_seen as _, 0),
                        Utc,
                    )
                    .time();
                    row.push(dt.to_string());
                });

            root.push(row);
        }
        if !root.is_empty() {
            self.items = root;
        } else {
            self.items = vec![vec!["".to_string(); TASK_NUM_COLUMNS]; TASK_NUM_COLUMNS];
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
