use std::sync::Arc;
use chrono::{NaiveDate, Utc};
use uuid::Uuid;

use crate::domain::entities::TimeLog;
use crate::domain::repositories::TimeLogRepository;
use crate::shared::DomainError;

#[derive(Debug)]
pub struct CreateTimeLogDto {
    pub task_id: Uuid,
    pub user_id: Uuid,
    pub hours: f32,
    pub date: NaiveDate,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct UpdateTimeLogDto {
    pub hours: Option<f32>,
    pub date: Option<NaiveDate>,
    pub description: Option<String>,
}

pub struct TimeLogAppService {
    time_log_repository: Arc<dyn TimeLogRepository>,
}

impl TimeLogAppService {
    pub fn new(time_log_repository: Arc<dyn TimeLogRepository>) -> Self {
        Self { time_log_repository }
    }

    pub async fn get_time_log(&self, id: Uuid) -> Result<Option<TimeLog>, DomainError> {
        self.time_log_repository.find_by_id(id).await
    }

    pub async fn get_user_time_logs(
        &self,
        user_id: Uuid,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
    ) -> Result<Vec<TimeLog>, DomainError> {
        self.time_log_repository.find_by_user(user_id, start_date, end_date).await
    }

    pub async fn get_task_time_logs(&self, task_id: Uuid) -> Result<Vec<TimeLog>, DomainError> {
        self.time_log_repository.find_by_task(task_id).await
    }

    pub async fn get_time_logs_by_date_range(
        &self,
        user_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<TimeLog>, DomainError> {
        self.time_log_repository.find_by_date_range(user_id, start_date, end_date).await
    }

    pub async fn create_time_log(&self, dto: CreateTimeLogDto) -> Result<TimeLog, DomainError> {
        let now = Utc::now();
        let time_log = TimeLog {
            id: Uuid::new_v4(),
            task_id: dto.task_id,
            user_id: dto.user_id,
            hours: dto.hours,
            date: dto.date,
            description: dto.description,
            created_at: now,
            updated_at: now,
            task_name: None,
            project_name: None,
            user_name: None,
        };

        self.time_log_repository.create(&time_log).await
    }

    pub async fn update_time_log(
        &self,
        id: Uuid,
        dto: UpdateTimeLogDto,
    ) -> Result<TimeLog, DomainError> {
        let existing = self.time_log_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Time log with id {} not found", id)))?;

        let updated = TimeLog {
            hours: dto.hours.unwrap_or(existing.hours),
            date: dto.date.unwrap_or(existing.date),
            description: dto.description.or(existing.description),
            updated_at: Utc::now(),
            ..existing
        };

        self.time_log_repository.update(&updated).await
    }

    pub async fn delete_time_log(&self, id: Uuid) -> Result<(), DomainError> {
        // Check if exists
        self.time_log_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Time log with id {} not found", id)))?;

        self.time_log_repository.delete(id).await
    }
}
