//path: src\crates\agent\src\lib.rs

// src/crates/agent/src/lib.rs

use agent_types::RunAgentRequest;
use axum::{
    http::{HeaderMap, Response, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use internal::run_agent::run_agent_stream;
use supabase::get_agent_graph;

mod internal;

const AGENT_RUN_ENDPOINT: &str = "/agent/run";
pub fn service() -> Router {
    Router::new().route(AGENT_RUN_ENDPOINT, post(get_apikey_and_jsonbody))
}

async fn get_apikey_and_jsonbody(
    headers: HeaderMap,
    Json(data): Json<RunAgentRequest>,
) -> Result<Json<RunAgentRequest>, StatusCode> {
    match headers.get("authorization") {
        Some(auth_header_value) => match auth_header_value.to_str() {
            Ok(auth_str) => {
                if auth_str.starts_with("Bearer ") {
                    let api_key = &auth_str["Bearer ".len()..];
                    match get_agent_graph(data.agent_id, (&api_key).to_string()).await {
                        Ok(agent_graph_data) => {
                            let result = run_agent_stream(agent_graph_data).await;
                            Response::builder()
                                .status(StatusCode::OK)
                                .body(result.into_response())
                                .unwrap();
                        }
                        Err(error) => {
                            let error_response = format!("Error fetching agent graph: {}", error);
                            Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(error_response.into_response())
                                .unwrap();
                        }
                    }
                }
            }
            _ => (),
        },
        _ => (),
    }

    Err(StatusCode::UNAUTHORIZED)
}
