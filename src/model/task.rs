use chrono::NaiveDate;
use chrono::prelude::*;
use core::fmt;
use serde::{Deserialize, Serialize};

use crate::error::creation_error::CreationError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub task_name: String,
    pub completed: bool,
    pub create_date: NaiveDate,
    pub echeance_date: NaiveDate,
    pub completed_date: Option<NaiveDate>,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let done_str = match self.completed_date {
            Some(date) => date.to_string(),
            None => "N/A".to_string(),
        };

        write!(
            f,
            "Task: '{}' [Status: {}] (Created: {}, Done: {}, Due: {})",
            self.task_name,
            if self.completed { "Done" } else { "Pending" },
            self.create_date,
            done_str,
            self.echeance_date
        )
    }
}

impl Task {
    pub fn new(task_name: String, echeance_date: NaiveDate) -> Result<Self, CreationError> {
        if echeance_date < Utc::now().date_naive() {
            return Err(CreationError::WrongEcheanceDate);
        }

        Ok(Self {
            task_name,
            completed: false,
            create_date: Utc::now().date_naive(),
            echeance_date,
            completed_date: None,
        })
    }
}
