use crate::handlers;
use actix_web::web;

/// Configure all application routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // Task endpoints
            .route("/tasks", web::get().to(handlers::get_tasks))
            .route("/tasks", web::post().to(handlers::create_task))
            .route("/tasks/{id}", web::get().to(handlers::get_task))
            .route("/tasks/{id}", web::put().to(handlers::update_task))
            .route("/tasks/{id}", web::delete().to(handlers::delete_task)),
    )
    // Health check (public, no auth required)
    .route("/health", web::get().to(handlers::health_check));
}
