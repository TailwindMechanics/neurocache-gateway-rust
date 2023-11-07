//path: src\main.rs

use axum::{routing::get, Router};
use logger::log_info;
use std::net::SocketAddr;

const HELLO_WORLD_MESSAGE: &str = "♫ Hello world, this is me ♫";

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));

    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);

    let address = SocketAddr::from(([0, 0, 0, 0], port));

    log_info(HELLO_WORLD_MESSAGE);

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world() -> &'static str {
    HELLO_WORLD_MESSAGE
}
