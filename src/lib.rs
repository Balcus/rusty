use std::fs::{self, File};
use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use std::path::Path;

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

    /// Used to delete a task from the list
    Delete {
        /// Name of the task to be deleted
        name: String,
    }
}

#[derive(Serialize, Deserialize ,Debug)]
pub struct Task {
    pub name: String,
    pub importance: u32,
    pub completion_date: String
}

impl Task {
    pub fn new(name: String, importance: u32, completion_date: String) -> Task {
        Task {
            name,
            importance,
            completion_date
        }
    }
}

pub fn create_file() {
    let path = Path::new("./todo.json");
    if !path.exists() {
        if let Ok(_file) = File::create(path) {
            println!("File todo.json successfully created!");
        }else {
            println!("Failed to create file todo.json!");
        }
    }
}

pub fn read_tasks_from_file() -> Vec<Task> {
    let file_content = fs::read_to_string("./todo.json").unwrap_or_else(|_| String::from("[]"));
    let deserialized: Vec<Task> = serde_json::from_str(&file_content).unwrap_or_else(|_| vec![]);
    deserialized
}

pub fn write_tasks_to_file(tasks: Vec<Task>) {
    if let Ok(serialized) = serde_json::to_string_pretty(&tasks) {
        fs::write("./todo.json", serialized).unwrap_or_else(|_| println!("Failed to write tasks to file todo.json"));
        return;   
    }
    println!("Failed to serialize task vector!");
}