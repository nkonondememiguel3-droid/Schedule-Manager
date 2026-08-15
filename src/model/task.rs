use chrono::NaiveDate;
use chrono::prelude::*;
use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    task_name: String,
    completed: bool,
    create_date: NaiveDate,
    echeance_date: NaiveDate,
    completed_date: Option<NaiveDate>,
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
            // TODO: handle error
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

// TODO: will see later how to make this a global error system.
#[derive(Debug)]
pub enum CreationError {
    WrongEcheanceDate,
}
