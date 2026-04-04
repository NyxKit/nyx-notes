use axum::{extract::State, Json};
use notes_core::slugify;

use crate::{auth_extractor::AuthenticatedUser, types::ServerMetadataResponse, AppState};

pub async fn get_server(
    State(_state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Json<ServerMetadataResponse> {
    let name = std::env::var("SERVER_NAME").unwrap_or_else(|_| "Main Server".into());
    Json(ServerMetadataResponse {
        id: format!("server-{}", slugify(&name)),
        slug: slugify(&name),
        name,
        current_user_id: user.id,
        role: match user.role {
            notes_core::ServerRole::Admin => "admin",
            notes_core::ServerRole::User => "user",
        },
        root_path: std::env::var("NYX_ROOT").ok(),
    })
}
