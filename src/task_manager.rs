use rusqlite::{params, Connection, Result};

pub fn add_task() {
    let conn = Connection::open("task_manager.db").expect("Failed to connect to DB");

    let mut title = String::new();
    println!("Enter task title:");
    std::io::stdin().read_line(&mut title).unwrap();
    let title = title.trim();


}