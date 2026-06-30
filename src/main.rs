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

fn add_task(task_list: &mut Vec<Task>, next_id: &mut usize) {
    println!("{SEPARATOR}");

    loop {
        let mut task_name = String::new();
        let task_id = *next_id;
        let task_status = TaskStatus::ToDo;

        println!("Enter your task: (or q to quit)");
        io::stdin()
            .read_line(&mut task_name)
            .expect("Error reading input");

        task_name = task_name.trim().to_string();

        if task_name == "q" {
            println!("Tasks added:");
            list_tasks(task_list);
            break;
        }

        let task = Task {
            id: task_id,
            title: task_name,
            status: task_status,
        };

        task_list.push(task);
        *next_id += 1;
    }
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
    loop {
        let mut input = String::new();

        println!("Which task do you want to delete? (Enter number or press q to quit)");

        list_tasks(task_list);

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        if input.trim() == "q" {
            break;
        }

        let task_to_delete: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        if let Some(index) = task_list.iter().position(|t| t.id == task_to_delete) {
            task_list.remove(index);
        } else {
            println!("Task cannot be removed, wrong number");
        }
    }
}

fn complete_task(task_list: &mut Vec<Task>) {
    //reference has to be mutable because im getting a mutable reference TO A TASK via get_mut????
    loop {
        let mut input = String::new();

        println!("Which task did you complete? (press q to quit)");
        list_tasks(task_list);

        io::stdin().read_line(&mut input).expect("Cant read input");

        if input.trim() == "q" {
            break;
        }

        let completed_task: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Task cant be found. Enter valid number");
                continue;
            }
        };

        if let Some(task) = task_list.iter_mut().find(|t| t.id == completed_task) {
            task.mark_as_completed();
            println!("Marked {} as completed", task.title);
        } else {
            println!("Error deleting task");
        }
    }
}
fn main() {
    let mut task_list: Vec<Task> = Vec::new();
    let mut next_id = 1;
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

        let user_choice: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter valid choice");
                continue;
            }
        };

        match user_choice {
            1 => add_task(&mut task_list, &mut next_id),
            2 => list_tasks(&task_list),
            3 => delete_task(&mut task_list),
            4 => complete_task(&mut task_list),
            5 => break,
            _ => println!("Invalid option"),
        }
    }
}
