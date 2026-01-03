use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use serde::Deserialize;

use crate::api::AppState;
use crate::db::repository::ScriptRepository;
use crate::security::auth::AuthUser;

#[derive(Debug, Deserialize)]
pub struct CreateScriptRequest {
    pub name: String,
    pub query: String,
    pub db_type: String,
}

pub async fn create_script(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Json(req): Json<CreateScriptRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let script = ScriptRepository::create(
        &state.db,
        &auth_user.user_id,
        &req.name,
        &req.query,
        &req.db_type,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({
        "id": script.id,
        "name": script.name,
        "query": script.query,
        "db_type": script.db_type,
        "created_at": script.created_at,
    })))
}

pub async fn list_scripts(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
) -> Result<Json<Vec<serde_json::Value>>, (StatusCode, String)> {
    let scripts = ScriptRepository::find_by_user(&state.db, &auth_user.user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response: Vec<_> = scripts
        .into_iter()
        .map(|script| {
            serde_json::json!({
                "id": script.id,
                "name": script.name,
                "query": script.query,
                "db_type": script.db_type,
                "created_at": script.created_at,
            })
        })
        .collect();

    Ok(Json(response))
}

pub async fn delete_script(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let deleted = ScriptRepository::delete(&state.db, &id, &auth_user.user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((StatusCode::NOT_FOUND, "Script not found".to_string()))
    }
}
