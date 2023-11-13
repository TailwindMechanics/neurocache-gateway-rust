//path: src\crates\agent\src\internal\api_key_utils.rs

use agent_types::{AgentGraphData, RunAgentRequest};
use axum::{
    http::{HeaderMap, StatusCode},
    Json,
};
use internal::run_agent::run_agent_stream;
use logger::log_error;
use serde_json::from_value;
use supabase::get_agent_graph;

pub async fn get_apikey_and_jsonbody(
    headers: HeaderMap,
    Json(data): Json<RunAgentRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let api_key = match extract_api_key(&headers) {
        Ok(key) => key,
        Err(status) => {
            log_error!("Failed to extract API key");
            return Err(status);
        }
    };

    let agent_id_clone = data.agent_id.clone();
    let agent_graph_json = match get_agent_graph(agent_id_clone, api_key).await {
        Ok(json) => json,
        Err(error) => {
            log_error!(format!("Error fetching agent graph: {}", error));
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let agent_graph_data: AgentGraphData = match from_value(agent_graph_json) {
        Ok(data) => data,
        Err(error) => {
            log_error!(format!("Error deserializing agent graph data: {}", error));
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(run_agent_stream(data.agent_id.clone(), agent_graph_data).await)
}

fn extract_api_key(headers: &HeaderMap) -> Result<String, StatusCode> {
    headers
        .get("apikey")
        .ok_or_else(|| {
            log_error!("API key not found in headers");
            StatusCode::UNAUTHORIZED
        })
        .and_then(|api_key_header_value| {
            api_key_header_value
                .to_str()
                .map_err(|_| {
                    log_error!("Failed to parse API key as string");
                    StatusCode::BAD_REQUEST
                })
                .map(|s| s.to_string())
        })
}
