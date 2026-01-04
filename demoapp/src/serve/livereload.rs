//! Live reload functionality via WebSocket.
//!
//! Watches for file changes and broadcasts reload messages to connected clients.
//! Uses the notify crate for native filesystem event notifications.

use rustmax::prelude::*;
use rustmax::axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use rustmax::tokio::sync::broadcast;
use rustmax::notify::{self, Watcher, RecursiveMode, EventKind, event::{CreateKind, ModifyKind, RemoveKind}};
use rustmax::log::{debug, info, warn};
use std::path::PathBuf;
use std::sync::Arc;

/// Type of change detected.
#[derive(Debug, Clone, Copy)]
pub enum ChangeType {
    /// Content or template changed.
    Content,
    /// CSS only changed.
    Css,
    /// Static asset changed.
    Asset,
}

impl ChangeType {
    /// Get the message string for this change type.
    fn as_str(&self) -> &'static str {
        match self {
            ChangeType::Content => "reload",
            ChangeType::Css => "css-reload",
            ChangeType::Asset => "reload",
        }
    }
}

/// State for live reload.
pub struct LiveReloadState {
    sender: broadcast::Sender<ChangeType>,
}

impl LiveReloadState {
    /// Create a new live reload state.
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(16);
        Self { sender }
    }

    /// Get a receiver for reload events.
    pub fn subscribe(&self) -> broadcast::Receiver<ChangeType> {
        self.sender.subscribe()
    }

    /// Trigger a reload event.
    pub fn trigger(&self, change_type: ChangeType) {
        // Ignore send errors (no receivers).
        let _ = self.sender.send(change_type);
    }
}

impl Default for LiveReloadState {
    fn default() -> Self {
        Self::new()
    }
}

/// WebSocket handler for live reload connections.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<LiveReloadState>>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

/// Handle a single WebSocket connection.
async fn handle_socket(mut socket: WebSocket, state: Arc<LiveReloadState>) {
    debug!("Live reload client connected");

    let mut receiver = state.subscribe();

    // Send connected message.
    if socket.send(Message::Text("connected".into())).await.is_err() {
        return;
    }

    loop {
        rustmax::tokio::select! {
            // Forward reload events to client.
            result = receiver.recv() => {
                match result {
                    Ok(change_type) => {
                        let msg = change_type.as_str();
                        if socket.send(Message::Text(msg.into())).await.is_err() {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        // Missed some messages, send generic reload.
                        if socket.send(Message::Text("reload".into())).await.is_err() {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
            // Handle incoming messages (ping/pong, close).
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(Message::Ping(data))) => {
                        if socket.send(Message::Pong(data)).await.is_err() {
                            break;
                        }
                    }
                    Some(Err(_)) => break,
                    _ => {}
                }
            }
        }
    }

    debug!("Live reload client disconnected");
}

/// File watcher using native filesystem events via notify crate.
pub struct FileWatcher {
    paths: Vec<PathBuf>,
}

impl FileWatcher {
    /// Create a new file watcher.
    pub fn new(paths: Vec<PathBuf>) -> Self {
        Self { paths }
    }

    /// Determine change type from a file path.
    fn classify_path(path: &PathBuf) -> ChangeType {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        match ext {
            "css" => ChangeType::Css,
            "md" | "html" => ChangeType::Content,
            _ => ChangeType::Asset,
        }
    }

    /// Start watching for changes and send events.
    ///
    /// Uses the notify crate for native filesystem event notifications
    /// instead of polling.
    pub async fn watch(self, state: Arc<LiveReloadState>) {
        info!("File watcher started (native events via notify)");

        // Create a channel to receive file events.
        let (tx, mut rx) = rustmax::tokio::sync::mpsc::channel(100);

        // Create the watcher in a separate thread since notify uses std sync.
        let paths = self.paths.clone();
        std::thread::spawn(move || {
            // Create the watcher with a callback that sends to our channel.
            let tx_clone = tx.clone();
            let mut watcher = match notify::recommended_watcher(
                move |result: Result<notify::Event, notify::Error>| {
                    if let Ok(event) = result {
                        // Filter to only relevant events.
                        let is_relevant = matches!(
                            event.kind,
                            EventKind::Create(CreateKind::File)
                                | EventKind::Modify(ModifyKind::Data(_))
                                | EventKind::Modify(ModifyKind::Name(_))
                                | EventKind::Remove(RemoveKind::File)
                        );

                        if is_relevant && !event.paths.is_empty() {
                            // Send the first affected path.
                            let _ = tx_clone.blocking_send(event.paths[0].clone());
                        }
                    }
                },
            ) {
                Ok(w) => w,
                Err(e) => {
                    warn!("Failed to create file watcher: {}", e);
                    return;
                }
            };

            // Watch all paths recursively.
            for path in &paths {
                if path.exists() {
                    if let Err(e) = watcher.watch(path, RecursiveMode::Recursive) {
                        warn!("Failed to watch {}: {}", path.display(), e);
                    } else {
                        debug!("Watching: {}", path.display());
                    }
                }
            }

            // Keep the watcher alive.
            loop {
                std::thread::park();
            }
        });

        // Process events from the channel.
        while let Some(path) = rx.recv().await {
            let change_type = Self::classify_path(&path);
            debug!("File changed ({:?}): {}", change_type, path.display());
            info!("Detected {:?} change, triggering reload", change_type);
            state.trigger(change_type);
        }
    }
}

/// JavaScript snippet to inject into pages for live reload via WebSocket.
pub fn live_reload_script(_port: u16) -> String {
    r#"<script>
(function() {
    var reconnectDelay = 1000;

    function connect() {
        var protocol = location.protocol === 'https:' ? 'wss:' : 'ws:';
        var ws = new WebSocket(protocol + '//' + location.host + '/livereload');

        ws.onopen = function() {
            console.log('[live-reload] Connected');
            reconnectDelay = 1000;
        };

        ws.onmessage = function(event) {
            if (event.data === 'reload') {
                console.log('[live-reload] Reloading page...');
                location.reload();
            } else if (event.data === 'css-reload') {
                console.log('[live-reload] Reloading CSS...');
                var links = document.querySelectorAll('link[rel="stylesheet"]');
                links.forEach(function(link) {
                    var href = link.href.split('?')[0];
                    link.href = href + '?v=' + Date.now();
                });
            } else if (event.data === 'connected') {
                console.log('[live-reload] Ready');
            }
        };

        ws.onclose = function() {
            console.log('[live-reload] Disconnected, reconnecting in', reconnectDelay, 'ms');
            setTimeout(connect, reconnectDelay);
            reconnectDelay = Math.min(reconnectDelay * 2, 30000);
        };

        ws.onerror = function() {
            ws.close();
        };
    }

    connect();
})();
</script>"#.to_string()
}

/// Inject live reload script into HTML content.
pub fn inject_script(html: &str, port: u16) -> String {
    let script = live_reload_script(port);

    if let Some(pos) = html.rfind("</body>") {
        let mut result = html.to_string();
        result.insert_str(pos, &script);
        result
    } else if let Some(pos) = html.rfind("</html>") {
        let mut result = html.to_string();
        result.insert_str(pos, &script);
        result
    } else {
        format!("{}{}", html, script)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_live_reload_state() {
        let state = LiveReloadState::new();
        let mut rx = state.subscribe();

        state.trigger(ChangeType::Content);

        match rx.try_recv() {
            Ok(ChangeType::Content) => {}
            other => panic!("expected Content, got {:?}", other),
        }
    }

    #[test]
    fn test_live_reload_script() {
        let script = live_reload_script(3000);
        assert!(script.contains("WebSocket"));
        assert!(script.contains("location.reload()"));
    }

    #[test]
    fn test_inject_script_body() {
        let html = "<html><body><p>Hello</p></body></html>";
        let result = inject_script(html, 3000);
        assert!(result.contains("<script>"));
        assert!(result.contains("</script></body>"));
    }

    #[test]
    fn test_inject_script_no_body() {
        let html = "<html><p>Hello</p></html>";
        let result = inject_script(html, 3000);
        assert!(result.contains("<script>"));
        assert!(result.contains("</script></html>"));
    }

    #[test]
    fn test_file_watcher_new() {
        let watcher = FileWatcher::new(vec![PathBuf::from("/tmp")]);
        assert_eq!(watcher.paths.len(), 1);
    }

    #[test]
    fn test_classify_path_css() {
        let path = PathBuf::from("style.css");
        assert!(matches!(FileWatcher::classify_path(&path), ChangeType::Css));
    }

    #[test]
    fn test_classify_path_content_md() {
        let path = PathBuf::from("post.md");
        assert!(matches!(FileWatcher::classify_path(&path), ChangeType::Content));
    }

    #[test]
    fn test_classify_path_content_html() {
        let path = PathBuf::from("template.html");
        assert!(matches!(FileWatcher::classify_path(&path), ChangeType::Content));
    }

    #[test]
    fn test_classify_path_asset() {
        let path = PathBuf::from("image.png");
        assert!(matches!(FileWatcher::classify_path(&path), ChangeType::Asset));
    }

    #[test]
    fn test_change_type_as_str() {
        assert_eq!(ChangeType::Content.as_str(), "reload");
        assert_eq!(ChangeType::Css.as_str(), "css-reload");
        assert_eq!(ChangeType::Asset.as_str(), "reload");
    }
}
