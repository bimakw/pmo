use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{Team, TeamMember};
use crate::shared::DomainError;

#[async_trait]
pub trait TeamRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Team>, DomainError>;
    async fn find_all(&self) -> Result<Vec<Team>, DomainError>;
    /// Find teams where user is lead OR member
    async fn find_accessible_by_user(&self, user_id: Uuid) -> Result<Vec<Team>, DomainError>;
    /// Check if user can access team (is lead OR member)
    async fn can_user_access(&self, team_id: Uuid, user_id: Uuid) -> Result<bool, DomainError>;
    /// Check if user is lead of team
    async fn is_lead(&self, team_id: Uuid, user_id: Uuid) -> Result<bool, DomainError>;
    async fn create(&self, team: &Team) -> Result<Team, DomainError>;
    async fn update(&self, team: &Team) -> Result<Team, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    async fn find_members(&self, team_id: Uuid) -> Result<Vec<TeamMember>, DomainError>;
    async fn add_member(&self, member: &TeamMember) -> Result<TeamMember, DomainError>;
    async fn remove_member(&self, team_id: Uuid, user_id: Uuid) -> Result<(), DomainError>;
}
