use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Manager,
    Member,
}

impl Default for UserRole {
    fn default() -> Self {
        Self::Member
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "project_status", rename_all = "lowercase")]
pub enum ProjectStatus {
    Planning,
    Active,
    #[serde(rename = "onhold")]
    #[sqlx(rename = "onhold")]
    OnHold,
    Completed,
    Cancelled,
}

impl Default for ProjectStatus {
    fn default() -> Self {
        Self::Planning
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "priority", rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Default for Priority {
    fn default() -> Self {
        Self::Medium
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "task_status", rename_all = "lowercase")]
pub enum TaskStatus {
    Todo,
    #[serde(rename = "inprogress")]
    #[sqlx(rename = "inprogress")]
    InProgress,
    Review,
    Done,
    Blocked,
}

impl Default for TaskStatus {
    fn default() -> Self {
        Self::Todo
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "team_member_role", rename_all = "lowercase")]
pub enum TeamMemberRole {
    Lead,
    Member,
}

impl Default for TeamMemberRole {
    fn default() -> Self {
        Self::Member
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============ UserRole Tests ============

    #[test]
    fn test_user_role_default_is_member() {
        assert_eq!(UserRole::default(), UserRole::Member);
    }

    #[test]
    fn test_user_role_all_variants_exist() {
        let _ = UserRole::Admin;
        let _ = UserRole::Manager;
        let _ = UserRole::Member;
    }

    #[test]
    fn test_user_role_clone() {
        let role = UserRole::Admin;
        let cloned = role.clone();
        assert_eq!(role, cloned);
    }

    #[test]
    fn test_user_role_equality() {
        assert_eq!(UserRole::Admin, UserRole::Admin);
        assert_ne!(UserRole::Admin, UserRole::Member);
    }

    // ============ ProjectStatus Tests ============

    #[test]
    fn test_project_status_default_is_planning() {
        assert_eq!(ProjectStatus::default(), ProjectStatus::Planning);
    }

    #[test]
    fn test_project_status_all_variants_exist() {
        let _ = ProjectStatus::Planning;
        let _ = ProjectStatus::Active;
        let _ = ProjectStatus::OnHold;
        let _ = ProjectStatus::Completed;
        let _ = ProjectStatus::Cancelled;
    }

    #[test]
    fn test_project_status_clone() {
        let status = ProjectStatus::Active;
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_project_status_equality() {
        assert_eq!(ProjectStatus::Completed, ProjectStatus::Completed);
        assert_ne!(ProjectStatus::Active, ProjectStatus::Cancelled);
    }

    // ============ Priority Tests ============

    #[test]
    fn test_priority_default_is_medium() {
        assert_eq!(Priority::default(), Priority::Medium);
    }

    #[test]
    fn test_priority_all_variants_exist() {
        let _ = Priority::Low;
        let _ = Priority::Medium;
        let _ = Priority::High;
        let _ = Priority::Critical;
    }

    #[test]
    fn test_priority_clone() {
        let priority = Priority::Critical;
        let cloned = priority.clone();
        assert_eq!(priority, cloned);
    }

    #[test]
    fn test_priority_equality() {
        assert_eq!(Priority::High, Priority::High);
        assert_ne!(Priority::Low, Priority::Critical);
    }

    // ============ TaskStatus Tests ============

    #[test]
    fn test_task_status_default_is_todo() {
        assert_eq!(TaskStatus::default(), TaskStatus::Todo);
    }

    #[test]
    fn test_task_status_all_variants_exist() {
        let _ = TaskStatus::Todo;
        let _ = TaskStatus::InProgress;
        let _ = TaskStatus::Review;
        let _ = TaskStatus::Done;
        let _ = TaskStatus::Blocked;
    }

    #[test]
    fn test_task_status_clone() {
        let status = TaskStatus::InProgress;
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_task_status_equality() {
        assert_eq!(TaskStatus::Done, TaskStatus::Done);
        assert_ne!(TaskStatus::Todo, TaskStatus::Blocked);
    }

    // ============ TeamMemberRole Tests ============

    #[test]
    fn test_team_member_role_default_is_member() {
        assert_eq!(TeamMemberRole::default(), TeamMemberRole::Member);
    }

    #[test]
    fn test_team_member_role_all_variants_exist() {
        let _ = TeamMemberRole::Lead;
        let _ = TeamMemberRole::Member;
    }

    #[test]
    fn test_team_member_role_clone() {
        let role = TeamMemberRole::Lead;
        let cloned = role.clone();
        assert_eq!(role, cloned);
    }

    #[test]
    fn test_team_member_role_equality() {
        assert_eq!(TeamMemberRole::Lead, TeamMemberRole::Lead);
        assert_ne!(TeamMemberRole::Lead, TeamMemberRole::Member);
    }

    // ============ Serde Serialization Tests ============

    #[test]
    fn test_user_role_serialize() {
        let admin = UserRole::Admin;
        let json = serde_json::to_string(&admin).unwrap();
        assert_eq!(json, "\"Admin\"");
    }

    #[test]
    fn test_user_role_deserialize() {
        let role: UserRole = serde_json::from_str("\"Member\"").unwrap();
        assert_eq!(role, UserRole::Member);
    }

    #[test]
    fn test_project_status_serialize_onhold() {
        let status = ProjectStatus::OnHold;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"onhold\"");
    }

    #[test]
    fn test_project_status_deserialize_onhold() {
        let status: ProjectStatus = serde_json::from_str("\"onhold\"").unwrap();
        assert_eq!(status, ProjectStatus::OnHold);
    }

    #[test]
    fn test_task_status_serialize_inprogress() {
        let status = TaskStatus::InProgress;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"inprogress\"");
    }

    #[test]
    fn test_task_status_deserialize_inprogress() {
        let status: TaskStatus = serde_json::from_str("\"inprogress\"").unwrap();
        assert_eq!(status, TaskStatus::InProgress);
    }

    #[test]
    fn test_priority_round_trip() {
        let priority = Priority::Critical;
        let json = serde_json::to_string(&priority).unwrap();
        let deserialized: Priority = serde_json::from_str(&json).unwrap();
        assert_eq!(priority, deserialized);
    }
}
