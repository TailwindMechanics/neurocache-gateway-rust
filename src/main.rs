//path: src\main.rs

use axum::{Router, Server};
use logger::log_info;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(health::service())
        .merge(root::service());

    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);

    let address = SocketAddr::from(([0, 0, 0, 0], port));

    log_info("---> Neurocache Gateway Started <---");

    Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
