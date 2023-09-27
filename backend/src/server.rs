
use axum::{Router};
use crate::route::Routes;

pub struct Server;

impl Server {
    pub async fn init() -> Result<(), hyper::Error> {
        let routes = Router::new()
            .merge(Routes::routes());

        let addr = std::net::SocketAddr::from(([0,0,0,0], 8000));

        axum::Server::bind(&addr)
            .serve(routes.into_make_service())
            .await?;

        Ok(())
    }
}