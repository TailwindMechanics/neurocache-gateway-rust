//path: src\crates\agent\src\internal\create_agent_stream.rs

use axum::response::sse::Event;
use tokio::sync::mpsc::{self, Receiver, Sender};
use uuid::Uuid;

pub struct AgentStream {
    pub instance_id: Uuid,
    pub sender: Sender<Result<Event, axum::Error>>,
    pub receiver: Receiver<Result<Event, axum::Error>>,
}

pub fn create_agent_stream() -> AgentStream {
    let (sender, receiver) = mpsc::channel(1);
    let instance_id = Uuid::new_v4();

    AgentStream {
        instance_id,
        sender,
        receiver,
    }
}
