/*
 * Licensed under the MIT License
 * Copyright (c) 2024 bimakw
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub id: Uuid,
    pub task_id: Uuid,
    pub uploaded_by: Uuid,
    pub filename: String,
    pub original_filename: String,
    pub content_type: String,
    pub size_bytes: i64,
    pub storage_path: String,
    pub created_at: DateTime<Utc>,
}

impl Attachment {
    pub fn new(
        task_id: Uuid,
        uploaded_by: Uuid,
        filename: String,
        original_filename: String,
        content_type: String,
        size_bytes: i64,
        storage_path: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            task_id,
            uploaded_by,
            filename,
            original_filename,
            content_type,
            size_bytes,
            storage_path,
            created_at: Utc::now(),
        }
    }

    pub fn is_image(&self) -> bool {
        self.content_type.starts_with("image/")
    }

    pub fn formatted_size(&self) -> String {
        const KB: i64 = 1024;
        const MB: i64 = KB * 1024;
        const GB: i64 = MB * 1024;

        if self.size_bytes >= GB {
            format!("{:.2} GB", self.size_bytes as f64 / GB as f64)
        } else if self.size_bytes >= MB {
            format!("{:.2} MB", self.size_bytes as f64 / MB as f64)
        } else if self.size_bytes >= KB {
            format!("{:.2} KB", self.size_bytes as f64 / KB as f64)
        } else {
            format!("{} B", self.size_bytes)
        }
    }
}
