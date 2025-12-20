use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::{Notification, NotificationType};
use crate::domain::repositories::NotificationRepository;
use crate::shared::DomainError;

pub struct PgNotificationRepository {
    pool: PgPool,
}

impl PgNotificationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NotificationRepository for PgNotificationRepository {
    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<Notification>, DomainError> {
        let notifications = sqlx::query_as!(
            Notification,
            r#"
            SELECT
                id,
                user_id,
                notification_type as "notification_type: NotificationType",
                title,
                message,
                link,
                is_read,
                created_at
            FROM notifications
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT 100
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(notifications)
    }

    async fn find_unread_by_user(&self, user_id: Uuid) -> Result<Vec<Notification>, DomainError> {
        let notifications = sqlx::query_as!(
            Notification,
            r#"
            SELECT
                id,
                user_id,
                notification_type as "notification_type: NotificationType",
                title,
                message,
                link,
                is_read,
                created_at
            FROM notifications
            WHERE user_id = $1 AND is_read = false
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(notifications)
    }

    async fn count_unread(&self, user_id: Uuid) -> Result<i64, DomainError> {
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as "count!"
            FROM notifications
            WHERE user_id = $1 AND is_read = false
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(count)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Notification>, DomainError> {
        let notification = sqlx::query_as!(
            Notification,
            r#"
            SELECT
                id,
                user_id,
                notification_type as "notification_type: NotificationType",
                title,
                message,
                link,
                is_read,
                created_at
            FROM notifications
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(notification)
    }

    async fn create(&self, notification: &Notification) -> Result<Notification, DomainError> {
        let created = sqlx::query_as!(
            Notification,
            r#"
            INSERT INTO notifications (id, user_id, notification_type, title, message, link, is_read, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING
                id,
                user_id,
                notification_type as "notification_type: NotificationType",
                title,
                message,
                link,
                is_read,
                created_at
            "#,
            notification.id,
            notification.user_id,
            notification.notification_type.to_string() as _,
            notification.title,
            notification.message,
            notification.link,
            notification.is_read,
            notification.created_at
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(created)
    }

    async fn mark_as_read(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            UPDATE notifications
            SET is_read = true
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn mark_all_as_read(&self, user_id: Uuid) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            UPDATE notifications
            SET is_read = true
            WHERE user_id = $1 AND is_read = false
            "#,
            user_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query!(
            r#"
            DELETE FROM notifications
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
