/*
 * Licensed under the MIT License
 * Copyright (c) 2024 bimakw
 */

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::Attachment;
use crate::shared::DomainError;

#[async_trait]
pub trait AttachmentRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Attachment>, DomainError>;
    async fn find_by_task(&self, task_id: Uuid) -> Result<Vec<Attachment>, DomainError>;
    async fn create(&self, attachment: &Attachment) -> Result<Attachment, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    async fn delete_by_task(&self, task_id: Uuid) -> Result<(), DomainError>;
}
