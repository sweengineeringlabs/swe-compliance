mod auth;
mod config;
mod db;
mod error;
mod middleware;
mod routes;
mod ws;

use std::net::SocketAddr;

use tracing_subscriber::EnvFilter;

use crate::config::ServerConfig;
use crate::db::Db;
use crate::middleware::{cors_layer, ScanSemaphore};
use crate::routes::{build_router, AppState};
use crate::ws::WsBroadcaster;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let config = ServerConfig::from_env();

    tracing::info!(
        "Starting swe-compliance server v{} on {}:{}",
        env!("CARGO_PKG_VERSION"),
        config.host,
        config.port
    );

    // Initialize database
    let db = Db::open(&config.db_path).expect("failed to initialize database");
    tracing::info!("Database initialized at {}", config.db_path.display());

    // Build application state
    let state = AppState {
        db,
        ws_broadcaster: WsBroadcaster::new(),
        scan_semaphore: ScanSemaphore::new(config.max_concurrent_scans),
        config: config.clone(),
    };

    // Build router with CORS
    let app = build_router(state).layer(cors_layer(&config.cors_origins));

    // Start server
    let addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .expect("invalid bind address");

    tracing::info!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind");

    axum::serve(listener, app)
        .await
        .expect("server error");
}
