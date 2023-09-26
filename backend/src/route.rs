use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use crate::task::routes::TaskRoutes;

pub struct Routes;

impl Routes {
    pub fn routes() ->  Router {
        let cors = CorsLayer::new()
            .allow_origin(Any);

        Router::new()
            .nest("/task",TaskRoutes::routes())
            .layer(cors)
    }
}