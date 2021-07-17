use crate::todo::TodoData;
use std::fs::{self, File};

const DB_FILENAME: &str = "db.json";

pub fn read() -> TodoData {
    let db_file = File::open(DB_FILENAME)
        .expect("Error while reading db file");

    let db: TodoData = serde_json::from_reader(db_file)
        .expect("Invalid json");

    db
}

pub fn save(todo: TodoData) {
    let db_content = serde_json::to_string(&todo)
        .expect("Error serializing database");

    fs::write(DB_FILENAME, db_content).expect("Error while saving database")
}
