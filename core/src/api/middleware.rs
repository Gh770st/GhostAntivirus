// API middleware
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
    http::StatusCode,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::api::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub role: String,
    pub exp: i64,
}

/// Authentication middleware
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    if let Some(auth_header) = auth_header {
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            // Verify token
            match verify_token(token, &state.config.jwt_secret) {
                Ok(claims) => {
                    // Add claims to request extensions
                    request.extensions_mut().insert(claims);
                    return Ok(next.run(request).await);
                }
                Err(_) => return Err(StatusCode::UNAUTHORIZED),
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

/// Verify JWT token
fn verify_token(token: &str, secret: &str) -> anyhow::Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

/// Admin role middleware
pub async fn admin_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get claims from request extensions
    let claims = request
        .extensions()
        .get::<Claims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Check if user is admin
    if claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(request).await)
}

/// Rate limiting middleware (simple implementation)
pub async fn rate_limit_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // TODO: Implement actual rate limiting
    // For now, just pass through
    Ok(next.run(request).await)
}

/// Logging middleware
pub async fn logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    
    tracing::info!("Request: {} {}", method, uri);
    
    let response = next.run(request).await;
    
    tracing::info!("Response: {} {} - {}", method, uri, response.status());
    
    response
}