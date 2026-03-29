mod frontmatter;
mod meta;

#[cfg(test)]
mod tests;

use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use notes_core::{
    Comment, CommentAnchor, CommentAttachment, CommentReply, CommentVisibility, Note, NoteMeta,
    NotePermission, StorageBackend, StorageError, Team, Vault, VaultIconUpdate, VaultOwner,
    VaultUpdate,
};
use serde::{Deserialize, Serialize};

use meta::{TeamJson, VaultJson};

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

    // --- Path helpers ---

    fn vault_path(&self, vault: &Vault) -> PathBuf {
        match &vault.owner {
            VaultOwner::User(uid) => self.root.join("users").join(uid).join(&vault.slug),
            VaultOwner::Team(team_id) => self.root.join("teams").join(team_id).join(&vault.slug),
        }
    }

    fn team_dir(&self, team_id: &str) -> PathBuf {
        self.root.join("teams").join(team_id)
    }

    /// Return the filesystem path of a note file.
    /// Intended for CLI use (opening the file in `$EDITOR`).
    pub fn note_file_path(&self, vault_id: &str, id: &str) -> Result<PathBuf, StorageError> {
        let vault_dir = self.find_vault_path(vault_id)?;
        Ok(vault_dir.join(format!("{id}.md")))
    }

    // --- Vault resolution ---

    /// Resolve a `vault_id` to its directory path by scanning `.vault.json` files.
    ///
    /// All vault directories (personal and team) carry a `.vault.json` with an `id` field,
    /// so this scan works uniformly across both ownership types.
    ///
    /// The scan is O(total vaults) and intended for installations with tens of vaults.
    fn find_vault_path(&self, vault_id: &str) -> Result<PathBuf, StorageError> {
        // Scan users/*/*/  .vault.json
        let users_root = self.root.join("users");
        if users_root.is_dir() {
            for user_entry in std::fs::read_dir(&users_root)? {
                let user_dir = user_entry?.path();
                if !user_dir.is_dir() {
                    continue;
                }
                if let Some(path) = self.scan_owner_dir_for_vault(&user_dir, vault_id)? {
                    return Ok(path);
                }
            }
        }

        // Scan teams/*/  */  .vault.json
        let teams_root = self.root.join("teams");
        if teams_root.is_dir() {
            for team_entry in std::fs::read_dir(&teams_root)? {
                let team_dir = team_entry?.path();
                if !team_dir.is_dir() {
                    continue;
                }
                if let Some(path) = self.scan_owner_dir_for_vault(&team_dir, vault_id)? {
                    return Ok(path);
                }
            }
        }

        Err(StorageError::NotFound)
    }

    /// Derive the `VaultOwner` from a vault directory path relative to `root`.
    /// Path layout: `<root>/users/<uid>/<slug>/` or `<root>/teams/<team_id>/<slug>/`.
    fn vault_owner_from_path(&self, vault_dir: &Path) -> Option<VaultOwner> {
        let rel = vault_dir.strip_prefix(&self.root).ok()?;
        let mut comps = rel.components();
        let owner_type = comps.next()?.as_os_str().to_str()?;
        let owner_id = comps.next()?.as_os_str().to_str()?.to_string();
        match owner_type {
            "users" => Some(VaultOwner::User(owner_id)),
            "teams" => Some(VaultOwner::Team(owner_id)),
            _ => None,
        }
    }

    fn scan_owner_dir_for_vault(
        &self,
        owner_dir: &Path,
        vault_id: &str,
    ) -> Result<Option<PathBuf>, StorageError> {
        for entry in std::fs::read_dir(owner_dir)? {
            let vault_dir = entry?.path();
            if !vault_dir.is_dir() {
                continue;
            }
            let vault_json_path = vault_dir.join(".vault.json");
            if !vault_json_path.is_file() {
                continue;
            }
            let content = std::fs::read_to_string(&vault_json_path)?;
            let meta: VaultJson = serde_json::from_str(&content)
                .map_err(|e| StorageError::ParseError(e.to_string()))?;
            if meta.id == vault_id {
                return Ok(Some(vault_dir));
            }
        }
        Ok(None)
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
            VaultOwner::User(uid) => self.root.join("users").join(uid),
            VaultOwner::Team(team_id) => self.root.join("teams").join(team_id),
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
                owner: owner.clone(),
                permission: meta.permission.unwrap_or(NotePermission::Restricted),
                icon: meta.icon,
            });
        }

        Ok(vaults)
    }

    fn load_vault(&self, vault_id: &str) -> Result<Vault, StorageError> {
        let vault_dir = self.find_vault_path(vault_id)?;
        let meta = Self::read_vault_json(&vault_dir)?;
        let owner = self.vault_owner_from_path(&vault_dir).ok_or_else(|| {
            StorageError::ParseError("cannot determine vault owner from path".into())
        })?;
        Ok(Vault {
            id: meta.id,
            slug: meta.slug,
            name: meta.name,
            description: meta.description,
            owner,
            permission: meta.permission.unwrap_or(NotePermission::Restricted),
            icon: meta.icon,
        })
    }

    fn create_vault(&self, vault: &Vault) -> Result<(), StorageError> {
        let vault_dir = self.vault_path(vault);
        std::fs::create_dir_all(&vault_dir)?;

        let meta = VaultJson {
            id: vault.id.clone(),
            name: vault.name.clone(),
            slug: vault.slug.clone(),
            description: vault.description.clone(),
            permission: match &vault.owner {
                VaultOwner::Team(_) => Some(vault.permission.clone()),
                VaultOwner::User(_) => None,
            },
            icon: vault.icon.clone(),
        };
        Self::write_vault_json(&vault_dir, &meta)?;

        Ok(())
    }

    fn delete_vault(&self, vault_id: &str) -> Result<(), StorageError> {
        let vault_dir = self.find_vault_path(vault_id)?;

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
        vault_id: &str,
        permission: NotePermission,
    ) -> Result<(), StorageError> {
        let vault_dir = self.find_vault_path(vault_id)?;
        let mut meta = Self::read_vault_json(&vault_dir)?;
        meta.permission = Some(permission);
        Self::write_vault_json(&vault_dir, &meta)?;
        Ok(())
    }

    fn update_vault(&self, vault_id: &str, update: &VaultUpdate) -> Result<(), StorageError> {
        let vault_dir = self.find_vault_path(vault_id)?;
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

    // --- Team management ---

    fn load_team(&self, team_id: &str) -> Result<Team, StorageError> {
        let path = self.team_dir(team_id).join(".team.json");
        if !path.is_file() {
            return Err(StorageError::NotFound);
        }
        let content = std::fs::read_to_string(&path)?;
        let meta: TeamJson =
            serde_json::from_str(&content).map_err(|e| StorageError::ParseError(e.to_string()))?;
        Ok(Team {
            id: meta.id,
            name: meta.name,
            members: meta.members,
        })
    }

    fn save_team(&self, team: &Team) -> Result<(), StorageError> {
        let team_dir = self.team_dir(&team.id);
        std::fs::create_dir_all(&team_dir)?;

        let meta = TeamJson {
            id: team.id.clone(),
            name: team.name.clone(),
            members: team.members.clone(),
        };
        let content = serde_json::to_string_pretty(&meta)
            .map_err(|e| StorageError::ParseError(e.to_string()))?;
        std::fs::write(team_dir.join(".team.json"), content)?;

        // Ensure the default `home` vault exists.
        let home_vault_dir = team_dir.join("home");
        if !home_vault_dir.is_dir() {
            let home_vault = Vault {
                id: format!("{}-home", team.id),
                slug: "home".to_string(),
                name: "Home".to_string(),
                description: None,
                owner: VaultOwner::Team(team.id.clone()),
                permission: NotePermission::Restricted,
                icon: None,
            };
            self.create_vault(&home_vault)?;
        }

        Ok(())
    }

    fn delete_team(&self, team_id: &str) -> Result<(), StorageError> {
        let team_dir = self.team_dir(team_id);
        if !team_dir.is_dir() {
            return Err(StorageError::NotFound);
        }
        std::fs::remove_dir_all(&team_dir)?;
        Ok(())
    }

    fn list_teams_for_user(&self, user_id: &str) -> Result<Vec<Team>, StorageError> {
        let teams_dir = self.root.join("teams");
        if !teams_dir.is_dir() {
            return Ok(Vec::new());
        }

        let mut teams = Vec::new();
        for entry in std::fs::read_dir(&teams_dir)? {
            let team_dir = entry?.path();
            if !team_dir.is_dir() {
                continue;
            }
            let path = team_dir.join(".team.json");
            if !path.is_file() {
                continue;
            }
            let content = std::fs::read_to_string(&path)?;
            let meta: TeamJson = serde_json::from_str(&content)
                .map_err(|e| StorageError::ParseError(e.to_string()))?;
            if meta.members.iter().any(|m| m.user_id == user_id) {
                teams.push(Team {
                    id: meta.id,
                    name: meta.name,
                    members: meta.members,
                });
            }
        }

        Ok(teams)
    }

    // --- Notes ---

    fn list_notes(&self, vault_id: &str) -> Result<Vec<NoteMeta>, StorageError> {
        let vault_dir = self.find_vault_path(vault_id)?;

        let mut metas: Vec<NoteMeta> = Vec::new();
        for entry in std::fs::read_dir(&vault_dir)? {
            let path = entry?.path();
            if path.extension().map_or(false, |ext| ext == "md") {
                let content = std::fs::read_to_string(&path)?;
                let meta = frontmatter::parse_frontmatter_only(&content)?;
                metas.push(meta);
            }
        }

        metas.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(metas)
    }

    fn load_note(&self, vault_id: &str, id: &str) -> Result<Note, StorageError> {
        let vault_dir = self.find_vault_path(vault_id)?;
        let note_path = vault_dir.join(format!("{id}.md"));

        if !note_path.is_file() {
            return Err(StorageError::NotFound);
        }

        let content = std::fs::read_to_string(&note_path)?;
        let (meta, body) = frontmatter::parse_note_file(&content)?;
        Ok(Note {
            meta,
            content: body,
        })
    }

    fn save_note(&self, note: &Note) -> Result<(), StorageError> {
        let vault_dir = self.find_vault_path(&note.meta.vault_id)?;
        let note_path = vault_dir.join(format!("{}.md", note.meta.id));
        let content = frontmatter::serialize_note_file(note)?;
        std::fs::write(&note_path, content)?;
        Ok(())
    }

    fn delete_note(&self, vault_id: &str, id: &str) -> Result<(), StorageError> {
        let vault_dir = self.find_vault_path(vault_id)?;
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

    fn load_comments(&self, vault_id: &str, note_id: &str) -> Result<Vec<Comment>, StorageError> {
        let vault_dir = self.find_vault_path(vault_id)?;
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
        vault_id: &str,
        note_id: &str,
        comments: &[Comment],
    ) -> Result<(), StorageError> {
        let vault_dir = self.find_vault_path(vault_id)?;
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
