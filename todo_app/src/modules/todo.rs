use rusqlite::{params, Connection, Result };

pub struct ToDoItem {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

impl ToDoItem {
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
    // Fetch by a specific id
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
    // Mark a task as completed
    pub fn task_mark_complete(conn: &Connection, task_id: u64) -> Result<()> {
        conn.execute(
            "UPDATE todo_item SET completed = 1 WHERE id = ?1", 
        params![task_id],
        )?;
        Ok(())
    }
    // Update the title of a task
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
}