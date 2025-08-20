# Todo List CLI

A simple, persistent command-line Todo List application written in Rust.

## Features

- Add new todo items
- List all todos with completion status
- Mark todos as complete or incomplete
- Remove todos by ID
- Persistent storage using JSON files
- Simple and intuitive command-line interface

## Usage

### Build and Run

```sh
cargo build --release
cargo run
```

### Commands

- `add <title>` — Add a new todo with the given title
- `list` — List all todos
- `complete <id>` — Mark the todo with the given ID as complete
- `uncomplete <id>` — Mark the todo with the given ID as incomplete
- `remove <id>` — Remove the todo with the given ID
- `quit` — Exit the application

### Example

```
> add Buy groceries
Added: Buy groceries

> list
[ ] 1 - Buy groceries

> complete 1
Marked as complete: 1

> list
[x] 1 - Buy groceries

> remove 1
Removed: 1
```

## Project Structure

```
src/
  main.rs         # CLI interface and main loop
  todo.rs         # Todo struct and logic
  todo_list.rs    # TodoList struct and operations
  storage.rs      # Persistence (save/load)
  ...
Cargo.toml        # Dependencies and metadata
README.md         # Project documentation
```

## Testing

Run unit tests with:

```sh
cargo test
```

## Extending

Ideas for future improvements:

- Edit todo titles
- Add priorities or due dates
- Colored output
- Import/export features
- Integration with other tools

## License

MIT
