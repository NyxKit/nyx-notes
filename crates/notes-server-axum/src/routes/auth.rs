use axum::{extract::State, Json};
use notes_core::{CreateUserInput, LoginToken};

use crate::{
    error::AppError,
    types::{AuthModeResponse, LoginRequest},
    AppState,
};

pub async fn get_mode(State(state): State<AppState>) -> Json<AuthModeResponse> {
    Json(AuthModeResponse::from(&state.auth_config))
}

#[derive(serde::Serialize)]
pub struct InitializedResponse {
    pub initialized: bool,
}

pub async fn get_initialized(State(state): State<AppState>) -> Json<InitializedResponse> {
    let initialized = state.auth.is_initialized().unwrap_or(false);
    Json(InitializedResponse { initialized })
}

/// Set up the initial admin user. Only works when the system has no users yet.
pub async fn setup(
    State(state): State<AppState>,
    Json(body): Json<CreateUserInput>,
) -> Result<Json<LoginToken>, AppError> {
    let mut input = body;
    input.role = Some(notes_core::domain::ServerRole::Admin);

    state
        .auth
        .setup_initial_user(input)
        .map(Json)
        .map_err(|e| match e {
            notes_core::AuthError::Conflict(_) => {
                AppError::Conflict("system already initialized".into())
            }
            notes_core::AuthError::Validation(msg) => AppError::UnprocessableEntity(msg),
            other => AppError::from(other),
        })
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
