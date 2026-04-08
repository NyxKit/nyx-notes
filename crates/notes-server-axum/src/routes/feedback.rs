use std::{fs, io::Cursor, path::{Path, PathBuf}};

use axum::{
    extract::{Path as AxumPath, State},
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use base64::Engine;
use chrono::Utc;
use image::{codecs::jpeg::JpegEncoder, ColorType, ImageEncoder};
use notes_core::{slugify, Note, NoteMeta, NotePermission, ServerRole, Vault, VaultOwner};
use uuid::Uuid;

use crate::{
    auth_extractor::AuthenticatedUser,
    error::AppError,
    types::{CreateFeedbackRequest, UpdateFeedbackRequest},
    AppState,
};

const FEEDBACK_VAULT_SLUG: &str = "feedback";

fn current_server_slug() -> String {
    slugify(&std::env::var("SERVER_NAME").unwrap_or_else(|_| "Main Server".into()))
}

fn feedback_owner() -> VaultOwner {
    VaultOwner::Server {
        server_slug: current_server_slug(),
    }
}

fn feedback_vault() -> Vault {
    Vault {
        id: FEEDBACK_VAULT_SLUG.into(),
        slug: FEEDBACK_VAULT_SLUG.into(),
        name: "Feedback".into(),
        description: Some("Submitted feedback".into()),
        owner: feedback_owner(),
        permission: NotePermission::Restricted,
        icon: Some("message-circle".into()),
    }
}

fn feedback_vault_dir(root_path: &Path) -> PathBuf {
    root_path
        .join(current_server_slug())
        .join("vaults")
        .join(FEEDBACK_VAULT_SLUG)
}

fn feedback_images_dir(root_path: &Path, note_id: &str) -> PathBuf {
    feedback_vault_dir(root_path).join("images").join(note_id)
}

fn sanitize_filename(name: &str) -> String {
    let mut out = String::new();
    for ch in name.chars() {
        match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' => out.push(ch),
            _ => out.push('-'),
        }
    }
    let trimmed = out.trim_matches('-').trim_matches('.');
    if trimmed.is_empty() {
        "image".into()
    } else {
        trimmed.chars().take(80).collect()
    }
}

fn truncate_console_output(input: &str) -> String {
    const MAX_BYTES: usize = 200 * 1024;
    let mut redacted = input.replace("Bearer ", "Bearer [redacted] ");
    redacted = redacted.replace("token=", "token=[redacted]");
    redacted = redacted.replace("access_token=", "access_token=[redacted]");
    if redacted.len() > MAX_BYTES {
        let mut truncated = redacted.chars().take(MAX_BYTES).collect::<String>();
        truncated.push_str("\n[truncated]");
        truncated
    } else {
        redacted
    }
}

fn decode_data_url(data: &str) -> Result<Vec<u8>, AppError> {
    if let Some(rest) = data.strip_prefix("data:") {
        let (_, payload) = rest
            .split_once("base64,")
            .ok_or_else(|| AppError::UnprocessableEntity("invalid image data URL".into()))?;
        base64::engine::general_purpose::STANDARD
            .decode(payload)
            .map_err(|e| AppError::UnprocessableEntity(format!("invalid image payload: {e}")))
    } else {
        base64::engine::general_purpose::STANDARD
            .decode(data)
            .map_err(|e| AppError::UnprocessableEntity(format!("invalid image payload: {e}")))
    }
}

fn compress_image(bytes: &[u8]) -> Result<Vec<u8>, AppError> {
    let image = image::load_from_memory(bytes)
        .map_err(|e| AppError::UnprocessableEntity(format!("invalid image file: {e}")))?;
    let rgb = image.to_rgb8();
    let mut out = Vec::new();
    let mut cursor = Cursor::new(&mut out);
    JpegEncoder::new_with_quality(&mut cursor, 82)
        .write_image(rgb.as_raw(), rgb.width(), rgb.height(), ColorType::Rgb8.into())
        .map_err(|e| AppError::Internal(format!("failed to encode image: {e}")))?;
    Ok(out)
}

fn save_images(root_path: &Path, note_id: &str, images: &[crate::types::FeedbackImageUpload]) -> Result<Vec<String>, AppError> {
    let dir = feedback_images_dir(root_path, note_id);
    fs::create_dir_all(&dir).map_err(|e| AppError::Internal(e.to_string()))?;

    let mut stored = Vec::new();
    for (idx, image) in images.iter().enumerate() {
        let decoded = decode_data_url(&image.data)?;
        let compressed = compress_image(&decoded)?;
        let stem = sanitize_filename(&Path::new(&image.name).file_stem().and_then(|s| s.to_str()).unwrap_or("image"));
        let filename = format!("{stem}-{idx}.jpg");
        fs::write(dir.join(&filename), compressed).map_err(|e| AppError::Internal(e.to_string()))?;
        stored.push(filename);
    }

    Ok(stored)
}

async fn ensure_feedback_vault(state: &AppState) -> Result<(), AppError> {
    if state
        .storage
        .load_vault(feedback_owner(), FEEDBACK_VAULT_SLUG.to_string())
        .await
        .is_ok()
    {
        return Ok(());
    }

    state
        .storage
        .create_vault(feedback_vault())
        .await?;
    Ok(())
}

fn feedback_storage_path(note_id: &str) -> String {
    format!("feedback/{note_id}")
}

fn build_feedback_note(
    user_id: &str,
    body: CreateFeedbackRequest,
    note_id: String,
    now: chrono::DateTime<Utc>,
    images: Vec<String>,
) -> Note {
    Note {
        meta: NoteMeta {
            id: note_id.clone(),
            vault_id: FEEDBACK_VAULT_SLUG.into(),
            title: body.title,
            description: Some(body.description.clone()),
            author_id: user_id.to_string(),
            images,
            tags: vec!["feedback".into(), body.feedback_type.clone()],
            category: Some("feedback".into()),
            created_at: now,
            updated_at: now,
            is_encrypted: false,
            permission: NotePermission::Restricted,
            feedback_type: Some(body.feedback_type),
            app_location: Some(body.app_location),
            storage_path: Some(feedback_storage_path(&note_id)),
            console_output: Some(truncate_console_output(&body.console_output)),
            interaction_trail: body.interaction_trail,
        },
        content: body.description,
    }
}

async fn load_feedback_note(
    state: &AppState,
    id: String,
) -> Result<Note, AppError> {
    ensure_feedback_vault(state).await?;
    state
        .storage
        .load_note(feedback_owner(), FEEDBACK_VAULT_SLUG.to_string(), id)
        .await
        .map_err(Into::into)
}

pub async fn list_feedback(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<Json<Vec<NoteMeta>>, AppError> {
    if !matches!(user.role, ServerRole::Admin) {
        return Err(AppError::Forbidden);
    }

    ensure_feedback_vault(&state).await?;
    let notes = state
        .storage
        .list_notes(feedback_owner(), FEEDBACK_VAULT_SLUG.to_string())
        .await?;

    Ok(Json(notes))
}

pub async fn get_feedback(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    AxumPath(id): AxumPath<String>,
) -> Result<Json<Note>, AppError> {
    if !matches!(user.role, ServerRole::Admin) {
        return Err(AppError::Forbidden);
    }

    let note = load_feedback_note(&state, id).await?;
    Ok(Json(note))
}

pub async fn create_feedback(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(body): Json<CreateFeedbackRequest>,
) -> Result<(StatusCode, Json<NoteMeta>), AppError> {
    ensure_feedback_vault(&state).await?;
    let note_id = Uuid::new_v4().to_string();
    let now = Utc::now();
    let images = save_images(&state.root_path, &note_id, &body.images)?;
    let note = build_feedback_note(&user.id, body, note_id.clone(), now, images);
    let meta = note.meta.clone();
    state.storage.save_note(feedback_owner(), note).await?;
    Ok((StatusCode::CREATED, Json(meta)))
}

pub async fn update_feedback(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    AxumPath(id): AxumPath<String>,
    Json(body): Json<UpdateFeedbackRequest>,
) -> Result<Json<NoteMeta>, AppError> {
    if !matches!(user.role, ServerRole::Admin) {
        return Err(AppError::Forbidden);
    }

    ensure_feedback_vault(&state).await?;
    let mut note = load_feedback_note(&state, id.clone()).await?;
    note.meta.title = body.title;
    note.meta.description = Some(body.description.clone());
    note.meta.images = save_images(&state.root_path, &id, &body.images)?;
    note.meta.tags = vec!["feedback".into(), body.feedback_type.clone()];
    note.meta.category = Some("feedback".into());
    note.meta.feedback_type = Some(body.feedback_type);
    note.meta.app_location = Some(body.app_location);
    note.meta.storage_path = Some(feedback_storage_path(&id));
    note.meta.console_output = Some(truncate_console_output(&body.console_output));
    note.meta.interaction_trail = body.interaction_trail;
    note.meta.updated_at = Utc::now();
    note.content = body.description;

    let meta = note.meta.clone();
    state.storage.save_note(feedback_owner(), note).await?;
    Ok(Json(meta))
}

pub async fn delete_feedback(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    AxumPath(id): AxumPath<String>,
) -> Result<StatusCode, AppError> {
    if !matches!(user.role, ServerRole::Admin) {
        return Err(AppError::Forbidden);
    }

    ensure_feedback_vault(&state).await?;
    state
        .storage
        .delete_note(feedback_owner(), FEEDBACK_VAULT_SLUG.to_string(), id.clone())
        .await?;

    let _ = fs::remove_dir_all(feedback_images_dir(&state.root_path, &id));

    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_feedback_image(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    AxumPath((id, filename)): AxumPath<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    if !matches!(user.role, ServerRole::Admin) {
        return Err(AppError::Forbidden);
    }

    let note = load_feedback_note(&state, id.clone()).await?;
    if !note.meta.images.iter().any(|item| item == &filename) {
        return Err(AppError::NotFound);
    }

    let path = feedback_images_dir(&state.root_path, &id).join(&filename);
    let bytes = fs::read(&path).map_err(|_| AppError::NotFound)?;
    Ok((
        [(header::CONTENT_TYPE, "image/jpeg")],
        bytes,
    ))
}
