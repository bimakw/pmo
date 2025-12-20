/*
 * Licensed under the MIT License
 * Copyright (c) 2024 bimakw
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub color: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Tag {
    pub fn new(name: String, color: Option<String>, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            color: color.unwrap_or_else(|| "#6b7280".to_string()),
            description,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self, name: Option<String>, color: Option<String>, description: Option<String>) {
        if let Some(n) = name {
            self.name = n;
        }
        if let Some(c) = color {
            self.color = c;
        }
        if description.is_some() {
            self.description = description;
        }
        self.updated_at = Utc::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTag {
    pub id: Uuid,
    pub task_id: Uuid,
    pub tag_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl TaskTag {
    pub fn new(task_id: Uuid, tag_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            task_id,
            tag_id,
            created_at: Utc::now(),
        }
    }
}
