// src/crates/supabase/src/lib.rs

use dotenv::dotenv;
use logger::{log_error, log_info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

#[derive(Deserialize, Serialize)]
struct GetAgentGraphRequest {
    agentid: String,
    userkey: String,
}

pub async fn get_agent_graph(
    agent_id: String,
    user_api_key: String,
) -> Result<Value, reqwest::Error> {
    dotenv().ok();
    let service_role_key = env::var("SUPABASE_SERVICE_ROLE_KEY").expect("No role key found");
    let base_url = env::var("SUPABASE_SERVICE_URL").expect("No base url found");
    let url = format!("{}/rest/v1/rpc/get_agent_graph", base_url);

    let client = Client::new();
    let request = client
        .post(&url)
        .header("apikey", &service_role_key)
        .json(&GetAgentGraphRequest {
            agentid: agent_id,
            userkey: user_api_key,
        })
        .build()?;

    let response = client.execute(request).await?;
    if response.status().is_success() {
        let json: Value = response.json().await?;
        log_info!("Raw JSON response: {:?}", json);
        Ok(json)
    } else {
        log_error!("Error response: {:?}", response.status());
        response.error_for_status()?;
        unreachable!()
    }
}
