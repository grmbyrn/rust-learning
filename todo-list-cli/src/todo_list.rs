use crate::todo::Todo;

pub struct TodoList {
    pub todos: Vec<Todo>,
    pub next_id: u32, // <-- make this field public
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, title: String) {
        let todo = Todo::new(self.next_id, title);
        self.todos.push(todo);
        self.next_id += 1;
    }

    pub fn list(&self) {
        for todo in &self.todos {
            println!(
                "[{}] {} - {}",
                if todo.completed { "x" } else { " " },
                todo.id,
                todo.title
            );
        }
    }

    pub fn complete(&mut self, id: u32) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.complete();
        }
    }

    pub fn uncomplete(&mut self, id: u32) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.uncomplete();
        }
    }

    pub fn remove(&mut self, id: u32) {
        self.todos.retain(|t| t.id != id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_list() {
        let mut list = TodoList::new();
        list.add("Test task".to_string());
        assert_eq!(list.todos.len(), 1);
        assert_eq!(list.todos[0].title, "Test task");
        assert!(!list.todos[0].completed);
    }

    #[test]
    fn test_complete_and_uncomplete() {
        let mut list = TodoList::new();
        list.add("Task".to_string());
        let id = list.todos[0].id;
        list.complete(id);
        assert!(list.todos[0].completed);
        list.uncomplete(id);
        assert!(!list.todos[0].completed);
    }

    #[test]
    fn test_remove() {
        let mut list = TodoList::new();
        list.add("Task 1".to_string());
        let id = list.todos[0].id;
        list.remove(id);
        assert!(list.todos.is_empty());
    }
}