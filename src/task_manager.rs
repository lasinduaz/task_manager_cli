mod sql_querries;
use rusqlite::Connection;
use sql_querries::{connecttion_, insert_task, view_tasks as db_view_tasks};
use std::io::{stdin, stdout, Write};


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

    let conn = connecttion_().expect("Failed to connect to database"); //call to sql funtions 
    match insert_task(&conn, title, description, due_date, status) {
        Ok(_) => println!("Task added successfully!"),
        Err(e) => println!("Failed to add task: {}", e),
    }
}

pub fn view_tasks() {
    let conn = connecttion_().expect("Failed to connect to database"); //call to sql funtions 
    if let Err(e) = db_view_tasks(&conn) {
        println!("Failed to view tasks: {}", e);
    }
}

pub fn init_db() -> rusqlite::Result<()> {
    sql_querries::init_db()
}

pub fn delete_task() {
    let mut id = String::new();
    println!("Enter task ID to delete:");
    std::io::stdin().read_line(&mut id).unwrap();
    let id: i32 = id.trim().parse().expect("Please enter a valid number");

    let conn: Connection = connecttion_().expect("Failed to connect to database"); //call to sql funtions 
    sql_querries::delete_task(&conn, id);       //SQL stment

    /*
    match sql_querries::delete_task(&conn, id) {
        Ok(rows_deleted) => {
            if rows_deleted > 0 {
                println!("Task deleted successfully!");
            } else {
                println!("Task not found!");
            }
        }
        Err(e) => println!("Failed to delete task: {}", e),
    }
     */
}

pub fn update_task() {
    let conn = connecttion_().expect("Failed to connect to database"); //call to sql funtions
    let mut id = String::new();
    println!("Enter task ID to update:");
    std::io::stdin().read_line(&mut id).unwrap();
    let id: i32 = id.trim().parse().expect("Please enter a valid number");

    // Gets updates from user
    let mut update_title = String::new();
    print!("Enter task title: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut update_title).unwrap();
    let title = update_title.trim();

    let mut update_description = String::new();
    print!("Enter task description: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut update_description).unwrap();
    let description = update_description.trim();

    let mut update_due_date = String::new();
    print!("Enter task due date: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut update_due_date).unwrap();
    let due_date = update_due_date.trim();

    let mut update_status = String::new();
    print!("Enter task status: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut update_status).unwrap();
    let status = update_status.trim();

/*
    match sql_querries::update_task(&conn, id, title, description, due_date, status) {
        Ok(rows_updated) => {
            if rows_updated > 0 {
                println!("Task updated successfully!");
            } else {
                println!("Task not found!");
            }
        }
        Err(e) => println!("Failed to update task: {}", e),
    }
*/
}