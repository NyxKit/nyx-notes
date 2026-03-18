use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use notes_core::{Comment, CommentReply, NotePermission};
use uuid::Uuid;

use crate::{
    auth_extractor::AuthenticatedUser,
    error::AppError,
    types::{CreateCommentRequest, CreateReplyRequest, PatchCommentRequest},
    AppState,
};

// Helper: assert the caller can read the note (not restricted, or is author).
async fn assert_can_read(
    state: &AppState,
    vault_id: &str,
    note_id: &str,
    user_id: &str,
) -> Result<notes_core::Note, AppError> {
    let note = state
        .storage
        .load_note(vault_id.to_string(), note_id.to_string())
        .await?;
    if note.meta.author_id != user_id && note.meta.permission == NotePermission::Restricted {
        return Err(AppError::Forbidden);
    }
    Ok(note)
}

pub async fn list_comments(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((vault_id, note_id)): Path<(String, String)>,
) -> Result<Json<Vec<Comment>>, AppError> {
    assert_can_read(&state, &vault_id, &note_id, &user.id).await?;
    let comments = state
        .storage
        .load_comments(vault_id, note_id)
        .await?;
    Ok(Json(comments))
}

pub async fn create_comment(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((vault_id, note_id)): Path<(String, String)>,
    Json(body): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<Comment>), AppError> {
    let note = assert_can_read(&state, &vault_id, &note_id, &user.id).await?;

    // Must have comment or edit permission (or be the author) to post.
    if note.meta.author_id != user.id && note.meta.permission == NotePermission::Restricted {
        return Err(AppError::Forbidden);
    }

    let mut comments = state
        .storage
        .load_comments(vault_id.clone(), note_id.clone())
        .await?;

    let comment = Comment {
        id: Uuid::new_v4().to_string(),
        note_id: note_id.clone(),
        author_id: user.id.clone(),
        author_name: user.display_name.clone(),
        body: body.body,
        quoted_text: body.quoted_text,
        resolved: false,
        created_at: Utc::now(),
        replies: Vec::new(),
    };

    comments.push(comment.clone());
    state
        .storage
        .save_comments(vault_id, note_id, comments)
        .await?;

    Ok((StatusCode::CREATED, Json(comment)))
}

pub async fn delete_comment(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((vault_id, note_id, comment_id)): Path<(String, String, String)>,
) -> Result<StatusCode, AppError> {
    let note = assert_can_read(&state, &vault_id, &note_id, &user.id).await?;

    let mut comments = state
        .storage
        .load_comments(vault_id.clone(), note_id.clone())
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
        .save_comments(vault_id, note_id, comments)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn patch_comment(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((vault_id, note_id, comment_id)): Path<(String, String, String)>,
    Json(body): Json<PatchCommentRequest>,
) -> Result<Json<Comment>, AppError> {
    let note = assert_can_read(&state, &vault_id, &note_id, &user.id).await?;

    // Only the note author may resolve/unresolve.
    if note.meta.author_id != user.id {
        return Err(AppError::Forbidden);
    }

    let mut comments = state
        .storage
        .load_comments(vault_id.clone(), note_id.clone())
        .await?;

    let comment = comments
        .iter_mut()
        .find(|c| c.id == comment_id)
        .ok_or(AppError::NotFound)?;

    comment.resolved = body.resolved;
    let updated = comment.clone();

    state
        .storage
        .save_comments(vault_id, note_id, comments)
        .await?;

    Ok(Json(updated))
}

pub async fn create_reply(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((vault_id, note_id, comment_id)): Path<(String, String, String)>,
    Json(body): Json<CreateReplyRequest>,
) -> Result<(StatusCode, Json<CommentReply>), AppError> {
    let note = assert_can_read(&state, &vault_id, &note_id, &user.id).await?;

    if note.meta.author_id != user.id && note.meta.permission == NotePermission::Restricted {
        return Err(AppError::Forbidden);
    }

    let mut comments = state
        .storage
        .load_comments(vault_id.clone(), note_id.clone())
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

    state
        .storage
        .save_comments(vault_id, note_id, comments)
        .await?;

    Ok((StatusCode::CREATED, Json(reply)))
}

pub async fn delete_reply(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((vault_id, note_id, comment_id, reply_id)): Path<(String, String, String, String)>,
) -> Result<StatusCode, AppError> {
    let note = assert_can_read(&state, &vault_id, &note_id, &user.id).await?;

    let mut comments = state
        .storage
        .load_comments(vault_id.clone(), note_id.clone())
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

    state
        .storage
        .save_comments(vault_id, note_id, comments)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
