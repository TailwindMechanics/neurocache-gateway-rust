//path: src\crates\root\src\lib.rs

use axum::{routing::get, Router};

const HELLO_WORLD_MESSAGE: &str = "♫ Hello world, this is me ♫";

pub fn service() -> Router {
    Router::new().route("/", get(hello_world))
}

async fn hello_world() -> &'static str {
    HELLO_WORLD_MESSAGE
}
