use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::domain::entities::{Milestone, Project, Task};
use crate::domain::repositories::ProjectRepository;
use crate::domain::value_objects::{Priority, ProjectStatus, TaskStatus};
use crate::shared::DomainError;

#[derive(Debug, FromRow)]
struct ProjectRow {
    id: Uuid,
    name: String,
    description: Option<String>,
    status: ProjectStatus,
    priority: Priority,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    budget: Option<Decimal>,
    owner_id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<ProjectRow> for Project {
    fn from(row: ProjectRow) -> Self {
        Project {
            id: row.id,
            name: row.name,
            description: row.description,
            status: row.status,
            priority: row.priority,
            start_date: row.start_date,
            end_date: row.end_date,
            budget: row.budget,
            owner_id: row.owner_id,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

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

#[derive(Debug, FromRow)]
struct MilestoneRow {
    id: Uuid,
    project_id: Uuid,
    name: String,
    description: Option<String>,
    due_date: Option<DateTime<Utc>>,
    completed: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<MilestoneRow> for Milestone {
    fn from(row: MilestoneRow) -> Self {
        Milestone {
            id: row.id,
            project_id: row.project_id,
            name: row.name,
            description: row.description,
            due_date: row.due_date,
            completed: row.completed,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

pub struct PgProjectRepository {
    pool: PgPool,
}

impl PgProjectRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProjectRepository for PgProjectRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Project>, DomainError> {
        let row = sqlx::query_as::<_, ProjectRow>("SELECT * FROM projects WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Into::into))
    }

    async fn find_all(&self) -> Result<Vec<Project>, DomainError> {
        let rows =
            sqlx::query_as::<_, ProjectRow>("SELECT * FROM projects ORDER BY created_at DESC")
                .fetch_all(&self.pool)
                .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn find_by_owner(&self, owner_id: Uuid) -> Result<Vec<Project>, DomainError> {
        let rows = sqlx::query_as::<_, ProjectRow>(
            "SELECT * FROM projects WHERE owner_id = $1 ORDER BY created_at DESC",
        )
        .bind(owner_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn find_accessible_by_user(&self, user_id: Uuid) -> Result<Vec<Project>, DomainError> {
        let rows = sqlx::query_as::<_, ProjectRow>(
            r#"
            SELECT DISTINCT p.* FROM projects p
            LEFT JOIN project_members pm ON p.id = pm.project_id
            WHERE p.owner_id = $1 OR pm.user_id = $1
            ORDER BY p.created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn can_user_access(&self, project_id: Uuid, user_id: Uuid) -> Result<bool, DomainError> {
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

    async fn is_owner(&self, project_id: Uuid, user_id: Uuid) -> Result<bool, DomainError> {
        let result: Option<(i64,)> = sqlx::query_as(
            "SELECT 1 FROM projects WHERE id = $1 AND owner_id = $2 LIMIT 1",
        )
        .bind(project_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.is_some())
    }

    async fn create(&self, project: &Project) -> Result<Project, DomainError> {
        let row = sqlx::query_as::<_, ProjectRow>(
            r#"
            INSERT INTO projects (id, name, description, status, priority, start_date, end_date, budget, owner_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#,
        )
        .bind(project.id)
        .bind(&project.name)
        .bind(&project.description)
        .bind(&project.status)
        .bind(&project.priority)
        .bind(project.start_date)
        .bind(project.end_date)
        .bind(project.budget)
        .bind(project.owner_id)
        .bind(project.created_at)
        .bind(project.updated_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn update(&self, project: &Project) -> Result<Project, DomainError> {
        let row = sqlx::query_as::<_, ProjectRow>(
            r#"
            UPDATE projects
            SET name = $1, description = $2, status = $3, priority = $4,
                start_date = $5, end_date = $6, budget = $7, updated_at = NOW()
            WHERE id = $8
            RETURNING *
            "#,
        )
        .bind(&project.name)
        .bind(&project.description)
        .bind(&project.status)
        .bind(&project.priority)
        .bind(project.start_date)
        .bind(project.end_date)
        .bind(project.budget)
        .bind(project.id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM projects WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn find_tasks(&self, project_id: Uuid) -> Result<Vec<Task>, DomainError> {
        let rows = sqlx::query_as::<_, TaskRow>(
            "SELECT * FROM tasks WHERE project_id = $1 ORDER BY created_at DESC",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn find_milestones(&self, project_id: Uuid) -> Result<Vec<Milestone>, DomainError> {
        let rows = sqlx::query_as::<_, MilestoneRow>(
            "SELECT * FROM milestones WHERE project_id = $1 ORDER BY due_date ASC",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }
}
