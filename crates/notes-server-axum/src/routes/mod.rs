pub mod auth;
pub mod notes;
pub mod teams;
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
        .route("/api/auth/login", post(auth::login))
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
        // Personal vaults
        .route(
            "/api/vaults",
            get(vaults::list_vaults).post(vaults::create_vault),
        )
        .route("/api/vaults/:vault_id", delete(vaults::delete_vault))
        // Team vault permission
        .route(
            "/api/teams/:team_id/vaults/:vault_id/permission",
            patch(vaults::patch_vault_permission),
        )
        // Teams
        .route(
            "/api/teams",
            get(teams::list_teams).post(teams::create_team),
        )
        .route(
            "/api/teams/:team_id",
            get(teams::get_team).delete(teams::delete_team),
        )
        .route(
            "/api/teams/:team_id/members",
            post(teams::add_member),
        )
        .route(
            "/api/teams/:team_id/members/:user_id",
            patch(teams::patch_member).delete(teams::remove_member),
        )
        .route(
            "/api/teams/:team_id/vaults",
            post(teams::create_team_vault),
        )
        .route(
            "/api/teams/:team_id/vaults/:vault_id",
            delete(teams::delete_team_vault),
        )
}
