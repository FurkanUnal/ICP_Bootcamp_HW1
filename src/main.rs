#[derive(Clone)]
struct Task {
    id: i32,
    description: String,
    completed: bool,
}

struct ToDoList {
    tasks: Vec<Task>,
}

impl ToDoList {
    fn add_task(&mut self, description: &str) -> Task {
        let id: i32 = self.tasks.len() as i32 + 1;
        let task: Task = Task {
            id,
            description: description.to_string(),
            completed: false,
        };
        self.tasks.push(task.clone());
        return task;
    }

    fn complete_task(&mut self, id: i32) -> Option<&Task> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            return Some(task);
        } else {
            return None;
        }
    }

    fn list_tasks(&self) {
        for task in &self.tasks {
            println!(
                "ID: {}, Description: {}, Completed: {}",
                task.id, task.description, task.completed
            );
        }
    }
}

fn main() {
    let mut todo_list = ToDoList { tasks: Vec::new() };

    let task1 = todo_list.add_task("Buy groceries");
    let task2 = todo_list.add_task("Finish homework");
    let task3 = todo_list.add_task("Read a book");

    todo_list.list_tasks();

    todo_list.complete_task(task2.id);

    println!("After completing task 2:");
    todo_list.list_tasks();
}
