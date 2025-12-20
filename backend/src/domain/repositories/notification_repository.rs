use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::Notification;
use crate::shared::DomainError;

#[async_trait]
pub trait NotificationRepository: Send + Sync {
    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<Notification>, DomainError>;
    async fn find_unread_by_user(&self, user_id: Uuid) -> Result<Vec<Notification>, DomainError>;
    async fn count_unread(&self, user_id: Uuid) -> Result<i64, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Notification>, DomainError>;
    async fn create(&self, notification: &Notification) -> Result<Notification, DomainError>;
    async fn mark_as_read(&self, id: Uuid) -> Result<(), DomainError>;
    async fn mark_all_as_read(&self, user_id: Uuid) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}
