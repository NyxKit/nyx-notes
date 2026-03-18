use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
    pub quoted_text: String,
    pub resolved: bool,
    pub created_at: DateTime<Utc>,
    pub replies: Vec<CommentReply>,
}

/// Controls who (beyond the owner) can interact with a note or team vault.
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

/// Identifies the owner of a vault: either a user or a team.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "id")]
pub enum VaultOwner {
    User(String),
    Team(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vault {
    /// Stable identifier (UUID or slug).
    pub id: String,
    /// Filesystem/URL-safe name, unique per owner (e.g. "home", "work").
    pub slug: String,
    /// Display name.
    pub name: String,
    pub owner: VaultOwner,
    /// Only meaningful for team vaults.
    /// Personal vaults are always implicitly `Restricted` to the owner.
    pub permission: NotePermission,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub members: Vec<TeamMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMember {
    pub user_id: String,
    pub role: TeamRole,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TeamRole {
    /// Full control: manage members, vaults, and vault permissions.
    Owner,
    /// Can manage vaults and their permissions; cannot manage members.
    Admin,
    /// Access governed by vault-level permissions.
    Member,
}
