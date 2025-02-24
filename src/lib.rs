use clap::{Parser, Subcommand};
use chrono::NaiveDate;

#[derive(Parser, Debug)]
#[command(
    author = "Balcus Bogdan",
    version = "0.0.1",
    about = "Terminal TODO App",
    long_about = None
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Used to add a Task to the list
    Add {
        /// Name of taks to be added
        name: String,

        /// Importance of the added task (optional)
        #[arg(short = 'i', long = "importance", default_value_t = 5)]
        importance: u32,

        /// Completion date of the task, format: YYYY-MM-DD (optional)
        #[arg(short = 'c', long = "completion_date")]
        completion_date: Option<String>
    },

    Delete {
        /// Name of the task to be deleted
        name: String,
    }
}

#[derive(Debug)]
pub struct Task {
    name: String,
    importance: u32,
    completion_date: Option<NaiveDate>
}

impl Task {
    pub fn new(name: String, importance: u32, completion_date: Option<NaiveDate>) -> Task {
        Task {
            name,
            importance,
            completion_date
        }
    }
}