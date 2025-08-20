use std::io::{self, Write};
use std::result::Result;

mod task_manager;
use task_manager::init_db;


fn main() {


    //create DB
    task_manager::init_db().expect("Failed to initialize database");
    loop {
        println!("\n==========================");
        println!(" Task Manager - Main Menu ");
        println!("==========================");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Update Task");
        println!("4. Delete Task");
        println!("5. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed before input

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice = choice.trim();
        match choice {
            "1" => task_manager::add_task(),
            "2" => task_manager::view_tasks(),
            "3" => println!("(Update Task - Not implemented yet)"),
            "4" => task_manager::delete_task(),
            "5" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
        /* 
        //For get id for delete task
        let mut delete_Id = String::new();
        println!("Enter task id to delete:");
        io::stdout().flush().unwrap();
        io::stdin()
        .read_line(&mut delete_Id).unwrap();
        let delete_Id:i32 = delete_Id.trim().parse().unwrap(); //convert to i32
    */

    }
    println!("Thank you for using the Task Manager!");
    
}



/*
task todo
create / update / delete / read
 */