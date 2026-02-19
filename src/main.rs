use rust_high_performance_api_server::{
    config::Config,
    create_app,
    errors::AppError,
};
use tracing::{info, error};
use tracing_subscriber::EnvFilter;

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    // Initialize tracing subscriber with environment-based filtering
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    // Load configuration
    dotenvy::dotenv().ok(); // Load .env if present, ignore if not
    let config = Config::from_env()?;

    info!(
        address = %config.server.address,
        port = config.server.port,
        "Starting Rust High-Performance API Server"
    );

    // Create and run the application
    let app = create_app(config).await?;

    info!("Server is ready to accept connections");
    
    app.await?;

    Ok(())
}
