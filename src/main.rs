mod error;
mod model;
mod parser;
mod service;
use clap::Parser;

use crate::parser::parser::{Cli, Commands, Mode};
use model::task::Task;
use service::task_manager::TaskManager;

fn main() {
    let cli = Cli::parse();

    let file = match std::fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open("tasks.json")
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Critical error: Could not open or create 'tasks.json': {e}");
            std::process::exit(1);
        }
    };

    let mut task_manager = TaskManager::new(file);

    match &cli.command {
        Commands::Create {
            name,
            echeance_date,
        } => {
            match Task::new(name.to_string(), *echeance_date) {
                Ok(new_task) => {
                    println!("successfully created : {new_task}"); // TODO: will need login later
                    // for monitoring
                    if let Err(e) = task_manager.save_task(&new_task) {
                        eprintln!("Failed to save the task to the file: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Task creation failed: {e}");
                }
            }
        }

        Commands::List { list } => match list {
            Mode::All => {
                if let Err(e) = task_manager.list_tasks() {
                    eprintln!("Failed to load or list the tasks: {}", e);
                }
            }
            Mode::Completed => {
                println!("completed");
            }
            Mode::Uncompleted => {
                println!("uncompleted");
            }
        },
    }
}
