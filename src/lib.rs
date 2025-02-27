use core::fmt;
use std::fs::{self, File};
use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use std::path::Path;
use chrono::NaiveDate;
use colored::Colorize;

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



pub mod file_management {
    use super::*;
    use manage_tasks::Task;

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
}
pub mod manage_tasks {
    use colored::ColoredString;

    use super::*;

    #[derive(Serialize, Deserialize ,Debug)]
    pub struct Task {
        pub name: String,
        pub importance: u8,
        pub completion_date: String,
        // TODO: status should probably be a enum, fix it !
        pub status: String,
    }

    impl Task {
        pub fn new(name: String, importance: u8, completion_date: Option<String>, status: String) -> Task {
            let valid_status = vec!["waiting", "done", "in_progress"];

            if !valid_status.contains(&status.as_str()) {
                panic!("The provided status is not supported!\nGiven status: {}, supported statuses: {:?}", status, valid_status);
            }

            match completion_date {
                Some(date_as_str) => {
                    match NaiveDate::parse_from_str(&date_as_str, "%Y-%m-%d") {
                        Ok(_date) => Task {
                            name,
                            importance,
                            completion_date: date_as_str,
                            status
                        },
                        Err(_e) => {
                            panic!("Failed at parsing date!\nPlease make sure the date format is: YYYY-MM-DD");
                        }
                    }
                },
                None => Task {
                    name,
                    importance,
                    completion_date: String::from("-"),
                    status
                }
            }
        }
    }

    impl fmt::Display for Task {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let icon: char;
            let name_colored: ColoredString;
            
            if self.status == "waiting".to_string() {
                icon = '❌';
                name_colored = self.name.bold().red();
            }else if self.status == "done".to_string() {
                icon = '✅';
                name_colored = self.name.bold().green();
            }else {
                icon = '🔧';
                name_colored = self.name.bold().blue();
            }

            write!(
                f,
                "[ {} ] {:<30} {:<5} {:<10}",
                icon, name_colored, self.importance, self.completion_date
            )
        }
    }

    pub fn add_task(list: &mut Vec<Task>, name: String, imp: u8, c_date: Option<String>, status: String) {
        let entry = Task::new(name, imp, c_date, status);
        list.push(entry);
    }

    // The delete method will delete ALL tasks with the given name
    pub fn delete_task(list: &mut Vec<Task>, name: String) {
        list.retain(|t| t.name != name);
        println!("Task {} deleted!", name);
    }

    pub fn list_tasks(list: &Vec<Task>) {
        for t in list {
            println!("{}", t);
        }
    }
}
