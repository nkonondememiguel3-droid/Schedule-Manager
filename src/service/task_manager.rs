use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};

use crate::Task;

pub struct TaskManager {
    tasks_loaded: Option<Vec<Task>>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self { tasks_loaded: None }
    }

    pub fn save_task(
        &self,
        task: &Task,
        mut json_file: File,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut json_string = serde_json::to_string(task)?;
        json_string.push('\n');

        json_file.write_all(json_string.as_bytes())?; // TODO: handle error later.
        Ok(())
    }

    pub fn load_tasks(&self, json_file: File) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        let mut tasks: Vec<Task> = Vec::new();
        let reader = BufReader::new(json_file);

        for line in reader.lines() {
            let line_content = line?;

            if line_content.trim().is_empty() {
                continue;
            }

            let task = serde_json::from_str(&line_content)?;
            tasks.push(task);
        }

        Ok(tasks)
    }
}
