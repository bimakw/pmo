use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "notification_type", rename_all = "snake_case")]
pub enum NotificationType {
    TaskAssigned,
    TaskUpdated,
    TaskCompleted,
    TaskDueSoon,
    ProjectUpdated,
    CommentAdded,
    Mention,
    System,
}

impl std::fmt::Display for NotificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationType::TaskAssigned => write!(f, "task_assigned"),
            NotificationType::TaskUpdated => write!(f, "task_updated"),
            NotificationType::TaskCompleted => write!(f, "task_completed"),
            NotificationType::TaskDueSoon => write!(f, "task_due_soon"),
            NotificationType::ProjectUpdated => write!(f, "project_updated"),
            NotificationType::CommentAdded => write!(f, "comment_added"),
            NotificationType::Mention => write!(f, "mention"),
            NotificationType::System => write!(f, "system"),
        }
    }
}

impl std::str::FromStr for NotificationType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "task_assigned" => Ok(NotificationType::TaskAssigned),
            "task_updated" => Ok(NotificationType::TaskUpdated),
            "task_completed" => Ok(NotificationType::TaskCompleted),
            "task_due_soon" => Ok(NotificationType::TaskDueSoon),
            "project_updated" => Ok(NotificationType::ProjectUpdated),
            "comment_added" => Ok(NotificationType::CommentAdded),
            "mention" => Ok(NotificationType::Mention),
            "system" => Ok(NotificationType::System),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    #[serde(rename = "type")]
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub link: Option<String>,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

impl Notification {
    pub fn new(
        user_id: Uuid,
        notification_type: NotificationType,
        title: String,
        message: String,
        link: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            notification_type,
            title,
            message,
            link,
            is_read: false,
            created_at: Utc::now(),
        }
    }
}
