use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct RunAgentRequest {
    pub prompt: String,
    pub agent_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct AgentGraphData {
    pub edges: Vec<Edge>,
    pub nodes: Vec<Node>,
    #[serde(skip_deserializing)]
    pub viewport: Option<Value>,
}

#[derive(Deserialize, Serialize)]
pub struct Edge {
    pub id: String,
    #[serde(rename = "type")]
    pub edge_type: String,
    pub source: String,
    pub target: String,
    #[serde(rename = "zIndex")]
    pub z_index: i32,
    #[serde(rename = "sourceHandle")]
    pub source_handle: String,
    #[serde(rename = "targetHandle")]
    pub target_handle: String,
}

#[derive(Deserialize, Serialize)]
pub struct Node {
    pub id: String,
    pub data: NodeData,
    #[serde(rename = "type")]
    pub node_type: String,
    pub width: f32,
    pub height: f32,
    pub position: Position,
    pub selected: bool,
    #[serde(rename = "positionAbsolute")]
    pub position_absolute: Position,
    pub dragging: Option<bool>,
}

#[derive(Deserialize, Serialize)]
pub struct NodeData {
    pub body: String,
    #[serde(rename = "nodeId")]
    pub node_id: String,
    pub handles: Vec<Handle>,
    #[serde(rename = "nodeName")]
    pub node_name: String,
    #[serde(rename = "nodeType")]
    pub node_type: String,
    #[serde(rename = "nodePosition")]
    pub node_position: Position,
}

#[derive(Deserialize, Serialize)]
pub struct Handle {
    pub id: String,
    #[serde(rename = "type")]
    pub handle_type: String,
    pub angle: f32,
    pub offset: Position,
}

#[derive(Deserialize, Serialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

// #[derive(Deserialize, Serialize)]
// pub struct Viewport {
//     pub x: f32,
//     pub y: f32,
//     pub zoom: f32,
// }
