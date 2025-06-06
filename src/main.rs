use axum::{extract::State, routing::post, Json, Router};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::net::{SocketAddr, TcpListener};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info, warn, debug};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

const WEBHOOK_DESCRIPTION: &str = "Webhook Client";

// Webhook endpoint structure
#[derive(Debug, Serialize)]
struct Endpoint {
    url: String,
    description: String,
}

// Webhook payload structure matching the sender
#[derive(Debug, Deserialize)]
struct WebhookPayload {
    event_type: String,
    timestamp: String,
    new_state: String,
}

// Circuit state enum
#[derive(Debug, Clone, PartialEq)]
enum CircuitState {
    Open,
    Closed,
}

impl From<&str> for CircuitState {
    fn from(s: &str) -> Self {
        match s {
            "open" => CircuitState::Open,
            "closed" => CircuitState::Closed,
            _ => {
                error!("Invalid circuit state received: {}", s);
                panic!("Invalid circuit state");
            }
        }
    }
}

// Shared state
struct AppState {
    current_state: CircuitState,
}

fn find_available_port(start_port: u16) -> anyhow::Result<u16> {
    debug!("Searching for available port starting from {}", start_port);
    let port = (start_port..start_port + 1000)
        .find(|port| TcpListener::bind(("127.0.0.1", *port)).is_ok())
        .ok_or_else(|| anyhow::anyhow!("No available ports found in range {} to {}", start_port, start_port + 1000))?;
    info!("Found available port: {}", port);
    Ok(port)
}

async fn register_with_server(client_url: &str, server_url: &str) -> Result<(), anyhow::Error> {
    debug!("Attempting to register with server at {}", server_url);
    let client = Client::new();
    let endpoint = Endpoint {
        url: client_url.to_string(),
        description: WEBHOOK_DESCRIPTION.to_string(),
    };

    let response = client
        .post(format!("{}/endpoints", server_url))
        .json(&endpoint)
        .send()
        .await?;

    debug!("Registration response status: {}", response.status());
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    debug!("Initializing logging system");
    fs::create_dir_all("logs")?;

    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("webhook-client")
        .filename_suffix("log")
        .max_log_files(7)
        .build("logs")?;

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let filter = tracing_subscriber::filter::Targets::new()
        .with_target("webhook_client", LevelFilter::DEBUG)
        .with_target("hyper", LevelFilter::WARN)
        .with_target("hyper_util", LevelFilter::WARN);

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::Layer::default().with_writer(std::io::stdout))
        .with(fmt::Layer::default().with_writer(non_blocking))
        .init();

    info!("Starting webhook client...");

    let state = Arc::new(Mutex::new(AppState {
        current_state: CircuitState::Open,
    }));

    let app = Router::new()
        .route("/webhook", post(handle_webhook))
        .with_state(state);

    let port = find_available_port(3000)?;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let client_url = format!("http://{}", addr);

    info!("Webhook client listening on {}", client_url);

    let server_url =
        std::env::var("SERVER_URL").unwrap_or_else(|_| {
            warn!("SERVER_URL not set, using default: http://127.0.0.1:8080");
            "http://127.0.0.1:8080".to_string()
        });
    
    let start_time = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(300);

    info!("Attempting to register with server at {}", server_url);
    while start_time.elapsed() < timeout {
        match register_with_server(&client_url, &server_url).await {
            Ok(_) => {
                info!("Successfully registered with server at {}", server_url);
                break;
            }
            Err(e) => {
                error!("Failed to register with server: {}", e);
                warn!("Retrying registration in 5 seconds...");
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }

    info!("Starting server on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service())
        .await?;

    Ok(())
}

async fn handle_webhook(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<WebhookPayload>,
) {
    debug!("Received webhook payload: {:?}", payload);
    let new_state = CircuitState::from(payload.new_state.as_str());

    let mut state = state.lock().await;
    let old_state = state.current_state.clone();
    state.current_state = new_state.clone();

    info!("Webhook event details:");
    info!("  Event Type: {}", payload.event_type);
    info!("  Timestamp: {}", payload.timestamp);
    info!("  State Change: {:?} -> {:?}", old_state, new_state);

    match new_state {
        CircuitState::Open => {
            info!("Circuit is now OPEN - No noise detected");
        }
        CircuitState::Closed => {
            info!("Circuit is now CLOSED - Noise detected!");
        }
    }
}
