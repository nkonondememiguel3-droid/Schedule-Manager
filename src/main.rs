mod model;
mod parser;
mod service;
use clap::Parser;

use crate::parser::parser::{Cli, Commands, Mode};
use model::task::Task;
use service::task_manager::TaskManager;

// TODO: will remove this when we're done with the error handling when opening a file.
fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("tasks.json")?; // TODO: handle the errors later.

    let task_manager = TaskManager::new();

    match &cli.command {
        Commands::Create {
            name,
            echeance_date,
        } => {
            let new_task = Task::new(name.to_string(), *echeance_date);
            match new_task {
                Ok(new_task) => {
                    println!("{new_task}");
                    let _ = task_manager.save_task(&new_task, file); // TODO: handle the errors later.
                }
                Err(_) => {
                    // TODO:: handle the error later.
                    eprintln!(
                        "Their is an error in the creation of your task.\nIt will ba handle in the later version of the tool.\nSorry for that."
                    );
                }
            }
        }

        Commands::List { list } => match list {
            Mode::All => {
                println!("all");
                match task_manager.load_tasks(file) {
                    Ok(tasks) => {
                        for taks in tasks {
                            println!("{taks}")
                        }
                    }
                    Err(_) => {
                        // TODO: handle the error later.
                        eprintln!("Failed to load the tasks.");
                    }
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

    // TODO: will remove this when we're done with the error handling when opening a file.
    Ok(())
}
