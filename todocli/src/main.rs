mod components;

use chrono::Utc;
use components::{load_todos, print_banner, print_todos_table, save_todos, Command, Todo};
use std::path::PathBuf;
use structopt::StructOpt;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        print_banner();
        Command::clap().print_help().unwrap();
        return;
    }

    let cmd = Command::from_args();
    let file_path = PathBuf::from("todos.json");

    match cmd {
        Command::Add { title, description } => {
            let mut todos = load_todos(&file_path);
            let id = todos.len() as u64 + 1;
            let now = Utc::now();
            let todo = Todo {
                id,
                title,
                description,
                completed: false,
                created_at: now,
                updated_at: now,
            };
            todos.push(todo);
            save_todos(&file_path, &todos);
            println!("Todo added successfully!");
        }
        Command::List => {
            let todos = load_todos(&file_path);
            if todos.is_empty() {
                println!("No todos found.");
                return;
            }
            print_todos_table(&todos);
        }
        Command::Complete { id } => {
            let mut todos = load_todos(&file_path);
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.completed = true;
                todo.updated_at = Utc::now();
                save_todos(&file_path, &todos);
                println!("Todo marked as completed!");
            } else {
                println!("Todo not found!");
            }
        }
        Command::Delete { id } => {
            let mut todos = load_todos(&file_path);
            todos.retain(|t| t.id != id);
            save_todos(&file_path, &todos);
            println!("Todo deleted successfully!");
        }
    }
}
