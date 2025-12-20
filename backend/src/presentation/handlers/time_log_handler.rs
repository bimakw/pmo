use axum::{
    extract::{Path, Query, State},
    Extension,
    Json,
};
use chrono::NaiveDate;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::application::services::{TimeLogAppService, CreateTimeLogDto, UpdateTimeLogDto};
use crate::domain::entities::TimeLog;
use crate::presentation::dto::ApiResponse;
use crate::presentation::middleware::AuthUser;
use crate::shared::DomainError;

#[derive(Debug, Deserialize)]
pub struct ListTimeLogsQuery {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTimeLogRequest {
    pub task_id: Uuid,
    pub hours: f32,
    pub date: NaiveDate,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTimeLogRequest {
    pub hours: Option<f32>,
    pub date: Option<NaiveDate>,
    pub description: Option<String>,
}

/// GET /time-logs - List current user's time logs
pub async fn list_my_time_logs(
    State(service): State<Arc<TimeLogAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Query(params): Query<ListTimeLogsQuery>,
) -> Result<Json<ApiResponse<Vec<TimeLog>>>, DomainError> {
    let time_logs = service.get_user_time_logs(auth_user.id, params.start_date, params.end_date).await?;
    Ok(Json(ApiResponse::success(time_logs)))
}

/// GET /time-logs/:id - Get single time log
pub async fn get_time_log(
    State(service): State<Arc<TimeLogAppService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<TimeLog>>, DomainError> {
    let time_log = service.get_time_log(id)
        .await?
        .ok_or_else(|| DomainError::NotFound(format!("Time log {} not found", id)))?;
    Ok(Json(ApiResponse::success(time_log)))
}

/// GET /tasks/:task_id/time-logs - List time logs for a task
pub async fn list_task_time_logs(
    State(service): State<Arc<TimeLogAppService>>,
    Path(task_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<TimeLog>>>, DomainError> {
    let time_logs = service.get_task_time_logs(task_id).await?;
    Ok(Json(ApiResponse::success(time_logs)))
}

/// GET /users/:user_id/time-logs - List time logs for a specific user
pub async fn list_user_time_logs(
    State(service): State<Arc<TimeLogAppService>>,
    Path(user_id): Path<Uuid>,
    Query(params): Query<ListTimeLogsQuery>,
) -> Result<Json<ApiResponse<Vec<TimeLog>>>, DomainError> {
    let time_logs = service.get_user_time_logs(user_id, params.start_date, params.end_date).await?;
    Ok(Json(ApiResponse::success(time_logs)))
}

/// POST /time-logs - Create a new time log
pub async fn create_time_log(
    State(service): State<Arc<TimeLogAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Json(payload): Json<CreateTimeLogRequest>,
) -> Result<Json<ApiResponse<TimeLog>>, DomainError> {
    let dto = CreateTimeLogDto {
        task_id: payload.task_id,
        user_id: auth_user.id,
        hours: payload.hours,
        date: payload.date,
        description: payload.description,
    };

    let time_log = service.create_time_log(dto).await?;
    Ok(Json(ApiResponse::success(time_log)))
}

/// PUT /time-logs/:id - Update a time log
pub async fn update_time_log(
    State(service): State<Arc<TimeLogAppService>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTimeLogRequest>,
) -> Result<Json<ApiResponse<TimeLog>>, DomainError> {
    let dto = UpdateTimeLogDto {
        hours: payload.hours,
        date: payload.date,
        description: payload.description,
    };

    let time_log = service.update_time_log(id, dto).await?;
    Ok(Json(ApiResponse::success(time_log)))
}

/// DELETE /time-logs/:id - Delete a time log
pub async fn delete_time_log(
    State(service): State<Arc<TimeLogAppService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, DomainError> {
    service.delete_time_log(id).await?;
    Ok(Json(ApiResponse::success(())))
}
