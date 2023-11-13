//path: src\crates\agent\src\internal\run_agent.rs

extern crate openssl;
use agent_types::AgentGraphData;
use axum::response::sse::{Event, Sse};
use futures::Stream;
use futures::StreamExt;
use logger::log_info;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio_stream::wrappers::ReceiverStream;

use super::create_agent_stream::create_agent_stream;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::Message;

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

async fn handle_kafka_message(
    msg: rdkafka::message::BorrowedMessage<'_>,
    sender: &Sender<Result<Event, axum::Error>>,
) {
    if let Some(payload) = msg.payload_view::<str>() {
        let payload_str = payload.unwrap_or("");
        log_info!(format!("Received message: {}", payload_str));
        sender
            .send(Ok(Event::default().data(payload_str)))
            .await
            .unwrap();
    }
}

pub async fn run_agent_stream(
    agent_id: String,
    agent_graph_data: AgentGraphData,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>> + Send + 'static> {
    log_info!(">>> Starting run_agent_stream");

    let stream = create_agent_stream();
    let topic = std::env::var("KAFKA_TOPIC").unwrap();
    let nexus_id = std::env::var("NEXUS_GATEWAY_ID").unwrap();

    // Send the start message to Kafka
    let start_message = format!(
        "<start nexus_id={} instance_id={} agent_id={} graph={:?}>",
        nexus_id, stream.instance_id, agent_id, agent_graph_data
    );
    produce_message(&topic, &start_message).await;

    // kafka consumer
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", nexus_id)
        .set("enable.auto.commit", "true")
        .set("security.protocol", "sasl_ssl")
        .set("auto.offset.reset", "earliest")
        .set(
            "bootstrap.servers",
            std::env::var("KAFKA_BOOTSTRAP_SERVERS").unwrap(),
        )
        .set("sasl.mechanism", "SCRAM-SHA-256")
        .set(
            "sasl.username",
            std::env::var("KAFKA_SASL_USERNAME").unwrap(),
        )
        .set(
            "sasl.password",
            std::env::var("KAFKA_SASL_PASSWORD").unwrap(),
        )
        .create()
        .expect("Consumer creation error");

    consumer
        .subscribe(&[&topic])
        .expect("Failed to subscribe to topic");

    tokio::spawn(async move {
        let mut stop = false;

        while let Some(message) = consumer.stream().next().await {
            let msg = message.unwrap();
            if let Some(payload) = msg.payload_view::<str>() {
                let payload_str = payload.unwrap_or("");
                if payload_str.starts_with("</end") {
                    stop = true;
                }
                handle_kafka_message(msg, &stream.sender).await;
            }

            if stop {
                log_info!("Received stop message, ending stream");
                break;
            }
        }
    });

    // Return the SSE stream
    Sse::new(ReceiverStream::new(stream.receiver))
}
