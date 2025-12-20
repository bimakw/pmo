use std::sync::Arc;
use uuid::Uuid;

use crate::domain::entities::{Notification, NotificationType};
use crate::domain::repositories::NotificationRepository;
use crate::shared::DomainError;

pub struct NotificationAppService {
    notification_repository: Arc<dyn NotificationRepository>,
}

impl NotificationAppService {
    pub fn new(notification_repository: Arc<dyn NotificationRepository>) -> Self {
        Self {
            notification_repository,
        }
    }

    pub async fn get_user_notifications(&self, user_id: Uuid) -> Result<Vec<Notification>, DomainError> {
        self.notification_repository.find_by_user(user_id).await
    }

    pub async fn get_unread_notifications(&self, user_id: Uuid) -> Result<Vec<Notification>, DomainError> {
        self.notification_repository.find_unread_by_user(user_id).await
    }

    pub async fn get_unread_count(&self, user_id: Uuid) -> Result<i64, DomainError> {
        self.notification_repository.count_unread(user_id).await
    }

    pub async fn create_notification(
        &self,
        user_id: Uuid,
        notification_type: NotificationType,
        title: String,
        message: String,
        link: Option<String>,
    ) -> Result<Notification, DomainError> {
        let notification = Notification::new(user_id, notification_type, title, message, link);
        self.notification_repository.create(&notification).await
    }

    pub async fn mark_as_read(&self, id: Uuid, user_id: Uuid) -> Result<(), DomainError> {
        // Verify the notification belongs to the user
        let notification = self.notification_repository.find_by_id(id).await?;
        match notification {
            Some(n) if n.user_id == user_id => {
                self.notification_repository.mark_as_read(id).await
            }
            Some(_) => Err(DomainError::Unauthorized("Not authorized to modify this notification".into())),
            None => Err(DomainError::NotFound("Notification not found".into())),
        }
    }

    pub async fn mark_all_as_read(&self, user_id: Uuid) -> Result<(), DomainError> {
        self.notification_repository.mark_all_as_read(user_id).await
    }

    pub async fn delete_notification(&self, id: Uuid, user_id: Uuid) -> Result<(), DomainError> {
        // Verify the notification belongs to the user
        let notification = self.notification_repository.find_by_id(id).await?;
        match notification {
            Some(n) if n.user_id == user_id => {
                self.notification_repository.delete(id).await
            }
            Some(_) => Err(DomainError::Unauthorized("Not authorized to delete this notification".into())),
            None => Err(DomainError::NotFound("Notification not found".into())),
        }
    }
}
