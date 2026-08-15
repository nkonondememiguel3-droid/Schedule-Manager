mod parser;
use clap::Parser;

use crate::parser::parser::{Cli, Commands, Mode};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create {
            name,
            echeance_date,
        } => {
            println!("taks name : {name}");
            println!("echeance date : {echeance_date}");
        }
        Commands::List { list } => match list {
            Mode::All => {
                println!("all");
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
