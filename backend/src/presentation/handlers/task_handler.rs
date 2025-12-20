use axum::{
    extract::{Path, State},
    Extension, Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::application::commands::{CreateTaskCommand, UpdateTaskCommand};
use crate::application::services::TaskAppService;
use crate::domain::entities::Task;
use crate::domain::value_objects::UserRole;
use crate::presentation::dto::ApiResponse;
use crate::presentation::middleware::AuthUser;
use crate::shared::DomainError;

pub async fn list_tasks(
    State(service): State<Arc<TaskAppService>>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<ApiResponse<Vec<Task>>>, DomainError> {
    // Admin can see all tasks, others only see tasks from accessible projects
    let tasks = if auth_user.role == UserRole::Admin {
        service.list_tasks().await?
    } else {
        service.list_accessible_tasks(auth_user.id).await?
    };
    Ok(Json(ApiResponse::success(tasks)))
}

pub async fn get_task(
    State(service): State<Arc<TaskAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Task>>, DomainError> {
    // Check access permission (admin can access all)
    if auth_user.role != UserRole::Admin {
        if !service.can_user_access(id, auth_user.id).await? {
            return Err(DomainError::Forbidden("You don't have access to this task".into()));
        }
    }
    let task = service.get_task(id).await?;
    Ok(Json(ApiResponse::success(task)))
}

pub async fn create_task(
    State(service): State<Arc<TaskAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Json(cmd): Json<CreateTaskCommand>,
) -> Result<Json<ApiResponse<Task>>, DomainError> {
    // Check access to project (admin can access all)
    if auth_user.role != UserRole::Admin {
        if !service.can_access_project(cmd.project_id, auth_user.id).await? {
            return Err(DomainError::Forbidden("You don't have access to this project".into()));
        }
    }

    tracing::info!(
        user_id = %auth_user.id,
        project_id = %cmd.project_id,
        "User creating new task"
    );
    let task = service.create_task(cmd).await?;
    Ok(Json(ApiResponse::success(task)))
}

pub async fn update_task(
    State(service): State<Arc<TaskAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(cmd): Json<UpdateTaskCommand>,
) -> Result<Json<ApiResponse<Task>>, DomainError> {
    // Check access permission (admin can access all)
    if auth_user.role != UserRole::Admin {
        if !service.can_user_access(id, auth_user.id).await? {
            return Err(DomainError::Forbidden("You don't have access to this task".into()));
        }
    }

    tracing::info!(
        user_id = %auth_user.id,
        task_id = %id,
        "User updating task"
    );
    let task = service.update_task(id, cmd).await?;
    Ok(Json(ApiResponse::success(task)))
}

pub async fn delete_task(
    State(service): State<Arc<TaskAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, DomainError> {
    // Only project owner or admin can delete tasks
    if auth_user.role != UserRole::Admin {
        if !service.is_project_owner(id, auth_user.id).await? {
            return Err(DomainError::Forbidden("Only project owner can delete tasks".into()));
        }
    }

    tracing::info!(
        user_id = %auth_user.id,
        task_id = %id,
        "User deleting task"
    );
    service.delete_task(id).await?;
    Ok(Json(ApiResponse::ok("Task deleted successfully")))
}
