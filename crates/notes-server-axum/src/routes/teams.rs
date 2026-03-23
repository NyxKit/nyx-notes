use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use notes_core::{NotePermission, Team, TeamMember, TeamRole, Vault, VaultOwner};
use uuid::Uuid;

use crate::{
    auth_extractor::AuthenticatedUser,
    error::AppError,
    types::{AddMemberRequest, CreateTeamRequest, CreateVaultRequest, PatchMemberRequest},
    AppState,
};

pub async fn list_teams(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<Json<Vec<Team>>, AppError> {
    let teams = state.storage.list_teams_for_user(user.id).await?;
    Ok(Json(teams))
}

pub async fn create_team(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(body): Json<CreateTeamRequest>,
) -> Result<(StatusCode, Json<Team>), AppError> {
    let team = Team {
        id: Uuid::new_v4().to_string(),
        name: body.name,
        members: vec![TeamMember {
            user_id: user.id,
            role: TeamRole::Owner,
        }],
    };
    // save_team also creates the default home vault.
    state.storage.save_team(team.clone()).await?;
    Ok((StatusCode::CREATED, Json(team)))
}

pub async fn get_team(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(team_id): Path<String>,
) -> Result<Json<Team>, AppError> {
    let team = state.storage.load_team(team_id).await?;
    require_member(&team, &user.id)?;
    Ok(Json(team))
}

pub async fn delete_team(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(team_id): Path<String>,
) -> Result<StatusCode, AppError> {
    let team = state.storage.load_team(team_id.clone()).await?;
    require_role(&team, &user.id, TeamRole::Owner)?;
    state.storage.delete_team(team_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn add_member(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(team_id): Path<String>,
    Json(body): Json<AddMemberRequest>,
) -> Result<Json<Team>, AppError> {
    let mut team = state.storage.load_team(team_id).await?;
    require_role(&team, &user.id, TeamRole::Owner)?;

    if body.role == TeamRole::Owner {
        return Err(AppError::UnprocessableEntity(
            "cannot assign Owner role; transfer ownership separately".into(),
        ));
    }
    if team.members.iter().any(|m| m.user_id == body.user_id) {
        return Err(AppError::UnprocessableEntity("user is already a member".into()));
    }

    team.members.push(TeamMember {
        user_id: body.user_id,
        role: body.role,
    });
    state.storage.save_team(team.clone()).await?;
    Ok(Json(team))
}

pub async fn patch_member(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((team_id, target_user_id)): Path<(String, String)>,
    Json(body): Json<PatchMemberRequest>,
) -> Result<Json<Team>, AppError> {
    let mut team = state.storage.load_team(team_id).await?;
    require_role(&team, &user.id, TeamRole::Owner)?;

    if body.role == TeamRole::Owner {
        return Err(AppError::UnprocessableEntity(
            "cannot assign Owner role; transfer ownership separately".into(),
        ));
    }

    let member = team
        .members
        .iter_mut()
        .find(|m| m.user_id == target_user_id)
        .ok_or(AppError::NotFound)?;
    member.role = body.role;

    state.storage.save_team(team.clone()).await?;
    Ok(Json(team))
}

pub async fn remove_member(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((team_id, target_user_id)): Path<(String, String)>,
) -> Result<Json<Team>, AppError> {
    let mut team = state.storage.load_team(team_id).await?;
    require_role(&team, &user.id, TeamRole::Owner)?;

    let before = team.members.len();
    team.members.retain(|m| m.user_id != target_user_id);
    if team.members.len() == before {
        return Err(AppError::NotFound);
    }

    state.storage.save_team(team.clone()).await?;
    Ok(Json(team))
}

pub async fn create_team_vault(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(team_id): Path<String>,
    Json(body): Json<CreateVaultRequest>,
) -> Result<(StatusCode, Json<Vault>), AppError> {
    let team = state.storage.load_team(team_id.clone()).await?;
    require_role(&team, &user.id, TeamRole::Admin)?;

    let vault = Vault {
        id: Uuid::new_v4().to_string(),
        slug: body.slug,
        name: body.name,
        owner: VaultOwner::Team(team_id),
        permission: NotePermission::Restricted,
        icon: None,
    };
    state.storage.create_vault(vault.clone()).await?;
    Ok((StatusCode::CREATED, Json(vault)))
}

pub async fn delete_team_vault(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path((team_id, vault_id)): Path<(String, String)>,
) -> Result<StatusCode, AppError> {
    let team = state.storage.load_team(team_id).await?;
    require_role(&team, &user.id, TeamRole::Admin)?;
    state.storage.delete_vault(vault_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// --- Helpers ---

/// Returns `Forbidden` if the user is not a member of the team.
fn require_member(team: &Team, user_id: &str) -> Result<(), AppError> {
    if team.members.iter().any(|m| m.user_id == user_id) {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

/// Returns `Forbidden` unless the user has at least `min_role` in the team.
/// Role hierarchy: Owner > Admin > Member.
fn require_role(team: &Team, user_id: &str, min_role: TeamRole) -> Result<(), AppError> {
    let role = team
        .members
        .iter()
        .find(|m| m.user_id == user_id)
        .map(|m| &m.role)
        .ok_or(AppError::Forbidden)?;

    let sufficient = match min_role {
        TeamRole::Member => true,
        TeamRole::Admin => matches!(role, TeamRole::Owner | TeamRole::Admin),
        TeamRole::Owner => matches!(role, TeamRole::Owner),
    };

    if sufficient { Ok(()) } else { Err(AppError::Forbidden) }
}
