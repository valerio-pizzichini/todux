use crate::database;

pub fn add(description: String) {
    let mut db = database::read();
    db.todos.push(description);  
    database::save(db);
}

pub fn list() {
    let db = database::read();
    if db.todos.len() == 0 {
        println!("The todo list is empty, please add one")
    }
    for todo in db.todos {
        println!("{}", todo)
    }  
}
