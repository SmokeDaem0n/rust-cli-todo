use std::io;

struct Task {
    title: String,
}

fn main() {
    let mut task_list: Vec<Task> = Vec::new();

    loop {
        println!("Enter a task / press q to quit");
        let mut user_task_title = String::new();

        io::stdin()
            .read_line(&mut user_task_title)
            .expect("Please enter a proper task");

        if user_task_title.trim() == "q" {
            break;
        }

        let user_task = Task {
            title: user_task_title.trim().to_string(),
        };
        task_list.push(user_task);
    }

    for task in task_list {
        print!("{} ", task.title);
    }
}
