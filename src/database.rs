use crate::todo::TodoData;
use std::fs::{self, File};
use crate::sys;
use crate::log;

pub fn read(db_filename: &str) -> TodoData {
    match File::open(db_filename) {
        Ok(db_file) => serde_json::from_reader(db_file).expect("Invalid json"),
        Err(_) => create(db_filename)
    }
}

fn create(db_filename: &str) -> TodoData {
    File::create(db_filename).expect("Error while creating db file");
    let db = TodoData {
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

pub fn get_db_filename_from_workspace_name(workspace_name: &str) -> String {
    let db_file_name = format!("db.{}.json", workspace_name);
    return sys::get_project_file_path(&db_file_name);
}
