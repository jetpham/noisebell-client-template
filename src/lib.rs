use axum::{extract::State, routing::post, Json, Router};
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info, warn};

pub const WEBHOOK_DESCRIPTION: &str = "Noisebell Client";

// Webhook payload structure matching the server
#[derive(Debug, Deserialize)]
pub struct WebhookPayload {
    pub event: String,
    pub timestamp: String,
    pub source: String,
}

// Status response structure
#[derive(Debug, Deserialize)]
pub struct StatusResponse {
    pub status: String,
    pub data: StatusData,
}

#[derive(Debug, Deserialize)]
pub struct StatusData {
    pub state: String,
}

// Health response structure
#[derive(Debug, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct WebhookListResponse {
    pub status: String,
    pub data: WebhookListData,
}

#[derive(Debug, Deserialize)]
pub struct WebhookListData {
    pub webhooks: Vec<WebhookInfo>,
}

#[derive(Debug, Deserialize)]
pub struct WebhookInfo {
    pub url: String,
    pub created_at: String,
}

// Shared state
pub struct AppState {
    pub current_state: String,
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

pub async fn register_with_server(client_url: &str, server_url: &str) -> Result<(), anyhow::Error> {
    debug!("Attempting to register with server at {}", server_url);
    let client = Client::new();
    let payload = serde_json::json!({
        "url": client_url
    });

    let response = client
        .post(format!("{}/webhooks", server_url))
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        info!("Successfully registered with server at {}", server_url);
        Ok(())
    } else {
        let error_msg = response.text().await?;
        Err(anyhow::anyhow!(
            "Failed to register with server: {}",
            error_msg
        ))
    }
}

pub async fn poll_status(server_url: &str) -> Result<String, anyhow::Error> {
    let client = Client::new();
    let response = client.get(format!("{}/status", server_url)).send().await?;

    if response.status().is_success() {
        let status: StatusResponse = response.json().await?;
        Ok(status.data.state)
    } else {
        let error_msg = response.text().await?;
        Err(anyhow::anyhow!("Failed to get status: {}", error_msg))
    }
}

pub async fn poll_health(server_url: &str) -> Result<HealthResponse, anyhow::Error> {
    let client = Client::new();
    let response = client.get(format!("{}/health", server_url)).send().await?;

    if response.status().is_success() {
        let health: HealthResponse = response.json().await?;
        Ok(health)
    } else {
        let error_msg = response.text().await?;
        Err(anyhow::anyhow!("Failed to get health: {}", error_msg))
    }
}

pub async fn list_webhooks(server_url: &str) -> Result<WebhookListResponse, anyhow::Error> {
    let client = Client::new();
    let response = client.get(format!("{}/webhooks", server_url)).send().await?;

    if response.status().is_success() {
        let webhooks: WebhookListResponse = response.json().await?;
        Ok(webhooks)
    } else {
        let error_msg = response.text().await?;
        Err(anyhow::anyhow!("Failed to list webhooks: {}", error_msg))
    }
}

pub async fn handle_webhook(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<WebhookPayload>,
) {
    debug!("Received webhook payload: {:?}", payload);
    let mut state = state.lock().await;
    state.current_state = payload.event.clone();

    info!("Webhook event details:");
    info!("  Event: {}", payload.event);
    info!("  Timestamp: {}", payload.timestamp);
    info!("  Source: {}", payload.source);

    match payload.event.as_str() {
        "open" => {
            info!("Circuit is now OPEN - No noise detected");
        }
        "closed" => {
            info!("Circuit is now CLOSED - Noise detected!");
        }
        _ => {
            warn!("Unknown event type: {}", payload.event);
        }
    }
}

pub fn create_app(state: Arc<Mutex<AppState>>) -> Router {
    Router::new()
        .route("/", post(handle_webhook))
        .with_state(state)
} 