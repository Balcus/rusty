use chrono::NaiveDate;
use todo_cli::{create_file, read_tasks_from_file, write_tasks_to_file, Args, Commands, Task};
use clap::Parser;

fn main() {
    create_file();
    let mut todo_list: Vec<Task> = read_tasks_from_file();

    let args = Args::parse();
    match args.command {
        Commands::Add{name, importance, completion_date} => {
            if let Some(date_as_str) = completion_date {
                match NaiveDate::parse_from_str(&date_as_str, "%Y-%m-%d") {
                    Ok(_date) => {
                        let entry = Task {
                            name,
                            importance,
                            completion_date: date_as_str
                        };
                        todo_list.push(entry);
                    }
                    Err(_e) => {
                        println!("Failed to parse given date. Correct format: YYYY-MM-DD");
                        return;
                    }   
                }
            }else {
                let entry = Task {
                    name,
                    importance,
                    completion_date: "-".to_string()
                };
                todo_list.push(entry);
            }
        },
        
        Commands::Delete{name} => {
            println!("Task {} deleted!", name);
        }
    }

    write_tasks_to_file(todo_list);
}
