use std::{env, fs};
use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    description: String,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

fn load_tasks() -> Vec<Task> {
    let tasks = fs::read_to_string("tasks.json");
    match tasks {
        Ok(tasks) => {
            let tasks: Vec<Task> = serde_json::from_str(&tasks).unwrap_or_default();
            tasks
        }
        Err(_) => Vec::new(),
    }
}

fn save_task(tasks: &Vec<Task>) {
    if let Ok(data) = serde_json::to_string_pretty(&tasks) {
        fs::write("tasks.json", data).expect("Failed to save tasks");
    }
}

fn add_task(description: String) {
    let mut tasks: Vec<Task> = load_tasks();
    let id = if let Some(last_task) = tasks.last() {
        last_task.id + 1
    } else { 1 };

    let task = Task {
        id,
        description,
        status: "todo".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    tasks.push(task);
    save_task(&tasks);
    println!("Task added successfully (ID: {})", id)
}

fn update_task(id: u32, description: String) {
    let mut tasks: Vec<Task> = load_tasks();
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.description = description;
        task.updated_at = Utc::now();
        save_task(&tasks);
        println!("Task updated successfully.")
    } else {
        println!("Task with ID {} not found.", id)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprint!("Usage: task-cli <command> [arguments...]");
        return;
    }
    
    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Usage: task-cli add <description>");
            } else {
                add_task(args[2..].join(" "))
            }
        }
        "update" => {
            if args.len() < 4 {
                eprintln!("Usage: task-cli update <id> <description>");
            } else if let Ok(id) = args[2].parse() {
                update_task(id, args[3..].join(" "));
            } else {
                eprintln!("Invalid ID.");
            }
        }
        "delete" => {
            if args.len() < 3 {
                eprintln!("Usage: task-cli delete <id>");
            } else {
                println!("Deleting task: {}", args[2]);
            }
        }
        "mark-in-progress" => {
            if args.len() < 3 {
                eprintln!("Usage: task-cli mark-in-progress <id>");
            } else {
                println!("Marking task in progress: {}", args[2]);
            }
        }
        "mark-done" => {
            if args.len() < 3 {
                eprintln!("Usage: task-cli mark-done <id>");
            } else {
                println!("Marking task done: {}", args[2]);
            }
        }
        "list" => {
            println!("Listing tasks");
        }
        _ => {
            eprint!("Unknown command: {}", args[1]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clear_tasks() {
        fs::write("tasks.json", "[]").expect("Failed to clear tasks.json")
    }
    
    #[test]
    #[ignore]
    fn test_load_tasks() {
        let tasks = load_tasks();
        assert!(tasks.is_empty(), "Tasks should loaded successfully");
    }
    
    #[test]
    fn test_add_task() {
        clear_tasks();
        let initial_task = load_tasks();
        add_task("Test Task".to_string());
        let tasks = load_tasks();
        assert_eq!(tasks.len(), initial_task.len() + 1);
        assert_eq!(tasks.last().unwrap().description, "Test Task");
    }
    
    #[test]
    fn test_update_task() {
        clear_tasks();
        let mut tasks = load_tasks();
        if tasks.is_empty() {
            add_task("New Task".to_string());
            tasks = load_tasks();
        }
        let id = tasks.last().unwrap().id;
        update_task(id, "Updated Task".to_string());
        let updated_task = load_tasks().into_iter().find(|t| t.id == id).unwrap();
        assert_eq!(updated_task.description, "Updated Task");
    }
}