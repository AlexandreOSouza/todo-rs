use std::collections::HashMap;

pub mod args;

pub struct Todo {
    pub todo: HashMap<bool, String>,
}

impl Todo {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            todo: HashMap::new(),
        })
    }

    pub fn create(&mut self, new_todo: String) {
        println!("{:?}", new_todo);
        self.todo.insert(false, new_todo);
    }

    pub fn mark(&mut self, todo: Option<String>) {
        println!("{:?}", todo);
    }

    pub fn unmark(&mut self, todo: Option<String>) {
        println!("{:?}", todo);
    }

    pub fn list(&mut self, mark: bool, unmark: bool) {
        println!("list marks: {:?} | unmarks: {:?}", mark, unmark);
    }

    pub fn delete(&self, todo: Option<String>) {
        println!("Delete {:?}", todo);
    }
}
