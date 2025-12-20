use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityLog {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub details: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
}

impl ActivityLog {
    pub fn new(
        user_id: Option<Uuid>,
        project_id: Option<Uuid>,
        action: String,
        entity_type: String,
        entity_id: Uuid,
        details: Option<JsonValue>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            project_id,
            action,
            entity_type,
            entity_id,
            details,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityLogWithDetails {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub user_name: Option<String>,
    pub project_id: Option<Uuid>,
    pub project_name: Option<String>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub details: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
}
