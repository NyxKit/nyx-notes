use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use notes_core::{NotePermission, ServerRole, Vault, VaultIconUpdate, VaultOwner, VaultUpdate, slugify};
use uuid::Uuid;

use crate::{
    auth_extractor::AuthenticatedUser,
    error::AppError,
    storage_adapter::AsyncStorageAdapter,
    types::{CreateVaultRequest, PatchVaultRequest},
    AppState,
};

const VALID_ICONS: &[&str] = &[
    "home", "book", "star", "briefcase", "code",
    "pen", "heart", "globe", "lock", "rocket",
    "lightbulb", "music", "camera", "folder", "compass",
    "flask", "graduation-cap", "chart", "leaf", "diamond",
];

fn current_server_slug() -> String {
    slugify(&std::env::var("SERVER_NAME").unwrap_or_else(|_| "Main Server".into()))
}

fn user_home_owner(username: &str) -> VaultOwner {
    VaultOwner::Home {
        server_slug: current_server_slug(),
        home_slug: username.to_string(),
    }
}

fn server_owner() -> VaultOwner {
    VaultOwner::Server {
        server_slug: current_server_slug(),
    }
}

/// Try to load a vault, trying user's home first, then server vault.
async fn load_vault_with_fallback(
    storage: &AsyncStorageAdapter,
    vault_id: &str,
    username: &str,
    is_admin: bool,
) -> Result<Vault, AppError> {
    // Try user's personal vault first
    let owner = user_home_owner(username);
    if let Ok(vault) = storage.load_vault(owner.clone(), vault_id.to_string()).await {
        return Ok(vault);
    }

    // Try server vault (only if admin)
    if is_admin {
        let owner = server_owner();
        if let Ok(vault) = storage.load_vault(owner, vault_id.to_string()).await {
            return Ok(vault);
        }
    }

    Err(AppError::NotFound)
}

/// List all vaults accessible to the user: personal vaults + shared server vaults.
pub async fn list_vaults(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<Json<Vec<Vault>>, AppError> {
    let mut vaults = state
        .storage
        .list_vaults(VaultOwner::Home {
            server_slug: current_server_slug(),
            home_slug: user.username.clone(),
        })
        .await?;

    let shared_vaults = state
        .storage
        .list_vaults(VaultOwner::Server {
            server_slug: current_server_slug(),
        })
        .await?;
    vaults.extend(shared_vaults);

    Ok(Json(vaults))
}

pub async fn list_personal_vaults(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<Json<Vec<Vault>>, AppError> {
    let vaults = state
        .storage
        .list_vaults(VaultOwner::Home {
            server_slug: current_server_slug(),
            home_slug: user.username.clone(),
        })
        .await?;

    Ok(Json(vaults))
}

pub async fn list_server_vaults(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
) -> Result<Json<Vec<Vault>>, AppError> {
    let vaults = state
        .storage
        .list_vaults(VaultOwner::Server {
            server_slug: current_server_slug(),
        })
        .await?;

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
        description: body.description,
        owner: VaultOwner::Home {
            server_slug: current_server_slug(),
            home_slug: user.username.clone(),
        },
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
    if body.name.is_none() && body.description.is_none() && body.icon.is_none() {
        return Err(AppError::UnprocessableEntity("at least one field required".into()));
    }
    if let Some(Some(slug)) = &body.icon {
        if !VALID_ICONS.contains(&slug.as_str()) {
            return Err(AppError::UnprocessableEntity(format!("invalid icon slug: {slug}")));
        }
    }

    let vault = load_vault_with_fallback(&state.storage, &vault_id, &user.username, matches!(user.role, ServerRole::Admin)).await?;

    let owner = match &vault.owner {
        VaultOwner::Home { home_slug, .. } if home_slug == &user.username => user_home_owner(&user.username),
        VaultOwner::Server { .. } if matches!(user.role, ServerRole::Admin) => server_owner(),
        _ => return Err(AppError::Forbidden),
    };

    let update = VaultUpdate {
        name: body.name,
        description: body.description,
        icon: body.icon.map(|opt| match opt {
            Some(slug) => VaultIconUpdate::Set(slug),
            None => VaultIconUpdate::Clear,
        }),
    };
    state.storage.update_vault(owner.clone(), vault_id.clone(), update).await?;
    let updated = state.storage.load_vault(owner, vault_id).await?;
    Ok(Json(updated))
}

pub async fn delete_vault(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(vault_id): Path<String>,
) -> Result<StatusCode, AppError> {
    let vault = state.storage.load_vault(user_home_owner(&user.username), vault_id.clone()).await?;

    match &vault.owner {
        VaultOwner::Home { home_slug, .. } if home_slug == &user.username => {}
        _ => return Err(AppError::Forbidden),
    }

    state.storage.delete_vault(user_home_owner(&user.username), vault_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn create_server_vault(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(body): Json<CreateVaultRequest>,
) -> Result<(StatusCode, Json<Vault>), AppError> {
    if !matches!(user.role, ServerRole::Admin) {
        return Err(AppError::Forbidden);
    }

    let vault = Vault {
        id: Uuid::new_v4().to_string(),
        slug: body.slug,
        name: body.name,
        description: body.description,
        owner: VaultOwner::Server {
            server_slug: current_server_slug(),
        },
        permission: NotePermission::Edit,
        icon: body.icon,
    };

    state.storage.create_vault(vault.clone()).await?;
    Ok((StatusCode::CREATED, Json(vault)))
}

pub async fn delete_server_vault(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(vault_id): Path<String>,
) -> Result<StatusCode, AppError> {
    if !matches!(user.role, ServerRole::Admin) {
        return Err(AppError::Forbidden);
    }

    let owner = server_owner();
    let vault = state.storage.load_vault(owner.clone(), vault_id.clone()).await?;
    if !matches!(vault.owner, VaultOwner::Server { .. }) {
        return Err(AppError::Forbidden);
    }

    state.storage.delete_vault(owner, vault_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
