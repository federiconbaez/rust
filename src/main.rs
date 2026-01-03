mod api;
mod config;
mod db;
mod models;
mod security;

use axum::{
    Router,
    middleware,
};
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use tower::ServiceBuilder;

use crate::api::{AppState, create_router};
use crate::config::Config;
use crate::db::create_pool;
use crate::security::auth::AuthService;
use crate::security::encryption::EncryptionService;
use crate::security::rate_limit::create_rate_limiter;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,nexusdb_backend=debug".into()),
        )
        .init();

    tracing::info!("Starting NexusDB Backend...");

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Configuration loaded");

    // Create database pool
    let db_pool = create_pool(&config.database_url).await?;
    tracing::info!("Database initialized");

    // Initialize services
    let auth_service = Arc::new(AuthService::new(
        &config.jwt_secret,
        config.jwt_expiration_hours,
    ));
    let encryption_service = Arc::new(EncryptionService::new(&config.encryption_key)?);

    // Create shared state
    let state = Arc::new(AppState {
        db: db_pool,
        auth_service: auth_service.clone(),
        encryption_service,
    });

    // Configure CORS
    let cors = CorsLayer::permissive(); // En producción, configurar de manera más restrictiva

    // Build protected routes (require authentication)
    let protected_routes = Router::new()
        .route("/api/auth/me", axum::routing::get(api::auth::get_current_user))
        .route("/api/connections", axum::routing::post(api::connections::create_connection))
        .route("/api/connections", axum::routing::get(api::connections::list_connections))
        .route("/api/connections/:id", axum::routing::get(api::connections::get_connection))
        .route("/api/connections/:id", axum::routing::delete(api::connections::delete_connection))
        .route("/api/scripts", axum::routing::post(api::scripts::create_script))
        .route("/api/scripts", axum::routing::get(api::scripts::list_scripts))
        .route("/api/scripts/:id", axum::routing::delete(api::scripts::delete_script))
        .layer(middleware::from_fn_with_state(
            auth_service.clone(),
            security::auth::auth_middleware,
        ));

    // Build public routes
    let public_routes = Router::new()
        .route("/health", axum::routing::get(api::health::health_check))
        .route("/api/auth/register", axum::routing::post(api::auth::register))
        .route("/api/auth/login", axum::routing::post(api::auth::login));

    // Combine routes
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(
            ServiceBuilder::new()
                .layer(cors)
                .layer(create_rate_limiter())
                .layer(tower_http::trace::TraceLayer::new_for_http()),
        )
        .with_state(state);

    // Start server
    let addr = format!("{}:{}", config.server_host, config.server_port);
    tracing::info!("Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
