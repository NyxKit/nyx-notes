use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveCollection {
    VaultListPersonal,
    VaultListShared,
    NoteList,
    Note,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveScopeKind {
    Collection,
    Document,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveSubscriptionStatus {
    Loading,
    Active,
    Reconnecting,
    Failed,
    Released,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveQuery {
    pub collection: LiveCollection,
    pub scope_kind: LiveScopeKind,
    pub server_slug: String,
    pub owner_context: Option<String>,
    pub user_context: Option<String>,
    pub vault_id: Option<String>,
    pub note_id: Option<String>,
    #[serde(default)]
    pub filters: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalQueryKey {
    pub resource_name: String,
    pub scope_kind: LiveScopeKind,
    #[serde(default)]
    pub normalized_identifiers: Vec<(String, String)>,
    #[serde(default)]
    pub normalized_filters: Vec<(String, String)>,
}

impl CanonicalQueryKey {
    pub fn as_string(&self) -> String {
        let mut parts = vec![
            self.resource_name.clone(),
            match self.scope_kind {
                LiveScopeKind::Collection => "collection".to_string(),
                LiveScopeKind::Document => "document".to_string(),
            },
        ];

        for (key, value) in &self.normalized_identifiers {
            parts.push(format!("{key}={value}"));
        }

        for (key, value) in &self.normalized_filters {
            parts.push(format!("filter:{key}={value}"));
        }

        parts.join("|")
    }
}
