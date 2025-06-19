use reqwest::Client;
use crate::models::{StatusResponse, HealthResponse, WebhookListResponse};
use tracing::debug;

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
        Ok(())
    } else {
        let error_msg = response.text().await?;
        if error_msg.contains("Webhook endpoint already exists") {
            tracing::info!("Webhook endpoint already exists, skipping registration");
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Failed to register with server: {}",
                error_msg
            ))
        }
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