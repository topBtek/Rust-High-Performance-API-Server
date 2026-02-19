pub mod config;
pub mod errors;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod state;

use actix_web::{web, App, HttpServer};
use config::Config;
use errors::AppError;
use middleware::{auth::ApiKeyAuth, logging::RequestLogging};
use routes::configure_routes;
use state::AppState;

/// Creates and configures the Actix-web application
pub async fn create_app(config: Config) -> Result<actix_web::dev::Server, AppError> {
    // Initialize application state
    let app_state = AppState::new();

    // Build the HTTP server
    let server = HttpServer::new(move || {
        App::new()
            // Attach application state
            .app_data(web::Data::new(app_state.clone()))
            // Configure CORS
            .wrap(actix_cors::Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600))
            // Request logging middleware with trace IDs
            .wrap(RequestLogging::default())
            // API key authentication middleware (for protected routes)
            .wrap(ApiKeyAuth::new(&config.api.api_key))
            // Configure routes
            .configure(configure_routes)
    })
    .bind((config.server.address.as_str(), config.server.port))?
    .workers(config.server.workers)
    .run();

    Ok(server)
}
