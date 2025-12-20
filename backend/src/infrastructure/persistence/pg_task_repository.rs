use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::domain::entities::Task;
use crate::domain::repositories::TaskRepository;
use crate::domain::value_objects::{Priority, TaskStatus};
use crate::shared::DomainError;

#[derive(Debug, FromRow)]
struct TaskRow {
    id: Uuid,
    project_id: Uuid,
    milestone_id: Option<Uuid>,
    title: String,
    description: Option<String>,
    status: TaskStatus,
    priority: Priority,
    assignee_id: Option<Uuid>,
    due_date: Option<DateTime<Utc>>,
    estimated_hours: Option<f32>,
    actual_hours: Option<f32>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<TaskRow> for Task {
    fn from(row: TaskRow) -> Self {
        Task {
            id: row.id,
            project_id: row.project_id,
            milestone_id: row.milestone_id,
            title: row.title,
            description: row.description,
            status: row.status,
            priority: row.priority,
            assignee_id: row.assignee_id,
            due_date: row.due_date,
            estimated_hours: row.estimated_hours,
            actual_hours: row.actual_hours,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

pub struct PgTaskRepository {
    pool: PgPool,
}

impl PgTaskRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskRepository for PgTaskRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Task>, DomainError> {
        let row = sqlx::query_as::<_, TaskRow>("SELECT * FROM tasks WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Into::into))
    }

    async fn find_all(&self) -> Result<Vec<Task>, DomainError> {
        let rows = sqlx::query_as::<_, TaskRow>("SELECT * FROM tasks ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn find_by_project(&self, project_id: Uuid) -> Result<Vec<Task>, DomainError> {
        let rows = sqlx::query_as::<_, TaskRow>(
            "SELECT * FROM tasks WHERE project_id = $1 ORDER BY created_at DESC",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn find_by_assignee(&self, user_id: Uuid) -> Result<Vec<Task>, DomainError> {
        let rows = sqlx::query_as::<_, TaskRow>(
            "SELECT * FROM tasks WHERE assignee_id = $1 ORDER BY created_at DESC",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn find_by_status(&self, status: TaskStatus) -> Result<Vec<Task>, DomainError> {
        let rows = sqlx::query_as::<_, TaskRow>(
            "SELECT * FROM tasks WHERE status = $1 ORDER BY created_at DESC",
        )
        .bind(status)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn find_accessible_by_user(&self, user_id: Uuid) -> Result<Vec<Task>, DomainError> {
        let rows = sqlx::query_as::<_, TaskRow>(
            r#"
            SELECT DISTINCT t.* FROM tasks t
            INNER JOIN projects p ON t.project_id = p.id
            LEFT JOIN project_members pm ON p.id = pm.project_id
            WHERE p.owner_id = $1 OR pm.user_id = $1
            ORDER BY t.created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn can_user_access(&self, task_id: Uuid, user_id: Uuid) -> Result<bool, DomainError> {
        let result: Option<(i64,)> = sqlx::query_as(
            r#"
            SELECT 1 FROM tasks t
            INNER JOIN projects p ON t.project_id = p.id
            LEFT JOIN project_members pm ON p.id = pm.project_id
            WHERE t.id = $1 AND (p.owner_id = $2 OR pm.user_id = $2)
            LIMIT 1
            "#,
        )
        .bind(task_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.is_some())
    }

    async fn is_project_owner(&self, task_id: Uuid, user_id: Uuid) -> Result<bool, DomainError> {
        let result: Option<(i64,)> = sqlx::query_as(
            r#"
            SELECT 1 FROM tasks t
            INNER JOIN projects p ON t.project_id = p.id
            WHERE t.id = $1 AND p.owner_id = $2
            LIMIT 1
            "#,
        )
        .bind(task_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.is_some())
    }

    async fn can_access_project(&self, project_id: Uuid, user_id: Uuid) -> Result<bool, DomainError> {
        let result: Option<(i64,)> = sqlx::query_as(
            r#"
            SELECT 1 FROM projects p
            LEFT JOIN project_members pm ON p.id = pm.project_id
            WHERE p.id = $1 AND (p.owner_id = $2 OR pm.user_id = $2)
            LIMIT 1
            "#,
        )
        .bind(project_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.is_some())
    }

    async fn create(&self, task: &Task) -> Result<Task, DomainError> {
        let row = sqlx::query_as::<_, TaskRow>(
            r#"
            INSERT INTO tasks (id, project_id, milestone_id, title, description, status, priority, assignee_id, due_date, estimated_hours, actual_hours, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING *
            "#,
        )
        .bind(task.id)
        .bind(task.project_id)
        .bind(task.milestone_id)
        .bind(&task.title)
        .bind(&task.description)
        .bind(&task.status)
        .bind(&task.priority)
        .bind(task.assignee_id)
        .bind(task.due_date)
        .bind(task.estimated_hours)
        .bind(task.actual_hours)
        .bind(task.created_at)
        .bind(task.updated_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn update(&self, task: &Task) -> Result<Task, DomainError> {
        let row = sqlx::query_as::<_, TaskRow>(
            r#"
            UPDATE tasks
            SET title = $1, description = $2, status = $3, priority = $4,
                assignee_id = $5, due_date = $6, estimated_hours = $7,
                actual_hours = $8, milestone_id = $9, updated_at = NOW()
            WHERE id = $10
            RETURNING *
            "#,
        )
        .bind(&task.title)
        .bind(&task.description)
        .bind(&task.status)
        .bind(&task.priority)
        .bind(task.assignee_id)
        .bind(task.due_date)
        .bind(task.estimated_hours)
        .bind(task.actual_hours)
        .bind(task.milestone_id)
        .bind(task.id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM tasks WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
