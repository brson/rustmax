//! Live reload functionality via polling.
//!
//! Watches for file changes and provides a polling endpoint for reload detection.

use rustmax::prelude::*;
use rustmax::axum::{
    extract::State,
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use rustmax::tokio::sync::RwLock;
use rustmax::tokio::time::{interval, Duration};
use rustmax::walkdir::WalkDir;
use rustmax::log::{debug, info};
use rustmax::serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;

/// State for live reload.
pub struct LiveReloadState {
    /// Monotonically increasing version number.
    version: AtomicU64,
    /// Last detected change type.
    last_change: RwLock<Option<ChangeType>>,
}

/// Type of change detected.
#[derive(Debug, Clone, Copy, serde::Serialize)]
pub enum ChangeType {
    /// Content or template changed.
    Content,
    /// CSS only changed.
    Css,
    /// Static asset changed.
    Asset,
}

impl LiveReloadState {
    /// Create a new live reload state.
    pub fn new() -> Self {
        Self {
            version: AtomicU64::new(0),
            last_change: RwLock::new(None),
        }
    }

    /// Get the current version.
    pub fn version(&self) -> u64 {
        self.version.load(Ordering::SeqCst)
    }

    /// Trigger a reload with a change type.
    pub async fn trigger(&self, change_type: ChangeType) {
        self.version.fetch_add(1, Ordering::SeqCst);
        *self.last_change.write().await = Some(change_type);
    }

    /// Get and clear the last change type.
    pub async fn take_change(&self) -> Option<ChangeType> {
        self.last_change.write().await.take()
    }
}

impl Default for LiveReloadState {
    fn default() -> Self {
        Self::new()
    }
}

/// HTTP handler for polling reload status.
pub async fn poll_handler(State(state): State<Arc<LiveReloadState>>) -> Response {
    let version = state.version();
    let change = state.take_change().await;

    let response = json!({
        "version": version,
        "reload": change.is_some(),
        "type": change,
    });

    Json(response).into_response()
}

/// HTTP handler to get current version only.
pub async fn version_handler(State(state): State<Arc<LiveReloadState>>) -> Response {
    let version = state.version();
    (StatusCode::OK, version.to_string()).into_response()
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
    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.poll_interval = interval;
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
                state.trigger(change_type).await;
            }
        }
    }
}

/// JavaScript snippet to inject into pages for live reload polling.
pub fn live_reload_script(_port: u16) -> String {
    // Port is unused since we use relative URLs.
    r#"<script>
(function() {
    var lastVersion = 0;
    var pollInterval = 1000;

    function poll() {
        fetch('/livereload/poll')
            .then(function(r) { return r.json(); })
            .then(function(data) {
                if (lastVersion === 0) {
                    lastVersion = data.version;
                    console.log('[live-reload] Connected, version:', lastVersion);
                } else if (data.version > lastVersion) {
                    console.log('[live-reload] Change detected:', data.type);
                    if (data.type === 'Css') {
                        var links = document.querySelectorAll('link[rel="stylesheet"]');
                        links.forEach(function(link) {
                            var href = link.href.split('?')[0];
                            link.href = href + '?v=' + Date.now();
                        });
                        lastVersion = data.version;
                    } else {
                        location.reload();
                    }
                }
            })
            .catch(function() {
                console.log('[live-reload] Poll failed, retrying...');
            })
            .finally(function() {
                setTimeout(poll, pollInterval);
            });
    }

    poll();
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
        let rt = rustmax::tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let state = LiveReloadState::new();
            assert_eq!(state.version(), 0);

            state.trigger(ChangeType::Content).await;
            assert_eq!(state.version(), 1);

            let change = state.take_change().await;
            assert!(matches!(change, Some(ChangeType::Content)));

            let change = state.take_change().await;
            assert!(change.is_none());
        });
    }

    #[test]
    fn test_live_reload_script() {
        let script = live_reload_script(3000);
        assert!(script.contains("/livereload/poll"));
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
}
