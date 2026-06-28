use std::io;

enum TaskStatus {
    ToDo,
    Completed,
}
struct Task {
    id: usize,
    title: String,
    status: TaskStatus,
}

impl Task {
    fn mark_as_completed(&mut self) {
        self.status = TaskStatus::Completed;
    }
}

const SEPARATOR: &str = "--------------------------------";

fn add_task(task_list: &mut Vec<Task>) {
    let mut task_name = String::new();
    let task_id = task_list.len() + 1;
    let task_status = TaskStatus::ToDo;

    println!("{SEPARATOR}");
    println!("Enter your task: ");
    io::stdin()
        .read_line(&mut task_name)
        .expect("Error reading input");

    task_name = task_name.trim().to_string();

    let task = Task {
        id: task_id,
        title: task_name,
        status: task_status,
    };

    task_list.push(task);
    if let Some(task) = task_list.last() {
        println!("Task added: {}", task.title); // calling the get method returns an option not an actual task
        // must deal with the outcome of Some(val) or None
    }
    println!("{SEPARATOR}");
}

fn list_tasks(task_list: &Vec<Task>) {
    println!("{SEPARATOR}");
    for task in task_list {
        let status = match &task.status {
            TaskStatus::ToDo => "work in progress",
            TaskStatus::Completed => "completed",
        };
        println!("{}.{},{}", task.id, task.title, status);
    }
    println!("{SEPARATOR}");
}

fn delete_task(task_list: &mut Vec<Task>) {
    let mut input = String::new();

    println!("Which task do you want to delete? (Enter number)");

    list_tasks(task_list);

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");

    let task_to_delete: usize = input.trim().parse().expect("Cannot parse val");

    if task_to_delete >= task_list.len() {
        println!("Invalid task")
    } else {
        task_list.remove(task_to_delete - 1);
        println!("Task removed successfully");
        println!("{SEPARATOR}");
    }
}

fn complete_task(task_list: &mut Vec<Task>) {
    //reference has to be mutable because im getting a mutable reference TO A TASK via get_mut????
    let mut input = String::new();

    println!("Which task did you complete?");
    list_tasks(task_list);

    io::stdin().read_line(&mut input).expect("Cant read input");
    let completed_task: usize = input.trim().parse().expect("Cant parse input");

    if let Some(task) = task_list.get_mut(completed_task - 1) {
        task.mark_as_completed();
    }
}
fn main() {
    let mut task_list: Vec<Task> = Vec::new();
    let options: [&str; 5] = [
        "Add Task",
        "List Tasks",
        "Delete Task",
        "Complete Task",
        "Quit",
    ];
    loop {
        println!("Welcome to your task manager, what do you want to do?");
        for (idx, option) in options.iter().enumerate() {
            println!("{}.{}", idx + 1, option);
        }

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Please enter a number that is in the list");

        let user_choice: i32 = input.trim().parse().expect("Input couldnt be validated");

        match user_choice {
            1 => add_task(&mut task_list),
            2 => list_tasks(&task_list),
            3 => delete_task(&mut task_list),
            4 => complete_task(&mut task_list),
            5 => break,
            _ => println!("Invalid option"),
        }
    }
}
