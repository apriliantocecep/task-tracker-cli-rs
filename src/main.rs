use std::env;

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
