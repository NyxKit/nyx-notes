use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use notes_core::{
    Comment, CommentAnchor, CommentAttachment, CommentReply, CommentVisibility, NotePermission,
    ServerRole, User, VaultOwner, slugify,
};
use uuid::Uuid;

use crate::{
    auth_extractor::AuthenticatedUser,
    error::AppError,
    storage_adapter::AsyncStorageAdapter,
    types::{CreateCommentRequest, CreateReplyRequest, PatchCommentRequest},
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

async fn resolve_vault_owner(
    storage: &AsyncStorageAdapter,
    vault_id: &str,
    username: &str,
    is_admin: bool,
) -> Result<VaultOwner, AppError> {
    let owner = user_home_owner(username);
    if storage.load_vault(owner.clone(), vault_id.to_string()).await.is_ok() {
        return Ok(owner);
    }

    if is_admin {
        let owner = server_owner();
        if storage.load_vault(owner.clone(), vault_id.to_string()).await.is_ok() {
            return Ok(owner);
        }
    }

    Err(AppError::NotFound)
}

// Helper: assert the caller can read the note (not restricted, or is author).
async fn assert_can_read(
    state: &AppState,
    user: &User,
    vault_id: &str,
    note_id: &str,
) -> Result<(notes_core::Note, VaultOwner), AppError> {
    let owner = resolve_vault_owner(&state.storage, vault_id, &user.username, matches!(user.role, ServerRole::Admin)).await?;
    let note = state
        .storage
        .load_note(owner.clone(), vault_id.to_string(), note_id.to_string())
        .await?;
    if note.meta.author_id != user.id && note.meta.permission == NotePermission::Restricted {
        return Err(AppError::Forbidden);
    }
    Ok((note, owner))
}

pub async fn list_comments(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((vault_id, note_id)): Path<(String, String)>,
) -> Result<Json<Vec<Comment>>, AppError> {
    let (_, owner) = assert_can_read(&state, &user, &vault_id, &note_id).await?;
    let comments = state
        .storage
        .load_comments(owner, vault_id, note_id)
        .await?;
    Ok(Json(
        comments
            .into_iter()
            .filter(|comment| !matches!(comment.visibility, CommentVisibility::HiddenLegacy))
            .collect(),
    ))
}

pub async fn create_comment(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((vault_id, note_id)): Path<(String, String)>,
    Json(body): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<Comment>), AppError> {
    let (note, owner) = assert_can_read(&state, &user, &vault_id, &note_id).await?;

    // Must have comment or edit permission (or be the author) to post.
    if note.meta.author_id != user.id && note.meta.permission == NotePermission::Restricted {
        return Err(AppError::Forbidden);
    }

    let mut comments = state
        .storage
        .load_comments(owner.clone(), vault_id.clone(), note_id.clone())
        .await?;

    let now = Utc::now();
    let comment = Comment {
        id: Uuid::new_v4().to_string(),
        note_id: note_id.clone(),
        author_id: user.id.clone(),
        author_name: user.display_name.clone(),
        body: body.body,
        anchor: CommentAnchor {
            text: body.anchor.text,
            prefix: body.anchor.prefix,
            suffix: body.anchor.suffix,
            range_from: body.anchor.range_from,
            range_to: body.anchor.range_to,
            attachment: CommentAttachment::Attached,
            line_preview: body.anchor.line_preview,
            last_matched_at: Some(now),
        },
        resolved: false,
        visibility: CommentVisibility::Visible,
        created_at: now,
        updated_at: now,
        replies: Vec::new(),
    };

    comments.push(comment.clone());
    state
        .storage
        .save_comments(owner, vault_id, note_id, comments)
        .await?;

    Ok((StatusCode::CREATED, Json(comment)))
}

pub async fn delete_comment(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((vault_id, note_id, comment_id)): Path<(String, String, String)>,
) -> Result<StatusCode, AppError> {
    let (note, owner) = assert_can_read(&state, &user, &vault_id, &note_id).await?;

    let mut comments = state
        .storage
        .load_comments(owner.clone(), vault_id.clone(), note_id.clone())
        .await?;

    let pos = comments
        .iter()
        .position(|c| c.id == comment_id)
        .ok_or(AppError::NotFound)?;

    // Comment author or note author may delete.
    if comments[pos].author_id != user.id && note.meta.author_id != user.id {
        return Err(AppError::Forbidden);
    }

    comments.remove(pos);
    state
        .storage
        .save_comments(owner, vault_id, note_id, comments)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn patch_comment(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((vault_id, note_id, comment_id)): Path<(String, String, String)>,
    Json(body): Json<PatchCommentRequest>,
) -> Result<Json<Comment>, AppError> {
    let (note, owner) = assert_can_read(&state, &user, &vault_id, &note_id).await?;

    // Only the note author may resolve/unresolve.
    if note.meta.author_id != user.id {
        return Err(AppError::Forbidden);
    }

    let mut comments = state
        .storage
        .load_comments(owner.clone(), vault_id.clone(), note_id.clone())
        .await?;

    let comment = comments
        .iter_mut()
        .find(|c| c.id == comment_id)
        .ok_or(AppError::NotFound)?;

    comment.resolved = body.resolved;
    comment.updated_at = Utc::now();
    let updated = comment.clone();

    state
        .storage
        .save_comments(owner, vault_id, note_id, comments)
        .await?;

    Ok(Json(updated))
}

pub async fn create_reply(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((vault_id, note_id, comment_id)): Path<(String, String, String)>,
    Json(body): Json<CreateReplyRequest>,
) -> Result<(StatusCode, Json<CommentReply>), AppError> {
    let (note, owner) = assert_can_read(&state, &user, &vault_id, &note_id).await?;

    if note.meta.author_id != user.id && note.meta.permission == NotePermission::Restricted {
        return Err(AppError::Forbidden);
    }

    let mut comments = state
        .storage
        .load_comments(owner.clone(), vault_id.clone(), note_id.clone())
        .await?;

    let comment = comments
        .iter_mut()
        .find(|c| c.id == comment_id)
        .ok_or(AppError::NotFound)?;

    let reply = CommentReply {
        id: Uuid::new_v4().to_string(),
        author_id: user.id.clone(),
        author_name: user.display_name.clone(),
        body: body.body,
        created_at: Utc::now(),
    };

    comment.replies.push(reply.clone());
    comment.updated_at = Utc::now();

    state
        .storage
        .save_comments(owner, vault_id, note_id, comments)
        .await?;

    Ok((StatusCode::CREATED, Json(reply)))
}

pub async fn delete_reply(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((vault_id, note_id, comment_id, reply_id)): Path<(String, String, String, String)>,
) -> Result<StatusCode, AppError> {
    let (note, owner) = assert_can_read(&state, &user, &vault_id, &note_id).await?;

    let mut comments = state
        .storage
        .load_comments(owner.clone(), vault_id.clone(), note_id.clone())
        .await?;

    let comment = comments
        .iter_mut()
        .find(|c| c.id == comment_id)
        .ok_or(AppError::NotFound)?;

    let pos = comment
        .replies
        .iter()
        .position(|r| r.id == reply_id)
        .ok_or(AppError::NotFound)?;

    // Reply author or note author may delete.
    if comment.replies[pos].author_id != user.id && note.meta.author_id != user.id {
        return Err(AppError::Forbidden);
    }

    comment.replies.remove(pos);
    comment.updated_at = Utc::now();

    state
        .storage
        .save_comments(owner, vault_id, note_id, comments)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
