use axum::{Json, extract::State, http::StatusCode};
use shared::{OrderEvent, OrderResponse};
use std::sync::Arc;

use crate::kafka::AppState;

pub async fn create_order(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<OrderEvent>,
) -> (StatusCode, Json<OrderResponse>) {
    let order_id = payload.order_id.clone();

    match state.tx.try_send(payload) {
        Ok(_) => (
            StatusCode::ACCEPTED,
            Json(OrderResponse {
                success: true,
                message: "Order queued for processing".to_string(),
                order_id: Some(order_id),
            }),
        ),
        Err(_) => {
            eprintln!("Internal buffer full! Dropping message.");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(OrderResponse {
                    success: false,
                    message: "Server busy, try again later".to_string(),
                    order_id: Some(order_id),
                }),
            )
        }
    }
}
