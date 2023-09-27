use axum::{response::IntoResponse, response::Response, Json};
use axum::http::StatusCode;
use serde_json::json;

#[derive(Debug)]
pub enum TaskError {
    TaskNotFound
}

impl IntoResponse for TaskError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            Self::TaskNotFound => (StatusCode::NOT_FOUND, "Task not found")
        };

        (status,Json(json!({"error": err_msg}))).into_response()
    }
}