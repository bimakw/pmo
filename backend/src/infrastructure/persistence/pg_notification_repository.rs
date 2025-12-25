use async_trait::async_trait;
use sqlx::{FromRow, PgPool, Row};
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

#[derive(FromRow)]
struct NotificationRow {
    id: Uuid,
    user_id: Uuid,
    notification_type: String,
    title: String,
    message: String,
    link: Option<String>,
    is_read: bool,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl From<NotificationRow> for Notification {
    fn from(row: NotificationRow) -> Self {
        Notification {
            id: row.id,
            user_id: row.user_id,
            notification_type: row.notification_type.parse().unwrap_or(NotificationType::System),
            title: row.title,
            message: row.message,
            link: row.link,
            is_read: row.is_read,
            created_at: row.created_at,
        }
    }
}

#[async_trait]
impl NotificationRepository for PgNotificationRepository {
    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<Notification>, DomainError> {
        let rows: Vec<NotificationRow> = sqlx::query_as(
            r#"
            SELECT id, user_id, notification_type, title, message, link, is_read, created_at
            FROM notifications
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT 100
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn find_unread_by_user(&self, user_id: Uuid) -> Result<Vec<Notification>, DomainError> {
        let rows: Vec<NotificationRow> = sqlx::query_as(
            r#"
            SELECT id, user_id, notification_type, title, message, link, is_read, created_at
            FROM notifications
            WHERE user_id = $1 AND is_read = false
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn count_unread(&self, user_id: Uuid) -> Result<i64, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT COUNT(*) as count
            FROM notifications
            WHERE user_id = $1 AND is_read = false
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(row.get::<i64, _>("count"))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Notification>, DomainError> {
        let row: Option<NotificationRow> = sqlx::query_as(
            r#"
            SELECT id, user_id, notification_type, title, message, link, is_read, created_at
            FROM notifications
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(row.map(Into::into))
    }

    async fn create(&self, notification: &Notification) -> Result<Notification, DomainError> {
        let row: NotificationRow = sqlx::query_as(
            r#"
            INSERT INTO notifications (id, user_id, notification_type, title, message, link, is_read, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, user_id, notification_type, title, message, link, is_read, created_at
            "#,
        )
        .bind(notification.id)
        .bind(notification.user_id)
        .bind(notification.notification_type.to_string())
        .bind(&notification.title)
        .bind(&notification.message)
        .bind(&notification.link)
        .bind(notification.is_read)
        .bind(notification.created_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(row.into())
    }

    async fn mark_as_read(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            UPDATE notifications
            SET is_read = true
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn mark_all_as_read(&self, user_id: Uuid) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            UPDATE notifications
            SET is_read = true
            WHERE user_id = $1 AND is_read = false
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            DELETE FROM notifications
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
