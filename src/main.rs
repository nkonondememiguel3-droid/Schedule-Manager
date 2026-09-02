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

// mod service;
// mod parser;
// mod model;
// mod error;
// 
// use rusqlite::{Connection, Result, params};
// 
// #[derive(Debug)]
// struct Person {
//     id: u32,
//     name: String,
//     data: Option<Vec<u8>>,
// }
// 
// #[derive(Debug)]
// struct PersonDto {
//     name: String,
//     data: Option<Vec<u8>>,
// }
// 
// fn main() -> Result<()> {
//     // let conn: Connection = Connection::open_in_memory()?;
//     let conn: Connection = Connection::open("./person")?;
// 
//     // create the database.
//     conn.execute(
//         "create table if not exists person ( id integer primary key autoincrement, name varchar(100) not null, data blob)",
//         (),
//     )?;
// 
//     let me = Person {
//         id: 0,
//         name: "NKONO NDEME Miguel".to_string(), // convert the value into a string.
//         data: None,
//     };
// 
//     // make insertions into the database.
//     conn.execute(
//         " insert into person (id, name, data) values (?1, ?2, ?3)",
//         (&me.id, &me.name, &me.data),
//     )?;
// 
//     // retrieve data from the data source.
//     let mut stmt = conn.prepare(" select id, name, data from person ")?;
//     let person_iter = stmt.query_map([], |row| {
//         Ok(PersonDto {
//             name: row.get(1)?,
//             data: row.get(2)?,
//         })
//     })?;
// 
//     for person in person_iter {
//         println!("Found person {:?}", person?)
//     }
// 
//     // try deleting everything in the database.
//     conn.execute( "delete from person", () )?;
// 
//     Ok(())
// }
