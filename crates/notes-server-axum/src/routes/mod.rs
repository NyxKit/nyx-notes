pub mod auth;
pub mod comments;
pub mod notes;
pub mod server;
pub mod users;
pub mod vaults;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        // Auth (unauthenticated)
        .route("/api/auth/mode", get(auth::get_mode))
        .route("/api/auth/initialized", get(auth::get_initialized))
        .route("/api/auth/setup", post(auth::setup))
        .route("/api/auth/login", post(auth::login))
        .route("/api/users", get(users::list_users).post(users::create_user))
        .route("/api/users/:user_id", patch(users::update_user).delete(users::delete_user))
        .route("/api/server", get(server::get_server))
        .route("/api/server/vaults", get(vaults::list_server_vaults).post(vaults::create_server_vault))
        // Notes (vault-scoped)
        .route(
            "/api/vaults/:vault_id/notes",
            get(notes::list_notes).post(notes::create_note),
        )
        .route(
            "/api/vaults/:vault_id/notes/:id",
            get(notes::get_note)
                .put(notes::update_note)
                .delete(notes::delete_note),
        )
        .route(
            "/api/vaults/:vault_id/notes/:id/permission",
            patch(notes::patch_note_permission),
        )
        // Comments (note-scoped)
        .route(
            "/api/vaults/:vault_id/notes/:note_id/comments",
            get(comments::list_comments).post(comments::create_comment),
        )
        .route(
            "/api/vaults/:vault_id/notes/:note_id/comments/:comment_id",
            patch(comments::patch_comment).delete(comments::delete_comment),
        )
        .route(
            "/api/vaults/:vault_id/notes/:note_id/comments/:comment_id/replies",
            post(comments::create_reply),
        )
        .route(
            "/api/vaults/:vault_id/notes/:note_id/comments/:comment_id/replies/:reply_id",
            delete(comments::delete_reply),
        )
        // Personal vaults
        .route(
            "/api/vaults",
            get(vaults::list_vaults).post(vaults::create_vault),
        )
        .route("/api/vaults/personal", get(vaults::list_personal_vaults))
        .route(
            "/api/vaults/:vault_id",
            patch(vaults::patch_vault).delete(vaults::delete_vault),
        )
        .route(
            "/api/server/vaults/:vault_id",
            delete(vaults::delete_server_vault),
        )
}
