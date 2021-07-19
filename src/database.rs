use crate::todo::TodoData;
use std::fs::{self, File};

const DB_FILENAME: &str = "db.json";

pub fn read() -> TodoData {
    let db_file = File::open(DB_FILENAME);
    if let Ok(db_file) = db_file {
        let db: TodoData = serde_json::from_reader(db_file)
            .expect("Invalid json");
        db
    } else {
        create()
    }
}

fn create() -> TodoData {
    File::create(DB_FILENAME).expect("Error while creating db file");
    let db =TodoData {
        todos: vec![]
    };
    save(&db);

    db
}

pub fn save(todo: &TodoData) {
    let db_content = serde_json::to_string(&todo)
        .expect("Error serializing database");

    fs::write(DB_FILENAME, db_content).expect("Error while saving database")
}
