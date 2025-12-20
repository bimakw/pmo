use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::entities::TimeLog;
use crate::shared::DomainError;

#[async_trait]
pub trait TimeLogRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<TimeLog>, DomainError>;
    async fn find_by_user(&self, user_id: Uuid, start_date: Option<NaiveDate>, end_date: Option<NaiveDate>) -> Result<Vec<TimeLog>, DomainError>;
    async fn find_by_task(&self, task_id: Uuid) -> Result<Vec<TimeLog>, DomainError>;
    async fn find_by_date_range(&self, user_id: Uuid, start_date: NaiveDate, end_date: NaiveDate) -> Result<Vec<TimeLog>, DomainError>;
    async fn create(&self, time_log: &TimeLog) -> Result<TimeLog, DomainError>;
    async fn update(&self, time_log: &TimeLog) -> Result<TimeLog, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}
