mod components;

use chrono::Utc;
use components::{print_banner, print_todos_table, Command, Todo, Storage, SqliteStorage};
use std::path::PathBuf;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        print_banner();
        Command::clap().print_help()?;
        return Ok(());
    }

    let cmd = Command::from_args();
    let file_path = PathBuf::from("todos.db");
    let storage = SqliteStorage::new(file_path)?;

    match cmd {
        Command::Add { title, description } => {
            let now = Utc::now();
            let todo = Todo {
                id: 0, // SQLite will auto-increment this
                title,
                description,
                completed: false,
                created_at: now,
                updated_at: now,
            };
            storage.add(&todo)?;
            println!("Todo added successfully!");
        }
        Command::List => {
            let todos = storage.list()?;
            if todos.is_empty() {
                println!("No todos found.");
                return Ok(());
            }
            print_todos_table(&todos);
        }
        Command::Complete { id } => {
            if let Some(mut todo) = storage.get(id)? {
                todo.completed = true;
                todo.updated_at = Utc::now();
                storage.update(&todo)?;
                println!("Todo marked as completed!");
            } else {
                println!("Todo not found!");
            }
        }
        Command::Delete { id } => {
            storage.delete(id)?;
            println!("Todo deleted successfully!");
        }
    }

    Ok(())
}
