use chrono::NaiveDate;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "schedule manager")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "create a new task.")]
    Create {
        #[arg(short = 'n', long = "task-name")]
        name: String,
        #[arg(short = 'd', long = "echeance-date", value_parser = parse_date_from_str)]
        echeance_date: NaiveDate,
    },

    #[command(about = "list all the tasks.")]
    List {
        #[arg(value_enum, default_value_t)]
        list: Mode,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default)]
pub enum Mode {
    #[default]
    All,
    Completed,
    Uncompleted,
}

pub(crate) fn parse_date_from_str(date_str: &str) -> Result<NaiveDate, String> {
    // let _date_format = "%Y-%m-%d";
    let _date_format = "%Y-%m-%dT%H:%M:%S%z";

    match NaiveDate::parse_from_str(date_str, _date_format) {
        Ok(parsed_date) => Ok(parsed_date),
        Err(e) => Err(format!(
            "Error: Invalid date format or values. Expected YYYY-MM-DD H:M:S. Details: {}",
            e
        )),
    }
}
