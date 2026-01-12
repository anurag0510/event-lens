# Event Lens

A high-performance event-driven analytics platform built with Rust, demonstrating real-time event streaming, indexing, and querying capabilities.

## Architecture

```
┌──────────────┐      ┌─────────┐      ┌──────────────┐      ┌────────────────┐
│   Producer   │─────▶│  Kafka  │─────▶│   Consumer   │─────▶│ Elasticsearch  │
│  (HTTP API)  │      │         │      │  (Indexer)   │      │                │
└──────────────┘      └─────────┘      └──────────────┘      └────────────────┘
                                      │
                                      ▼
                                ┌────────────────┐
                                │ Query Service  │
                                │   (GraphQL)    │
                                └────────────────┘
```

## Services

### Producer Service

- **Port**: 3000
- **Endpoint**: `POST /orders`
- **Technology**: Axum web framework with asynchronous Kafka producer
- **Features**:
  - High-throughput order event generation
  - Background worker for non-blocking Kafka publishing
  - Buffered channel (10,000 capacity) for event queuing
- **Code**: [crates/producer/src/main.rs](crates/producer/src/main.rs)

### Consumer Service

- **Technology**: Kafka consumer with asynchronous processing
- **Features**:
  - Real-time event consumption from Kafka topics
  - Event indexing and processing
  - Integration with downstream services
- **Code**: [crates/consumer/src/main.rs](crates/consumer/src/main.rs)

## Prerequisites

- Rust 1.70+ (edition 2024)
- Apache Kafka (localhost:9092)
- pkg-config (for building dependencies)

## Installation

```bash
# Install Rust dependencies
cargo build

# Install pkg-config (macOS)
brew install pkgconf
```

## Running the Service

```bash
# Start Producer
cargo run --bin producer

# Start Consumer
cargo run --bin consumer
```

## Usage

### Creating Orders (Producer)

```bash
curl -X POST http://localhost:3000/orders \
  -H "Content-Type: application/json" \
  -d '{
  "order_id": "ORDER-001",
  "region": "US-WEST",
  "category": "electronics"
  }'
```

## Project Structure

```
event-lens/
├── crates/
│   ├── producer/       # HTTP API for event generation
│   │   └── src/
│   │       ├── main.rs
│   │       ├── config/
│   │       ├── handlers/
│   │       └── kafka/  # Kafka producer logic
│   ├── consumer/       # Kafka consumer for event processing
│   │   └── src/
│   │       └── main.rs
│   └── shared/         # Common data structures
│       └── src/
├── Cargo.toml          # Workspace configuration
└── README.md
```

## Key Technologies

- **Web Framework**: [Axum](https://github.com/tokio-rs/axum)
- **Async Runtime**: [Tokio](https://tokio.rs)
- **Kafka Client**: [rdkafka](https://github.com/fede1024/rust-rdkafka)

## Configuration

### Kafka Brokers

Configure in [`Config::default()`](crates/producer/src/config/mod.rs)

## Health Checks

- **Producer**: Check logs for "High-throughput Producer listening on"
- **Consumer**: Check logs for successful Kafka connection and consumption

## Warnings

The project currently has an unused field:

- `channel_buffer_size` in producer config

This can be removed or will be utilized in future iterations.
