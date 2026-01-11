use elasticsearch::{
    BulkOperation, BulkParts, Elasticsearch,
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::{ClientConfig, Message};
use shared::OrderEvent;
use std::time::{Duration, Instant};
use url::Url;

#[tokio::main]
async fn main() {
    let url = Url::parse("https://elastic:anurag0510@localhost:9200").unwrap();
    let conn_pool = SingleNodeConnectionPool::new(url);
    let transport = TransportBuilder::new(conn_pool)
        .disable_proxy()
        .cert_validation(elasticsearch::cert::CertificateValidation::None)
        .build()
        .unwrap();
    let es_client = Elasticsearch::new(transport);

    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "analytics-group")
        .set("bootstrap.servers", "localhost:9092")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create()
        .expect("Consumer creation failed");

    let health = es_client
        .cluster()
        .health(elasticsearch::cluster::ClusterHealthParts::None)
        .send()
        .await;

    match health {
        Ok(res) => {
            println!("✅ ES health check status: {}", res.status_code());
            let body = res.json::<serde_json::Value>().await.unwrap();
            println!("Cluster health response: {}", body);
        }
        Err(e) => {
            eprintln!("❌ ES health check failed: {}", e);
            return; // STOP here if this fails
        }
    }

    consumer.subscribe(&["orders"]).expect("Can't subscribe");

    let mut buffer = Vec::new();
    let mut last_flush = Instant::now();
    let batch_size = 100;
    let max_delay = Duration::from_millis(200);

    println!("🚀 Consumer started. Ready to bulk-gulp into Elasticsearch...");

    loop {
        // We use a timeout so we can check our 'max_delay' timer
        match tokio::time::timeout(Duration::from_millis(50), consumer.recv()).await {
            Ok(Ok(msg)) => {
                let payload = msg.payload_view::<str>().unwrap().unwrap();
                if let Ok(order) = serde_json::from_str::<OrderEvent>(payload) {
                    buffer.push(order);
                }
            }
            _ => {} // Timeout or Kafka error, just proceed to check flush conditions
        }

        if buffer.len() >= batch_size || (last_flush.elapsed() >= max_delay && !buffer.is_empty()) {
            flush_to_es(&es_client, &mut buffer).await;
            last_flush = Instant::now();
        }
    }
}

// ...existing code...

async fn flush_to_es(client: &Elasticsearch, buffer: &mut Vec<OrderEvent>) {
    if buffer.is_empty() {
        return;
    }

    let mut body: Vec<BulkOperation<_>> = Vec::new();

    for order in buffer.drain(..) {
        body.push(
            BulkOperation::index(order.clone())
                .id(order.order_id.clone())
                .into(),
        );
    }

    let count = body.len();
    let response = client
        .bulk(BulkParts::Index("orders"))
        .body(body)
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status_code().is_success() {
                println!("🔥 Bulk Gulp successful! Synced {} orders.", count);
            } else {
                eprintln!(
                    "❌ Bulk operation returned error status: {}",
                    res.status_code()
                );
            }
        }
        Err(e) => eprintln!("❌ Bulk operation failed: {:?}", e.to_string()),
    }
}
