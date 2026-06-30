# Rust CLI To-Do List

A simple command-line to-do list application written in Rust. This project was built as a way to practice Rust fundamentals, including ownership, borrowing, structs, enums, pattern matching, vectors, and error handling.

## Features

* Add new tasks
* List all tasks
* Mark tasks as completed
* Delete tasks
* Input validation for user choices
* Automatically assigns unique task IDs

## Technologies

* Rust
* Rust Standard Library (`std::io`)

## What I Learned

Building this project helped me become familiar with several core Rust concepts:

* Structs and enums
* Implementing methods with `impl`
* Ownership and borrowing
* Mutable and immutable references
* Pattern matching with `match`
* Working with `Option` and `Result`
* Iterators (`find`, `position`)
* Error handling
* User input with `stdin`
* Organizing program logic into functions

## Running the Project

Clone the repository:

```bash
git clone <repository-url>
cd <repository-name>
```

Run the application:

```bash
cargo run
```

Or build an optimized release version:

```bash
cargo build --release
```

## Example

```text
Welcome to your task manager!

1. Add Task
2. List Tasks
3. Delete Task
4. Complete Task
5. Quit
```
## Project Goal

This project was created as a learning exercise while studying Rust. The focus was on understanding the language's ownership model and writing clean, idiomatic Rust code rather than building a production-ready application.
