use crate::components::Todo;
use chrono::{DateTime, Utc};
use rusqlite::OptionalExtension;
use rusqlite::{params, Connection, Result as SqliteResult};
use std::error::Error;
use std::path::Path;

pub trait Storage {
    fn add(&self, todo: &Todo) -> Result<(), Box<dyn Error>>;
    fn get(&self, id: u64) -> Result<Option<Todo>, Box<dyn Error>>;
    fn list(&self) -> Result<Vec<Todo>, Box<dyn Error>>;
    fn update(&self, todo: &Todo) -> Result<(), Box<dyn Error>>;
    fn delete(&self, id: u64) -> Result<(), Box<dyn Error>>;
}
use std::sync::Mutex;

pub struct SqliteStorage {
    conn: Mutex<Connection>,
}

impl SqliteStorage {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                completed BOOLEAN NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;
        Ok(SqliteStorage {
            conn: Mutex::new(conn),
        })
    }
}

impl Storage for SqliteStorage {
    fn add(&self, todo: &Todo) -> Result<(), Box<dyn Error>> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO todos (title, description, completed, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                todo.title,
                todo.description,
                todo.completed,
                todo.created_at.to_rfc3339(),
                todo.updated_at.to_rfc3339()
            ],
        )?;
        Ok(())
    }

    fn get(&self, id: u64) -> Result<Option<Todo>, Box<dyn Error>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM todos WHERE id = ?1")?;
        let todo = stmt.query_row(params![id], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                completed: row.get(3)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        });

        match todo {
            Ok(todo) => Ok(Some(todo)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn list(&self) -> Result<Vec<Todo>, Box<dyn Error>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM todos")?;
        let todos = stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                completed: row.get(3)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        })?;
        todos
            .collect::<SqliteResult<Vec<Todo>>>()
            .map_err(|e| e.into())
    }

    fn update(&self, todo: &Todo) -> Result<(), Box<dyn Error>> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE todos SET title = ?1, description = ?2, completed = ?3, updated_at = ?4 WHERE id = ?5",
            params![
                todo.title,
                todo.description,
                todo.completed,
                todo.updated_at.to_rfc3339(),
                todo.id
            ],
        )?;
        Ok(())
    }

    fn delete(&self, id: u64) -> Result<(), Box<dyn Error>> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM todos WHERE id = ?1", params![id])?;
        Ok(())
    }
}
