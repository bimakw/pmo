/*
 * Licensed under the MIT License
 * Copyright (c) 2024 bimakw
 */

use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::{header, StatusCode},
    response::Response,
    Extension, Json,
};
use std::sync::Arc;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use crate::application::services::AttachmentAppService;
use crate::domain::entities::Attachment;
use crate::presentation::dto::ApiResponse;
use crate::presentation::middleware::AuthUser;
use crate::shared::DomainError;

pub async fn get_task_attachments(
    State(service): State<Arc<AttachmentAppService>>,
    Extension(_auth_user): Extension<AuthUser>,
    Path(task_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<Attachment>>>, DomainError> {
    let attachments = service.get_task_attachments(task_id).await?;
    Ok(Json(ApiResponse::success(attachments)))
}

pub async fn upload_attachment(
    State(service): State<Arc<AttachmentAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Path(task_id): Path<Uuid>,
    mut multipart: Multipart,
) -> Result<Json<ApiResponse<Attachment>>, DomainError> {
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        DomainError::ValidationError(format!("Failed to read multipart field: {}", e))
    })? {
        if field.name() == Some("file") {
            let filename = field
                .file_name()
                .map(|s| s.to_string())
                .ok_or_else(|| DomainError::ValidationError("Missing filename".into()))?;

            let content_type = field
                .content_type()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "application/octet-stream".to_string());

            let data = field.bytes().await.map_err(|e| {
                DomainError::ValidationError(format!("Failed to read file data: {}", e))
            })?;

            tracing::info!(
                user_id = %auth_user.id,
                task_id = %task_id,
                filename = %filename,
                size = data.len(),
                "User uploading attachment"
            );

            let attachment = service
                .upload_attachment(task_id, auth_user.id, filename, content_type, data.to_vec())
                .await?;

            return Ok(Json(ApiResponse::success(attachment)));
        }
    }

    Err(DomainError::ValidationError("No file provided".into()))
}

pub async fn download_attachment(
    State(service): State<Arc<AttachmentAppService>>,
    Extension(_auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Response, DomainError> {
    let attachment = service.get_attachment(id).await?;
    let file_path = service.get_file_path(id).await?;

    let file = tokio::fs::File::open(&file_path).await.map_err(|e| {
        DomainError::InternalError(format!("Failed to open file: {}", e))
    })?;

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let content_disposition = format!(
        "attachment; filename=\"{}\"",
        attachment.original_filename.replace('"', "\\\"")
    );

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, &attachment.content_type)
        .header(header::CONTENT_DISPOSITION, content_disposition)
        .header(header::CONTENT_LENGTH, attachment.size_bytes)
        .body(body)
        .unwrap())
}

pub async fn delete_attachment(
    State(service): State<Arc<AttachmentAppService>>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, DomainError> {
    tracing::info!(
        user_id = %auth_user.id,
        attachment_id = %id,
        "User deleting attachment"
    );

    service.delete_attachment(id).await?;
    Ok(Json(ApiResponse::ok("Attachment deleted successfully")))
}
