use axum::{extract::State, Json};
use notes_core::LoginToken;

use crate::{
    error::AppError,
    types::{AuthModeResponse, LoginRequest},
    AppState,
};

pub async fn get_mode(State(state): State<AppState>) -> Json<AuthModeResponse> {
    Json(AuthModeResponse::from(&state.auth_config))
}

/// Exchange credentials for a signed JWT. Only meaningful in `secret_key` mode.
/// Returns `503 Service Unavailable` in all other modes (the default `login` impl
/// returns `AuthError::ServiceError`, which maps to 500 — override that here to 503).
pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<LoginToken>, AppError> {
    state
        .auth
        .login(&body.username, &body.password)
        .map(Json)
        .map_err(|e| match e {
            notes_core::AuthError::InvalidCredentials => AppError::Unauthorized("invalid credentials".into()),
            notes_core::AuthError::ServiceError(msg) if msg.contains("not supported") => {
                AppError::UnprocessableEntity("login is not available in this auth mode".into())
            }
            other => AppError::from(other),
        })
}
