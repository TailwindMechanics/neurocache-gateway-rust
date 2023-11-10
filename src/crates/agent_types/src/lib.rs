//path: src\crates\agent_types\src\lib.rs

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RunAgentRequest {
    pub prompt: String,
    pub agent_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct AgentGraphData {
    pub nodes: Vec<Node>,
}

#[derive(Deserialize, Serialize)]
pub struct Node {
    pub data: CustomNodeData,
}

#[derive(Deserialize, Serialize)]
pub struct CustomNodeData {
    pub node_type: String,
    pub node_name: String,
    pub category: String,
    pub node_id: String,
    pub body: String,
    pub handles: Vec<PositionId>,
    pub node_position: HashMap<String, f32>,
}

#[derive(Deserialize, Serialize)]
pub struct PositionId {
    pub id: String,
    pub type_field: String,
}
