use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Task model representing a todo item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    /// Create a new task
    pub fn new(title: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            completed: false,
            created_at: now,
            updated_at: now,
        }
    }

    /// Update task fields
    pub fn update(&mut self, title: Option<String>, description: Option<Option<String>>, completed: Option<bool>) {
        if let Some(t) = title {
            self.title = t;
        }
        if let Some(d) = description {
            self.description = d;
        }
        if let Some(c) = completed {
            self.completed = c;
        }
        self.updated_at = Utc::now();
    }
}

/// Request DTO for creating a task
#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
}

/// Request DTO for updating a task
#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<Option<String>>,
    #[serde(default)]
    pub completed: Option<bool>,
}

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub version: String,
}

impl HealthResponse {
    pub fn ok() -> Self {
        Self {
            status: "healthy".to_string(),
            timestamp: Utc::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}
