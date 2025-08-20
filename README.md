# Rust Learning Roadmap 🚀

This repository is a collection of small-to-medium Rust projects I’m building as part of my journey to learn and master the Rust programming language.

The focus here is not just writing code, but approaching each project with the discipline of a professional developer:

- Clear project structure
- Documentation and testing
- Clean commit history
- Incremental improvement and refactoring

---

## 📂 Repository Structure

Each project lives in its own folder as part of a Cargo workspace:

```

rust-learning/
├── Cargo.toml          # Workspace definition
├── hello-rust/         # Project 1: Hello, Rust!
├── calculator/         # Project 2: Simple Calculator CLI
└── ...

```

You can run a specific project with:

```bash
cargo run -p <project-name>
```

For example:

```bash
cargo run -p hello-rust
```

And test it with:

```bash
cargo test -p hello-rust
```

---

## 🛣️ Roadmap

I’m following a structured learning path that starts with beginner projects and builds toward advanced and expert-level Rust development:

- **Beginner:** Hello Rust, CLI calculator, todo list
- **Intermediate:** Weather CLI, Markdown → HTML converter, static site generator, file downloader
- **Advanced:** REST API server, terminal UI app, key-value store, chat app

---

## 🧑‍💻 Goals

1. Learn Rust fundamentals and idiomatic patterns.
2. Practice professional-grade project organization and preparation.
3. Build a portfolio of Rust projects that demonstrate increasing skill.
4. Share my progress publicly for accountability and feedback.

---

## ⚡ Usage Notes

- Each project is self-contained and can be run independently.
- Some projects depend on external APIs or crates (documented in each project’s README).
- Code is formatted (`cargo fmt`) and linted (`cargo clippy`) regularly.

---

## 📜 License

This repository is for learning and sharing. Unless otherwise noted, projects are licensed under the MIT License.
