use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::domain::entities::{Team, TeamMember};
use crate::domain::repositories::TeamRepository;
use crate::domain::value_objects::TeamMemberRole;
use crate::shared::DomainError;

#[derive(Debug, FromRow)]
struct TeamRow {
    id: Uuid,
    name: String,
    description: Option<String>,
    lead_id: Option<Uuid>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<TeamRow> for Team {
    fn from(row: TeamRow) -> Self {
        Team {
            id: row.id,
            name: row.name,
            description: row.description,
            lead_id: row.lead_id,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct TeamMemberRow {
    id: Uuid,
    team_id: Uuid,
    user_id: Uuid,
    role: TeamMemberRole,
    joined_at: DateTime<Utc>,
}

impl From<TeamMemberRow> for TeamMember {
    fn from(row: TeamMemberRow) -> Self {
        TeamMember {
            id: row.id,
            team_id: row.team_id,
            user_id: row.user_id,
            role: row.role,
            joined_at: row.joined_at,
        }
    }
}

pub struct PgTeamRepository {
    pool: PgPool,
}

impl PgTeamRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TeamRepository for PgTeamRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Team>, DomainError> {
        let row = sqlx::query_as::<_, TeamRow>("SELECT * FROM teams WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Into::into))
    }

    async fn find_all(&self) -> Result<Vec<Team>, DomainError> {
        let rows = sqlx::query_as::<_, TeamRow>("SELECT * FROM teams ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn find_accessible_by_user(&self, user_id: Uuid) -> Result<Vec<Team>, DomainError> {
        let rows = sqlx::query_as::<_, TeamRow>(
            r#"
            SELECT DISTINCT t.* FROM teams t
            LEFT JOIN team_members tm ON t.id = tm.team_id
            WHERE t.lead_id = $1 OR tm.user_id = $1
            ORDER BY t.created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn can_user_access(&self, team_id: Uuid, user_id: Uuid) -> Result<bool, DomainError> {
        let result: Option<(i64,)> = sqlx::query_as(
            r#"
            SELECT 1 FROM teams t
            LEFT JOIN team_members tm ON t.id = tm.team_id
            WHERE t.id = $1 AND (t.lead_id = $2 OR tm.user_id = $2)
            LIMIT 1
            "#,
        )
        .bind(team_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.is_some())
    }

    async fn is_lead(&self, team_id: Uuid, user_id: Uuid) -> Result<bool, DomainError> {
        let result: Option<(i64,)> = sqlx::query_as(
            "SELECT 1 FROM teams WHERE id = $1 AND lead_id = $2 LIMIT 1",
        )
        .bind(team_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.is_some())
    }

    async fn create(&self, team: &Team) -> Result<Team, DomainError> {
        let row = sqlx::query_as::<_, TeamRow>(
            r#"
            INSERT INTO teams (id, name, description, lead_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(team.id)
        .bind(&team.name)
        .bind(&team.description)
        .bind(team.lead_id)
        .bind(team.created_at)
        .bind(team.updated_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn update(&self, team: &Team) -> Result<Team, DomainError> {
        let row = sqlx::query_as::<_, TeamRow>(
            r#"
            UPDATE teams
            SET name = $1, description = $2, lead_id = $3, updated_at = NOW()
            WHERE id = $4
            RETURNING *
            "#,
        )
        .bind(&team.name)
        .bind(&team.description)
        .bind(team.lead_id)
        .bind(team.id)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM teams WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn find_members(&self, team_id: Uuid) -> Result<Vec<TeamMember>, DomainError> {
        let rows = sqlx::query_as::<_, TeamMemberRow>(
            "SELECT * FROM team_members WHERE team_id = $1 ORDER BY joined_at DESC",
        )
        .bind(team_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn add_member(&self, member: &TeamMember) -> Result<TeamMember, DomainError> {
        let row = sqlx::query_as::<_, TeamMemberRow>(
            r#"
            INSERT INTO team_members (id, team_id, user_id, role, joined_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(member.id)
        .bind(member.team_id)
        .bind(member.user_id)
        .bind(&member.role)
        .bind(member.joined_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    async fn remove_member(&self, team_id: Uuid, user_id: Uuid) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM team_members WHERE team_id = $1 AND user_id = $2")
            .bind(team_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
