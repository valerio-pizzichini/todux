use crate::sys;
use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::prelude::*;

const LOG_FILENAME: &str = "todux.log";

pub fn info(tag: String, message: String) {
    let mut log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(sys::get_project_file_path(LOG_FILENAME))
        .unwrap();

    let date = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    if let Err(e) = writeln!(log_file, "[{}][{}] {}", date, tag, message) {
        eprintln!("Couldn't write log to file: {}", e);
    }
}
