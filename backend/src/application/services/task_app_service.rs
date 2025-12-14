use std::sync::Arc;
use uuid::Uuid;

use crate::application::commands::{CreateTaskCommand, UpdateTaskCommand};
use crate::domain::entities::Task;
use crate::domain::repositories::TaskRepository;
use crate::shared::DomainError;

pub struct TaskAppService {
    task_repository: Arc<dyn TaskRepository>,
}

impl TaskAppService {
    pub fn new(task_repository: Arc<dyn TaskRepository>) -> Self {
        Self { task_repository }
    }

    pub async fn list_tasks(&self) -> Result<Vec<Task>, DomainError> {
        self.task_repository.find_all().await
    }

    /// List tasks from projects user can access (owner OR member)
    pub async fn list_accessible_tasks(&self, user_id: Uuid) -> Result<Vec<Task>, DomainError> {
        self.task_repository.find_accessible_by_user(user_id).await
    }

    /// Check if user can access task (via project access)
    pub async fn can_user_access(&self, task_id: Uuid, user_id: Uuid) -> Result<bool, DomainError> {
        self.task_repository.can_user_access(task_id, user_id).await
    }

    /// Check if user is owner of the project containing the task
    pub async fn is_project_owner(&self, task_id: Uuid, user_id: Uuid) -> Result<bool, DomainError> {
        self.task_repository.is_project_owner(task_id, user_id).await
    }

    /// Check if user can access project (for create task)
    pub async fn can_access_project(&self, project_id: Uuid, user_id: Uuid) -> Result<bool, DomainError> {
        self.task_repository.can_access_project(project_id, user_id).await
    }

    pub async fn get_task(&self, id: Uuid) -> Result<Task, DomainError> {
        self.task_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Task not found".into()))
    }

    pub async fn create_task(&self, cmd: CreateTaskCommand) -> Result<Task, DomainError> {
        let task = Task::new(
            cmd.project_id,
            cmd.title,
            cmd.description,
            cmd.priority,
            cmd.assignee_id,
            cmd.due_date,
            cmd.estimated_hours,
        );

        self.task_repository.create(&task).await
    }

    pub async fn update_task(&self, id: Uuid, cmd: UpdateTaskCommand) -> Result<Task, DomainError> {
        let mut task = self
            .task_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Task not found".into()))?;

        if let Some(title) = cmd.title {
            task.title = title;
        }
        if let Some(description) = cmd.description {
            task.description = Some(description);
        }
        if let Some(status) = cmd.status {
            task.update_status(status);
        }
        if let Some(priority) = cmd.priority {
            task.priority = priority;
        }
        if cmd.assignee_id.is_some() {
            task.assign_to(cmd.assignee_id);
        }
        if let Some(due_date) = cmd.due_date {
            task.due_date = Some(due_date);
        }
        if let Some(estimated_hours) = cmd.estimated_hours {
            task.estimated_hours = Some(estimated_hours);
        }
        if let Some(actual_hours) = cmd.actual_hours {
            task.actual_hours = Some(actual_hours);
        }

        self.task_repository.update(&task).await
    }

    pub async fn delete_task(&self, id: Uuid) -> Result<(), DomainError> {
        // Check if task exists
        self.task_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Task not found".into()))?;

        self.task_repository.delete(id).await
    }

    pub async fn get_tasks_by_project(&self, project_id: Uuid) -> Result<Vec<Task>, DomainError> {
        self.task_repository.find_by_project(project_id).await
    }

    pub async fn get_tasks_by_assignee(&self, user_id: Uuid) -> Result<Vec<Task>, DomainError> {
        self.task_repository.find_by_assignee(user_id).await
    }
}
