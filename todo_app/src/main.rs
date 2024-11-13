use anyhow::Result;
use console::Emoji;
use dialoguer::{theme::ColorfulTheme, Select};
use rusqlite::Connection;

mod modules;

use crate::modules::ui::{ display_tasks, prompt_task_id, prompt_task_title };
use crate::modules::utils::clear_console;
use modules::todo::ToDoItem;

fn display_menu() -> Result<()> {
    // open the database if exists, if not create it
    let conn = Connection::open("todo_list.db")?;
    ToDoItem::create_table(&conn)?;
    // main loop to keep propgram running
    loop {
        clear_console()?;

        let (tasks_incomplete, tasks_complete) = ToDoItem::task_fetch_all_by_completion(&conn)?;
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
            0 => add_task(&conn)?,
            1 => complete_task(&conn)?,
            2 => update_task(&conn)?,
            3 => delete_task(&conn)?,
            4 => std::process::exit(0),
            _ => println!()
        }
    }
}

fn add_task(conn: &Connection) -> Result<()> {
    let title = prompt_task_title("Enter task title: ");
    ToDoItem::task_insert(conn, &title?)?;
    Ok(())
}

fn complete_task(conn: &Connection) -> Result<()> {
    let task_id = prompt_task_id("Enter task ID to mark as complete: ")?;
    ToDoItem::task_mark_complete(conn, task_id)?;
    Ok(())
}

fn update_task(conn: &Connection) -> Result<()> {
    let task_id = prompt_task_id("Enter task ID to update: ")?;
    if let Some(mut task) = ToDoItem::task_fetch_by_id(conn, task_id) {
        let new_title = prompt_task_title("Enter new task title: ")?;
        task.title = new_title.clone();
        ToDoItem::task_update_title(conn, task_id, &new_title)?;
    } else {
        println!("Task with ID {} not found", task_id);
    }
    Ok(())
}

fn delete_task(conn: &Connection) -> Result<()> {
    let task_id = prompt_task_id("Enter task ID to delete: ")?;
    ToDoItem::task_delete(conn, task_id)?;
    println!("Task deleted.");
    Ok(())
}

fn main() -> Result<()>{
    display_menu()?;
    Ok(())
}
