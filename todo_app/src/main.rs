mod todo;

use std::io::{self};
use console::{style, Emoji};
use dialoguer::{theme::ColorfulTheme, Input};
use rusqlite::{params, Connection, Result };


fn main() -> Result<()>{
    // open the database if exists, if not create it
    let conn = Connection::open("todo_list.db")?;
    todo::ToDoItem::create_table(&conn)?;

    let mut task_list = todo::ToDoItem::task_fetch_all(&conn)?;

    println!("Task list");
    if task_list.is_empty() {
        println!("Task list is empty, consider adding a task");
    }


    // loop {
    //     println!("");
    //     println!("============================");
    //     println!(" My TODO app");
    //     println!("============================");
    //     println!("1. Add item");
    //     println!("2. List items");
    //     println!("3. Complete item");
    //     println!("4. Exit");
    //     println!("============================");
    // }
    
    Ok(())

}






















    // let mut todo_list = todo::TodoList::new();

    // println!("{} {}", Emoji("✅", ""), style(" Your TODO list").green(),);

    // let input: String = Input::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Please enter a menu choice:")
    //     .interact_text()
    //     .unwrap();

    // println!("Email: {}", input);

    // let mail: String = Input::with_theme(&ColorfulTheme::default())
    // .with_prompt("Your planet")
    // .default("Earth".to_string())
    // .interact_text()
    // .unwrap();

    // println!("Planet: {}", mail);

    
    // loop {
    //     println!("");
    //     println!("============================");
    //     println!(" My TODO app");
    //     println!("============================");
    //     println!("1. Add item");
    //     println!("2. List items");
    //     println!("3. Complete item");
    //     println!("4. Exit");
    //     println!("============================");


    //     println!("Enter menu choice: ");



    //     let mut choice = String::new();
    //     io::stdin().read_line(&mut choice).expect("Incorrect input");
    //     let choice: u32 = match choice.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue
    //     };
    //     println!("============================");
    //     println!("");


    //     match choice {
    //         1 => {
    //             println!("============================");
    //             println!("Enter the tile of the new item: ");
    //             let mut title = String::new();
    //             io::stdin().read_line(&mut title).expect("Failed to read line");
    //             todo_list.add_item(title.trim().to_string());
    //             // println!("============================");
    //             println!("");


    //         }
    //         2 => {
    //             todo_list.list_items();
    //         }
    //         3 => {
    //             todo_list.list_items();
    //             println!("Enter the ID of the completed item: ");
    //             let mut id = String::new();
    //             io::stdin().read_line(&mut id).expect("Failed to read line");
                
    //             let id: u64 = match id.trim().parse() {
    //                 Ok(num) => num,
    //                 Err(_) => continue
    //             };
    //             todo_list.complete_item(id);
    //         }
    //         4 => {
    //             println!("Exiting the program");
    //             break;
    //         }
    //         _ => {
    //             println!("Invalid choice, please enter a number between 1 and 4...")
    //         }
    //     }
    // }