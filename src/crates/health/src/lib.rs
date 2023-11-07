//path: src\crates\health\src\lib.rs

use axum::{response::IntoResponse, routing::get, Router};

const HEALTHY_MESSAGE: &str = "Neurocache Gateway is healthy!";
const HEALTH_CHECK_ENDPOINT: &str = "/health";

pub fn service() -> Router {
    Router::new().route(HEALTH_CHECK_ENDPOINT, get(health_check))
}

async fn health_check() -> impl IntoResponse {
    HEALTHY_MESSAGE
}
