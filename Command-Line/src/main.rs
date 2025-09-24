use std::fs;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    title: String,
    done: bool,
}

fn main() {
    let mut tasks: Vec<Task> = load_tasks();

    loop {
        println!("\n==== Todo List ====");
        for (i, t) in tasks.iter().enumerate() {
            println!("{}: [{}] {}", i + 1, if t.done { "x" } else { " " }, t.title);
        }

        println!("\nCommands: add <task>, done <num>, remove <num>, save, exit");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();

        match parts[0] {
            "add" if parts.len() > 1 => {
                tasks.push(Task { title: parts[1].to_string(), done: false });
                println!("Task added: {}", parts[1]);
            }
            "done" if parts.len() > 1 => {
                if let Ok(num) = parts[1].parse::<usize>() {
                    if let Some(t) = tasks.get_mut(num - 1) {
                        t.done = true;
                        println!("Task {} marked as done!", num);
                    }
                }
            }
            "remove" if parts.len() > 1 => {
                if let Ok(num) = parts[1].parse::<usize>() {
                    if num > 0 && num <= tasks.len() {
                        let removed = tasks.remove(num - 1);
                        println!("Removed task: {}", removed.title);
                    }
                }
            }
            "save" => save_tasks(&tasks),
            "exit" => {
                save_tasks(&tasks);
                break;
            }
            _ => println!("Unknown command."),
        }
    }
}

fn load_tasks() -> Vec<Task> {
    fs::read_to_string("tasks.json")
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_else(Vec::new)
}

fn save_tasks(tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).unwrap();
    fs::write("tasks.json", data).unwrap();
    println!("Tasks saved ✅");
}
