use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{ActivityLog, ActivityLogWithDetails};
use crate::shared::DomainError;

#[async_trait]
pub trait ActivityLogRepository: Send + Sync {
    async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<ActivityLogWithDetails>, DomainError>;
    async fn find_by_project(&self, project_id: Uuid, limit: i64) -> Result<Vec<ActivityLogWithDetails>, DomainError>;
    async fn find_by_user(&self, user_id: Uuid, limit: i64) -> Result<Vec<ActivityLogWithDetails>, DomainError>;
    async fn create(&self, log: &ActivityLog) -> Result<ActivityLog, DomainError>;
    async fn count(&self) -> Result<i64, DomainError>;
}
