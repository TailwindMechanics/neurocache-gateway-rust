//path: src\crates\agent\src\internal\run_agent.rs

use agent_types::AgentGraphData;
use axum::response::sse::{Event, Sse};
use futures::Stream;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tokio_stream::wrappers::ReceiverStream;

pub async fn run_agent_stream(
    agent_graph_data: AgentGraphData,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>> + Send + 'static> {
    let (tx, rx) = mpsc::channel(1);

    tokio::spawn(async move {
        tx.send(Ok(Event::default().data("<start>"))).await.unwrap();

        for node in agent_graph_data.nodes {
            sleep(Duration::from_secs(1)).await;
            let node_data = serde_json::to_string(&node).unwrap();
            tx.send(Ok(Event::default().data(node_data))).await.unwrap();
        }

        tx.send(Ok(Event::default().data("</end>"))).await.unwrap();
    });

    Sse::new(ReceiverStream::new(rx))
}
