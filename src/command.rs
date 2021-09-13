use crate::database;
use crate::todo::Todo;

pub fn add(description: String, db_filename: &str) {
    let mut db = database::read(db_filename);
    db.todos.push( Todo {
        title: description.clone(),
        done: false
    });
    database::save(&db, db_filename);

    println!("\"{}\" added successfully [v]", description);
}

pub fn list(db_filename: &str) {
    let db = database::read(db_filename);
    if db.todos.len() == 0 {
        println!("The todo list is empty, please add one")
    }
    for todo in db.todos {
        println!("{}", todo.title)
    }  
}
