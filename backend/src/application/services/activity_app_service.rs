use std::sync::Arc;
use uuid::Uuid;

use crate::domain::entities::ActivityLogWithDetails;
use crate::domain::repositories::ActivityLogRepository;
use crate::shared::DomainError;

pub struct ActivityAppService {
    activity_repository: Arc<dyn ActivityLogRepository>,
}

impl ActivityAppService {
    pub fn new(activity_repository: Arc<dyn ActivityLogRepository>) -> Self {
        Self { activity_repository }
    }

    pub async fn list_activities(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<ActivityLogWithDetails>, DomainError> {
        let limit = limit.unwrap_or(50).min(100);
        let offset = offset.unwrap_or(0);
        self.activity_repository.find_all(limit, offset).await
    }

    pub async fn get_activities_by_project(&self, project_id: Uuid, limit: Option<i64>) -> Result<Vec<ActivityLogWithDetails>, DomainError> {
        let limit = limit.unwrap_or(50).min(100);
        self.activity_repository.find_by_project(project_id, limit).await
    }

    pub async fn get_activities_by_user(&self, user_id: Uuid, limit: Option<i64>) -> Result<Vec<ActivityLogWithDetails>, DomainError> {
        let limit = limit.unwrap_or(50).min(100);
        self.activity_repository.find_by_user(user_id, limit).await
    }

    pub async fn count_activities(&self) -> Result<i64, DomainError> {
        self.activity_repository.count().await
    }
}
