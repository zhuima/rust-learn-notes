use crate::components::todo::Todo;
use chrono::{DateTime, Utc};
use colored::*;
use prettytable::{format, Cell, Row, Table};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

pub fn load_todos(file_path: &PathBuf) -> Vec<Todo> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => return Vec::new(),
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
}

pub fn save_todos(file_path: &PathBuf, todos: &Vec<Todo>) {
    let json = serde_json::to_string_pretty(todos).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

pub fn print_banner() {
    println!(
        "
 _____         _        ____ _     ___ 
|_   _|__   __| | ___  / ___| |   |_ _|
  | |/ _ \\ / _` |/ _ \\| |   | |    | | 
  | | (_) | (_| | (_) | |___| |___ | | 
  |_|\\___/ \\__,_|\\___/ \\____|_____|___|
    "
    );
}

pub fn format_completed_status(completed: bool) -> String {
    if completed {
        "Yes".green().to_string()
    } else {
        "No".red().to_string()
    }
}

pub fn format_datetime(dt: &DateTime<Utc>) -> String {
    dt.format("%Y/%m/%d %H:%M:%S").to_string()
}

pub fn print_todos_table(todos: &[Todo]) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.set_titles(Row::new(vec![
        Cell::new("ID"),
        Cell::new("Title"),
        Cell::new("Description"),
        Cell::new("Completed"),
        Cell::new("Created"),
        Cell::new("Updated"),
    ]));

    for todo in todos {
        table.add_row(Row::new(vec![
            Cell::new(&todo.id.to_string()),
            Cell::new(&todo.title),
            Cell::new(&todo.description.as_deref().unwrap_or("N/A")),
            Cell::new(&format_completed_status(todo.completed)),
            Cell::new(&format_datetime(&todo.created_at)),
            Cell::new(&format_datetime(&todo.updated_at)),
        ]));
    }

    table.printstd();
}
