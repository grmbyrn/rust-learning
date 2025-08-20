mod todo;
mod todo_list;
mod storage;

use std::io::{self, Write};
use todo_list::TodoList;

const SAVE_PATH: &str = "todos.json";

fn main() {
    let mut todo_list = TodoList::new();
    let _ = todo_list.load(SAVE_PATH);

    loop {
        print!("\nCommands: add <title>, list, complete <id>, uncomplete <id>, remove <id>, quit\n> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();

        match parts[0] {
            "add" => {
                if let Some(title) = parts.get(1) {
                    todo_list.add(title.to_string());
                    println!("Added: {}", title);
                    let _ = todo_list.save(SAVE_PATH);
                } else {
                    println!("Usage: add <title>");
                }
            }
            "list" => todo_list.list(),
            "complete" => {
                if let Some(id_str) = parts.get(1) {
                    if let Ok(id) = id_str.parse() {
                        todo_list.complete(id);
                        println!("Marked as complete: {}", id);
                        let _ = todo_list.save(SAVE_PATH);
                    } else {
                        println!("Invalid id");
                    }
                }
            }
            "uncomplete" => {
                if let Some(id_str) = parts.get(1) {
                    if let Ok(id) = id_str.parse() {
                        todo_list.uncomplete(id);
                        println!("Marked as incomplete: {}", id);
                        let _ = todo_list.save(SAVE_PATH);
                    } else {
                        println!("Invalid id");
                    }
                }
            }
            "remove" => {
                if let Some(id_str) = parts.get(1) {
                    if let Ok(id) = id_str.parse() {
                        todo_list.remove(id);
                        println!("Removed: {}", id);
                        let _ = todo_list.save(SAVE_PATH);
                    } else {
                        println!("Invalid id");
                    }
                }
            }
            "quit" => break,
            _ => println!("Unknown command"),
        }
    }
}
