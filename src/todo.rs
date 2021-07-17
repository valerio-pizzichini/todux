use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TodoData {
    pub todos: Vec<Todo>
}

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub done: bool
}

impl Todo {
    pub fn toggle(&mut self) {
        self.done = !self.done
    }
}
