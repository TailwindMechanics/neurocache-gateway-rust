//path: src\crates\agent\src\lib.rs

use axum::{routing::get, Router};
use internal::run_agent::run_agent;
use serde::Deserialize;

mod internal;

const AGENT_RUN_ENDPOINT: &str = "/agent/run";

#[derive(Deserialize)]
pub struct AgentQueryParams {
    payload: String,
    agent_id: String,
}

pub fn service() -> Router {
    Router::new().route(AGENT_RUN_ENDPOINT, get(run_agent))
}
