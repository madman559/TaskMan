use chrono::{DateTime, Utc};
use uuid::Uuid;

struct Task {
    task_id: Uuid,
    task_name: String,
    task_description: String,
    is_complete: bool,
    creation_date: chrono::DateTime<Utc>,
}

fn create_task(
    task_id: Uuid,
    task_name: String,
    task_description: String,
    creation_date: chrono::DateTime<Utc>,
) -> Task {
    Task {
        task_id,
        task_name,
        task_description,
        is_complete: false,
        creation_date,
    }
}

fn main() {
    let mut task_vector: Vec<Task> = Vec::new();

    let task1 = create_task(
        Uuid::new_v4(),
        String::from("Learn Rust"),
        String::from("Learn the rust programming language."),
        Utc::now(),
    );
    let task2 = create_task(
        Uuid::new_v4(),
        String::from("Learn Python"),
        String::from("Learn the Python programming language."),
        Utc::now(),
    );

    task_vector.push(task1);
    task_vector.push(task2);

    for task in task_vector.iter() {
        println!("\n\nTask ID: {}", task.task_id);
        println!("Task Name: {}", task.task_name);
        println!("Task Description: {}", task.task_description);
        println!("Task Complete: {}", task.is_complete);
        println!(
            "Creation Date: {}\n\n",
            task.creation_date.format("%a %b %e %T %Y")
        )
    }
}
