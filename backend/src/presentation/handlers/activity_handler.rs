use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::application::services::ActivityAppService;
use crate::domain::entities::ActivityLogWithDetails;
use crate::presentation::dto::ApiResponse;
use crate::shared::DomainError;

#[derive(Debug, Deserialize)]
pub struct ListActivitiesQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub project_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
}

pub async fn list_activities(
    State(service): State<Arc<ActivityAppService>>,
    Query(query): Query<ListActivitiesQuery>,
) -> Result<Json<ApiResponse<Vec<ActivityLogWithDetails>>>, DomainError> {
    let activities = if let Some(project_id) = query.project_id {
        service.get_activities_by_project(project_id, query.limit).await?
    } else if let Some(user_id) = query.user_id {
        service.get_activities_by_user(user_id, query.limit).await?
    } else {
        service.list_activities(query.limit, query.offset).await?
    };

    Ok(Json(ApiResponse::success(activities)))
}
