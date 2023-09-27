use axum::Router;
use axum::routing::get;
use crate::task::handler::TaskHandler;

pub struct TaskRoutes;

impl TaskRoutes {
    pub fn routes() -> Router {
        Router::new()
            .route("/", get(TaskHandler::get))
    }
}