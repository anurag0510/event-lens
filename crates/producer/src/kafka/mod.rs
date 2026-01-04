use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use shared::OrderEvent;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

pub struct AppState {
    pub tx: mpsc::Sender<OrderEvent>,
}

impl AppState {
    pub fn new(tx: mpsc::Sender<OrderEvent>) -> Arc<Self> {
        Arc::new(Self { tx })
    }
}

pub fn start_kafka_worker(producer: FutureProducer) -> mpsc::Sender<OrderEvent> {
    let (tx, mut rx) = mpsc::channel::<OrderEvent>(10_000);

    tokio::spawn(async move {
        println!("Background Kafka Worker started.");
        while let Some(event) = rx.recv().await {
            let payload_json = serde_json::to_string(&event).unwrap();
            let record = FutureRecord::to("orders")
                .payload(&payload_json)
                .key(&event.order_id);

            match producer.send(record, Duration::from_secs(2)).await {
                Ok(_) => println!("Sent order: {}", event.order_id),
                Err((e, _)) => eprintln!("Failed to send to Kafka: {:?}", e),
            }
        }
    });

    tx
}

pub fn create_kafka_producer(brokers: &str) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("queue.buffering.max.ms", "20")
        .set("batch.num.messages", "1000")
        .set("queue.buffering.max.kbytes", "4000")
        .set("compression.codec", "snappy")
        .set("message.timeout.ms", "5000")
        .set("request.required.acks", "1")
        .create()
        .expect("Producer creation error")
}
