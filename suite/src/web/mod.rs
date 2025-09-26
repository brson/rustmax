// Web server module using axum.

pub mod routes;
// pub mod websocket; // Disabled for now

use rmx::prelude::*;
use crate::infrastructure::state::AppState;

// Start the web server.
pub async fn serve(host: &str, port: u16, state: AppState) -> crate::Result<()> {
    use rmx::axum::Router;
    use std::net::SocketAddr;

    // Build the router.
    let app = routes::build_router(state);

    // Parse address.
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;

    // Start server with graceful shutdown.
    rmx::log::info!("Web server listening on http://{}", addr);

    let listener = rmx::tokio::net::TcpListener::bind(addr).await?;
    rmx::axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

// Graceful shutdown signal handler.
async fn shutdown_signal() {
    use rmx::tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    rmx::tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    rmx::log::info!("Shutdown signal received");
}