//path: src\crates\agent\src\internal\run_agent.rs

use axum::{
    response::sse::{Event, Sse},
    Json,
};
use futures::Stream;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tokio_stream::wrappers::ReceiverStream;

use crate::AgentData;

pub async fn run_agent(
    Json(agent_data): Json<AgentData>,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>> + Send + 'static> {
    let (tx, rx) = mpsc::channel(1);
    let words = agent_data
        .payload
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();

    tokio::spawn(async move {
        tx.send(Ok(
            Event::default().data(format!("<start agentid={}>", agent_data.agent_id))
        ))
        .await
        .ok();

        for word in words {
            sleep(Duration::from_secs(1)).await;
            tx.send(Ok(Event::default().data(word))).await.ok();
        }

        tx.send(Ok(Event::default().data("</end>"))).await.ok();
    });

    Sse::new(ReceiverStream::new(rx))
}
