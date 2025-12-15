/*
 * Licensed under the MIT License
 * Copyright (c) 2024 bimakw
 */

use std::sync::Arc;
use uuid::Uuid;

use crate::application::commands::{CreateTagCommand, UpdateTagCommand};
use crate::domain::entities::{Tag, TaskTag};
use crate::domain::repositories::TagRepository;
use crate::shared::DomainError;

pub struct TagAppService {
    tag_repository: Arc<dyn TagRepository>,
}

impl TagAppService {
    pub fn new(tag_repository: Arc<dyn TagRepository>) -> Self {
        Self { tag_repository }
    }

    pub async fn list_tags(&self) -> Result<Vec<Tag>, DomainError> {
        self.tag_repository.find_all().await
    }

    pub async fn get_tag(&self, id: Uuid) -> Result<Tag, DomainError> {
        self.tag_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Tag not found".into()))
    }

    pub async fn create_tag(&self, cmd: CreateTagCommand) -> Result<Tag, DomainError> {
        // Check if tag with same name already exists
        if let Some(_) = self.tag_repository.find_by_name(&cmd.name).await? {
            return Err(DomainError::ValidationError("Tag with this name already exists".into()));
        }

        let tag = Tag::new(cmd.name, cmd.color, cmd.description);
        self.tag_repository.create(&tag).await
    }

    pub async fn update_tag(&self, id: Uuid, cmd: UpdateTagCommand) -> Result<Tag, DomainError> {
        let mut tag = self
            .tag_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Tag not found".into()))?;

        // Check if new name conflicts with existing tag
        if let Some(name) = &cmd.name {
            if let Some(existing) = self.tag_repository.find_by_name(name).await? {
                if existing.id != id {
                    return Err(DomainError::ValidationError("Tag with this name already exists".into()));
                }
            }
        }

        tag.update(cmd.name, cmd.color, cmd.description);
        self.tag_repository.update(&tag).await
    }

    pub async fn delete_tag(&self, id: Uuid) -> Result<(), DomainError> {
        // Check if tag exists
        self.tag_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Tag not found".into()))?;

        self.tag_repository.delete(id).await
    }

    pub async fn get_task_tags(&self, task_id: Uuid) -> Result<Vec<Tag>, DomainError> {
        self.tag_repository.find_tags_by_task(task_id).await
    }

    pub async fn add_tag_to_task(&self, task_id: Uuid, tag_id: Uuid) -> Result<TaskTag, DomainError> {
        // Verify tag exists
        self.tag_repository
            .find_by_id(tag_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Tag not found".into()))?;

        let task_tag = TaskTag::new(task_id, tag_id);
        self.tag_repository.add_tag_to_task(&task_tag).await
    }

    pub async fn remove_tag_from_task(&self, task_id: Uuid, tag_id: Uuid) -> Result<(), DomainError> {
        self.tag_repository.remove_tag_from_task(task_id, tag_id).await
    }

    pub async fn set_task_tags(&self, task_id: Uuid, tag_ids: Vec<Uuid>) -> Result<Vec<Tag>, DomainError> {
        // Verify all tags exist
        for tag_id in &tag_ids {
            self.tag_repository
                .find_by_id(*tag_id)
                .await?
                .ok_or_else(|| DomainError::NotFound(format!("Tag {} not found", tag_id)))?;
        }

        self.tag_repository.set_task_tags(task_id, tag_ids).await?;
        self.tag_repository.find_tags_by_task(task_id).await
    }
}
