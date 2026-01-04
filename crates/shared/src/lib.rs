use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProductCategory {
    Electronics,
    Books,
    Clothing,
    Home,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrderStatus {
    Completed,
    Pending,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderEvent {
    pub order_id: String,
    pub user_id: String,
    pub product_id: String,
    pub category: ProductCategory,
    pub quantity: i32,
    pub price: f64,
    pub region: String,
    pub status: OrderStatus,
    pub timestamp: u64,
}

#[derive(Serialize)]
pub struct OrderResponse {
    pub success: bool,
    pub message: String,
    pub order_id: Option<String>,
}
