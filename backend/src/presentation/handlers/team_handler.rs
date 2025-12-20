use axum::{
    extract::{Path, State},
    Extension, Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::application::commands::{AddTeamMemberCommand, CreateTeamCommand, UpdateTeamCommand};
use crate::application::services::TeamAppService;
use crate::domain::entities::{Team, TeamMember};
use crate::domain::value_objects::UserRole;
use crate::presentation::dto::ApiResponse;
use crate::presentation::middleware::AuthUser;
use crate::shared::DomainError;

pub async fn list_teams(
    State(service): State<Arc<TeamAppService>>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<ApiResponse<Vec<Team>>>, DomainError> {
    // Admin can see all teams, others only see accessible ones
    let teams = if auth_user.role == UserRole::Admin {
        service.list_teams().await?
    } else {
        service.list_accessible_teams(auth_user.id).await?
    };
    Ok(Json(ApiResponse::success(teams)))
}

pub async fn get_team(
    State(service): State<Arc<TeamAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Team>>, DomainError> {
    // Check access permission (admin can access all)
    if auth_user.role != UserRole::Admin {
        if !service.can_user_access(id, auth_user.id).await? {
            return Err(DomainError::Forbidden("You don't have access to this team".into()));
        }
    }
    let team = service.get_team(id).await?;
    Ok(Json(ApiResponse::success(team)))
}

pub async fn create_team(
    State(service): State<Arc<TeamAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Json(cmd): Json<CreateTeamCommand>,
) -> Result<Json<ApiResponse<Team>>, DomainError> {
    tracing::info!(
        user_id = %auth_user.id,
        "User creating new team"
    );
    let team = service.create_team(cmd).await?;
    Ok(Json(ApiResponse::success(team)))
}

pub async fn update_team(
    State(service): State<Arc<TeamAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(cmd): Json<UpdateTeamCommand>,
) -> Result<Json<ApiResponse<Team>>, DomainError> {
    // Only lead or admin can update team
    if auth_user.role != UserRole::Admin {
        if !service.is_lead(id, auth_user.id).await? {
            return Err(DomainError::Forbidden("Only team lead can update this team".into()));
        }
    }

    tracing::info!(
        user_id = %auth_user.id,
        team_id = %id,
        "User updating team"
    );
    let team = service.update_team(id, cmd).await?;
    Ok(Json(ApiResponse::success(team)))
}

pub async fn delete_team(
    State(service): State<Arc<TeamAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, DomainError> {
    // Only lead or admin can delete team
    if auth_user.role != UserRole::Admin {
        if !service.is_lead(id, auth_user.id).await? {
            return Err(DomainError::Forbidden("Only team lead can delete this team".into()));
        }
    }

    tracing::info!(
        user_id = %auth_user.id,
        team_id = %id,
        "User deleting team"
    );
    service.delete_team(id).await?;
    Ok(Json(ApiResponse::ok("Team deleted successfully")))
}

pub async fn get_team_members(
    State(service): State<Arc<TeamAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<TeamMember>>>, DomainError> {
    // Check access permission (admin can access all)
    if auth_user.role != UserRole::Admin {
        if !service.can_user_access(id, auth_user.id).await? {
            return Err(DomainError::Forbidden("You don't have access to this team".into()));
        }
    }
    let members = service.get_team_members(id).await?;
    Ok(Json(ApiResponse::success(members)))
}

pub async fn add_team_member(
    State(service): State<Arc<TeamAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Path(team_id): Path<Uuid>,
    Json(cmd): Json<AddTeamMemberCommand>,
) -> Result<Json<ApiResponse<TeamMember>>, DomainError> {
    // Only lead or admin can add team members
    if auth_user.role != UserRole::Admin {
        if !service.is_lead(team_id, auth_user.id).await? {
            return Err(DomainError::Forbidden("Only team lead can add members".into()));
        }
    }

    tracing::info!(
        user_id = %auth_user.id,
        team_id = %team_id,
        new_member_id = %cmd.user_id,
        "User adding team member"
    );
    let member = service.add_team_member(team_id, cmd).await?;
    Ok(Json(ApiResponse::success(member)))
}
