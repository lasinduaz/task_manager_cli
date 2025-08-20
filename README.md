# Task Manager CLI

A simple command-line application for managing tasks using Rust and SQLite.

## Features

- Add new tasks
- View all tasks
- (Planned) Update and delete tasks

## Getting Started

### Prerequisites

- Rust (https://rust-lang.org)
- SQLite (installed automatically via the `rusqlite` crate)

### Build and Run

```sh
cargo build
cargo run
```

## Project Structure

- `src/main.rs` — Entry point and menu logic
- `src/task_manager.rs` or `src/task_manager/mod.rs` — Task management functions
- `src/task_manager/sql_querries.rs` — Database functions

## Usage

Follow the on-screen menu to add, view, update, or delete tasks.

## Contributing

Feel free to open issues or pull requests!

##