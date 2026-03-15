use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct CliConfig {
    pub notes_root: String,
    pub user_id: String,
    /// Default vault slug (or `team:<id>/<slug>` for team vaults).
    pub vault: String,
    pub editor: String,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            notes_root: default_notes_root(),
            user_id: "default".into(),
            vault: "home".into(),
            editor: "vi".into(),
        }
    }
}

impl CliConfig {
    /// Load config from `~/.config/nyx-notes/config.toml`, then overlay env vars.
    pub fn load() -> Self {
        let mut cfg = Self::from_file().unwrap_or_default();

        if let Ok(v) = std::env::var("NOTES_ROOT") {
            cfg.notes_root = v;
        }
        if let Ok(v) = std::env::var("NOTES_USER_ID") {
            cfg.user_id = v;
        }
        if let Ok(v) = std::env::var("NOTES_VAULT") {
            cfg.vault = v;
        }
        if let Ok(v) = std::env::var("EDITOR") {
            cfg.editor = v;
        }

        cfg
    }

    pub fn notes_root_path(&self) -> PathBuf {
        expand_tilde(&self.notes_root)
    }

    fn from_file() -> Option<Self> {
        let path = config_toml_path();
        let content = std::fs::read_to_string(path).ok()?;
        toml::from_str(&content).ok()
    }
}

fn default_notes_root() -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    format!("{home}/notes")
}

fn config_toml_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home)
        .join(".config")
        .join("nyx-notes")
        .join("config.toml")
}

fn expand_tilde(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/") {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        PathBuf::from(home).join(rest)
    } else {
        PathBuf::from(path)
    }
}
