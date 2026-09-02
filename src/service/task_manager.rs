use std::fs::File;
use std::io::{BufRead, BufReader, Seek, Write};

use crate::model::task::Task;
use crate::error::creation_error::CreationError;
use crate::error::loading_error::LoadingError;

pub struct TaskManager {
    tasks_loaded: Option<Vec<Task>>,
    json_file: File,
}

impl TaskManager {
    pub fn new(json_file: File) -> Self {
        Self {
            tasks_loaded: None,
            json_file,
        }
    }

    pub fn save_task(&mut self, task: &Task) -> Result<(), CreationError> {
        let mut json_string =
            serde_json::to_string(task).map_err(CreationError::SerializationError)?;

        json_string.push('\n');

        // make sure to write at the very end of the file
        self.json_file
            .seek(std::io::SeekFrom::End(0))
            .map_err(CreationError::WriteError)?;

        self.json_file
            .write_all(json_string.as_bytes())
            .map_err(CreationError::WriteError)?;

        json_string.push('\n');

        if let Some(tasks) = &mut self.tasks_loaded {
            tasks.push(task.clone());
        }

        Ok(())
    }

    pub fn list_tasks(&mut self) -> Result<(), LoadingError> {
        // load the tasks
        self.load_tasks()?;

        if let Some(tasks) = &self.tasks_loaded {
            for task in tasks {
                println!("{task}")
            }
        }

        Ok(())
    }

    fn load_tasks(&mut self) -> Result<(), LoadingError> {
        // prevent from reading from the file while the tasks are already in memory.
        if self.tasks_loaded.is_some() {
            return Ok(());
        }

        self.json_file.rewind().map_err(LoadingError::IoError)?;

        let tasks_vec = self.tasks_loaded.get_or_insert_with(Vec::new);

        let reader = BufReader::new(&self.json_file);

        for line in reader.lines() {
            let line_content = line.map_err(LoadingError::IoError)?;

            if line_content.trim().is_empty() {
                continue;
            }

            let task: Task =
                serde_json::from_str(&line_content).map_err(LoadingError::DeserializationError)?;

            tasks_vec.push(task);
        }
        Ok(())
    }
}
