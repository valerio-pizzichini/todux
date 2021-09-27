use crate::sys;
use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::prelude::*;

const LOG_FILENAME: &str = "todux.log";

pub fn info(tag: &str, message: &str) {
    let mut log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(sys::get_project_file_path(LOG_FILENAME))
        .unwrap();

    let date = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    log(tag, message, &mut log_file, &date);
}

fn log(
    tag: &str, 
    message: &str, 
    mut writer: impl Write,
    timestamp: &str
) {
    if let Err(e) = writeln!(writer, "[{}][{}] {}", timestamp, tag, message) {
        eprintln!("Couldn't write log to file: {}", e);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn info_should_write_log_correctly() {
        let mut res = Vec::new();
        log("TEST", "Log message", &mut res, "2021-09-28 15:15:00");
        assert_eq!(res, b"[2021-09-28 15:15:00][TEST] Log message\n");
    }
}
