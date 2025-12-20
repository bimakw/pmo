/*
 * Licensed under the MIT License
 * Copyright (c) 2024 bimakw
 */

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{Tag, TaskTag};
use crate::shared::DomainError;

#[async_trait]
pub trait TagRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tag>, DomainError>;
    async fn find_all(&self) -> Result<Vec<Tag>, DomainError>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Tag>, DomainError>;
    async fn create(&self, tag: &Tag) -> Result<Tag, DomainError>;
    async fn update(&self, tag: &Tag) -> Result<Tag, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;

    // Task-Tag relationships
    async fn find_tags_by_task(&self, task_id: Uuid) -> Result<Vec<Tag>, DomainError>;
    async fn find_tasks_by_tag(&self, tag_id: Uuid) -> Result<Vec<Uuid>, DomainError>;
    async fn add_tag_to_task(&self, task_tag: &TaskTag) -> Result<TaskTag, DomainError>;
    async fn remove_tag_from_task(&self, task_id: Uuid, tag_id: Uuid) -> Result<(), DomainError>;
    async fn set_task_tags(&self, task_id: Uuid, tag_ids: Vec<Uuid>) -> Result<(), DomainError>;
}
