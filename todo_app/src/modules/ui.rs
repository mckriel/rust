use anyhow::Result;
use dialoguer::Input;
use crate::modules::todo::ToDoItem;

// display list of tasks
pub fn display_task_list(tasks: Vec<ToDoItem>, title: &str) {
    println!("{}", title);
    if tasks.is_empty() {
        println!("Task list empty \n");
    } else {
        for task in tasks {
            let completed_status = if task.completed { "[X]" } else { "[ ]" };
            println!("{} | {} | {}", completed_status, task.id, task.title);
        }
        println!();
    }
}
// display completed and incompleted tasks
pub fn display_tasks(incomplete_tasks: Vec<ToDoItem>, complete_tasks: Vec<ToDoItem>) {
    display_task_list(incomplete_tasks, "-- Outstanding Tasks --");
    display_task_list(complete_tasks, "-- Completed Tasks --");
}
// user input prompt for task titles
pub fn prompt_task_title(prompt: &str) -> Result<String> {
    Input::new()
        .with_prompt(prompt)
        .interact_text()
        .map_err(|e| e.into())
}
// user input prompt for task ids
pub fn prompt_task_id(prompt: &str) -> Result<u64> {
    let task_id: String = Input::new()
        .with_prompt(prompt)
        .interact_text()?;
    Ok(task_id.trim().parse().unwrap_or(0))
}
