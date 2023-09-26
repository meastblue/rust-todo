use crate::task::model::{Task, TaskModel};

pub struct TaskHandler;

impl TaskHandler {
    pub fn index() -> String {
        "Hello Task".into()
    }

    pub fn get() -> Result<Task> {
        let task = TaskModel::get()?;

        Ok(task)
    }
}