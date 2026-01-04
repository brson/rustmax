//! Live reload functionality via WebSocket.
//!
//! Watches for file changes and broadcasts reload messages to connected clients.

use rustmax::prelude::*;
use rustmax::axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use rustmax::tokio::sync::broadcast;
use rustmax::tokio::time::{interval, Duration};
use rustmax::walkdir::WalkDir;
use rustmax::log::{debug, info};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

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

/// File watcher that polls for changes.
pub struct FileWatcher {
    paths: Vec<PathBuf>,
    mtimes: HashMap<PathBuf, SystemTime>,
    poll_interval: Duration,
}

impl FileWatcher {
    /// Create a new file watcher.
    pub fn new(paths: Vec<PathBuf>) -> Self {
        Self {
            paths,
            mtimes: HashMap::new(),
            poll_interval: Duration::from_millis(500),
        }
    }

    /// Set the poll interval.
    pub fn with_interval(mut self, poll_interval: Duration) -> Self {
        self.poll_interval = poll_interval;
        self
    }

    /// Scan all watched paths and record modification times.
    fn scan(&mut self) -> Vec<PathBuf> {
        let mut changed = Vec::new();

        for base_path in &self.paths {
            if !base_path.exists() {
                continue;
            }

            for entry in WalkDir::new(base_path).into_iter().filter_map(|e| e.ok()) {
                let path = entry.path().to_path_buf();

                if path.is_file() {
                    if let Ok(metadata) = path.metadata() {
                        if let Ok(mtime) = metadata.modified() {
                            match self.mtimes.get(&path) {
                                Some(&prev_mtime) if mtime > prev_mtime => {
                                    changed.push(path.clone());
                                    self.mtimes.insert(path, mtime);
                                }
                                None => {
                                    self.mtimes.insert(path, mtime);
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        changed
    }

    /// Determine change type from file paths.
    fn classify_changes(changed: &[PathBuf]) -> ChangeType {
        let all_css = changed.iter().all(|p| {
            p.extension()
                .and_then(|e| e.to_str())
                .map(|e| e == "css")
                .unwrap_or(false)
        });

        if all_css {
            ChangeType::Css
        } else {
            let has_content = changed.iter().any(|p| {
                p.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| e == "md" || e == "html")
                    .unwrap_or(false)
            });

            if has_content {
                ChangeType::Content
            } else {
                ChangeType::Asset
            }
        }
    }

    /// Start watching for changes and send events.
    pub async fn watch(mut self, state: Arc<LiveReloadState>) {
        info!("File watcher started");

        // Initial scan.
        self.scan();

        let mut ticker = interval(self.poll_interval);

        loop {
            ticker.tick().await;

            let changed = self.scan();

            if !changed.is_empty() {
                let change_type = Self::classify_changes(&changed);
                debug!("Files changed ({:?}): {:?}", change_type, changed);
                info!("Detected {:?} change, triggering reload", change_type);
                state.trigger(change_type);
            }
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
        assert_eq!(watcher.poll_interval, Duration::from_millis(500));
    }

    #[test]
    fn test_file_watcher_interval() {
        let watcher = FileWatcher::new(vec![PathBuf::from("/tmp")])
            .with_interval(Duration::from_secs(1));
        assert_eq!(watcher.poll_interval, Duration::from_secs(1));
    }

    #[test]
    fn test_classify_changes_css() {
        let paths = vec![PathBuf::from("style.css"), PathBuf::from("theme.css")];
        assert!(matches!(FileWatcher::classify_changes(&paths), ChangeType::Css));
    }

    #[test]
    fn test_classify_changes_content() {
        let paths = vec![PathBuf::from("post.md"), PathBuf::from("style.css")];
        assert!(matches!(FileWatcher::classify_changes(&paths), ChangeType::Content));
    }

    #[test]
    fn test_classify_changes_asset() {
        let paths = vec![PathBuf::from("image.png"), PathBuf::from("script.js")];
        assert!(matches!(FileWatcher::classify_changes(&paths), ChangeType::Asset));
    }

    #[test]
    fn test_change_type_as_str() {
        assert_eq!(ChangeType::Content.as_str(), "reload");
        assert_eq!(ChangeType::Css.as_str(), "css-reload");
        assert_eq!(ChangeType::Asset.as_str(), "reload");
    }
}
