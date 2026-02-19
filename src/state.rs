use crate::models::Task;
use dashmap::DashMap;
use std::sync::Arc;
use uuid::Uuid;

/// Application state shared across all request handlers
#[derive(Clone)]
pub struct AppState {
    /// In-memory task storage (thread-safe)
    pub tasks: Arc<DashMap<Uuid, Task>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(DashMap::new()),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
