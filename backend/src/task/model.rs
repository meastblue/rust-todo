use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
     id: String,
     label: String,
     completed: bool,
    // updated_at: DateTime<Local>,
    // created_at: DateTime<Local>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tasks {
    tasks: Vec<Task>
}

pub struct TaskModel;

impl TaskModel {
    pub async fn get() -> Option<Task> {
        let task = Task {
            id: "8466571346519576".into(),
            label: "Learn Rust".into(),
            completed: false,
        };

        Some(task)
    }
}