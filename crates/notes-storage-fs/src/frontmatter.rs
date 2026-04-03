use chrono::{DateTime, Utc};
use notes_core::{distill_markdown_description, Note, NoteMeta, NotePermission, StorageError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiskNoteMeta {
    id: String,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    author_id: String,
    tags: Vec<String>,
    category: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    is_encrypted: bool,
    permission: NotePermission,
}

/// Parse a full note file (frontmatter + body).
///
/// Expected format:
/// ```text
/// ---
/// <yaml fields>
/// ---
/// <markdown body>
/// ```
pub fn parse_note_file(input: &str, vault_id: &str) -> Result<(NoteMeta, String), StorageError> {
    let rest = input
        .strip_prefix("---\n")
        .ok_or_else(|| StorageError::ParseError("missing opening frontmatter delimiter".into()))?;

    let end = rest
        .find("\n---\n")
        .ok_or_else(|| StorageError::ParseError("missing closing frontmatter delimiter".into()))?;

    let yaml = &rest[..end];
    // Skip past "\n---\n" (5 bytes) to get the body; preserve trailing content as-is.
    let body = rest[end + 5..].to_string();

    let disk_meta: DiskNoteMeta =
        serde_yaml::from_str(yaml).map_err(|e| StorageError::ParseError(e.to_string()))?;

    let mut meta = NoteMeta {
        id: disk_meta.id,
        vault_id: vault_id.to_string(),
        title: disk_meta.title,
        description: disk_meta.description,
        author_id: disk_meta.author_id,
        tags: disk_meta.tags,
        category: disk_meta.category,
        created_at: disk_meta.created_at,
        updated_at: disk_meta.updated_at,
        is_encrypted: disk_meta.is_encrypted,
        permission: disk_meta.permission,
    };

    if meta.description.is_none() {
        meta.description = distill_markdown_description(&body);
    }

    Ok((meta, body))
}

/// Parse only the frontmatter, discarding the body.
/// Used by `list_notes` to avoid reading the full content of every note.
pub fn parse_frontmatter_only(input: &str, vault_id: &str) -> Result<NoteMeta, StorageError> {
    let (meta, _) = parse_note_file(input, vault_id)?;
    Ok(meta)
}

/// Serialize a `Note` into the on-disk format: YAML frontmatter + Markdown body.
pub fn serialize_note_file(note: &Note) -> Result<String, StorageError> {
    let meta = DiskNoteMeta {
        id: note.meta.id.clone(),
        title: note.meta.title.clone(),
        description: distill_markdown_description(&note.content),
        author_id: note.meta.author_id.clone(),
        tags: note.meta.tags.clone(),
        category: note.meta.category.clone(),
        created_at: note.meta.created_at,
        updated_at: note.meta.updated_at,
        is_encrypted: note.meta.is_encrypted,
        permission: note.meta.permission.clone(),
    };
    let yaml = serde_yaml::to_string(&meta).map_err(|e| StorageError::ParseError(e.to_string()))?;
    Ok(format!("---\n{}---\n{}", yaml, note.content))
}
