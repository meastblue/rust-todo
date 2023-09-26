use crate::server::Server;

mod server;
mod route;
mod task;

#[tokio::main]
async fn  main() {
    Server::init()
        .await
        .expect("TODO: panic message");
}
