use axum::{
    routing::{get, post},
    Router,
};
use std::env;

use tower_http::trace::TraceLayer;

mod config;
mod db;
mod errors;
mod handlers;
mod models;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // Load .env
    dotenvy::dotenv().ok();

    let pool = db::init_db().await;

    // Build our application with a route
    let app = Router::new()
        .route("/api/shorten", post(handlers::shorten_url))
        .route("/:code", get(handlers::redirect_url))
        .route("/api/stats/:code", get(handlers::get_stats))
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::info!("listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
