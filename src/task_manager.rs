mod sql_querries;
use sql_querries::{connecttion_, insert_task, view_tasks as db_view_tasks};


pub fn add_task() {
    let mut title = String::new();
    println!("Enter task title:");
    std::io::stdin().read_line(&mut title).unwrap();
    let title = title.trim();

    let mut description = String::new();
    println!("Enter task description:");
    std::io::stdin().read_line(&mut description).unwrap();
    let description = description.trim();

    let mut due_date = String::new();
    println!("Enter task due date:");
    std::io::stdin().read_line(&mut due_date).unwrap();
    let due_date = due_date.trim();

    let mut status = String::new();
    println!("What is status:");
    std::io::stdin().read_line(&mut status).unwrap();
    let status = status.trim();

    let conn = connecttion_().expect("Failed to connect to database");
    match insert_task(&conn, title, description, due_date, status) {
        Ok(_) => println!("Task added successfully!"),
        Err(e) => println!("Failed to add task: {}", e),
    }
}

pub fn view_tasks() {
    let conn = connecttion_().expect("Failed to connect to database");
    if let Err(e) = db_view_tasks(&conn) {
        println!("Failed to view tasks: {}", e);
    }
}

pub fn init_db() -> rusqlite::Result<()> {
    init_db()
}

