use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use validator::Validate;

use crate::api::AppState;
use crate::db::repository::ConnectionRepository;
use crate::models::CreateConnectionRequest;
use crate::security::auth::AuthUser;

pub async fn create_connection(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(req): Json<CreateConnectionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    req.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Validation error: {}", e)))?;

    // Encrypt password
    let encrypted_password = state
        .encryption_service
        .encrypt_credentials(&req.password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Create connection
    let conn = ConnectionRepository::create(
        &state.db,
        &auth_user.user_id,
        &req.name,
        &req.db_type,
        &req.host,
        req.port,
        &req.username,
        &encrypted_password,
        req.database_name.as_deref(),
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({
        "id": conn.id,
        "name": conn.name,
        "db_type": conn.db_type,
        "host": conn.host,
        "port": conn.port,
        "username": conn.username,
        "database_name": conn.database_name,
        "status": conn.status,
        "created_at": conn.created_at,
    })))
}

pub async fn list_connections(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
) -> Result<Json<Vec<serde_json::Value>>, (StatusCode, String)> {
    let connections = ConnectionRepository::find_by_user(&state.db, &auth_user.user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response: Vec<_> = connections
        .into_iter()
        .map(|conn| {
            serde_json::json!({
                "id": conn.id,
                "name": conn.name,
                "db_type": conn.db_type,
                "host": conn.host,
                "port": conn.port,
                "username": conn.username,
                "database_name": conn.database_name,
                "status": conn.status,
                "created_at": conn.created_at,
            })
        })
        .collect();

    Ok(Json(response))
}

pub async fn get_connection(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let conn = ConnectionRepository::find_by_id(&state.db, &id, &auth_user.user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Connection not found".to_string()))?;

    Ok(Json(serde_json::json!({
        "id": conn.id,
        "name": conn.name,
        "db_type": conn.db_type,
        "host": conn.host,
        "port": conn.port,
        "username": conn.username,
        "database_name": conn.database_name,
        "status": conn.status,
        "created_at": conn.created_at,
    })))
}

pub async fn delete_connection(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let deleted = ConnectionRepository::delete(&state.db, &id, &auth_user.user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((StatusCode::NOT_FOUND, "Connection not found".to_string()))
    }
}
