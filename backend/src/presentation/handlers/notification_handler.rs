use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde::Serialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::application::services::{Claims, NotificationAppService};
use crate::domain::entities::Notification;

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: Option<String>,
}

#[derive(Serialize)]
struct UnreadCountResponse {
    count: i64,
}

pub async fn list_notifications(
    State(service): State<Arc<NotificationAppService>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    match service.get_user_notifications(claims.sub).await {
        Ok(notifications) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(notifications),
                message: None,
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<Vec<Notification>> {
                success: false,
                data: None,
                message: Some(e.to_string()),
            }),
        ),
    }
}

pub async fn get_unread_count(
    State(service): State<Arc<NotificationAppService>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    match service.get_unread_count(claims.sub).await {
        Ok(count) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(UnreadCountResponse { count }),
                message: None,
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<UnreadCountResponse> {
                success: false,
                data: None,
                message: Some(e.to_string()),
            }),
        ),
    }
}

pub async fn mark_as_read(
    State(service): State<Arc<NotificationAppService>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match service.mark_as_read(id, claims.sub).await {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::<()> {
                success: true,
                data: None,
                message: Some("Notification marked as read".to_string()),
            }),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some(e.to_string()),
            }),
        ),
    }
}

pub async fn mark_all_as_read(
    State(service): State<Arc<NotificationAppService>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    match service.mark_all_as_read(claims.sub).await {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::<()> {
                success: true,
                data: None,
                message: Some("All notifications marked as read".to_string()),
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some(e.to_string()),
            }),
        ),
    }
}

pub async fn delete_notification(
    State(service): State<Arc<NotificationAppService>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match service.delete_notification(id, claims.sub).await {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::<()> {
                success: true,
                data: None,
                message: Some("Notification deleted".to_string()),
            }),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some(e.to_string()),
            }),
        ),
    }
}
