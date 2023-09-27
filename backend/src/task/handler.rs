use axum::Json;
use serde_json::{json, Value};
use crate::task::error::TaskError;
use crate::task::model::{TaskModel};

pub struct TaskHandler;

impl TaskHandler {
    pub async fn get() -> Result<Json<Value>, TaskError>  {
        return if let Some(task) = TaskModel::get().await {
            Ok(Json(json!(task)))
        } else {
            Err(TaskError::TaskNotFound)
        }
    }
}