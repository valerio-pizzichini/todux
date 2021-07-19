use crate::database;
use crate::todo::Todo;

pub fn add(description: String) {
    let mut db = database::read();
    db.todos.push( Todo {
        title: description,
        done: false
    });
    database::save(&db);
}

pub fn list() {
    let db = database::read();
    if db.todos.len() == 0 {
        println!("The todo list is empty, please add one")
    }
    for todo in db.todos {
        println!("{}", todo.title)
    }  
}
