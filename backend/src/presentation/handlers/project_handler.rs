use axum::{
    extract::{Path, State},
    Extension, Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::application::commands::{CreateProjectCommand, UpdateProjectCommand};
use crate::application::services::ProjectAppService;
use crate::domain::entities::{Milestone, Project, Task};
use crate::domain::value_objects::UserRole;
use crate::presentation::dto::ApiResponse;
use crate::presentation::middleware::AuthUser;
use crate::shared::DomainError;

pub async fn list_projects(
    State(service): State<Arc<ProjectAppService>>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<ApiResponse<Vec<Project>>>, DomainError> {
    // Admin can see all projects, others only see accessible ones
    let projects = if auth_user.role == UserRole::Admin {
        service.list_projects().await?
    } else {
        service.list_accessible_projects(auth_user.id).await?
    };
    Ok(Json(ApiResponse::success(projects)))
}

pub async fn get_project(
    State(service): State<Arc<ProjectAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Project>>, DomainError> {
    // Check access permission (admin can access all)
    if auth_user.role != UserRole::Admin {
        if !service.can_user_access(id, auth_user.id).await? {
            return Err(DomainError::Forbidden("You don't have access to this project".into()));
        }
    }
    let project = service.get_project(id).await?;
    Ok(Json(ApiResponse::success(project)))
}

pub async fn create_project(
    State(service): State<Arc<ProjectAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Json(cmd): Json<CreateProjectCommand>,
) -> Result<Json<ApiResponse<Project>>, DomainError> {
    tracing::info!(
        user_id = %auth_user.id,
        user_email = %auth_user.email,
        "User creating new project"
    );
    let project = service.create_project(cmd, auth_user.id).await?;
    Ok(Json(ApiResponse::success(project)))
}

pub async fn update_project(
    State(service): State<Arc<ProjectAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(cmd): Json<UpdateProjectCommand>,
) -> Result<Json<ApiResponse<Project>>, DomainError> {
    // Only owner or admin can update project
    if auth_user.role != UserRole::Admin {
        if !service.is_owner(id, auth_user.id).await? {
            return Err(DomainError::Forbidden("Only project owner can update this project".into()));
        }
    }

    tracing::info!(
        user_id = %auth_user.id,
        project_id = %id,
        "User updating project"
    );
    let project = service.update_project(id, cmd).await?;
    Ok(Json(ApiResponse::success(project)))
}

pub async fn delete_project(
    State(service): State<Arc<ProjectAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, DomainError> {
    // Only owner or admin can delete project
    if auth_user.role != UserRole::Admin {
        if !service.is_owner(id, auth_user.id).await? {
            return Err(DomainError::Forbidden("Only project owner can delete this project".into()));
        }
    }

    tracing::info!(
        user_id = %auth_user.id,
        project_id = %id,
        "User deleting project"
    );
    service.delete_project(id).await?;
    Ok(Json(ApiResponse::ok("Project deleted successfully")))
}

pub async fn get_project_tasks(
    State(service): State<Arc<ProjectAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<Task>>>, DomainError> {
    // Check access permission (admin can access all)
    if auth_user.role != UserRole::Admin {
        if !service.can_user_access(id, auth_user.id).await? {
            return Err(DomainError::Forbidden("You don't have access to this project".into()));
        }
    }
    let tasks = service.get_project_tasks(id).await?;
    Ok(Json(ApiResponse::success(tasks)))
}

pub async fn get_project_milestones(
    State(service): State<Arc<ProjectAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<Milestone>>>, DomainError> {
    // Check access permission (admin can access all)
    if auth_user.role != UserRole::Admin {
        if !service.can_user_access(id, auth_user.id).await? {
            return Err(DomainError::Forbidden("You don't have access to this project".into()));
        }
    }
    let milestones = service.get_project_milestones(id).await?;
    Ok(Json(ApiResponse::success(milestones)))
}
