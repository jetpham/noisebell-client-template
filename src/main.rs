use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info};

mod logging;

use noisebell_client_template::{AppState, create_app, find_available_port, register_with_server, WebhookEvent};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    logging::init()?;
    info!("Starting Noisebell client...");

    let state = Arc::new(Mutex::new(AppState {
        current_state: WebhookEvent::Unknown("unknown".to_string()),
    }));

    let app = create_app(state);

    let port = find_available_port(3000)?;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let client_url = format!("http://{}", addr);

    info!("Webhook client listening on {}", client_url);

    let server_url = std::env::var("SERVER_URL").unwrap_or_else(|_| {
        error!("SERVER_URL environment variable is required");
        std::process::exit(1);
    });

    // Register with server
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
                info!("Retrying registration in 5 seconds...");
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }
    
    // Start webhook server
    info!("Starting server on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
