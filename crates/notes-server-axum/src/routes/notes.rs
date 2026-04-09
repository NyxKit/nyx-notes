use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use notes_core::{
    Note, NoteMeta, NotePermission, ServerRole, User, VaultOwner, distill_markdown_description, slugify,
};
use uuid::Uuid;

use crate::{
    auth_extractor::AuthenticatedUser,
    error::AppError,
    storage_adapter::AsyncStorageAdapter,
    types::{CreateNoteRequest, PatchPermissionRequest, UpdateNoteRequest},
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

/// Resolve the vault owner for a given vault_id, trying user's home first,
/// then server vaults. FsStorage's vault_dir_from_owner scans all home
/// directories as fallback, so migrated vaults are found even when the
/// owner's home_slug doesn't match.
async fn resolve_vault_owner(
    storage: &AsyncStorageAdapter,
    vault_id: &str,
    username: &str,
    _is_admin: bool,
) -> Result<VaultOwner, AppError> {
    // Try user's home — FsStorage will scan all homes as fallback
    let owner = user_home_owner(username);
    if storage.load_vault(owner.clone(), vault_id.to_string()).await.is_ok() {
        return Ok(owner);
    }

    // Server vaults are shared; any authenticated user may resolve them.
    let owner = server_owner();
    if storage.load_vault(owner.clone(), vault_id.to_string()).await.is_ok() {
        return Ok(owner);
    }

    Err(AppError::NotFound)
}

pub async fn list_notes(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(vault_id): Path<String>,
) -> Result<Json<Vec<NoteMeta>>, AppError> {
    let owner = resolve_vault_owner(&state.storage, &vault_id, &user.username, matches!(user.role, ServerRole::Admin)).await?;
    let shared_server_vault = matches!(owner, VaultOwner::Server { .. });
    let all = state.storage.list_notes(owner, vault_id).await?;

    let visible = if shared_server_vault {
        all
    } else {
        all
            .into_iter()
            .filter(|n| is_owned_by_user(&n.author_id, &user) || n.permission != NotePermission::Restricted)
            .collect()
    };

    Ok(Json(visible))
}

pub async fn get_note(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((vault_id, id)): Path<(String, String)>,
) -> Result<Json<Note>, AppError> {
    let owner = resolve_vault_owner(&state.storage, &vault_id, &user.username, matches!(user.role, ServerRole::Admin)).await?;
    let shared_server_vault = matches!(owner, VaultOwner::Server { .. });
    let note = state.storage.load_note(owner, vault_id, id).await?;

    if !shared_server_vault
        && !is_owned_by_user(&note.meta.author_id, &user)
        && note.meta.permission == NotePermission::Restricted
    {
        return Err(AppError::Forbidden);
    }

    Ok(Json(note))
}

pub async fn create_note(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(vault_id): Path<String>,
    Json(body): Json<CreateNoteRequest>,
) -> Result<(StatusCode, Json<NoteMeta>), AppError> {
    let owner = resolve_vault_owner(&state.storage, &vault_id, &user.username, matches!(user.role, ServerRole::Admin)).await?;
    let now = Utc::now();
    let permission = match body.permission {
        Some(p) => p,
        None => state
            .storage
            .load_vault(owner.clone(), vault_id.clone())
            .await
            .map(|v| v.permission)
            .unwrap_or(NotePermission::Restricted),
    };

    let note = Note {
        meta: NoteMeta {
            id: Uuid::new_v4().to_string(),
            vault_id: vault_id.clone(),
            title: body.title,
            description: distill_markdown_description(&body.content),
            author_id: user.id.clone(),
            images: body.images,
            tags: body.tags,
            category: body.category,
            created_at: now,
            updated_at: now,
            is_encrypted: false,
            permission,
            feedback_type: None,
            app_location: None,
            storage_path: None,
            console_output: None,
            interaction_trail: None,
        },
        content: body.content,
    };

    let meta = note.meta.clone();
    state.storage.save_note(owner, note).await?;
    Ok((StatusCode::CREATED, Json(meta)))
}

pub async fn update_note(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((vault_id, id)): Path<(String, String)>,
    Json(body): Json<UpdateNoteRequest>,
) -> Result<Json<NoteMeta>, AppError> {
    let owner = resolve_vault_owner(&state.storage, &vault_id, &user.username, matches!(user.role, ServerRole::Admin)).await?;
    let mut note = state.storage.load_note(owner.clone(), vault_id.clone(), id).await?;

    if !is_owned_by_user(&note.meta.author_id, &user) && note.meta.permission != NotePermission::Edit {
        return Err(AppError::Forbidden);
    }

    note.meta.title = body.title;
    note.meta.description = distill_markdown_description(&body.content);
    note.meta.images = body.images;
    note.meta.tags = body.tags;
    note.meta.category = body.category;
    note.meta.updated_at = Utc::now();
    note.content = body.content;

    let meta = note.meta.clone();
    state.storage.save_note(owner, note).await?;
    Ok(Json(meta))
}

pub async fn delete_note(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((vault_id, id)): Path<(String, String)>,
) -> Result<StatusCode, AppError> {
    let owner = resolve_vault_owner(&state.storage, &vault_id, &user.username, matches!(user.role, ServerRole::Admin)).await?;
    let note = state.storage.load_note(owner.clone(), vault_id.clone(), id.clone()).await?;

    if !is_owned_by_user(&note.meta.author_id, &user) {
        let vault = state.storage.load_vault(owner.clone(), vault_id.clone()).await?;
        let is_owner = match &vault.owner {
            VaultOwner::Home { home_slug, .. } => home_slug == &user.username,
            VaultOwner::Server { .. } => matches!(user.role, ServerRole::Admin),
            VaultOwner::Local => false,
        };
        if !is_owner {
            return Err(AppError::Forbidden);
        }
    }

    state.storage.delete_note(owner, vault_id, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn patch_note_permission(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((vault_id, id)): Path<(String, String)>,
    Json(body): Json<PatchPermissionRequest>,
) -> Result<Json<NoteMeta>, AppError> {
    let owner = resolve_vault_owner(&state.storage, &vault_id, &user.username, matches!(user.role, ServerRole::Admin)).await?;
    let mut note = state.storage.load_note(owner.clone(), vault_id, id).await?;

    if !is_owned_by_user(&note.meta.author_id, &user) {
        return Err(AppError::Forbidden);
    }

    note.meta.permission = body.permission;
    note.meta.updated_at = Utc::now();

    let meta = note.meta.clone();
    state.storage.save_note(owner, note).await?;
    Ok(Json(meta))
}
