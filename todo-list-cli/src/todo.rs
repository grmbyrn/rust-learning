use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: u32, title: String) -> Self {
        Todo {
            id,
            title,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }

    pub fn uncomplete(&mut self) {
        self.completed = false;
    }
}