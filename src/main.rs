use todo_cli::{manage_tasks::*, file_management::*, Args, Commands};
use clap::Parser;

// TODO: 
// 1. make it possible for user to change task status
// 2. start writing TESTS (very important)
// 3. make it possible to order tasks
// 4. make the status an enum
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
