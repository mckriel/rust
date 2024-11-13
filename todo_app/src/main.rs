mod todo;
use crossterm::{execute, terminal::{Clear, ClearType}};
use std::io::stdout;
use anyhow::Result;
use console::Emoji;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use rusqlite::Connection;
use todo::ToDoItem;

fn display_menu() -> Result<()> {
    // open the database if exists, if not create it
    let conn = Connection::open("todo_list.db")?;
    todo::ToDoItem::create_table(&conn)?;
    
    loop {
        clear_console()?;
        let (tasks_incomplete, tasks_complete) = todo::ToDoItem::task_fetch_all_by_completion(&conn)?;
    
        println!("\n--- {} TODO LIST ---\n", Emoji("✅", ""));
        display_tasks(tasks_incomplete, tasks_complete);
        
        println!("\n--- ACTIONS ---");
        let options = &[
            "1. Add task",
            "2. Complete task",
            "3. Update task",
            "4. Delete task",
            "5. Exit",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose and option: ")
            .items(options)
            .default(0)
            .interact()?;

        match selection {
            // 1. Add a task
            0 => {
                let title: String = Input::new()
                    .with_prompt("Enter task title: ")
                    .interact_text()?;
                todo::ToDoItem::task_insert(&conn, &title)?;
                println!("Task added.")
            }
            // 2. Complete a task
            1 => {
                let task_id: String = Input::new()
                    .with_prompt("Enter task ID: ")
                    .interact_text()?;

                let task_id: u64 = task_id.trim()
                    .parse::<u64>()
                    .unwrap_or_else(|_|0_u64);

                todo::ToDoItem::task_mark_complete(&conn, task_id)?;
            }
            // 3. Update a task
            2 => {
                let task_id: String = Input::new()
                    .with_prompt("Enter task ID: ")
                    .interact_text()?;

                let task_id: u64 = task_id.trim()
                    .parse::<u64>()
                    .unwrap_or_else(|_|0_u64);

                let mut task = match todo::ToDoItem::task_fetch_by_id(&conn, task_id) {
                    Some(t) => t,
                    None => {
                        println!("Task with ID {} not found", task_id);
                        return Ok(());
                    }
                };

                let new_title: String = Input::new()
                    .with_prompt("Enter new task title: ")
                    .interact_text()?;

                task.title = new_title.clone();
                todo::ToDoItem::task_update_title(&conn, task_id, &new_title.as_str())?;
            }
            // 4. Delete a task
            3 => {
                let task_id: String = Input::new()
                    .with_prompt("Enter task ID: ")
                    .interact_text()?;

                let task_id: u64 = task_id.trim()
                    .parse::<u64>()
                    .unwrap_or_else(|_|0_u64);

                todo::ToDoItem::task_delete(&conn, task_id)?;
            }
            _ => println!("some shit")
            
        }
    }
}


fn display_task_list(tasks: Vec<ToDoItem>, title: &str) {
    println!("{}", title);
    if tasks.is_empty() {
        println!("Task list empty \n");
    } else {
        for task in tasks {
            let completed_status = if task.completed { "[X]" } else { "[ ]"};
            println!("{} | {} | {}", completed_status, task.id, task.title);
        }
        println!()
    }
}

fn display_tasks(incomplete_tasks: Vec<ToDoItem>, complete_tasks: Vec<ToDoItem>) {
    display_task_list(incomplete_tasks, "-- Outstanding Tasks");
    display_task_list(complete_tasks, "-- Completed Tasks --");
}

fn clear_console() -> Result<()> {
    execute!(stdout(), Clear(ClearType::All))?;
    Ok(())
}


fn main() -> Result<()>{
    display_menu()?;
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