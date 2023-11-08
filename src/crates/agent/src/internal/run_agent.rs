//path: src\crates\agent\src\internal\run_agent.rs

use axum::{
    extract,
    response::sse::{Event, Sse},
};
use futures::Stream;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tokio_stream::wrappers::ReceiverStream;

use crate::AgentQueryParams;

pub async fn run_agent(
    extract::Query(query_params): extract::Query<AgentQueryParams>,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>> + Send + 'static> {
    let (tx, rx) = mpsc::channel(1);
    let words = query_params
        .payload
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();

    tokio::spawn(async move {
        tx.send(Ok(Event::default().data("<start>"))).await.ok();
        for word in words {
            sleep(Duration::from_secs(3)).await;
            tx.send(Ok(Event::default().data(word))).await.ok();
        }

        tx.send(Ok(Event::default().data("<end>"))).await.ok();
    });

    Sse::new(ReceiverStream::new(rx))
}
