use crate::database;
use crate::sys;

use std::{fs::{self, File}, io, io::BufReader};
use termion::input::TermRead;

const WORKSPACE_FILENAME: &str = ".workspace";

pub fn get_workspace() -> io::Result<String> {
    let workspace_file_path = sys::get_project_file_path(WORKSPACE_FILENAME);
    let workspace_file = match File::open(workspace_file_path) {
        Ok(workspace_file) => workspace_file,
        Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unable to open workspace file"))
    };

    let mut buffer = BufReader::new(workspace_file);
    let workspace = buffer
        .read_line()
        .expect("Unable to read workspace")
        .unwrap();
    
    Ok(workspace)
}

pub fn set_workspace(workspace_name: &str) {
    unset_workspace();
    
    if workspace_name == "default" {
        /*
        * When setting the default db is enough to just unset the current ones
        */
        return;
    }

    fs::write(sys::get_project_file_path(WORKSPACE_FILENAME), workspace_name)
        .expect("Unable to write workspace name into workspace file");
}

pub fn unset_workspace() {
    fs::remove_file(sys::get_project_file_path(WORKSPACE_FILENAME));
}

pub fn list_workspaces() -> io::Result<Vec<String>> {
    let read_dir_result = match fs::read_dir(sys::get_home_project_dir()) {
        Ok(result) => result,
        Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unable to read current directory to list workspaces"))
    };

    let dir_entries_names = read_dir_result.filter(|e| {
        let dir_entry = match e {
            Ok(dir_entry) => dir_entry,
            Err(_) => panic!("Unable to read current directory to list workspaces")
        };

        let is_file = dir_entry
            .file_type()
            .unwrap()
            .is_file();

        let contains_db_keyword = dir_entry
            .file_name()
            .to_str()
            .unwrap()
            .contains("db");

        is_file && contains_db_keyword
    })
    .map(|e| {
        let db_file_name = e
            .unwrap()
            .file_name()
            .into_string()
            .unwrap();
        
        if db_file_name == "db.json" {
            return String::from("default");
        }

        return String::from(
                db_file_name
                .split(".")
                .nth(1)
                .expect("Db filename malformed ")
        );
        
    })
    .collect();

    Ok(dir_entries_names)
}

pub fn remove_workspace(workspace_name: &str) {
    fs::remove_file(database::get_db_filename_from_workspace_name(workspace_name));
}
