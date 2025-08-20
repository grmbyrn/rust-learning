use crate::todo_list::TodoList;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter};
use serde_json;

impl TodoList {
    pub fn save(&self, path: &str) -> io::Result<()> {
        let file = OpenOptions::new().write(true).create(true).truncate(true).open(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self.todos).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    pub fn load(&mut self, path: &str) -> io::Result<()> {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return Ok(()), // If file doesn't exist, start empty
        };
        let reader = BufReader::new(file);
        self.todos = serde_json::from_reader(reader).unwrap_or_default();
        self.next_id = self.todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        Ok(())
    }
}