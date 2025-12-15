/*
 * Licensed under the MIT License
 * Copyright (c) 2024 bimakw
 */

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::domain::entities::Attachment;
use crate::domain::repositories::AttachmentRepository;
use crate::shared::DomainError;

#[derive(Debug, FromRow)]
struct AttachmentRow {
    id: Uuid,
    task_id: Uuid,
    uploaded_by: Uuid,
    filename: String,
    original_filename: String,
    content_type: String,
    size_bytes: i64,
    storage_path: String,
    created_at: DateTime<Utc>,
}

impl From<AttachmentRow> for Attachment {
    fn from(row: AttachmentRow) -> Self {
        Attachment {
            id: row.id,
            task_id: row.task_id,
            uploaded_by: row.uploaded_by,
            filename: row.filename,
            original_filename: row.original_filename,
            content_type: row.content_type,
            size_bytes: row.size_bytes,
            storage_path: row.storage_path,
            created_at: row.created_at,
        }
    }
}

pub struct PgAttachmentRepository {
    pool: PgPool,
}

impl PgAttachmentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AttachmentRepository for PgAttachmentRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Attachment>, DomainError> {
        let row = sqlx::query_as::<_, AttachmentRow>("SELECT * FROM attachments WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Into::into))
    }

    async fn find_by_task(&self, task_id: Uuid) -> Result<Vec<Attachment>, DomainError> {
        let rows = sqlx::query_as::<_, AttachmentRow>(
            "SELECT * FROM attachments WHERE task_id = $1 ORDER BY created_at DESC",
        )
        .bind(task_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn create(&self, attachment: &Attachment) -> Result<Attachment, DomainError> {
        let row = sqlx::query_as::<_, AttachmentRow>(
            r#"
            INSERT INTO attachments (id, task_id, uploaded_by, filename, original_filename, content_type, size_bytes, storage_path, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
        )
        .bind(attachment.id)
        .bind(attachment.task_id)
        .bind(attachment.uploaded_by)
        .bind(&attachment.filename)
        .bind(&attachment.original_filename)
        .bind(&attachment.content_type)
        .bind(attachment.size_bytes)
        .bind(&attachment.storage_path)
        .bind(attachment.created_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM attachments WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn delete_by_task(&self, task_id: Uuid) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM attachments WHERE task_id = $1")
            .bind(task_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
