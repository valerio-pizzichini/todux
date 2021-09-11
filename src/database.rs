use crate::todo::TodoData;
use std::fs::{self, File};

pub fn read(db_filename: &str) -> TodoData {
    let db_file = File::open(db_filename);
    if let Ok(db_file) = db_file {
        let db: TodoData = serde_json::from_reader(db_file)
            .expect("Invalid json");
        db
    } else {
        create(db_filename)
    }
}

fn create(db_filename: &str) -> TodoData {
    File::create(db_filename).expect("Error while creating db file");
    let db =TodoData {
        todos: vec![]
    };
    save(&db, db_filename);

    db
}

pub fn save(todo: &TodoData, db_filename: &str) {
    let db_content = serde_json::to_string(&todo)
        .expect("Error serializing database");

    fs::write(db_filename, db_content).expect("Error while saving database")
}

pub fn get_db_filename_from_workspace_name(workspace_name: String) -> String {
    let mut db_filename = String::from("db.").to_owned();
    db_filename.push_str(&workspace_name);
    db_filename.push_str(".json");

    db_filename
}
