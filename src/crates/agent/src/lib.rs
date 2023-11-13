// src/crates/agent/src/lib.rs

use agent_types::{AgentGraphData, RunAgentRequest};
use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use internal::api_key_utils::get_apikey_and_jsonbody;
use internal::run_agent::run_agent_stream;

mod internal;

const AGENT_RUN_ENDPOINT: &str = "/agent/run";

pub fn service() -> Router {
    Router::new().route(
        AGENT_RUN_ENDPOINT,
        post(|header_map, json_body| async move {
            get_apikey_and_jsonbody(header_map, json_body).await
        }),
    )
}
