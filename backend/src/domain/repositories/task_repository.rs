use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::Task;
use crate::domain::value_objects::TaskStatus;
use crate::shared::DomainError;

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Task>, DomainError>;
    async fn find_all(&self) -> Result<Vec<Task>, DomainError>;
    async fn find_by_project(&self, project_id: Uuid) -> Result<Vec<Task>, DomainError>;
    async fn find_by_assignee(&self, user_id: Uuid) -> Result<Vec<Task>, DomainError>;
    async fn find_by_status(&self, status: TaskStatus) -> Result<Vec<Task>, DomainError>;
    /// Find tasks from projects user can access (owner OR member)
    async fn find_accessible_by_user(&self, user_id: Uuid) -> Result<Vec<Task>, DomainError>;
    /// Check if user can access task (via project access)
    async fn can_user_access(&self, task_id: Uuid, user_id: Uuid) -> Result<bool, DomainError>;
    /// Check if user is owner of the project containing the task
    async fn is_project_owner(&self, task_id: Uuid, user_id: Uuid) -> Result<bool, DomainError>;
    /// Check if user can access project (for create task)
    async fn can_access_project(&self, project_id: Uuid, user_id: Uuid) -> Result<bool, DomainError>;
    async fn create(&self, task: &Task) -> Result<Task, DomainError>;
    async fn update(&self, task: &Task) -> Result<Task, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}
