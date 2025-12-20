/*
 * Licensed under the MIT License
 * Copyright (c) 2024 bimakw
 */

use axum::{
    extract::{Path, State},
    Extension, Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::application::commands::{CreateTagCommand, SetTaskTagsCommand, UpdateTagCommand};
use crate::application::services::TagAppService;
use crate::domain::entities::{Tag, TaskTag};
use crate::presentation::dto::ApiResponse;
use crate::presentation::middleware::AuthUser;
use crate::shared::DomainError;

pub async fn list_tags(
    State(service): State<Arc<TagAppService>>,
    Extension(_auth_user): Extension<AuthUser>,
) -> Result<Json<ApiResponse<Vec<Tag>>>, DomainError> {
    let tags = service.list_tags().await?;
    Ok(Json(ApiResponse::success(tags)))
}

pub async fn get_tag(
    State(service): State<Arc<TagAppService>>,
    Extension(_auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Tag>>, DomainError> {
    let tag = service.get_tag(id).await?;
    Ok(Json(ApiResponse::success(tag)))
}

pub async fn create_tag(
    State(service): State<Arc<TagAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Json(cmd): Json<CreateTagCommand>,
) -> Result<Json<ApiResponse<Tag>>, DomainError> {
    tracing::info!(
        user_id = %auth_user.id,
        tag_name = %cmd.name,
        "User creating new tag"
    );
    let tag = service.create_tag(cmd).await?;
    Ok(Json(ApiResponse::success(tag)))
}

pub async fn update_tag(
    State(service): State<Arc<TagAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(cmd): Json<UpdateTagCommand>,
) -> Result<Json<ApiResponse<Tag>>, DomainError> {
    tracing::info!(
        user_id = %auth_user.id,
        tag_id = %id,
        "User updating tag"
    );
    let tag = service.update_tag(id, cmd).await?;
    Ok(Json(ApiResponse::success(tag)))
}

pub async fn delete_tag(
    State(service): State<Arc<TagAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, DomainError> {
    tracing::info!(
        user_id = %auth_user.id,
        tag_id = %id,
        "User deleting tag"
    );
    service.delete_tag(id).await?;
    Ok(Json(ApiResponse::ok("Tag deleted successfully")))
}

pub async fn get_task_tags(
    State(service): State<Arc<TagAppService>>,
    Extension(_auth_user): Extension<AuthUser>,
    Path(task_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<Tag>>>, DomainError> {
    let tags = service.get_task_tags(task_id).await?;
    Ok(Json(ApiResponse::success(tags)))
}

pub async fn add_tag_to_task(
    State(service): State<Arc<TagAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Path((task_id, tag_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<TaskTag>>, DomainError> {
    tracing::info!(
        user_id = %auth_user.id,
        task_id = %task_id,
        tag_id = %tag_id,
        "User adding tag to task"
    );
    let task_tag = service.add_tag_to_task(task_id, tag_id).await?;
    Ok(Json(ApiResponse::success(task_tag)))
}

pub async fn remove_tag_from_task(
    State(service): State<Arc<TagAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Path((task_id, tag_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<()>>, DomainError> {
    tracing::info!(
        user_id = %auth_user.id,
        task_id = %task_id,
        tag_id = %tag_id,
        "User removing tag from task"
    );
    service.remove_tag_from_task(task_id, tag_id).await?;
    Ok(Json(ApiResponse::ok("Tag removed from task")))
}

pub async fn set_task_tags(
    State(service): State<Arc<TagAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Path(task_id): Path<Uuid>,
    Json(cmd): Json<SetTaskTagsCommand>,
) -> Result<Json<ApiResponse<Vec<Tag>>>, DomainError> {
    tracing::info!(
        user_id = %auth_user.id,
        task_id = %task_id,
        tag_count = cmd.tag_ids.len(),
        "User setting task tags"
    );
    let tags = service.set_task_tags(task_id, cmd.tag_ids).await?;
    Ok(Json(ApiResponse::success(tags)))
}
