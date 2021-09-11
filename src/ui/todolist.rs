use crate::utils::StatefulList;
use crate::todo::{ Todo, TodoData };

pub struct TodoList {
    pub items: StatefulList<Todo>
}

impl<'a> TodoList {
    pub fn new(data: TodoData) -> TodoList {
        TodoList {
            items: StatefulList::with_items(data.todos)
        }
    }
}
