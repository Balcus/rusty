use clap::{Parser, Subcommand};
use chrono::NaiveDate;

#[derive(Parser, Debug)]
#[command(
    author = "Balcus Bogdan",
    version = "0.0.1",
    about = "Terminal TODO App",
    long_about = None
)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Used to add a Task to the list
    Add {
        /// name of taks to be added
        name: String,

        /// Importance of the added task (optional)
        #[arg(short = 'i', long = "importance", default_value_t = 5)]
        importance: u32,

        /// Completion date of the task, format: YYYY-MM-DD (optional)
        #[arg(short = 'd', long = "completion_date")]
        completion_date: Option<String>
    },

    Delete {
        /// Name of the task to be deleted
        task: String,
    }
}

pub struct Task {
    name: String,
    importance: u32,
    completion_date: Option<NaiveDate>
}