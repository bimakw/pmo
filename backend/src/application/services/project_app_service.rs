use std::sync::Arc;
use uuid::Uuid;

use crate::application::commands::{CreateProjectCommand, UpdateProjectCommand};
use crate::domain::entities::{Milestone, Project, Task};
use crate::domain::repositories::ProjectRepository;
use crate::shared::DomainError;

pub struct ProjectAppService {
    project_repository: Arc<dyn ProjectRepository>,
}

impl ProjectAppService {
    pub fn new(project_repository: Arc<dyn ProjectRepository>) -> Self {
        Self { project_repository }
    }

    /// List all projects (admin only - use list_accessible_projects for regular users)
    pub async fn list_projects(&self) -> Result<Vec<Project>, DomainError> {
        self.project_repository.find_all().await
    }

    /// List projects accessible by user (owner OR member)
    pub async fn list_accessible_projects(&self, user_id: Uuid) -> Result<Vec<Project>, DomainError> {
        self.project_repository.find_accessible_by_user(user_id).await
    }

    /// Check if user can access project
    pub async fn can_user_access(&self, project_id: Uuid, user_id: Uuid) -> Result<bool, DomainError> {
        self.project_repository.can_user_access(project_id, user_id).await
    }

    /// Check if user is owner of project
    pub async fn is_owner(&self, project_id: Uuid, user_id: Uuid) -> Result<bool, DomainError> {
        self.project_repository.is_owner(project_id, user_id).await
    }

    pub async fn get_project(&self, id: Uuid) -> Result<Project, DomainError> {
        self.project_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Project not found".into()))
    }

    pub async fn create_project(
        &self,
        cmd: CreateProjectCommand,
        owner_id: Uuid,
    ) -> Result<Project, DomainError> {
        let project = Project::new(
            cmd.name,
            cmd.description,
            owner_id,
            cmd.status,
            cmd.priority,
            cmd.start_date,
            cmd.end_date,
            cmd.budget,
        );

        self.project_repository.create(&project).await
    }

    pub async fn update_project(
        &self,
        id: Uuid,
        cmd: UpdateProjectCommand,
    ) -> Result<Project, DomainError> {
        let mut project = self
            .project_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Project not found".into()))?;

        if let Some(name) = cmd.name {
            project.name = name;
        }
        if let Some(description) = cmd.description {
            project.description = Some(description);
        }
        if let Some(status) = cmd.status {
            project.update_status(status);
        }
        if let Some(priority) = cmd.priority {
            project.priority = priority;
        }
        if let Some(start_date) = cmd.start_date {
            project.start_date = Some(start_date);
        }
        if let Some(end_date) = cmd.end_date {
            project.end_date = Some(end_date);
        }
        if let Some(budget) = cmd.budget {
            project.budget = Some(budget);
        }

        self.project_repository.update(&project).await
    }

    pub async fn delete_project(&self, id: Uuid) -> Result<(), DomainError> {
        // Check if project exists
        self.project_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Project not found".into()))?;

        self.project_repository.delete(id).await
    }

    pub async fn get_project_tasks(&self, project_id: Uuid) -> Result<Vec<Task>, DomainError> {
        // Verify project exists
        self.project_repository
            .find_by_id(project_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Project not found".into()))?;

        self.project_repository.find_tasks(project_id).await
    }

    pub async fn get_project_milestones(
        &self,
        project_id: Uuid,
    ) -> Result<Vec<Milestone>, DomainError> {
        // Verify project exists
        self.project_repository
            .find_by_id(project_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Project not found".into()))?;

        self.project_repository.find_milestones(project_id).await
    }
}
