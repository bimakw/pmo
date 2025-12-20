use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeLog {
    pub id: Uuid,
    pub task_id: Uuid,
    pub user_id: Uuid,
    pub hours: f32,
    pub date: NaiveDate,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Joined fields (populated from queries)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl TimeLog {
    pub fn new(
        task_id: Uuid,
        user_id: Uuid,
        hours: f32,
        date: NaiveDate,
        description: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            task_id,
            user_id,
            hours,
            date,
            description,
            created_at: now,
            updated_at: now,
            task_name: None,
            project_name: None,
            user_name: None,
        }
    }
}
