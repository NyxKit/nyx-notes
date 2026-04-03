use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut last_was_separator = false;

    for ch in value.chars() {
        let mapped = match ch {
            'a'..='z' | '0'..='9' => Some(ch),
            'A'..='Z' => Some(ch.to_ascii_lowercase()),
            _ => Some('-'),
        };

        match mapped {
            Some('-') => {
                if !slug.is_empty() && !last_was_separator {
                    slug.push('-');
                    last_was_separator = true;
                }
            }
            Some(valid) => {
                slug.push(valid);
                last_was_separator = false;
            }
            None => {}
        }
    }

    let trimmed = slug.trim_matches('-');
    if trimmed.is_empty() {
        "untitled".into()
    } else {
        trimmed.chars().take(64).collect()
    }
}

pub fn distill_markdown_description(content: &str) -> Option<String> {
    content
        .split("\n\n")
        .map(str::trim)
        .filter(|block| !block.is_empty())
        .find_map(|block| {
            let first_line = block.lines().find(|line| !line.trim().is_empty())?.trim();

            let is_numbered_list = first_line
                .split_once('.')
                .map(|(prefix, _)| {
                    !prefix.is_empty() && prefix.chars().all(|ch| ch.is_ascii_digit())
                })
                .unwrap_or(false);

            if first_line.starts_with('#')
                || first_line.starts_with("- ")
                || first_line.starts_with("* ")
                || first_line.starts_with("+ ")
                || first_line.starts_with('>')
                || first_line.starts_with("```")
                || is_numbered_list
            {
                return None;
            }

            let distilled = block.split_whitespace().collect::<Vec<_>>().join(" ");
            if distilled.is_empty() {
                None
            } else {
                Some(distilled)
            }
        })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CommentAttachment {
    Attached,
    Detached,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommentVisibility {
    Visible,
    HiddenLegacy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentAnchor {
    pub text: String,
    pub prefix: String,
    pub suffix: String,
    pub range_from: u32,
    pub range_to: u32,
    pub attachment: CommentAttachment,
    pub line_preview: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_matched_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentReply {
    pub id: String,
    pub author_id: String,
    pub author_name: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub note_id: String,
    pub author_id: String,
    pub author_name: String,
    pub body: String,
    pub anchor: CommentAnchor,
    pub resolved: bool,
    pub visibility: CommentVisibility,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub replies: Vec<CommentReply>,
}

/// Controls who (beyond the owner) can interact with a note or shared vault.
/// The owner always retains full access regardless of this value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotePermission {
    /// Private. Only the owner can view, edit, or comment.
    Restricted,
    /// Any authenticated user can view and add comments, but cannot edit.
    Comment,
    /// Any authenticated user can view, comment, and edit.
    Edit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteMeta {
    pub id: String,
    pub vault_id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub author_id: String,
    pub tags: Vec<String>,
    pub category: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Reserved for future E2EE support.
    pub is_encrypted: bool,
    pub permission: NotePermission,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub meta: NoteMeta,
    /// Raw body. In non-E2EE mode: plaintext Markdown.
    /// In E2EE mode: opaque ciphertext — the server never parses this.
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum VaultOwner {
    Home {
        server_slug: String,
        home_slug: String,
    },
    Server {
        server_slug: String,
    },
    Local,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServerRole {
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vault {
    /// Stable identifier (UUID or slug).
    pub id: String,
    /// Filesystem/URL-safe name, unique per owner (e.g. "home", "work").
    pub slug: String,
    /// Display name.
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub owner: VaultOwner,
    /// Default note permission for new notes created in this vault.
    pub permission: NotePermission,
    /// Optional decorative icon slug (e.g. "briefcase"). One of 20 curated slugs or None.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

/// Payload for `StorageBackend::update_vault`. All fields are optional; absent means no change.
#[derive(Debug, Clone)]
pub struct VaultUpdate {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub icon: Option<VaultIconUpdate>,
}

#[derive(Debug, Clone)]
pub enum VaultIconUpdate {
    Set(String),
    Clear,
}
