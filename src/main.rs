use std::io::{self, Write};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct Task {
    description: String,
    completed: bool,
    due_date: Option<String>,
}

impl Task {
    fn new(description: String, due_date: Option<String>) -> Self {
        Task {
            description,
            completed: false,
            due_date,
        }
    }

    fn mark_complete(&mut self) {
        self.completed = true;
    }

    fn display(&self) {
        println!(
            "{} - [{}] {}",
            if self.completed { "✓" } else { " " },
            self.due_date.clone().unwrap_or("No Due Date".to_string()),
            self.description
        );
    }
}

struct ToDoList {
    tasks: Vec<Task>,
}

impl ToDoList {
    fn new() -> ToDoList {
        ToDoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: String, due_date: Option<String>) {
        let task = Task::new(description, due_date);
        self.tasks.push(task);
        println!("Task added successfully!");
    }

    fn remove_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            println!("Task removed successfully!");
        } else {
            println!("Invalid task index!");
        }
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found!");
        } else {
            println!("Your To-Do List:");
            for (i, task) in self.tasks.iter().enumerate() {
                println!("{}. ", i + 1);
                task.display();
            }
        }
    }

    fn mark_complete(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.mark_complete();
            println!("Task marked as completed!");
        } else {
            println!("Invalid task index!");
        }
    }

    fn save_to_file(&self) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("to_do_list.txt")
            .expect("Unable to open file");

        for task in &self.tasks {
            let task_data = format!(
                "{}|{}|{}\n",
                task.description,
                task.completed,
                task.due_date.clone().unwrap_or("".to_string())
            );
            file.write_all(task_data.as_bytes())
                .expect("Unable to write data");
        }
        println!("Tasks saved to file.");
    }

    fn load_from_file(&mut self) {
        let file = File::open("to_do_list.txt");
        if file.is_ok() {
            let reader = BufReader::new(file.unwrap());
            for line in reader.lines() {
                if let Ok(task_data) = line {
                    let parts: Vec<&str> = task_data.split('|').collect();
                    if parts.len() == 3 {
                        let completed = parts[1] == "true";
                        let due_date = if parts[2].is_empty() {
                            None
                        } else {
                            Some(parts[2].to_string())
                        };
                        self.tasks.push(Task {
                            description: parts[0].to_string(),
                            completed,
                            due_date,
                        });
                    }
                }
            }
            println!("Tasks loaded from file.");
        } else {
            println!("No saved tasks found.");
        }
    }

    // Sort tasks by completion status
    fn sort_by_completion(&mut self) {
        self.tasks.sort_by(|a, b| a.completed.cmp(&b.completed));
        println!("Tasks sorted by completion status.");
    }

    // Filter tasks by due date
    fn filter_by_due_date(&self, due_date: &str) {
        let filtered_tasks: Vec<&Task> = self
            .tasks
            .iter()
            .filter(|task| task.due_date.as_deref() == Some(due_date))
            .collect();

        if filtered_tasks.is_empty() {
            println!("No tasks found with due date: {}", due_date);
        } else {
            println!("Filtered tasks with due date {}:", due_date);
            for task in filtered_tasks {
                task.display();
            }
        }
    }

    // Mark all tasks as completed
    fn mark_all_complete(&mut self) {
        for task in &mut self.tasks {
            task.mark_complete();
        }
        println!("All tasks marked as completed!");
    }

    // Search tasks by description
    fn search_tasks(&self, keyword: &str) {
        let found_tasks: Vec<&Task> = self
            .tasks
            .iter()
            .filter(|task| task.description.contains(keyword))
            .collect();

        if found_tasks.is_empty() {
            println!("No tasks found with keyword: {}", keyword);
        } else {
            println!("Search results for '{}':", keyword);
            for task in found_tasks {
                task.display();
            }
        }
    }
}

fn main() {
    let mut to_do_list = ToDoList::new();
    to_do_list.load_from_file();

    loop {
        println!("\n==== To-Do List ====");
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
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => {
                println!("Enter the task description:");
                let mut task_description = String::new();
                io::stdin()
                    .read_line(&mut task_description)
                    .expect("Failed to read task description");

                println!("Enter the due date (optional, format: YYYY-MM-DD):");
                let mut due_date = String::new();
                io::stdin()
                    .read_line(&mut due_date)
                    .expect("Failed to read due date");

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
                io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to read input");
                if let Ok(index) = index.trim().parse::<usize>() {
                    to_do_list.remove_task(index - 1);
                } else {
                    println!("Please enter a valid number!");
                }
            }
            "3" => {
                to_do_list.list_tasks();
            }
            "4" => {
                println!("Enter the task number to mark as completed:");
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to read input");
                if let Ok(index) = index.trim().parse::<usize>() {
                    to_do_list.mark_complete(index - 1);
                } else {
                    println!("Please enter a valid number!");
                }
            }
            "5" => {
                to_do_list.save_to_file();
            }
            "6" => {
                to_do_list.sort_by_completion();
            }
            "7" => {
                println!("Enter the due date to filter tasks (YYYY-MM-DD):");
                let mut due_date = String::new();
                io::stdin()
                    .read_line(&mut due_date)
                    .expect("Failed to read due date");
                to_do_list.filter_by_due_date(due_date.trim());
            }
            "8" => {
                to_do_list.mark_all_complete();
            }
            "9" => {
                println!("Enter the keyword to search tasks:");
                let mut keyword = String::new();
                io::stdin()
                    .read_line(&mut keyword)
                    .expect("Failed to read keyword");
                to_do_list.search_tasks(keyword.trim());
            }
            "10" => {
                println!("Exiting... Goodbye!");
                break;
            }
            _ => println!("Invalid option, please try again!"),
        }
    }
}
