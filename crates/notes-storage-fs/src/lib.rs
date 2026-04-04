mod frontmatter;
mod meta;

#[cfg(test)]
mod tests;

use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use notes_core::{
    slugify, Comment, CommentAnchor, CommentAttachment, CommentReply, CommentVisibility, Note,
    NoteMeta, NotePermission, StorageBackend, StorageError, Vault, VaultIconUpdate, VaultOwner,
    VaultUpdate,
};
use serde::{Deserialize, Serialize};

use meta::{HomeJson, LocalJson, ServerJson, VaultJson};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SidecarCommentAnchor {
    text: String,
    prefix: String,
    suffix: String,
    range_from: u32,
    range_to: u32,
    attachment: CommentAttachment,
    line_preview: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_matched_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SidecarComment {
    id: String,
    note_id: String,
    author_id: String,
    author_name: String,
    body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    anchor: Option<SidecarCommentAnchor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quoted_text: Option<String>,
    resolved: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<CommentVisibility>,
    created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<DateTime<Utc>>,
    replies: Vec<CommentReply>,
}

impl SidecarComment {
    fn into_domain(self) -> Comment {
        let visibility = self.visibility.unwrap_or_else(|| {
            if self.anchor.is_some() {
                CommentVisibility::Visible
            } else {
                CommentVisibility::HiddenLegacy
            }
        });

        let anchor = match self.anchor {
            Some(anchor) => CommentAnchor {
                text: anchor.text,
                prefix: anchor.prefix,
                suffix: anchor.suffix,
                range_from: anchor.range_from,
                range_to: anchor.range_to,
                attachment: anchor.attachment,
                line_preview: anchor.line_preview,
                last_matched_at: anchor.last_matched_at,
            },
            None => {
                let quoted_text = self.quoted_text.unwrap_or_default();
                CommentAnchor {
                    text: quoted_text.clone(),
                    prefix: String::new(),
                    suffix: String::new(),
                    range_from: 0,
                    range_to: 0,
                    attachment: CommentAttachment::Detached,
                    line_preview: quoted_text,
                    last_matched_at: None,
                }
            }
        };

        Comment {
            id: self.id,
            note_id: self.note_id,
            author_id: self.author_id,
            author_name: self.author_name,
            body: self.body,
            anchor,
            resolved: self.resolved,
            visibility,
            created_at: self.created_at,
            updated_at: self.updated_at.unwrap_or(self.created_at),
            replies: self.replies,
        }
    }
}

impl From<&Comment> for SidecarComment {
    fn from(comment: &Comment) -> Self {
        Self {
            id: comment.id.clone(),
            note_id: comment.note_id.clone(),
            author_id: comment.author_id.clone(),
            author_name: comment.author_name.clone(),
            body: comment.body.clone(),
            anchor: Some(SidecarCommentAnchor {
                text: comment.anchor.text.clone(),
                prefix: comment.anchor.prefix.clone(),
                suffix: comment.anchor.suffix.clone(),
                range_from: comment.anchor.range_from,
                range_to: comment.anchor.range_to,
                attachment: comment.anchor.attachment.clone(),
                line_preview: comment.anchor.line_preview.clone(),
                last_matched_at: comment.anchor.last_matched_at,
            }),
            quoted_text: None,
            resolved: comment.resolved,
            visibility: Some(comment.visibility.clone()),
            created_at: comment.created_at,
            updated_at: Some(comment.updated_at),
            replies: comment.replies.clone(),
        }
    }
}

pub struct FsStorage {
    root: PathBuf,
}

impl FsStorage {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn active_server_slug(&self) -> String {
        let name = std::env::var("SERVER_NAME").unwrap_or_else(|_| "Main Server".into());
        slugify(&name)
    }

    fn active_server_name(&self) -> String {
        std::env::var("SERVER_NAME").unwrap_or_else(|_| "Main Server".into())
    }

    // --- Path helpers ---

    fn server_dir(&self) -> PathBuf {
        self.root.join(self.active_server_slug())
    }

    fn homes_dir(&self) -> PathBuf {
        self.server_dir().join("homes")
    }

    fn home_dir(&self, home_slug: &str) -> PathBuf {
        self.homes_dir().join(home_slug)
    }

    fn server_vaults_dir(&self) -> PathBuf {
        self.server_dir().join("vaults")
    }

    fn local_dir(&self) -> PathBuf {
        self.root.join("local")
    }

    fn ensure_server_metadata(&self) -> Result<(), StorageError> {
        let server_dir = self.server_dir();
        std::fs::create_dir_all(&server_dir)?;
        let path = server_dir.join(".server.json");
        if !path.exists() {
            let meta = ServerJson {
                id: format!("server-{}", self.active_server_slug()),
                slug: self.active_server_slug(),
                name: self.active_server_name(),
                roles: vec!["admin".into(), "user".into()],
            };
            let content = serde_json::to_string_pretty(&meta)
                .map_err(|e| StorageError::ParseError(e.to_string()))?;
            std::fs::write(path, content)?;
        }
        Ok(())
    }

    fn ensure_home_metadata(&self, home_slug: &str) -> Result<(), StorageError> {
        self.ensure_server_metadata()?;
        let home_dir = self.home_dir(home_slug);
        std::fs::create_dir_all(&home_dir)?;
        let path = home_dir.join(".home.json");
        if !path.exists() {
            let meta = HomeJson {
                id: format!("home-{home_slug}"),
                slug: home_slug.into(),
                name: format!("{}'s Home", home_slug),
                owner_user_id: home_slug.into(),
            };
            let content = serde_json::to_string_pretty(&meta)
                .map_err(|e| StorageError::ParseError(e.to_string()))?;
            std::fs::write(path, content)?;
        }
        Ok(())
    }

    fn ensure_local_metadata(&self) -> Result<(), StorageError> {
        let local_dir = self.local_dir();
        std::fs::create_dir_all(&local_dir)?;
        let path = local_dir.join(".local.json");
        if !path.exists() {
            let meta = LocalJson {
                id: "local".into(),
                name: "Local Storage".into(),
            };
            let content = serde_json::to_string_pretty(&meta)
                .map_err(|e| StorageError::ParseError(e.to_string()))?;
            std::fs::write(path, content)?;
        }
        Ok(())
    }

    fn vault_path(&self, vault: &Vault) -> PathBuf {
        match &vault.owner {
            VaultOwner::Home { home_slug, .. } => self.home_dir(home_slug).join(&vault.slug),
            VaultOwner::Server { .. } => self.server_vaults_dir().join(&vault.slug),
            VaultOwner::Local => self.local_dir().join(&vault.slug),
        }
    }

    fn vault_dir_from_owner(
        &self,
        owner: &VaultOwner,
        vault_id: &str,
    ) -> Result<PathBuf, StorageError> {
        let base_dir = match owner {
            VaultOwner::Home { home_slug, .. } => self.home_dir(home_slug),
            VaultOwner::Server { .. } => self.server_vaults_dir(),
            VaultOwner::Local => self.local_dir(),
        };

        // First try exact match on vault slug
        let slug_path = base_dir.join(vault_id);
        if slug_path.is_dir() && slug_path.join(".vault.json").is_file() {
            return Ok(slug_path);
        }

        // If not found by slug, scan for matching ID (fallback for stable IDs)
        if base_dir.is_dir() {
            for entry in std::fs::read_dir(&base_dir)? {
                let vault_dir = entry?.path();
                if !vault_dir.is_dir() {
                    continue;
                }
                let vault_json_path = vault_dir.join(".vault.json");
                if !vault_json_path.is_file() {
                    continue;
                }
                if let Ok(content) = std::fs::read_to_string(&vault_json_path) {
                    if let Ok(meta) = serde_json::from_str::<VaultJson>(&content) {
                        if meta.id == vault_id {
                            return Ok(vault_dir);
                        }
                    }
                }
            }
        }

        Err(StorageError::NotFound)
    }

    /// Return the filesystem path of a note file.
    /// Intended for CLI use (opening the file in `$EDITOR`).
    pub fn note_file_path(
        &self,
        owner: &VaultOwner,
        vault_id: &str,
        id: &str,
    ) -> Result<PathBuf, StorageError> {
        let vault_dir = self.vault_dir_from_owner(owner, vault_id)?;
        Ok(vault_dir.join(format!("{id}.md")))
    }

    // --- JSON helpers ---

    fn read_vault_json(vault_dir: &Path) -> Result<VaultJson, StorageError> {
        let content = std::fs::read_to_string(vault_dir.join(".vault.json"))?;
        serde_json::from_str(&content).map_err(|e| StorageError::ParseError(e.to_string()))
    }

    fn write_vault_json(vault_dir: &Path, meta: &VaultJson) -> Result<(), StorageError> {
        let content = serde_json::to_string_pretty(meta)
            .map_err(|e| StorageError::ParseError(e.to_string()))?;
        std::fs::write(vault_dir.join(".vault.json"), content)?;
        Ok(())
    }
}

impl StorageBackend for FsStorage {
    // --- Vault management ---

    fn list_vaults(&self, owner: &VaultOwner) -> Result<Vec<Vault>, StorageError> {
        let owner_dir = match owner {
            VaultOwner::Home { home_slug, .. } => self.home_dir(home_slug),
            VaultOwner::Server { .. } => self.server_vaults_dir(),
            VaultOwner::Local => self.local_dir(),
        };

        if !owner_dir.is_dir() {
            return Ok(Vec::new());
        }

        let mut vaults = Vec::new();
        for entry in std::fs::read_dir(&owner_dir)? {
            let vault_dir = entry?.path();
            if !vault_dir.is_dir() {
                continue;
            }
            let vault_json_path = vault_dir.join(".vault.json");
            if !vault_json_path.is_file() {
                continue;
            }
            let meta = Self::read_vault_json(&vault_dir)?;
            vaults.push(Vault {
                id: meta.id,
                slug: meta.slug,
                name: meta.name,
                description: meta.description,
                owner: meta.owner,
                permission: meta.permission.unwrap_or(NotePermission::Restricted),
                icon: meta.icon,
            });
        }

        Ok(vaults)
    }

    fn load_vault(&self, owner: &VaultOwner, vault_id: &str) -> Result<Vault, StorageError> {
        let vault_dir = self.vault_dir_from_owner(owner, vault_id)?;
        let meta = Self::read_vault_json(&vault_dir)?;
        Ok(Vault {
            id: meta.id,
            slug: meta.slug,
            name: meta.name,
            description: meta.description,
            owner: owner.clone(),
            permission: meta.permission.unwrap_or(NotePermission::Restricted),
            icon: meta.icon,
        })
    }

    fn create_vault(&self, vault: &Vault) -> Result<(), StorageError> {
        match &vault.owner {
            VaultOwner::Home { home_slug, .. } => self.ensure_home_metadata(home_slug)?,
            VaultOwner::Server { .. } => self.ensure_server_metadata()?,
            VaultOwner::Local => self.ensure_local_metadata()?,
        }

        let vault_dir = self.vault_path(vault);
        std::fs::create_dir_all(&vault_dir)?;

        let meta = VaultJson {
            id: vault.id.clone(),
            name: vault.name.clone(),
            slug: vault.slug.clone(),
            kind: match &vault.owner {
                VaultOwner::Home { .. } => "personal",
                VaultOwner::Server { .. } => "server",
                VaultOwner::Local => "local",
            }
            .into(),
            owner: vault.owner.clone(),
            description: vault.description.clone(),
            permission: Some(vault.permission.clone()),
            icon: vault.icon.clone(),
        };
        Self::write_vault_json(&vault_dir, &meta)?;

        Ok(())
    }

    fn delete_vault(&self, owner: &VaultOwner, vault_id: &str) -> Result<(), StorageError> {
        let vault_dir = self.vault_dir_from_owner(owner, vault_id)?;

        let has_notes = std::fs::read_dir(&vault_dir)?
            .filter_map(|e| e.ok())
            .any(|e| e.path().extension().map_or(false, |ext| ext == "md"));

        if has_notes {
            return Err(StorageError::VaultNotEmpty);
        }

        std::fs::remove_dir_all(&vault_dir)?;
        Ok(())
    }

    fn update_vault_permission(
        &self,
        owner: &VaultOwner,
        vault_id: &str,
        permission: NotePermission,
    ) -> Result<(), StorageError> {
        let vault_dir = self.vault_dir_from_owner(owner, vault_id)?;
        let mut meta = Self::read_vault_json(&vault_dir)?;
        meta.permission = Some(permission);
        Self::write_vault_json(&vault_dir, &meta)?;
        Ok(())
    }

    fn update_vault(
        &self,
        owner: &VaultOwner,
        vault_id: &str,
        update: &VaultUpdate,
    ) -> Result<(), StorageError> {
        let vault_dir = self.vault_dir_from_owner(owner, vault_id)?;
        let mut meta = Self::read_vault_json(&vault_dir)?;
        if let Some(name) = &update.name {
            meta.name = name.clone();
        }
        if let Some(description) = &update.description {
            meta.description = description.clone();
        }
        if let Some(icon_update) = &update.icon {
            match icon_update {
                VaultIconUpdate::Set(slug) => meta.icon = Some(slug.clone()),
                VaultIconUpdate::Clear => meta.icon = None,
            }
        }
        Self::write_vault_json(&vault_dir, &meta)?;
        Ok(())
    }

    // --- Notes ---

    fn list_notes(
        &self,
        owner: &VaultOwner,
        vault_id: &str,
    ) -> Result<Vec<NoteMeta>, StorageError> {
        let vault_dir = self.vault_dir_from_owner(owner, vault_id)?;

        let mut metas: Vec<NoteMeta> = Vec::new();
        for entry in std::fs::read_dir(&vault_dir)? {
            let path = entry?.path();
            if path.extension().map_or(false, |ext| ext == "md") {
                let content = std::fs::read_to_string(&path)?;
                let meta = frontmatter::parse_frontmatter_only(&content, vault_id)?;
                metas.push(meta);
            }
        }

        metas.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(metas)
    }

    fn load_note(
        &self,
        owner: &VaultOwner,
        vault_id: &str,
        id: &str,
    ) -> Result<Note, StorageError> {
        let vault_dir = self.vault_dir_from_owner(owner, vault_id)?;
        let note_path = vault_dir.join(format!("{id}.md"));

        if !note_path.is_file() {
            return Err(StorageError::NotFound);
        }

        let content = std::fs::read_to_string(&note_path)?;
        let (meta, body) = frontmatter::parse_note_file(&content, vault_id)?;
        Ok(Note {
            meta,
            content: body,
        })
    }

    fn save_note(&self, owner: &VaultOwner, note: &Note) -> Result<(), StorageError> {
        let vault_dir = self.vault_dir_from_owner(owner, &note.meta.vault_id)?;
        let note_path = vault_dir.join(format!("{}.md", note.meta.id));
        let content = frontmatter::serialize_note_file(note)?;
        std::fs::write(&note_path, content)?;
        Ok(())
    }

    fn rename_note(
        &self,
        owner: &VaultOwner,
        vault_id: &str,
        old_id: &str,
        new_id: &str,
    ) -> Result<(), StorageError> {
        let vault_dir = self.vault_dir_from_owner(owner, vault_id)?;
        let old_path = vault_dir.join(format!("{old_id}.md"));
        let new_path = vault_dir.join(format!("{new_id}.md"));
        std::fs::rename(&old_path, &new_path)?;
        Ok(())
    }

    fn delete_note(
        &self,
        owner: &VaultOwner,
        vault_id: &str,
        id: &str,
    ) -> Result<(), StorageError> {
        let vault_dir = self.vault_dir_from_owner(owner, vault_id)?;
        let note_path = vault_dir.join(format!("{id}.md"));

        if !note_path.is_file() {
            return Err(StorageError::NotFound);
        }

        std::fs::remove_file(&note_path)?;

        // Best-effort: remove the sidecar if it exists.
        let comments_path = vault_dir.join(format!("{id}.comments.json"));
        if comments_path.is_file() {
            let _ = std::fs::remove_file(&comments_path);
        }

        Ok(())
    }

    // --- Comments ---

    fn load_comments(
        &self,
        owner: &VaultOwner,
        vault_id: &str,
        note_id: &str,
    ) -> Result<Vec<Comment>, StorageError> {
        let vault_dir = self.vault_dir_from_owner(owner, vault_id)?;
        let path = vault_dir.join(format!("{note_id}.comments.json"));

        if !path.is_file() {
            return Ok(Vec::new());
        }

        let content = std::fs::read_to_string(&path)?;
        let sidecar: Vec<SidecarComment> =
            serde_json::from_str(&content).map_err(|e| StorageError::ParseError(e.to_string()))?;
        Ok(sidecar
            .into_iter()
            .map(SidecarComment::into_domain)
            .collect())
    }

    fn save_comments(
        &self,
        owner: &VaultOwner,
        vault_id: &str,
        note_id: &str,
        comments: &[Comment],
    ) -> Result<(), StorageError> {
        let vault_dir = self.vault_dir_from_owner(owner, vault_id)?;
        let path = vault_dir.join(format!("{note_id}.comments.json"));

        if comments.is_empty() {
            if path.is_file() {
                std::fs::remove_file(&path)?;
            }
            return Ok(());
        }

        let serializable = comments
            .iter()
            .map(SidecarComment::from)
            .collect::<Vec<_>>();
        let content = serde_json::to_string_pretty(&serializable)
            .map_err(|e| StorageError::ParseError(e.to_string()))?;
        std::fs::write(&path, content)?;
        Ok(())
    }
}
