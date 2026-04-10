use axum::{
    extract::{Query, State},
    response::{sse::{Event, KeepAlive, Sse}, IntoResponse, Response},
};
use async_stream::stream;
use std::{convert::Infallible, time::Duration};
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

async fn build_payload(
    state: &AppState,
    user: &notes_core::User,
    live_query: &notes_core::LiveQuery,
) -> Result<serde_json::Value, AppError> {
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
                    .filter(|note| is_owned_by_user(&note.author_id, user) || note.permission != NotePermission::Restricted)
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
                && !is_owned_by_user(&note.meta.author_id, user)
                && note.meta.permission == NotePermission::Restricted
            {
                return Err(AppError::Forbidden);
            }

            serde_json::to_value(note).map_err(|error| AppError::Internal(error.to_string()))?
        }
    };

    Ok(payload)
}

pub async fn subscribe(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(query): Query<LiveSubscribeRequest>,
) -> Result<Response, AppError> {
    let live_query = query.into_live_query();
    let key = normalize_query_key(&live_query)?;

    let scope = key.as_string();
    let broker = state.live_broker.clone();
    let state_for_stream = state.clone();
    let user_for_stream = user.clone();
    let live_query_for_stream = live_query.clone();
    broker.attach(&scope);
    let mut receiver = broker
        .subscribe(&scope)
        .ok_or_else(|| AppError::Internal("failed to subscribe to live broker".into()))?;

    let stream = stream! {
        struct ListenerGuard {
            broker: crate::live::broker::LiveBroker,
            scope: String,
        }

        impl Drop for ListenerGuard {
            fn drop(&mut self) {
                self.broker.release(&self.scope);
            }
        }

        let _guard = ListenerGuard { broker: broker.clone(), scope: scope.clone() };
        match build_payload(&state_for_stream, &user_for_stream, &live_query_for_stream).await {
            Ok(payload) => {
                yield Ok::<Event, Infallible>(Event::default().event("snapshot").data(serde_json::json!({
                    "scope": scope,
                    "type": "snapshot",
                    "version": 1,
                    "data": payload,
                }).to_string()));
            }
            Err(error) => {
                yield Ok::<Event, Infallible>(Event::default().event("error").data(serde_json::json!({
                    "scope": scope,
                    "type": "error",
                    "message": format!("{error:?}"),
                }).to_string()));
                return;
            }
        }

        loop {
            match receiver.recv().await {
                Ok(current_version) => match build_payload(&state_for_stream, &user_for_stream, &live_query_for_stream).await {
                    Ok(payload) => {
                        yield Ok::<Event, Infallible>(Event::default().event("snapshot").data(serde_json::json!({
                            "scope": scope,
                            "type": "snapshot",
                            "version": current_version,
                            "data": payload,
                        }).to_string()));
                    }
                    Err(error) => {
                        yield Ok::<Event, Infallible>(Event::default().event("error").data(serde_json::json!({
                            "scope": scope,
                            "type": "error",
                            "message": format!("{error:?}"),
                        }).to_string()));
                    }
                },
                Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
            }
        }
    };

    Ok(Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)).text("keep-alive")).into_response())
}
