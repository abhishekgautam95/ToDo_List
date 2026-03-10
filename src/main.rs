use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

/// Represents a single task in the to-do list
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    description: String,
    completed: bool,
    due_date: Option<String>,
}

impl Task {
    /// Creates a new task with the given description and optional due date
    fn new(description: impl Into<String>, due_date: Option<String>) -> Self {
        Task {
            description: description.into(),
            completed: false,
            due_date,
        }
    }

    /// Marks the task as completed
    fn mark_complete(&mut self) {
        self.completed = true;
    }

    /// Displays the task in a formatted way
    fn display(&self, index: usize) {
        let status = if self.completed { "✓" } else { " " };
        let due = self.due_date.as_deref().unwrap_or("No Due Date");
        println!("{}. {} - [{}] {}", index, status, due, self.description);
    }
}

/// Manages a collection of tasks
struct ToDoList {
    tasks: Vec<Task>,
}

impl ToDoList {
    /// Creates a new empty to-do list
    fn new() -> Self {
        ToDoList { tasks: Vec::new() }
    }

    /// Adds a new task to the list
    fn add_task(&mut self, description: String, due_date: Option<String>) {
        let task = Task::new(description, due_date);
        self.tasks.push(task);
        println!("Task added successfully!");
    }

    /// Removes a task at the specified index (0-based)
    fn remove_task(&mut self, index: usize) -> Result<(), &'static str> {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            println!("Task removed successfully!");
            Ok(())
        } else {
            Err("Invalid task index!")
        }
    }

    /// Lists all tasks in the to-do list
    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found!");
        } else {
            println!("\n=== Your To-Do List ===");
            for (i, task) in self.tasks.iter().enumerate() {
                task.display(i + 1);
            }
        }
    }

    /// Marks a task as completed by index (0-based)
    fn mark_complete(&mut self, index: usize) -> Result<(), &'static str> {
        if let Some(task) = self.tasks.get_mut(index) {
            task.mark_complete();
            println!("Task marked as completed!");
            Ok(())
        } else {
            Err("Invalid task index!")
        }
    }

    /// Saves all tasks to a JSON file
    fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(filename)?;

        serde_json::to_writer_pretty(file, &self.tasks)?;
        println!("Tasks saved to file.");
        Ok(())
    }

    /// Loads tasks from a JSON file
    fn load_from_file(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        if !Path::new(filename).exists() {
            println!("No saved tasks found.");
            return Ok(());
        }

        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        
        match serde_json::from_reader(reader) {
            Ok(tasks) => {
                self.tasks = tasks;
                println!("Tasks loaded from file.");
            }
            Err(e) => {
                eprintln!("Warning: Could not parse tasks file: {}", e);
                println!("Starting with empty task list.");
            }
        }
        Ok(())
    }

    /// Sorts tasks by completion status (incomplete first)
    fn sort_by_completion(&mut self) {
        self.tasks.sort_by_key(|task| task.completed);
        println!("Tasks sorted by completion status.");
    }

    /// Filters and displays tasks by due date
    fn filter_by_due_date(&self, due_date: &str) {
        let filtered: Vec<&Task> = self
            .tasks
            .iter()
            .filter(|task| task.due_date.as_deref() == Some(due_date))
            .collect();

        if filtered.is_empty() {
            println!("No tasks found with due date: {}", due_date);
        } else {
            println!("\n=== Tasks due on {} ===", due_date);
            for (i, task) in filtered.iter().enumerate() {
                task.display(i + 1);
            }
        }
    }

    /// Marks all tasks as completed
    fn mark_all_complete(&mut self) {
        for task in &mut self.tasks {
            task.mark_complete();
        }
        println!("All tasks marked as completed!");
    }

    /// Searches tasks by keyword in description
    fn search_tasks(&self, keyword: &str) {
        let found: Vec<&Task> = self
            .tasks
            .iter()
            .filter(|task| task.description.to_lowercase().contains(&keyword.to_lowercase()))
            .collect();

        if found.is_empty() {
            println!("No tasks found with keyword: {}", keyword);
        } else {
            println!("\n=== Search results for '{}' ===", keyword);
            for (i, task) in found.iter().enumerate() {
                task.display(i + 1);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    const FILENAME: &str = "tasks.json";
    
    let mut to_do_list = ToDoList::new();
    to_do_list.load_from_file(FILENAME)?;

    loop {
        println!("\n==== To-Do List Manager ====");
        println!("1. Add Task");
        println!("2. Remove Task");
        println!("3. View Tasks");
        println!("4. Mark Task Complete");
        println!("5. Save Tasks");
        println!("6. Sort Tasks by Completion");
        println!("7. Filter Tasks by Due Date");
        println!("8. Mark All Tasks Complete");
        println!("9. Search Tasks");
        println!("10. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                println!("Enter the task description:");
                let mut task_description = String::new();
                io::stdin().read_line(&mut task_description)?;

                println!("Enter the due date (optional, format: YYYY-MM-DD):");
                let mut due_date = String::new();
                io::stdin().read_line(&mut due_date)?;

                let due_date = if due_date.trim().is_empty() {
                    None
                } else {
                    Some(due_date.trim().to_string())
                };

                to_do_list.add_task(task_description.trim().to_string(), due_date);
            }
            "2" => {
                println!("Enter the task number to remove:");
                let mut index = String::new();
                io::stdin().read_line(&mut index)?;
                
                match index.trim().parse::<usize>() {
                    Ok(idx) => {
                        if let Err(e) = to_do_list.remove_task(idx - 1) {
                            println!("{}", e);
                        }
                    }
                    Err(_) => println!("Please enter a valid number!"),
                }
            }
            "3" => {
                to_do_list.list_tasks();
            }
            "4" => {
                println!("Enter the task number to mark as completed:");
                let mut index = String::new();
                io::stdin().read_line(&mut index)?;
                
                match index.trim().parse::<usize>() {
                    Ok(idx) => {
                        if let Err(e) = to_do_list.mark_complete(idx - 1) {
                            println!("{}", e);
                        }
                    }
                    Err(_) => println!("Please enter a valid number!"),
                }
            }
            "5" => {
                if let Err(e) = to_do_list.save_to_file(FILENAME) {
                    eprintln!("Error saving tasks: {}", e);
                }
            }
            "6" => {
                to_do_list.sort_by_completion();
            }
            "7" => {
                println!("Enter the due date to filter tasks (YYYY-MM-DD):");
                let mut due_date = String::new();
                io::stdin().read_line(&mut due_date)?;
                to_do_list.filter_by_due_date(due_date.trim());
            }
            "8" => {
                to_do_list.mark_all_complete();
            }
            "9" => {
                println!("Enter the keyword to search tasks:");
                let mut keyword = String::new();
                io::stdin().read_line(&mut keyword)?;
                to_do_list.search_tasks(keyword.trim());
            }
            "10" => {
                println!("Exiting... Goodbye!");
                break;
            }
            _ => println!("Invalid option, please try again!"),
        }
    }

    Ok(())
}
