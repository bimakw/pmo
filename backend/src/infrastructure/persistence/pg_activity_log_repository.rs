use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::domain::entities::{ActivityLog, ActivityLogWithDetails};
use crate::domain::repositories::ActivityLogRepository;
use crate::shared::DomainError;

#[derive(Debug, FromRow)]
struct ActivityLogRow {
    id: Uuid,
    user_id: Option<Uuid>,
    project_id: Option<Uuid>,
    action: String,
    entity_type: String,
    entity_id: Uuid,
    details: Option<JsonValue>,
    created_at: DateTime<Utc>,
}

impl From<ActivityLogRow> for ActivityLog {
    fn from(row: ActivityLogRow) -> Self {
        ActivityLog {
            id: row.id,
            user_id: row.user_id,
            project_id: row.project_id,
            action: row.action,
            entity_type: row.entity_type,
            entity_id: row.entity_id,
            details: row.details,
            created_at: row.created_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct ActivityLogWithDetailsRow {
    id: Uuid,
    user_id: Option<Uuid>,
    user_name: Option<String>,
    project_id: Option<Uuid>,
    project_name: Option<String>,
    action: String,
    entity_type: String,
    entity_id: Uuid,
    details: Option<JsonValue>,
    created_at: DateTime<Utc>,
}

impl From<ActivityLogWithDetailsRow> for ActivityLogWithDetails {
    fn from(row: ActivityLogWithDetailsRow) -> Self {
        ActivityLogWithDetails {
            id: row.id,
            user_id: row.user_id,
            user_name: row.user_name,
            project_id: row.project_id,
            project_name: row.project_name,
            action: row.action,
            entity_type: row.entity_type,
            entity_id: row.entity_id,
            details: row.details,
            created_at: row.created_at,
        }
    }
}

pub struct PgActivityLogRepository {
    pool: PgPool,
}

impl PgActivityLogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ActivityLogRepository for PgActivityLogRepository {
    async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<ActivityLogWithDetails>, DomainError> {
        let rows = sqlx::query_as::<_, ActivityLogWithDetailsRow>(
            r#"
            SELECT
                al.id,
                al.user_id,
                u.name as user_name,
                al.project_id,
                p.name as project_name,
                al.action,
                al.entity_type,
                al.entity_id,
                al.details,
                al.created_at
            FROM activity_logs al
            LEFT JOIN users u ON al.user_id = u.id
            LEFT JOIN projects p ON al.project_id = p.id
            ORDER BY al.created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn find_by_project(&self, project_id: Uuid, limit: i64) -> Result<Vec<ActivityLogWithDetails>, DomainError> {
        let rows = sqlx::query_as::<_, ActivityLogWithDetailsRow>(
            r#"
            SELECT
                al.id,
                al.user_id,
                u.name as user_name,
                al.project_id,
                p.name as project_name,
                al.action,
                al.entity_type,
                al.entity_id,
                al.details,
                al.created_at
            FROM activity_logs al
            LEFT JOIN users u ON al.user_id = u.id
            LEFT JOIN projects p ON al.project_id = p.id
            WHERE al.project_id = $1
            ORDER BY al.created_at DESC
            LIMIT $2
            "#,
        )
        .bind(project_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn find_by_user(&self, user_id: Uuid, limit: i64) -> Result<Vec<ActivityLogWithDetails>, DomainError> {
        let rows = sqlx::query_as::<_, ActivityLogWithDetailsRow>(
            r#"
            SELECT
                al.id,
                al.user_id,
                u.name as user_name,
                al.project_id,
                p.name as project_name,
                al.action,
                al.entity_type,
                al.entity_id,
                al.details,
                al.created_at
            FROM activity_logs al
            LEFT JOIN users u ON al.user_id = u.id
            LEFT JOIN projects p ON al.project_id = p.id
            WHERE al.user_id = $1
            ORDER BY al.created_at DESC
            LIMIT $2
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create(&self, log: &ActivityLog) -> Result<ActivityLog, DomainError> {
        let row = sqlx::query_as::<_, ActivityLogRow>(
            r#"
            INSERT INTO activity_logs (id, user_id, project_id, action, entity_type, entity_id, details, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
        )
        .bind(log.id)
        .bind(log.user_id)
        .bind(log.project_id)
        .bind(&log.action)
        .bind(&log.entity_type)
        .bind(log.entity_id)
        .bind(&log.details)
        .bind(log.created_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn count(&self) -> Result<i64, DomainError> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM activity_logs")
            .fetch_one(&self.pool)
            .await?;

        Ok(row.0)
    }
}
