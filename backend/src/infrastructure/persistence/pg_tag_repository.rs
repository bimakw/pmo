/*
 * Licensed under the MIT License
 * Copyright (c) 2024 bimakw
 */

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::domain::entities::{Tag, TaskTag};
use crate::domain::repositories::TagRepository;
use crate::shared::DomainError;

#[derive(Debug, FromRow)]
struct TagRow {
    id: Uuid,
    name: String,
    color: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<TagRow> for Tag {
    fn from(row: TagRow) -> Self {
        Tag {
            id: row.id,
            name: row.name,
            color: row.color,
            description: row.description,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct TaskTagRow {
    id: Uuid,
    task_id: Uuid,
    tag_id: Uuid,
    created_at: DateTime<Utc>,
}

impl From<TaskTagRow> for TaskTag {
    fn from(row: TaskTagRow) -> Self {
        TaskTag {
            id: row.id,
            task_id: row.task_id,
            tag_id: row.tag_id,
            created_at: row.created_at,
        }
    }
}

pub struct PgTagRepository {
    pool: PgPool,
}

impl PgTagRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TagRepository for PgTagRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tag>, DomainError> {
        let row = sqlx::query_as::<_, TagRow>("SELECT * FROM tags WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Into::into))
    }

    async fn find_all(&self) -> Result<Vec<Tag>, DomainError> {
        let rows = sqlx::query_as::<_, TagRow>("SELECT * FROM tags ORDER BY name ASC")
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Tag>, DomainError> {
        let row = sqlx::query_as::<_, TagRow>("SELECT * FROM tags WHERE name = $1")
            .bind(name)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Into::into))
    }

    async fn create(&self, tag: &Tag) -> Result<Tag, DomainError> {
        let row = sqlx::query_as::<_, TagRow>(
            r#"
            INSERT INTO tags (id, name, color, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(tag.id)
        .bind(&tag.name)
        .bind(&tag.color)
        .bind(&tag.description)
        .bind(tag.created_at)
        .bind(tag.updated_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn update(&self, tag: &Tag) -> Result<Tag, DomainError> {
        let row = sqlx::query_as::<_, TagRow>(
            r#"
            UPDATE tags
            SET name = $1, color = $2, description = $3, updated_at = NOW()
            WHERE id = $4
            RETURNING *
            "#,
        )
        .bind(&tag.name)
        .bind(&tag.color)
        .bind(&tag.description)
        .bind(tag.id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM tags WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn find_tags_by_task(&self, task_id: Uuid) -> Result<Vec<Tag>, DomainError> {
        let rows = sqlx::query_as::<_, TagRow>(
            r#"
            SELECT t.* FROM tags t
            INNER JOIN task_tags tt ON t.id = tt.tag_id
            WHERE tt.task_id = $1
            ORDER BY t.name ASC
            "#,
        )
        .bind(task_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn find_tasks_by_tag(&self, tag_id: Uuid) -> Result<Vec<Uuid>, DomainError> {
        let rows: Vec<(Uuid,)> = sqlx::query_as(
            "SELECT task_id FROM task_tags WHERE tag_id = $1",
        )
        .bind(tag_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.0).collect())
    }

    async fn add_tag_to_task(&self, task_tag: &TaskTag) -> Result<TaskTag, DomainError> {
        let row = sqlx::query_as::<_, TaskTagRow>(
            r#"
            INSERT INTO task_tags (id, task_id, tag_id, created_at)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (task_id, tag_id) DO NOTHING
            RETURNING *
            "#,
        )
        .bind(task_tag.id)
        .bind(task_tag.task_id)
        .bind(task_tag.tag_id)
        .bind(task_tag.created_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn remove_tag_from_task(&self, task_id: Uuid, tag_id: Uuid) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM task_tags WHERE task_id = $1 AND tag_id = $2")
            .bind(task_id)
            .bind(tag_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn set_task_tags(&self, task_id: Uuid, tag_ids: Vec<Uuid>) -> Result<(), DomainError> {
        // Delete existing tags
        sqlx::query("DELETE FROM task_tags WHERE task_id = $1")
            .bind(task_id)
            .execute(&self.pool)
            .await?;

        // Insert new tags
        for tag_id in tag_ids {
            let task_tag = TaskTag::new(task_id, tag_id);
            sqlx::query(
                "INSERT INTO task_tags (id, task_id, tag_id, created_at) VALUES ($1, $2, $3, $4)",
            )
            .bind(task_tag.id)
            .bind(task_tag.task_id)
            .bind(task_tag.tag_id)
            .bind(task_tag.created_at)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }
}
