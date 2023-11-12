//path: src\crates\agent\src\internal\run_agent.rs

extern crate openssl;
use agent_types::AgentGraphData;
use axum::response::sse::{Event, Sse};
use futures::Stream;
use logger::log_info;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tokio_stream::wrappers::ReceiverStream;

use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};

async fn produce_message(topic: &str, message: &str) {
    let producer: FutureProducer = ClientConfig::new()
        .set("security.protocol", "sasl_ssl")
        .set("sasl.mechanism", "SCRAM-SHA-256")
        .set(
            "bootstrap.servers",
            std::env::var("KAFKA_BOOTSTRAP_SERVERS").unwrap(),
        )
        .set(
            "sasl.username",
            std::env::var("KAFKA_SASL_USERNAME").unwrap(),
        )
        .set(
            "sasl.password",
            std::env::var("KAFKA_SASL_PASSWORD").unwrap(),
        )
        .create()
        .expect("Producer creation error");

    let record = FutureRecord::to(topic).payload(message).key("key");

    producer.send(record, Duration::from_secs(0)).await.unwrap();
}

pub async fn run_agent_stream(
    agent_graph_data: AgentGraphData,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>> + Send + 'static> {
    // kafka
    let topic = std::env::var("KAFKA_TOPIC").unwrap();
    log_info!("Kafka topic: {}", topic);
    produce_message(&topic, "<start>").await;

    log_info!("Starting run_agent_stream");
    // sse
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

    produce_message(&topic, "</end>").await;
    log_info!("Ending run_agent_stream");
    Sse::new(ReceiverStream::new(rx))
}
