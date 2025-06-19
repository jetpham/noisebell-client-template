use axum::{extract::State, routing::post, Json, Router, response::IntoResponse, http::StatusCode};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info, warn};

pub mod client;
pub mod models;

pub use client::{register_with_server, poll_status, poll_health, list_webhooks};
pub use models::*;

pub const WEBHOOK_DESCRIPTION: &str = "Noisebell Client";

// Shared state
pub struct AppState {
    pub current_state: WebhookEvent,
}

pub fn find_available_port(start_port: u16) -> anyhow::Result<u16> {
    debug!("Searching for available port starting from {}", start_port);
    let port = (start_port..start_port + 1000)
        .find(|port| std::net::TcpListener::bind(("127.0.0.1", *port)).is_ok())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "No available ports found in range {} to {}",
                start_port,
                start_port + 1000
            )
        })?;
    info!("Found available port: {}", port);
    Ok(port)
}

pub async fn handle_webhook(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<models::WebhookPayload>,
) -> impl IntoResponse {
    debug!("Received webhook payload: {:?}", payload);

    let mut state = state.lock().await;
    state.current_state = payload.event.clone();

    info!("Webhook event details:");
    info!("  Event: {}", payload.event);
    info!("  Timestamp: {}", payload.timestamp);
    info!("  Source: {}", payload.source);

    match payload.event {
        WebhookEvent::Open => {
            info!("Circuit is now OPEN - No noise detected");
            StatusCode::OK
        }
        WebhookEvent::Closed => {
            info!("Circuit is now CLOSED - Noise detected!");
            StatusCode::OK
        }
        WebhookEvent::Unknown(ref event_type) => {
            warn!("Unknown event type: {}", event_type);
            StatusCode::BAD_REQUEST
        }
    }
}

pub fn create_app(state: Arc<Mutex<AppState>>) -> Router {
    Router::new()
        .route("/", post(handle_webhook))
        .with_state(state)
} 