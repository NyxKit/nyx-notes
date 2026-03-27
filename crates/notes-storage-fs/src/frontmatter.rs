use notes_core::{distill_markdown_description, Note, NoteMeta, StorageError};

/// Parse a full note file (frontmatter + body).
///
/// Expected format:
/// ```text
/// ---
/// <yaml fields>
/// ---
/// <markdown body>
/// ```
pub fn parse_note_file(input: &str) -> Result<(NoteMeta, String), StorageError> {
    let rest = input
        .strip_prefix("---\n")
        .ok_or_else(|| StorageError::ParseError("missing opening frontmatter delimiter".into()))?;

    let end = rest
        .find("\n---\n")
        .ok_or_else(|| StorageError::ParseError("missing closing frontmatter delimiter".into()))?;

    let yaml = &rest[..end];
    // Skip past "\n---\n" (5 bytes) to get the body; preserve trailing content as-is.
    let body = rest[end + 5..].to_string();

    let mut meta: NoteMeta =
        serde_yaml::from_str(yaml).map_err(|e| StorageError::ParseError(e.to_string()))?;

    if meta.description.is_none() {
        meta.description = distill_markdown_description(&body);
    }

    Ok((meta, body))
}

/// Parse only the frontmatter, discarding the body.
/// Used by `list_notes` to avoid reading the full content of every note.
pub fn parse_frontmatter_only(input: &str) -> Result<NoteMeta, StorageError> {
    let (meta, _) = parse_note_file(input)?;
    Ok(meta)
}

/// Serialize a `Note` into the on-disk format: YAML frontmatter + Markdown body.
pub fn serialize_note_file(note: &Note) -> Result<String, StorageError> {
    let mut meta = note.meta.clone();
    meta.description = distill_markdown_description(&note.content);
    let yaml = serde_yaml::to_string(&meta).map_err(|e| StorageError::ParseError(e.to_string()))?;
    Ok(format!("---\n{}---\n{}", yaml, note.content))
}
