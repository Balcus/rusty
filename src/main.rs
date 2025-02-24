use chrono::NaiveDate;
use todo_cli::{Task, Commands, Args};
use clap::Parser;

fn main() {
    let mut todo_list: Vec<Task> = Vec::new();

    let args = Args::parse();
    match args.command {
        Commands::Add{name, importance, completion_date} => {
            match completion_date {
                Some(date) => {
                    match NaiveDate::parse_from_str(&date, "%Y-%m-%d"){
                        Ok(date) => {
                            let entry = Task::new(name, importance, Some(date));
                            todo_list.push(entry);
                        }
                        Err(err) => {
                            println!("Unsuported date format! Correct date format: YYYY-MM-DD");
                            return;
                        }
                    }

                }
                None => {
                    let entry = Task::new(name, importance, Option::None);
                    todo_list.push(entry);
                }
            }
        },
        Commands::Delete{name} => {
            println!("Task {} deleted!", name);
        }
    }

    println!("{:#?}", todo_list);
}
