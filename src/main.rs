use chrono::NaiveDate;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "schedule manager")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    create_task: CreateCommand,
}

#[derive(Subcommand)]
#[command(about = "create a new task")]
enum CreateCommand {
    Create {
        #[arg(short = 'n', long = "task-name")]
        name: String,
        #[arg(short = 'd', long = "echeance-date", value_parser = parse_date_from_str)]
        echeance_date: NaiveDate,
    },
}

fn parse_date_from_str(date_str: &str) -> Result<NaiveDate, String> {
    let _date_format = "%Y-%m-%d";

    match NaiveDate::parse_from_str(date_str, _date_format) {
        Ok(parsed_date) => Ok(parsed_date),
        Err(e) => Err(format!(
            "Error: Invalid date format or values. Expected YYYY-MM-DD. Details: {}",
            e
        )),
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.create_task {
        CreateCommand::Create {
            name,
            echeance_date,
        } => {
            println!("taks name : {name}");
            println!("echeance date : {echeance_date}");
        }
    }
}
