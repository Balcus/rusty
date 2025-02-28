use std::fs::{self, File};
use std::path::Path;
use crate::task_logic::*;

pub fn create_file() {
    let path = Path::new("./todo.json");
    if !path.exists() {
        if let Ok(_file) = File::create(path) {
            println!("File todo.json successfully created!");
        } else {
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

// #[cfg(test)]
// mod test {
//     use super::*;
// }