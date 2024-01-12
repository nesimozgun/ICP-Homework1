#[derive(Debug, Clone)]

// create Task struct with its 'id', 'description' and 'completed' fields
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

// create a vector to store instances of Task struct
struct ToDoList {
    tasks: Vec<Task>,
}

// implement functions on ToDoList struct
impl ToDoList {
    fn add_task(&mut self, description: &str) -> Task {
        // generate a unique ID for the new task using current length of the list
        let id = self.tasks.len() + 1; 
        
        // Create a new Task instance
        let new_task = Task {
            id, // generated ID
            description: String::from(description), // description as the argument of the function
            completed: false, // declare as not completed
        };

        // add the new task to the vector
        self.tasks.push(new_task.clone());

        // return the created/added task
        new_task
    }

    fn complete_task(&mut self, id: usize) -> Option<&Task> {
        // find the task with the given ID
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true; // mark the task as completed
            Some(task) // return a reference to the task
        } else {

            // return None if the task with the given ID does not exist
            None
        }
    }

    fn list_tasks(&self) {
        // print the details of all tasks in the ToDo list
        for task in &self.tasks {
            print!("Task {}: {}", task.id, task.description);
            // let's make it a bit user friendly :D
            if task.completed {
                println!(" (done)")
            }
            else {
                println!(" (undone)")
            }
        }
    }
}

fn main() {
    // create a ToDoList instance
    let mut todo_list = ToDoList { tasks: Vec::new() };

    // add tasks to the ToDo list
    let task1 = todo_list.add_task("Finish Homework 1");
    let task2 = todo_list.add_task("Submit Final Project");

    todo_list.complete_task(task1.id);


    // print the ToDo list
    println!("To Do:");
    todo_list.list_tasks();
}