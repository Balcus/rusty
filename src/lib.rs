use core::fmt;
use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use colored::Colorize;

pub mod file_logic;
pub mod task_logic;

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
        /// Name of taks to be added (required)
        name: String,

        /// Importance of the added task (optional)
        #[arg(short = 'i', long = "importance", default_value_t = 5)]
        importance: u8,

        /// Expected completion date for task, format: YYYY-MM-DD (optional)
        #[arg(short = 'c', long = "completion_date")]
        completion_date: Option<String>,

        /// Status of the given task (optional)
        #[arg(short = 's', long = "status", default_value_t = String::from("waiting"))]
        status: String,
    },

    /// Used to delete a task from the list
    Delete {
        /// Name of the task to be deleted
        name: String,
    },

    /// Used to list tasks
    List, 
}    
