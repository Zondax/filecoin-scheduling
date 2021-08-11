use chrono::{DateTime, NaiveDateTime, Utc};
use tui::widgets::TableState;

use crate::MonitorInfo;

pub const TASK_NUM_COLUMNS: usize = 4;

#[derive(Clone)]
pub struct TableItems {
    pub job_id: String,
    pub devices: Vec<String>,
    pub end: String,
    pub last_seen: String,
}

impl Default for TableItems {
    fn default() -> Self {
        TableItems {
            job_id: "".to_string(),
            devices: vec!["".to_string()],
            end: "".to_string(),
            last_seen: "".to_string(),
        }
    }
}

pub struct TaskTable {
    pub state: TableState,
    pub items: Vec<TableItems>,
}

impl TaskTable {
    pub fn new() -> TaskTable {
        TaskTable {
            state: TableState::default(),
            items: vec![],
        }
    }

    pub fn update(&mut self, info: &MonitorInfo) {
        // job, gpus, end, last_seen
        let mut root = vec![];

        for job_id in info.job_plan.iter() {
            info.task_states
                .iter()
                .filter(|job| job.id == *job_id)
                .for_each(|job| {
                    let job_id = if job.stalled {
                        format!("{} **", job.id)
                    } else {
                        format!("{}", job.id)
                    };
                    let end = if let Some(d) = job.deadline {
                        d.end.format("%H:%M:%S").to_string()
                    } else {
                        "".to_string()
                    };

                    let last_seen = DateTime::<Utc>::from_utc(
                        NaiveDateTime::from_timestamp(job.last_seen as _, 0),
                        Utc,
                    )
                    .time()
                    .to_string();
                    let mut ids = vec![];
                    for id in job.alloc.devices.as_slice() {
                        ids.push(id.0.to_string());
                    }
                    root.push(TableItems {
                        job_id,
                        devices: ids,
                        end,
                        last_seen,
                    });
                });
        }
        if !root.is_empty() {
            self.items = root;
        } else {
            self.items = vec![Default::default(); TASK_NUM_COLUMNS];
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
