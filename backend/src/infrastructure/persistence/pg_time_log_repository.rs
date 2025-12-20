use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::domain::entities::TimeLog;
use crate::domain::repositories::TimeLogRepository;
use crate::shared::DomainError;

#[derive(Debug, FromRow)]
struct TimeLogRow {
    id: Uuid,
    task_id: Uuid,
    user_id: Uuid,
    hours: f32,
    date: NaiveDate,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    task_name: Option<String>,
    project_name: Option<String>,
    user_name: Option<String>,
}

impl From<TimeLogRow> for TimeLog {
    fn from(row: TimeLogRow) -> Self {
        TimeLog {
            id: row.id,
            task_id: row.task_id,
            user_id: row.user_id,
            hours: row.hours,
            date: row.date,
            description: row.description,
            created_at: row.created_at,
            updated_at: row.updated_at,
            task_name: row.task_name,
            project_name: row.project_name,
            user_name: row.user_name,
        }
    }
}

pub struct PgTimeLogRepository {
    pool: PgPool,
}

impl PgTimeLogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn base_query() -> &'static str {
        r#"
        SELECT
            tl.id,
            tl.task_id,
            tl.user_id,
            tl.hours,
            tl.date,
            tl.description,
            tl.created_at,
            tl.updated_at,
            t.title as task_name,
            p.name as project_name,
            u.name as user_name
        FROM time_logs tl
        JOIN tasks t ON tl.task_id = t.id
        JOIN projects p ON t.project_id = p.id
        JOIN users u ON tl.user_id = u.id
        "#
    }
}

#[async_trait]
impl TimeLogRepository for PgTimeLogRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<TimeLog>, DomainError> {
        let query = format!("{} WHERE tl.id = $1", Self::base_query());
        let row = sqlx::query_as::<_, TimeLogRow>(&query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Into::into))
    }

    async fn find_by_user(
        &self,
        user_id: Uuid,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
    ) -> Result<Vec<TimeLog>, DomainError> {
        let query = match (start_date, end_date) {
            (Some(_), Some(_)) => format!(
                "{} WHERE tl.user_id = $1 AND tl.date >= $2 AND tl.date <= $3 ORDER BY tl.date DESC, tl.created_at DESC",
                Self::base_query()
            ),
            (Some(_), None) => format!(
                "{} WHERE tl.user_id = $1 AND tl.date >= $2 ORDER BY tl.date DESC, tl.created_at DESC",
                Self::base_query()
            ),
            (None, Some(_)) => format!(
                "{} WHERE tl.user_id = $1 AND tl.date <= $2 ORDER BY tl.date DESC, tl.created_at DESC",
                Self::base_query()
            ),
            (None, None) => format!(
                "{} WHERE tl.user_id = $1 ORDER BY tl.date DESC, tl.created_at DESC",
                Self::base_query()
            ),
        };

        let rows = match (start_date, end_date) {
            (Some(s), Some(e)) => {
                sqlx::query_as::<_, TimeLogRow>(&query)
                    .bind(user_id)
                    .bind(s)
                    .bind(e)
                    .fetch_all(&self.pool)
                    .await?
            }
            (Some(s), None) => {
                sqlx::query_as::<_, TimeLogRow>(&query)
                    .bind(user_id)
                    .bind(s)
                    .fetch_all(&self.pool)
                    .await?
            }
            (None, Some(e)) => {
                sqlx::query_as::<_, TimeLogRow>(&query)
                    .bind(user_id)
                    .bind(e)
                    .fetch_all(&self.pool)
                    .await?
            }
            (None, None) => {
                sqlx::query_as::<_, TimeLogRow>(&query)
                    .bind(user_id)
                    .fetch_all(&self.pool)
                    .await?
            }
        };

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn find_by_task(&self, task_id: Uuid) -> Result<Vec<TimeLog>, DomainError> {
        let query = format!(
            "{} WHERE tl.task_id = $1 ORDER BY tl.date DESC, tl.created_at DESC",
            Self::base_query()
        );
        let rows = sqlx::query_as::<_, TimeLogRow>(&query)
            .bind(task_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn find_by_date_range(
        &self,
        user_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<TimeLog>, DomainError> {
        let query = format!(
            "{} WHERE tl.user_id = $1 AND tl.date >= $2 AND tl.date <= $3 ORDER BY tl.date DESC, tl.created_at DESC",
            Self::base_query()
        );
        let rows = sqlx::query_as::<_, TimeLogRow>(&query)
            .bind(user_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create(&self, time_log: &TimeLog) -> Result<TimeLog, DomainError> {
        sqlx::query(
            r#"
            INSERT INTO time_logs (id, task_id, user_id, hours, date, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(time_log.id)
        .bind(time_log.task_id)
        .bind(time_log.user_id)
        .bind(time_log.hours)
        .bind(time_log.date)
        .bind(&time_log.description)
        .bind(time_log.created_at)
        .bind(time_log.updated_at)
        .execute(&self.pool)
        .await?;

        // Fetch the created time log with joined data
        self.find_by_id(time_log.id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Time log not found after creation".into()))
    }

    async fn update(&self, time_log: &TimeLog) -> Result<TimeLog, DomainError> {
        sqlx::query(
            r#"
            UPDATE time_logs
            SET hours = $1, date = $2, description = $3, updated_at = NOW()
            WHERE id = $4
            "#,
        )
        .bind(time_log.hours)
        .bind(time_log.date)
        .bind(&time_log.description)
        .bind(time_log.id)
        .execute(&self.pool)
        .await?;

        self.find_by_id(time_log.id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Time log not found after update".into()))
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM time_logs WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
