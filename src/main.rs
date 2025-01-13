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
                println!("Adding task: {}", args[2]);
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
    fn test_load_tasks() {
        let tasks = load_tasks();
        assert!(tasks.is_empty(), "Tasks should loaded successfully");
    }
}