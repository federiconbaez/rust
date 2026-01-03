pub mod auth;
pub mod connections;
pub mod scripts;
pub mod health;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::config::Config;
use crate::db::DbPool;
use crate::security::auth::AuthService;
use crate::security::encryption::EncryptionService;

pub struct AppState {
    pub db: DbPool,
    pub auth_service: Arc<AuthService>,
    pub encryption_service: Arc<EncryptionService>,
}

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        // Health check (no auth required)
        .route("/health", get(health::health_check))
        // Auth routes
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/me", get(auth::get_current_user))
        // Connection routes (auth required)
        .route("/api/connections", post(connections::create_connection))
        .route("/api/connections", get(connections::list_connections))
        .route("/api/connections/:id", get(connections::get_connection))
        .route("/api/connections/:id", axum::routing::delete(connections::delete_connection))
        // Script routes (auth required)
        .route("/api/scripts", post(scripts::create_script))
        .route("/api/scripts", get(scripts::list_scripts))
        .route("/api/scripts/:id", axum::routing::delete(scripts::delete_script))
        .with_state(state)
}
