use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use notes_core::{NotePermission, Vault, VaultIconUpdate, VaultOwner, VaultUpdate};
use uuid::Uuid;

use crate::{
    auth_extractor::AuthenticatedUser,
    error::AppError,
    types::{CreateVaultRequest, PatchPermissionRequest, PatchVaultRequest},
    AppState,
};

const VALID_ICONS: &[&str] = &[
    "home", "book", "star", "briefcase", "code",
    "pen", "heart", "globe", "lock", "rocket",
    "lightbulb", "music", "camera", "folder", "compass",
    "flask", "graduation-cap", "chart", "leaf", "diamond",
];

/// List all vaults accessible to the user: personal vaults + all team vaults.
pub async fn list_vaults(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<Json<Vec<Vault>>, AppError> {
    let mut vaults = state
        .storage
        .list_vaults(VaultOwner::User(user.id.clone()))
        .await?;

    let teams = state.storage.list_teams_for_user(user.id).await?;
    for team in teams {
        let team_vaults = state
            .storage
            .list_vaults(VaultOwner::Team(team.id))
            .await?;
        vaults.extend(team_vaults);
    }

    Ok(Json(vaults))
}

pub async fn create_vault(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(body): Json<CreateVaultRequest>,
) -> Result<(StatusCode, Json<Vault>), AppError> {
    if let Some(slug) = &body.icon {
        if !VALID_ICONS.contains(&slug.as_str()) {
            return Err(AppError::UnprocessableEntity(format!("invalid icon slug: {slug}")));
        }
    }
    let vault = Vault {
        id: Uuid::new_v4().to_string(),
        slug: body.slug,
        name: body.name,
        owner: VaultOwner::User(user.id),
        permission: NotePermission::Restricted,
        icon: body.icon,
    };
    state.storage.create_vault(vault.clone()).await?;
    Ok((StatusCode::CREATED, Json(vault)))
}

pub async fn patch_vault(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(vault_id): Path<String>,
    Json(body): Json<PatchVaultRequest>,
) -> Result<Json<Vault>, AppError> {
    if body.name.is_none() && body.icon.is_none() {
        return Err(AppError::UnprocessableEntity("at least one field required".into()));
    }
    if let Some(Some(slug)) = &body.icon {
        if !VALID_ICONS.contains(&slug.as_str()) {
            return Err(AppError::UnprocessableEntity(format!("invalid icon slug: {slug}")));
        }
    }

    let vault = state.storage.load_vault(vault_id.clone()).await?;

    match &vault.owner {
        VaultOwner::User(uid) if uid == &user.id => {}
        VaultOwner::Team(team_id) => {
            let team = state.storage.load_team(team_id.clone()).await?;
            let role = team
                .members
                .iter()
                .find(|m| m.user_id == user.id)
                .map(|m| &m.role)
                .ok_or(AppError::Forbidden)?;
            use notes_core::TeamRole;
            match role {
                TeamRole::Owner | TeamRole::Admin => {}
                TeamRole::Member => return Err(AppError::Forbidden),
            }
        }
        _ => return Err(AppError::Forbidden),
    }

    let update = VaultUpdate {
        name: body.name,
        icon: body.icon.map(|opt| match opt {
            Some(slug) => VaultIconUpdate::Set(slug),
            None => VaultIconUpdate::Clear,
        }),
    };
    state.storage.update_vault(vault_id.clone(), update).await?;
    let updated = state.storage.load_vault(vault_id).await?;
    Ok(Json(updated))
}

pub async fn delete_vault(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(vault_id): Path<String>,
) -> Result<StatusCode, AppError> {
    let vault = state.storage.load_vault(vault_id.clone()).await?;

    match &vault.owner {
        VaultOwner::User(uid) if uid == &user.id => {}
        _ => return Err(AppError::Forbidden),
    }

    state.storage.delete_vault(vault_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Change a team vault's permission. Only the team owner or admin may do this.
pub async fn patch_vault_permission(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((team_id, vault_id)): Path<(String, String)>,
    Json(body): Json<PatchPermissionRequest>,
) -> Result<StatusCode, AppError> {
    let team = state.storage.load_team(team_id).await?;

    let role = team
        .members
        .iter()
        .find(|m| m.user_id == user.id)
        .map(|m| &m.role)
        .ok_or(AppError::Forbidden)?;

    use notes_core::TeamRole;
    match role {
        TeamRole::Owner | TeamRole::Admin => {}
        TeamRole::Member => return Err(AppError::Forbidden),
    }

    state
        .storage
        .update_vault_permission(vault_id, body.permission)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
