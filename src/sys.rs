use std::env;
use std::fs;

pub fn initialize() {
    let home_project_dir = get_home_project_dir();
    match fs::create_dir(&home_project_dir) {
        Ok(_) => println!("Home project dir created at: {}", home_project_dir),
        Err(_) => println!("Home project dir already exists at: {}", home_project_dir)
    }
}

pub fn get_home_project_dir() -> String {
    let mut home_dir = env::home_dir()
        .expect("Error")
        .to_str()
        .expect("Error")
        .to_owned();
    home_dir.push_str("/.todux");

    home_dir
}

pub fn get_project_file_path(file_name: &str) -> String {
    let mut path = get_home_project_dir();
    path.push_str(&format!("/{}", file_name));

    path
}
