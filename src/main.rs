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
            } else {
                println!("Updating task {}: {}", args[2], args[3]);
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
    
    #[test]
    #[ignore]
    fn test_load_tasks() {
        let tasks = load_tasks();
        assert!(tasks.is_empty(), "Tasks should loaded successfully");
    }
    
    #[test]
    fn test_add_task() {
        let initial_task = load_tasks();
        add_task("Test Task".to_string());
        let tasks = load_tasks();
        assert_eq!(tasks.len(), initial_task.len() + 1);
        assert_eq!(tasks.last().unwrap().description, "Test Task");
    }
}