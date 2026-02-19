use crate::{
    errors::AppError,
    models::{CreateTaskRequest, Task, UpdateTaskRequest},
    state::AppState,
};
use actix_web::{web, HttpResponse, Responder};
use std::str::FromStr;
use uuid::Uuid;

/// Health check endpoint
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(crate::models::HealthResponse::ok())
}

/// Get all tasks
pub async fn get_tasks(state: web::Data<AppState>) -> impl Responder {
    let tasks: Vec<Task> = state
        .tasks
        .iter()
        .map(|entry| entry.value().clone())
        .collect();

    HttpResponse::Ok().json(tasks)
}

/// Get a single task by ID
pub async fn get_task(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let id = Uuid::from_str(&path.into_inner())
        .map_err(|_| AppError::Validation("Invalid UUID format".to_string()))?;

    let task = state
        .tasks
        .get(&id)
        .map(|entry| entry.value().clone())
        .ok_or_else(|| AppError::NotFound(format!("Task with id {} not found", id)))?;

    Ok(HttpResponse::Ok().json(task))
}

/// Create a new task
pub async fn create_task(
    req: web::Json<CreateTaskRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    // Validate input
    if req.title.trim().is_empty() {
        return Err(AppError::Validation("Title cannot be empty".to_string()));
    }

    let task = Task::new(req.title.clone(), req.description.clone());
    let id = task.id;

    state.tasks.insert(id, task.clone());

    Ok(HttpResponse::Created().json(task))
}

/// Update an existing task
pub async fn update_task(
    path: web::Path<String>,
    req: web::Json<UpdateTaskRequest>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let id = Uuid::from_str(&path.into_inner())
        .map_err(|_| AppError::Validation("Invalid UUID format".to_string()))?;

    let mut task = state
        .tasks
        .get_mut(&id)
        .ok_or_else(|| AppError::NotFound(format!("Task with id {} not found", id)))?;

    // Validate title if provided
    if let Some(ref title) = req.title {
        if title.trim().is_empty() {
            return Err(AppError::Validation("Title cannot be empty".to_string()));
        }
    }

    task.update(req.title.clone(), req.description.clone(), req.completed);

    Ok(HttpResponse::Ok().json(task.value().clone()))
}

/// Delete a task
pub async fn delete_task(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let id = Uuid::from_str(&path.into_inner())
        .map_err(|_| AppError::Validation("Invalid UUID format".to_string()))?;

    state
        .tasks
        .remove(&id)
        .ok_or_else(|| AppError::NotFound(format!("Task with id {} not found", id)))?;

    Ok(HttpResponse::NoContent().finish())
}
