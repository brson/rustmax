// Web server binary entry point.

use rustmax_suite::{infrastructure::{config::Config, logging::init_logging, state::AppState}, web};

#[rmx::tokio::main]
async fn main() -> rustmax_suite::Result<()> {
    // Load configuration.
    let mut config = Config::default();
    config.merge_env()?;

    // Initialize logging.
    init_logging(&config.log_level)?;

    // Create application state.
    let state = AppState::new(config.clone());

    println!("Starting Rustmax Suite Web Server");
    println!("Listening on http://{}:{}", config.web.host, config.web.port);

    // Start web server.
    web::serve(&config.web.host, config.web.port, state).await
}