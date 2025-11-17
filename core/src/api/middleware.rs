// API middleware
use std::sync::Arc;
use std::net::IpAddr;
use governor::{Quota, RateLimiter};
use governor::state::{InMemoryState, NotKeyed};
use governor::clock::DefaultClock;
use lazy_static::lazy_static;
use std::num::NonZeroU32;

lazy_static! {
    static ref RATE_LIMITER: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>> = {
        // Allow 100 requests per minute
        let quota = Quota::per_minute(NonZeroU32::new(100).unwrap());
        Arc::new(RateLimiter::direct(quota))
    };
}

use log::warn;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
    http::StatusCode,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::api::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    // Check rate limit
    match RATE_LIMITER.check() {
        Ok(_) => {
            // Request allowed
            Ok(next.run(request).await)
        }
        Err(_) => {
            // Rate limit exceeded
            warn!("Rate limit exceeded for request");
            Err(StatusCode::TOO_MANY_REQUESTS)
        }
    }
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