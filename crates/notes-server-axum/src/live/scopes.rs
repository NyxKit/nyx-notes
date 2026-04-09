use notes_core::{CanonicalQueryKey, LiveCollection, LiveQuery};

use crate::error::AppError;

pub fn normalize_query_key(query: &LiveQuery) -> Result<CanonicalQueryKey, AppError> {
    if query.server_slug.trim().is_empty() {
        return Err(AppError::UnprocessableEntity(
            "server_slug is required".into(),
        ));
    }

    match query.collection {
        LiveCollection::VaultListPersonal | LiveCollection::VaultListShared => {}
        LiveCollection::NoteList if query.vault_id.as_deref().unwrap_or_default().is_empty() => {
            return Err(AppError::UnprocessableEntity("vault_id is required".into()));
        }
        LiveCollection::Note
            if query.vault_id.as_deref().unwrap_or_default().is_empty()
                || query.note_id.as_deref().unwrap_or_default().is_empty() =>
        {
            return Err(AppError::UnprocessableEntity(
                "vault_id and note_id are required".into(),
            ));
        }
        _ => {}
    }

    let mut normalized_identifiers = vec![("server_slug".into(), query.server_slug.clone())];

    if let Some(owner_context) = &query.owner_context {
        normalized_identifiers.push(("owner_context".into(), owner_context.clone()));
    }
    if let Some(user_context) = &query.user_context {
        normalized_identifiers.push(("user_context".into(), user_context.clone()));
    }
    if let Some(vault_id) = &query.vault_id {
        normalized_identifiers.push(("vault_id".into(), vault_id.clone()));
    }
    if let Some(note_id) = &query.note_id {
        normalized_identifiers.push(("note_id".into(), note_id.clone()));
    }

    normalized_identifiers.sort();

    let mut normalized_filters = query.filters.clone();
    normalized_filters.sort();

    Ok(CanonicalQueryKey {
        resource_name: format!("{:?}", query.collection).to_lowercase(),
        scope_kind: query.scope_kind.clone(),
        normalized_identifiers,
        normalized_filters,
    })
}
