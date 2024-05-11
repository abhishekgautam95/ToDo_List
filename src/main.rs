use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Self {
        Task {
            description,
            completed: false,
        }
    }
}

fn main() {
    let mut tasks: Vec<Task> = load_tasks();

    loop {
        println!("To-Do List");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Mark Task as Completed");
        println!("4. Save and Quit");

        let choice: u32 = get_user_input("Enter your choice: ").trim().parse().expect("Please enter a valid number");
        
        match choice {
            1 => {
                let description = get_user_input("Enter task description: ");
                tasks.push(Task::new(description));
            }
            2 => list_tasks(&tasks),
            3 => {
                list_tasks(&tasks);
                let task_index: usize = get_user_input("Enter task index to mark as completed: ").trim().parse().expect("Please enter a valid index");
                if let Some(task) = tasks.get_mut(task_index) {
                    task.completed = true;
                } else {
                    println!("Invalid task index!");
                }
            }
            4 => {
                save_tasks(&tasks);
                println!("Tasks saved. Exiting.");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    use std::io::{self, Write};
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn list_tasks(tasks: &[Task]) {
    println!("Tasks:");
    for (index, task) in tasks.iter().enumerate() {
        println!("{} [{}] {}", index, if task.completed { "X" } else { " " }, task.description);
    }
}

fn load_tasks() -> Vec<Task> {
    match fs::read_to_string("tasks.json") {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

fn save_tasks(tasks: &[Task]) {
    let json_data = serde_json::to_string_pretty(tasks).unwrap();
    fs::write("tasks.json", json_data).expect("Failed to write to file!");
}