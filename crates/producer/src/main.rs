mod config;
mod handlers;
mod kafka;

use axum::{Router, routing::post};
use config::Config;
use handlers::create_order;
use kafka::{AppState, create_kafka_producer, start_kafka_worker};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let config = Config::default();
    let producer = create_kafka_producer(&config.kafka_brokers);

    let tx = start_kafka_worker(producer);

    let shared_state = AppState::new(tx);

    let app = Router::new()
        .route("/orders", post(create_order))
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("High-throughput Producer listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(config.server_addr)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
