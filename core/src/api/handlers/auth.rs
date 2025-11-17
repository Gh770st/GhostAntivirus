// Authentication handlers
use axum::{
    extract::State,
    response::Response,
    http::StatusCode,
    Json,
};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

use crate::api::{
    AppState,
    models::{LoginRequest, LoginResponse, RefreshTokenRequest, UserInfo},
};
use super::{error_response, success_response};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    username: String,
    role: String,
    exp: i64,
}

/// Login handler
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Response {
    // Demo authentication - replace with real authentication
    if payload.username == "admin" && payload.password == "admin123" {
        let user = UserInfo {
            id: "1".to_string(),
            username: payload.username.clone(),
            role: "admin".to_string(),
            email: Some("admin@ghostantivirus.com".to_string()),
        };

        let token = match generate_token(&user, &state.config.jwt_secret, 3600) {
            Ok(t) => t,
            Err(e) => return error_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        let refresh_token = match generate_token(&user, &state.config.jwt_secret, 86400) {
            Ok(t) => t,
            Err(e) => return error_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        let response = LoginResponse {
            token,
            refresh_token,
            expires_in: 3600,
            user,
        };

        success_response(response)
    } else {
        error_response(StatusCode::UNAUTHORIZED, "Invalid credentials".to_string())
    }
}

/// Logout handler
pub async fn logout() -> Response {
    success_response(serde_json::json!({
        "message": "Logged out successfully"
    }))
}

/// Refresh token handler
pub async fn refresh_token(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Response {
    // Verify the refresh token
    match verify_token(&payload.refresh_token, &state.config.jwt_secret) {
        Ok(claims) => {
            let user = UserInfo {
                id: claims.sub,
                username: claims.username,
                role: claims.role,
                email: None,
            };

            let new_token = match generate_token(&user, &state.config.jwt_secret, 3600) {
                Ok(t) => t,
                Err(e) => return error_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            };

            success_response(serde_json::json!({
                "token": new_token,
                "expires_in": 3600
            }))
        }
        Err(_) => error_response(StatusCode::UNAUTHORIZED, "Invalid refresh token".to_string()),
    }
}

/// Generate JWT token
fn generate_token(user: &UserInfo, secret: &str, expires_in: i64) -> anyhow::Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(expires_in))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user.id.clone(),
        username: user.username.clone(),
        role: user.role.clone(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
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