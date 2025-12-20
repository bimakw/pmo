use std::sync::Arc;
use uuid::Uuid;

use crate::application::commands::{AddTeamMemberCommand, CreateTeamCommand, UpdateTeamCommand};
use crate::domain::entities::{Team, TeamMember};
use crate::domain::repositories::TeamRepository;
use crate::shared::DomainError;

pub struct TeamAppService {
    team_repository: Arc<dyn TeamRepository>,
}

impl TeamAppService {
    pub fn new(team_repository: Arc<dyn TeamRepository>) -> Self {
        Self { team_repository }
    }

    pub async fn list_teams(&self) -> Result<Vec<Team>, DomainError> {
        self.team_repository.find_all().await
    }

    /// List teams accessible by user (lead OR member)
    pub async fn list_accessible_teams(&self, user_id: Uuid) -> Result<Vec<Team>, DomainError> {
        self.team_repository.find_accessible_by_user(user_id).await
    }

    /// Check if user can access team (is lead OR member)
    pub async fn can_user_access(&self, team_id: Uuid, user_id: Uuid) -> Result<bool, DomainError> {
        self.team_repository.can_user_access(team_id, user_id).await
    }

    /// Check if user is lead of team
    pub async fn is_lead(&self, team_id: Uuid, user_id: Uuid) -> Result<bool, DomainError> {
        self.team_repository.is_lead(team_id, user_id).await
    }

    pub async fn get_team(&self, id: Uuid) -> Result<Team, DomainError> {
        self.team_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Team not found".into()))
    }

    pub async fn create_team(&self, cmd: CreateTeamCommand) -> Result<Team, DomainError> {
        let team = Team::new(cmd.name, cmd.description, cmd.lead_id);
        self.team_repository.create(&team).await
    }

    pub async fn update_team(&self, id: Uuid, cmd: UpdateTeamCommand) -> Result<Team, DomainError> {
        let mut team = self
            .team_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Team not found".into()))?;

        if let Some(name) = cmd.name {
            team.name = name;
        }
        if let Some(description) = cmd.description {
            team.description = Some(description);
        }
        if cmd.lead_id.is_some() {
            team.set_lead(cmd.lead_id);
        }

        self.team_repository.update(&team).await
    }

    pub async fn delete_team(&self, id: Uuid) -> Result<(), DomainError> {
        // Check if team exists
        self.team_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Team not found".into()))?;

        self.team_repository.delete(id).await
    }

    pub async fn get_team_members(&self, team_id: Uuid) -> Result<Vec<TeamMember>, DomainError> {
        // Verify team exists
        self.team_repository
            .find_by_id(team_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Team not found".into()))?;

        self.team_repository.find_members(team_id).await
    }

    pub async fn add_team_member(
        &self,
        team_id: Uuid,
        cmd: AddTeamMemberCommand,
    ) -> Result<TeamMember, DomainError> {
        // Verify team exists
        self.team_repository
            .find_by_id(team_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Team not found".into()))?;

        let member = TeamMember::new(team_id, cmd.user_id, cmd.role);
        self.team_repository.add_member(&member).await
    }

    pub async fn remove_team_member(&self, team_id: Uuid, user_id: Uuid) -> Result<(), DomainError> {
        self.team_repository.remove_member(team_id, user_id).await
    }
}
