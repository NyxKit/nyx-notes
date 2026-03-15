use axum::{
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap},
};
use notes_core::User;

use crate::{error::AppError, types::AuthConfig, AppState};

/// Axum extractor that authenticates the incoming request.
///
/// In `local` mode, all requests are accepted and a fixed local user is returned —
/// no `Authorization` header is required.
///
/// In all other modes, a `Authorization: Bearer <token>` header is required and
/// validated by the configured `AuthStore`.
pub struct AuthenticatedUser(pub User);

impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        if state.auth_config == AuthConfig::Local {
            return Ok(AuthenticatedUser(local_user()));
        }

        let token = bearer_token(&parts.headers)
            .ok_or_else(|| AppError::Unauthorized("missing Bearer token".into()))?;

        // `verify_token` is synchronous (see StorageBackend rationale in backend-api.md).
        // For secret_key mode this is pure HMAC verification — negligible CPU.
        // For OIDC mode, the implementation is expected to cache the JWKS in-process.
        let user = state.auth.verify_token(token).map_err(AppError::from)?;
        Ok(AuthenticatedUser(user))
    }
}

fn bearer_token(headers: &HeaderMap) -> Option<&str> {
    headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
}

pub fn local_user() -> User {
    User {
        id: "local".into(),
        email: "local@localhost".into(),
        display_name: "Local User".into(),
    }
}
