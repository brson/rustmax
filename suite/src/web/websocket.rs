// WebSocket support for real-time communication.

use rmx::prelude::*;
use rmx::serde::{Deserialize};
use rmx::axum::{
    extract::{ws::{WebSocket, WebSocketUpgrade}, State},
    response::IntoResponse,
};
use crate::infrastructure::state::AppState;

// WebSocket handler.
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| websocket_connection(socket, state))
}

// Handle WebSocket connection.
async fn websocket_connection(socket: WebSocket, state: AppState) {
    use rmx::axum::extract::ws::Message;
    use rmx::futures::{SinkExt, StreamExt};

    let (mut sender, mut receiver) = socket.split();

    // Send initial greeting.
    let greeting = rmx::serde_json::json!({
        "type": "connected",
        "message": "Connected to Rustmax Suite WebSocket",
        "version": crate::VERSION,
    });

    if sender.send(Message::Text(greeting.to_string())).await.is_err() {
        return;
    }

    // Spawn task to send periodic metrics.
    let state_clone = state.clone();
    let mut send_task = rmx::tokio::spawn(async move {
        let mut interval = rmx::tokio::time::interval(std::time::Duration::from_secs(5));

        loop {
            interval.tick().await;

            let metrics = state_clone.metrics_snapshot().await;
            let message = rmx::serde_json::json!({
                "type": "metrics",
                "data": metrics,
            });

            if sender.send(Message::Text(message.to_string())).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages.
    let mut recv_task = rmx::tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    // Parse and handle command.
                    if let Ok(command) = serde_json::from_str::<WsCommand>(&text) {
                        handle_websocket_command(command, &state).await;
                    }
                }
                Ok(Message::Close(_)) => break,
                _ => {}
            }
        }
    });

    // Wait for either task to finish.
    rmx::tokio::select! {
        _ = &mut send_task => {
            recv_task.abort();
        }
        _ = &mut recv_task => {
            send_task.abort();
        }
    }

    rmx::log::info!("WebSocket connection closed");
}

// WebSocket command structure.
#[derive(Debug)]
struct WsCommand {
    command: String,
    params: Option<serde_json::Value>,
}

// Handle WebSocket commands.
async fn handle_websocket_command(command: WsCommand, state: &AppState) {
    match command.command.as_str() {
        "ping" => {
            rmx::log::debug!("Received ping from WebSocket client");
        }
        "get_metrics" => {
            // Metrics are already sent periodically.
        }
        "clear_cache" => {
            state.cache_clear().await;
            rmx::log::info!("Cache cleared via WebSocket command");
        }
        _ => {
            rmx::log::warn!("Unknown WebSocket command: {}", command.command);
        }
    }
}