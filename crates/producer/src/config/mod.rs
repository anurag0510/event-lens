use std::net::SocketAddr;

pub struct Config {
    pub kafka_brokers: String,
    pub server_addr: SocketAddr,
    pub channel_buffer_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            kafka_brokers: "localhost:9092".to_string(),
            server_addr: SocketAddr::from(([127, 0, 0, 1], 3000)),
            channel_buffer_size: 10_000,
        }
    }
}
