use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::value_objects::{Priority, ProjectStatus, TaskStatus, TeamMemberRole, UserRole};

// ==================== USER COMMANDS ====================
#[derive(Debug, Deserialize)]
pub struct CreateUserCommand {
    pub email: String,
    pub password: String,
    pub name: String,
    pub role: Option<UserRole>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserCommand {
    pub name: Option<String>,
    pub role: Option<UserRole>,
    pub avatar_url: Option<String>,
}

// ==================== PROJECT COMMANDS ====================
#[derive(Debug, Deserialize)]
pub struct CreateProjectCommand {
    pub name: String,
    pub description: Option<String>,
    pub status: Option<ProjectStatus>,
    pub priority: Option<Priority>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub budget: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectCommand {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<ProjectStatus>,
    pub priority: Option<Priority>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub budget: Option<Decimal>,
}

// ==================== TASK COMMANDS ====================
#[derive(Debug, Deserialize)]
pub struct CreateTaskCommand {
    pub project_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<Priority>,
    pub assignee_id: Option<Uuid>,
    pub due_date: Option<DateTime<Utc>>,
    pub estimated_hours: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskCommand {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
    pub priority: Option<Priority>,
    pub assignee_id: Option<Uuid>,
    pub due_date: Option<DateTime<Utc>>,
    pub estimated_hours: Option<f32>,
    pub actual_hours: Option<f32>,
}

// ==================== TEAM COMMANDS ====================
#[derive(Debug, Deserialize)]
pub struct CreateTeamCommand {
    pub name: String,
    pub description: Option<String>,
    pub lead_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTeamCommand {
    pub name: Option<String>,
    pub description: Option<String>,
    pub lead_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct AddTeamMemberCommand {
    pub user_id: Uuid,
    pub role: Option<TeamMemberRole>,
}

// ==================== AUTH COMMANDS ====================
#[derive(Debug, Deserialize)]
pub struct LoginCommand {
    pub email: String,
    pub password: String,
}

// ==================== TAG COMMANDS ====================
#[derive(Debug, Deserialize)]
pub struct CreateTagCommand {
    pub name: String,
    pub color: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTagCommand {
    pub name: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SetTaskTagsCommand {
    pub tag_ids: Vec<Uuid>,
}
