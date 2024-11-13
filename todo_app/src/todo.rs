use core::task;

// use anyhow::Ok;
// use anyhow::Ok;
use rusqlite::{params, Connection, Result };

pub struct ToDoItem {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

impl ToDoItem {
    // DATABASE FUNCTIONS START
    // create the todo list table in the sqlite database
    pub fn create_table(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS todo_item (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed INTEGER NOT NULL
        )", 
        []
        )?;
        Ok(())
    }
    // insert a task into the sqlite database
    pub fn task_insert(conn: &Connection, title: &str) -> Result<()> {
        conn.execute(
            "INSERT INTO todo_item (title, completed) VALUES (?1, ?2)", 
            params![title, 0]
        )?;
        Ok(())
    }

    // fetch all the tasks in the sqlite database
    // returns two vectors - one with completed tasks, one with incomplete tasks
    pub fn task_fetch_all_by_completion(conn: &Connection) -> Result<(Vec<ToDoItem>, Vec<ToDoItem>)> {
        let mut stmt = conn.prepare(
            "SELECT id, title, completed FROM todo_item"
        )?;
        let todo_iter = stmt.query_map([], |row| {
            Ok(ToDoItem {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get(2)?,
            })
        })?;

        let mut completed_tasks = Vec::new();
        let mut incomplete_tasks = Vec::new();

        for item in todo_iter {
            let task: ToDoItem = item?;
            if task.completed {
                completed_tasks.push(task);
            }
            else {
                incomplete_tasks.push(task);
            }
        }
        Ok((incomplete_tasks, completed_tasks))
    }

    pub fn task_fetch_by_id(conn: &Connection, task_id: u64) -> Option<ToDoItem> {
        let mut stmt = conn.prepare(
            "SELECT id, title, completed FROM todo_item WHERE id = ?1"
        ).expect("Failed to prepare statement");
        
        let task_iter = stmt.query_map(params![task_id], |row| {
            Ok(ToDoItem {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get(2)?,
            })
        }).expect("Failed");

        for task in task_iter {
            return Some(task.expect("Failed"));
        }

        None
    }

    pub fn task_mark_complete(conn: &Connection, task_id: u64) -> Result<()> {
        conn.execute(
            "UPDATE todo_item SET completed = 1 WHERE id = ?1", 
        params![task_id],
        )?;
        Ok(())
    }

    pub fn task_update_title(conn: &Connection, task_id: u64, new_title: &str) -> Result<()> {
        conn.execute(
            "UPDATE todo_item SET title = ?1 WHERE id = ?2", 
        params![new_title, task_id],
        )?;
        Ok(())
    }

    pub fn task_delete(conn: &Connection, task_id: u64) -> Result<()> {
        conn.execute(
            "DELETE FROM todo_item WHERE id = ?1", 
        params![task_id],
    )?;
    Ok(())
    }
    // DATABASE FUNCTIONS END


}




// pub struct TodoList {
//     items: Vec<ToDoItem>,
// }






// impl TodoList {
//     pub fn new() -> TodoList {
//         TodoList { items: Vec::new() }
//     }

//     pub fn add_item(&mut self, title: String) {
//         let id:u64 = self.items.len() as u64 + 1;
//         let new_item = ToDoItem {
//             id: id,
//             title: title.clone(),
//             completed: false,
//         };
//         self.items.push(new_item);
//         println!("============================");
//         println!("Item added: {title}");
//         println!("============================");

//     }

//     pub fn list_items(&self) {
//         if self.items.is_empty() {
//             println!("No items")
//         }
//         else {
//             println!("");
//             println!("============================");
//             println!("Your tasks:");
//             println!("============================");
//             for item in &self.items {
//                 let status = if item.completed { "[X]"} else { "[ ]"};
//                 println!("{} | {} - {}", status, item.id, item.title);
//             }
//             println!("============================");
//             println!("");
//         }
//     }

//     pub fn complete_item(&mut self, id: u64) {
//         if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
//             item.completed = true;
//             println!("Completed {}", item.title);
//         } 
//         else {
//             println!("Item with ID {} not found", id);
//         }
//     }
// }