/*
 * Licensed under the MIT License
 * Copyright (c) 2024 bimakw
 */

use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::domain::entities::Attachment;
use crate::domain::repositories::AttachmentRepository;
use crate::shared::DomainError;

const MAX_FILE_SIZE: i64 = 10 * 1024 * 1024; // 10MB
const ALLOWED_EXTENSIONS: &[&str] = &[
    "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "txt", "csv", "zip", "rar", "7z",
    "png", "jpg", "jpeg", "gif", "webp", "svg", "bmp",
];

pub struct AttachmentAppService {
    attachment_repository: Arc<dyn AttachmentRepository>,
    upload_dir: PathBuf,
}

impl AttachmentAppService {
    pub fn new(attachment_repository: Arc<dyn AttachmentRepository>, upload_dir: PathBuf) -> Self {
        Self {
            attachment_repository,
            upload_dir,
        }
    }

    pub async fn get_task_attachments(&self, task_id: Uuid) -> Result<Vec<Attachment>, DomainError> {
        self.attachment_repository.find_by_task(task_id).await
    }

    pub async fn get_attachment(&self, id: Uuid) -> Result<Attachment, DomainError> {
        self.attachment_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Attachment not found".into()))
    }

    pub async fn upload_attachment(
        &self,
        task_id: Uuid,
        uploaded_by: Uuid,
        original_filename: String,
        content_type: String,
        data: Vec<u8>,
    ) -> Result<Attachment, DomainError> {
        let size_bytes = data.len() as i64;

        // Validate file size
        if size_bytes > MAX_FILE_SIZE {
            return Err(DomainError::ValidationError(format!(
                "File size exceeds maximum allowed size of {} MB",
                MAX_FILE_SIZE / 1024 / 1024
            )));
        }

        // Validate file extension
        let extension = original_filename
            .rsplit('.')
            .next()
            .unwrap_or("")
            .to_lowercase();

        if !ALLOWED_EXTENSIONS.contains(&extension.as_str()) {
            return Err(DomainError::ValidationError(format!(
                "File type '{}' is not allowed",
                extension
            )));
        }

        // Generate unique filename
        let file_id = Uuid::new_v4();
        let filename = format!("{}.{}", file_id, extension);

        // Create task directory if it doesn't exist
        let task_dir = self.upload_dir.join(task_id.to_string());
        fs::create_dir_all(&task_dir).await.map_err(|e| {
            DomainError::InternalError(format!("Failed to create upload directory: {}", e))
        })?;

        // Write file to disk
        let file_path = task_dir.join(&filename);
        let mut file = fs::File::create(&file_path).await.map_err(|e| {
            DomainError::InternalError(format!("Failed to create file: {}", e))
        })?;

        file.write_all(&data).await.map_err(|e| {
            DomainError::InternalError(format!("Failed to write file: {}", e))
        })?;

        // Create attachment record
        let storage_path = format!("{}/{}", task_id, filename);
        let attachment = Attachment::new(
            task_id,
            uploaded_by,
            filename,
            original_filename,
            content_type,
            size_bytes,
            storage_path,
        );

        self.attachment_repository.create(&attachment).await
    }

    pub async fn delete_attachment(&self, id: Uuid) -> Result<(), DomainError> {
        let attachment = self
            .attachment_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Attachment not found".into()))?;

        // Delete file from disk
        let file_path = self.upload_dir.join(&attachment.storage_path);
        if file_path.exists() {
            fs::remove_file(&file_path).await.map_err(|e| {
                DomainError::InternalError(format!("Failed to delete file: {}", e))
            })?;
        }

        // Delete from database
        self.attachment_repository.delete(id).await
    }

    pub async fn get_file_path(&self, id: Uuid) -> Result<PathBuf, DomainError> {
        let attachment = self
            .attachment_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Attachment not found".into()))?;

        let file_path = self.upload_dir.join(&attachment.storage_path);
        if !file_path.exists() {
            return Err(DomainError::NotFound("File not found on disk".into()));
        }

        Ok(file_path)
    }
}
