//path: src\crates\supabase\src\lib.rs

use agent_types::AgentGraphData;
use dotenv::dotenv;
use logger::{log_error, log_info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize)]
struct GetAgentGraphRequest {
    agent_id: String,
    user_api_key: String,
}

pub async fn get_agent_graph(
    agent_id: String,
    user_api_key: String,
) -> Result<AgentGraphData, reqwest::Error> {
    dotenv().ok();
    let service_role_key = env::var("SUPABASE_SERVICE_ROLE_KEY").expect("No role key found");
    let base_url = env::var("SUPABASE_SERVICE_URL").expect("No base url found");
    let url = format!("{}/rest/v1/rpc/getagentgraph", base_url);

    log_info!("url: {}", &url);
    log_info!("service_role_key: {}", &service_role_key);
    log_info!("agent_id: {}", &agent_id);
    log_info!("user_api_key: {}", &user_api_key);

    let client = Client::new();
    let request = client
        .post(&url)
        .bearer_auth(&service_role_key)
        .json(&GetAgentGraphRequest {
            agent_id: agent_id.clone(),
            user_api_key: user_api_key.clone(),
        })
        .build()?;

    log_info!("Prepared request: {:?}", request);

    let response = client.execute(request).await?;
    if response.status().is_success() {
        log_info!("Received successful response");
    } else {
        log_error!("Error response: {:?}", response.status());
    }

    response.json().await
}
