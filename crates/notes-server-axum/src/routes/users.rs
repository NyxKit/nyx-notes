use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use notes_core::{CreateUserInput, ManagedUserSummary, UpdateUserInput};

use crate::{
    auth_extractor::AuthenticatedUser,
    error::AppError,
    types::{AuthConfig, CreateUserRequest, UpdateUserRequest},
    AppState,
};

fn ensure_secret_key_mode(state: &AppState) -> Result<(), AppError> {
    if state.auth_config == AuthConfig::SecretKey {
        Ok(())
    } else {
        Err(AppError::UnprocessableEntity(
            "user management is only available in secret_key mode".into(),
        ))
    }
}

pub async fn list_users(
    State(state): State<AppState>,
    AuthenticatedUser(actor): AuthenticatedUser,
) -> Result<Json<Vec<ManagedUserSummary>>, AppError> {
    ensure_secret_key_mode(&state)?;
    state.auth.list_users(&actor).map(Json).map_err(AppError::from)
}

pub async fn create_user(
    State(state): State<AppState>,
    AuthenticatedUser(actor): AuthenticatedUser,
    Json(body): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<ManagedUserSummary>), AppError> {
    ensure_secret_key_mode(&state)?;
    let created = state.auth.create_user(
        &actor,
        CreateUserInput {
            username: body.username,
            email: body.email,
            display_name: body.display_name,
            role: body.role,
            password: body.password,
        },
    )?;
    Ok((StatusCode::CREATED, Json(created)))
}

pub async fn update_user(
    State(state): State<AppState>,
    AuthenticatedUser(actor): AuthenticatedUser,
    Path(user_id): Path<String>,
    Json(body): Json<UpdateUserRequest>,
) -> Result<Json<ManagedUserSummary>, AppError> {
    ensure_secret_key_mode(&state)?;
    state
        .auth
        .update_user(
            &actor,
            &user_id,
            UpdateUserInput {
                email: body.email,
                display_name: body.display_name,
                role: body.role,
                password: body.password,
            },
        )
        .map(Json)
        .map_err(AppError::from)
}

pub async fn delete_user(
    State(state): State<AppState>,
    AuthenticatedUser(actor): AuthenticatedUser,
    Path(user_id): Path<String>,
) -> Result<StatusCode, AppError> {
    ensure_secret_key_mode(&state)?;
    state.auth.delete_user(&actor, &user_id)?;
    Ok(StatusCode::NO_CONTENT)
}
