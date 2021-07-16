use std::fs::{self, File};
use serde::{Deserialize, Serialize};

const DB_FILENAME: &str = "db.json";

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub todos: Vec<String>,
    pub project_name: String
}

pub fn read() -> Todo {
    let db_file = File::open(DB_FILENAME)
        .expect("Error while reading db file");

    let db: Todo = serde_json::from_reader(db_file)
        .expect("Invalid json");

    db
}

pub fn save(todo: Todo) {
    let db_content = serde_json::to_string(&todo)
        .expect("Error serializing database");

    fs::write(DB_FILENAME, db_content).expect("Error while saving database")
}
