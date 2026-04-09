use axum::{
    extract::{Query, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use notes_core::{LiveCollection, NotePermission, ServerRole, User, Vault, VaultOwner, slugify};

use crate::{
    auth_extractor::AuthenticatedUser,
    error::AppError,
    live::scopes::normalize_query_key,
    storage_adapter::AsyncStorageAdapter,
    types::LiveSubscribeRequest,
    AppState,
};

fn current_server_slug() -> String {
    slugify(&std::env::var("SERVER_NAME").unwrap_or_else(|_| "Main Server".into()))
}

fn user_home_owner(username: &str) -> VaultOwner {
    VaultOwner::Home {
        server_slug: current_server_slug(),
        home_slug: username.to_string(),
    }
}

fn server_owner() -> VaultOwner {
    VaultOwner::Server {
        server_slug: current_server_slug(),
    }
}

fn is_owned_by_user(author_id: &str, user: &User) -> bool {
    author_id == user.id || author_id == user.username
}

async fn resolve_vault_owner(
    storage: &AsyncStorageAdapter,
    vault_id: &str,
    username: &str,
    _is_admin: bool,
) -> Result<VaultOwner, AppError> {
    let owner = user_home_owner(username);
    if storage.load_vault(owner.clone(), vault_id.to_string()).await.is_ok() {
        return Ok(owner);
    }

    let owner = server_owner();
    if storage.load_vault(owner.clone(), vault_id.to_string()).await.is_ok() {
        return Ok(owner);
    }

    Err(AppError::NotFound)
}

fn sse_json(data: serde_json::Value) -> Response {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/event-stream"));
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"));

    (
        StatusCode::OK,
        headers,
        format!("event: snapshot\ndata: {}\n\n", data),
    )
        .into_response()
}

pub async fn subscribe(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(query): Query<LiveSubscribeRequest>,
) -> Result<Response, AppError> {
    let live_query = query.into_live_query();
    let key = normalize_query_key(&live_query)?;

    let payload = match live_query.collection {
        LiveCollection::VaultListPersonal => {
            let vaults = state.storage.list_vaults(user_home_owner(&user.username)).await?;
            serde_json::to_value(vaults).map_err(|error| AppError::Internal(error.to_string()))?
        }
        LiveCollection::VaultListShared => {
            let vaults = state.storage.list_vaults(server_owner()).await?;
            let visible: Vec<Vault> = vaults
                .into_iter()
                .filter(|vault| matches!(user.role, ServerRole::Admin) || vault.slug != "feedback")
                .collect();
            serde_json::to_value(visible).map_err(|error| AppError::Internal(error.to_string()))?
        }
        LiveCollection::NoteList => {
            let vault_id = live_query
                .vault_id
                .clone()
                .ok_or_else(|| AppError::UnprocessableEntity("vault_id is required".into()))?;
            let owner = resolve_vault_owner(&state.storage, &vault_id, &user.username, matches!(user.role, ServerRole::Admin)).await?;
            let shared_server_vault = matches!(owner, VaultOwner::Server { .. });
            let notes = state.storage.list_notes(owner, vault_id).await?;
            let visible: Vec<_> = if shared_server_vault {
                notes
            } else {
                notes
                    .into_iter()
                    .filter(|note| is_owned_by_user(&note.author_id, &user) || note.permission != NotePermission::Restricted)
                    .collect()
            };
            serde_json::to_value(visible).map_err(|error| AppError::Internal(error.to_string()))?
        }
        LiveCollection::Note => {
            let vault_id = live_query
                .vault_id
                .clone()
                .ok_or_else(|| AppError::UnprocessableEntity("vault_id is required".into()))?;
            let note_id = live_query
                .note_id
                .clone()
                .ok_or_else(|| AppError::UnprocessableEntity("note_id is required".into()))?;
            let owner = resolve_vault_owner(&state.storage, &vault_id, &user.username, matches!(user.role, ServerRole::Admin)).await?;
            let shared_server_vault = matches!(owner, VaultOwner::Server { .. });
            let note = state.storage.load_note(owner, vault_id, note_id).await?;

            if !shared_server_vault
                && !is_owned_by_user(&note.meta.author_id, &user)
                && note.meta.permission == NotePermission::Restricted
            {
                return Err(AppError::Forbidden);
            }

            serde_json::to_value(note).map_err(|error| AppError::Internal(error.to_string()))?
        }
    };

    Ok(sse_json(serde_json::json!({
        "scope": key.as_string(),
        "type": "snapshot",
        "version": 1,
        "data": payload,
    })))
}
