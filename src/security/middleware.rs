use axum::{
    extract::{Request, State},
    http::{StatusCode, HeaderValue},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use crate::api::AppState;
use crate::security::repository::SecurityRepository;
use axum_extra::extract::ConnectInfo;
use std::net::SocketAddr;

pub async fn security_middleware(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    // 1. Check IP Ban
    let ip = addr.ip().to_string();
    let is_banned = SecurityRepository::is_banned(&state.db, "IP", &ip)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if is_banned {
        tracing::warn!("Blocked request from banned IP: {}", ip);
        return Err((StatusCode::FORBIDDEN, "Access Denied".to_string()));
    }

    // 2. Add Security Headers
    let mut response = next.run(req).await;
    let headers = response.headers_mut();

    headers.insert("X-Frame-Options", HeaderValue::from_static("DENY"));
    headers.insert("X-Content-Type-Options", HeaderValue::from_static("nosniff"));
    headers.insert("X-XSS-Protection", HeaderValue::from_static("1; mode=block"));
    headers.insert("Strict-Transport-Security", HeaderValue::from_static("max-age=31536000; includeSubDomains"));
    headers.insert("Content-Security-Policy", HeaderValue::from_static("default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'"));

    Ok(response)
}
