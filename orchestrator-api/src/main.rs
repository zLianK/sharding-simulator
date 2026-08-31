use crate::log::log_install;
use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tracing::info;

mod log;

#[tokio::main]
async fn main() {
    log_install();

    let listener = get_listener().await;

    let app = Router::new()
        .route("/seed", get(seed))
        .route("/shard", get(shard))
        .route("/reshard", get(reshard));

    axum::serve(listener, app).await.unwrap();
}

/// Returns a TCP listener for the orchestrator API.
async fn get_listener() -> TcpListener {
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let addr = listener.local_addr().unwrap();
    info!("listening on {addr}");
    listener
}

/// Starts the seeding process.
async fn seed() -> String {
    "The seeding process has started successfully.".to_string()
}

/// Starts the sharding process.
async fn shard() -> String {
    "The sharding process has started successfully.".to_string()
}

/// Starts the resharding process.
async fn reshard() -> String {
    "The resharding process has started successfully.".to_string()
}
