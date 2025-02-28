use todo_cli::{Args, Commands};
use clap::Parser;
use todo_cli::file_logic::{create_file, read_tasks_from_file, write_tasks_to_file};
use todo_cli::task_logic::*;

// TODO: 
// 1. make it possible for user to change task status
// 2. work on tests
// 3. make it possible to order tasks
// 4. make the status an enum (will consider later)
// 5. show them in a calendar (if possible)
// 6. make it possible to change task importance
fn main() {
    create_file();
    let mut todo_list: Vec<Task> = read_tasks_from_file();

    let args = Args::parse();
    match args.command {
        Commands::Add { name, importance, completion_date, status } => {
            add_task(&mut todo_list, name, importance, completion_date, status);
        },

        Commands::Delete { name } => {
            delete_task(&mut todo_list, name);
        },

        Commands::List => {
            list_tasks(&todo_list);
        }
    }

    write_tasks_to_file(todo_list);
}
