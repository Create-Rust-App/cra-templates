//! HS256 JWT authentication for the Axum service.
//!
//! [`AuthUser`] is an extractor that rejects requests without a valid
//! `Authorization: Bearer <token>` header. Tokens are minted with
//! [`create_token`] and verified against `AuthConfig::from_env()`.

use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts, StatusCode},
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::{
    future::Future,
    pin::Pin,
    time::{SystemTime, UNIX_EPOCH},
};

/// Claims carried by every token minted with [`create_token`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Claims {
    /// Subject the token was issued for (e.g. a user id).
    pub sub: String,
    /// Issued-at timestamp (seconds since epoch).
    pub iat: u64,
    /// Expiry timestamp (seconds since epoch).
    pub exp: u64,
}

/// Authenticated caller extracted from a valid bearer token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthUser {
    /// Subject from the verified token claims.
    pub sub: String,
}

/// Runtime configuration for JWT signing and verification.
#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// HMAC secret used to sign and verify tokens.
    pub secret: String,
    /// Token time-to-live in seconds.
    pub ttl_secs: u64,
}

impl AuthConfig {
    /// Load configuration from the environment.
    ///
    /// Reads `JWT_SECRET` (falls back to a development-only default) and
    /// `JWT_TTL_SECS` (defaults to one hour).
    pub fn from_env() -> Self {
        let ttl_secs = std::env::var("JWT_TTL_SECS")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(3600);
        Self {
            secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "dev-only-change-me".to_string()),
            ttl_secs,
        }
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before epoch")
        .as_secs()
}

/// Mint a signed HS256 token for `sub` valid for `config.ttl_secs`.
pub fn create_token(config: &AuthConfig, sub: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = now_secs();
    let claims = Claims {
        sub: sub.to_string(),
        iat: now,
        exp: now + config.ttl_secs,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
}

/// Verify `token` and return its claims, or the validation error.
pub fn verify_token(
    config: &AuthConfig,
    token: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}

// Manual implementation matching the `#[async_trait]` expansion used by
// axum 0.7 (`Pin<Box<dyn Future>>`), so no extra dependency is needed.
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    fn from_request_parts<'life0, 'life1, 'async_trait>(
        parts: &'life0 mut Parts,
        _state: &'life1 S,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Rejection>> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        let header: Option<String> = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        Box::pin(async move {
            let header = header.ok_or(StatusCode::UNAUTHORIZED)?;
            let token = header
                .strip_prefix("Bearer ")
                .ok_or(StatusCode::UNAUTHORIZED)?;
            let config = AuthConfig::from_env();
            let claims =
                verify_token(&config, token).map_err(|_| StatusCode::UNAUTHORIZED)?;
            if claims.exp <= now_secs() {
                return Err(StatusCode::UNAUTHORIZED);
            }
            Ok(Self { sub: claims.sub })
        })
    }
}
