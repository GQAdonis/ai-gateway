use crate::config::AppConfig;
use axum::{
    routing::{get, post},
    Router,
};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
};
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod context;
mod error;
mod handlers;
mod providers;
mod proxy;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "magicapi_ai_gateway=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Arc::new(AppConfig::new());

    // Initialize the proxy
    proxy::initialize_proxy(config.clone())
        .await
        .expect("Failed to initialize proxy");

    // Set up CORS
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    // Build the router
    let app = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/v1/*path", post(handlers::handle_request))
        .layer(cors)
        .layer(CompressionLayer::new())
        .with_state(config.clone());

    // Get the bind address
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    info!(
        "Starting server with {} workers",
        config.workers
    );

    debug!(
        "Listening on {}:{} with {} workers",
        config.host,
        config.port,
        config.workers
    );

    // Start the server
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
