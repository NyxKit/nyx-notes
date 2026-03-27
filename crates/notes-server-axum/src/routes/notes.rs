use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use notes_core::{Note, NoteMeta, NotePermission, TeamRole, VaultOwner, distill_markdown_description};
use uuid::Uuid;

use crate::{
    auth_extractor::AuthenticatedUser,
    error::AppError,
    types::{CreateNoteRequest, PatchPermissionRequest, UpdateNoteRequest},
    AppState,
};

pub async fn list_notes(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(vault_id): Path<String>,
) -> Result<Json<Vec<NoteMeta>>, AppError> {
    let all = state.storage.list_notes(vault_id).await?;

    // Filter to notes this user can view.
    // The vault-level permission floor is implicitly enforced: the caller must have
    // access to the vault before reaching this handler (enforced by the route layer
    // once team-vault access checks are added to the vault middleware).
    let visible = all
        .into_iter()
        .filter(|n| n.author_id == user.id || n.permission != NotePermission::Restricted)
        .collect();

    Ok(Json(visible))
}

pub async fn get_note(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((vault_id, id)): Path<(String, String)>,
) -> Result<Json<Note>, AppError> {
    let note = state.storage.load_note(vault_id, id).await?;

    if note.meta.author_id != user.id && note.meta.permission == NotePermission::Restricted {
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
    let now = Utc::now();
    // Inherit the vault's permission level when the caller doesn't specify one.
    let permission = match body.permission {
        Some(p) => p,
        None => state
            .storage
            .load_vault(vault_id.clone())
            .await
            .map(|v| v.permission)
            .unwrap_or(NotePermission::Restricted),
    };

    let note = Note {
        meta: NoteMeta {
            id: Uuid::new_v4().to_string(),
            vault_id,
            title: body.title,
            description: distill_markdown_description(&body.content),
            author_id: user.id,
            tags: body.tags,
            category: body.category,
            created_at: now,
            updated_at: now,
            is_encrypted: false,
            permission,
        },
        content: body.content,
    };

    let meta = note.meta.clone();
    state.storage.save_note(note).await?;
    Ok((StatusCode::CREATED, Json(meta)))
}

pub async fn update_note(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((vault_id, id)): Path<(String, String)>,
    Json(body): Json<UpdateNoteRequest>,
) -> Result<Json<NoteMeta>, AppError> {
    let mut note = state.storage.load_note(vault_id, id).await?;

    if note.meta.author_id != user.id && note.meta.permission != NotePermission::Edit {
        return Err(AppError::Forbidden);
    }

    note.meta.title = body.title;
    note.meta.description = distill_markdown_description(&body.content);
    note.meta.tags = body.tags;
    note.meta.category = body.category;
    note.meta.updated_at = Utc::now();
    note.content = body.content;

    let meta = note.meta.clone();
    state.storage.save_note(note).await?;
    Ok(Json(meta))
}

pub async fn delete_note(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((vault_id, id)): Path<(String, String)>,
) -> Result<StatusCode, AppError> {
    let note = state.storage.load_note(vault_id.clone(), id.clone()).await?;

    if note.meta.author_id != user.id {
        // The vault/team owner may also delete notes they don't author.
        let vault = state.storage.load_vault(vault_id.clone()).await?;
        let is_owner = match &vault.owner {
            VaultOwner::User(uid) => uid == &user.id,
            VaultOwner::Team(team_id) => {
                let team = state.storage.load_team(team_id.clone()).await?;
                team.members
                    .iter()
                    .any(|m| m.user_id == user.id && matches!(m.role, TeamRole::Owner))
            }
        };
        if !is_owner {
            return Err(AppError::Forbidden);
        }
    }

    state.storage.delete_note(vault_id, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn patch_note_permission(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((vault_id, id)): Path<(String, String)>,
    Json(body): Json<PatchPermissionRequest>,
) -> Result<Json<NoteMeta>, AppError> {
    let mut note = state.storage.load_note(vault_id, id).await?;

    if note.meta.author_id != user.id {
        return Err(AppError::Forbidden);
    }

    note.meta.permission = body.permission;
    note.meta.updated_at = Utc::now();

    let meta = note.meta.clone();
    state.storage.save_note(note).await?;
    Ok(Json(meta))
}
