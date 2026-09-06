//! Development server with live reload.

mod browser;
mod livereload;

pub use browser::{open as open_browser, opener, server_url};
pub use livereload::{LiveReloadState, ChangeType, FileWatcher, live_reload_script, inject_script};

use rmx::prelude::*;
use rmx::axum::{
    Router,
    routing::get,
    response::{Html, IntoResponse, Response},
    extract::{State, Path as AxumPath, Query},
    http::StatusCode,
};
use rmx::http::header::CONTENT_TYPE;
use rmx::socket2::{Domain, Protocol, Socket, Type};
use rmx::tower::limit::ConcurrencyLimitLayer;
use rmx::tokio::net::TcpListener;
use rmx::tokio::sync::oneshot;
use rmx::log::info;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use crate::collection::{Collection, Config, Document};
use crate::build::{render_markdown, TemplateEngine};
use crate::search::SearchIndex;
use crate::{Error, Result};

/// How many requests the dev server handles at once.
///
/// One person with a browser, not a load balancer, so this is about bounding
/// file descriptors rather than throughput.
const MAX_CONCURRENT_REQUESTS: usize = 64;

/// Shared server state.
pub(crate) struct AppState {
    collection: Collection,
    config: Config,
    engine: TemplateEngine,
    search_index: SearchIndex,
    include_drafts: bool,
    port: u16,
}

/// Query parameters for search endpoint.
#[rmx::derive(Deserialize)]
struct SearchQuery {
    q: String,
}

/// Query parameters for suggest endpoint.
#[rmx::derive(Deserialize)]
struct SuggestQuery {
    q: String,
}

/// Assemble the dev server's routes.
///
/// Split out from [`serve_with_options`] so that tests can drive the same
/// router the real server runs, rather than a rebuilt approximation of it.
pub(crate) fn router(
    state: Arc<AppState>,
    static_dir: &std::path::Path,
    live_reload: Option<Arc<LiveReloadState>>,
) -> Router {
    let mut app = Router::new()
        .route("/", get(handle_index))
        .route("/{slug}/", get(handle_document))
        .route("/tags/{tag}/", get(handle_tag))
        .route("/api/documents", get(api_documents))
        .route("/api/documents/{slug}", get(api_document))
        .route("/api/search", get(api_search))
        .route("/api/search/suggest", get(api_suggest))
        .with_state(state);

    if let Some(live_reload) = live_reload {
        app = app.merge(
            Router::new()
                .route("/livereload", get(livereload::ws_handler))
                .with_state(live_reload),
        );
    }

    // Serve static files if the directory exists.
    if static_dir.exists() {
        app = app.merge(
            Router::new()
                .route("/static/{*path}", get(handle_static))
                .with_state(Arc::new(static_dir.to_path_buf())),
        );
    }

    // Every request opens files and renders a template. A browser reloading a
    // page with many assets can otherwise put the whole site in flight at
    // once, which on a large collection runs the process out of file
    // descriptors. Requests over the limit wait rather than fail.
    app.layer(ConcurrencyLimitLayer::new(MAX_CONCURRENT_REQUESTS))
}

/// Bind a listening socket for the dev server.
///
/// The socket sets `SO_REUSEADDR` before binding, so restarting the server
/// after a page was still being fetched does not fail with "address already in
/// use" while the old socket sits in `TIME_WAIT`. Tokio's `TcpListener::bind`
/// gives no way to set an option before the bind, so the socket is built with
/// `socket2` and handed over afterwards.
pub(crate) fn bind_listener(addr: SocketAddr) -> Result<TcpListener> {
    let domain = Domain::for_address(addr);
    let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP))
        .map_err(|e| Error::server(format!("failed to create socket: {e}")))?;

    socket
        .set_reuse_address(true)
        .map_err(|e| Error::server(format!("failed to set SO_REUSEADDR: {e}")))?;
    socket
        .bind(&addr.into())
        .map_err(|e| Error::server(format!("failed to bind to {addr}: {e}")))?;
    socket
        .listen(1024)
        .map_err(|e| Error::server(format!("failed to listen on {addr}: {e}")))?;

    // Tokio requires a non-blocking socket to register it with the reactor.
    socket
        .set_nonblocking(true)
        .map_err(|e| Error::server(format!("failed to set the socket non-blocking: {e}")))?;

    TcpListener::from_std(socket.into())
        .map_err(|e| Error::server(format!("failed to register the listener: {e}")))
}

/// Start the development server.
pub fn serve(
    collection: Collection,
    config: Config,
    port: u16,
    include_drafts: bool,
) -> Result<()> {
    serve_with_options(collection, config, port, include_drafts, false)
}

/// Start the development server, optionally opening a browser once it is up.
pub fn serve_with_options(
    collection: Collection,
    config: Config,
    port: u16,
    include_drafts: bool,
    open: bool,
) -> Result<()> {
    let templates_dir = collection.root.join("templates");
    let engine = TemplateEngine::new(&templates_dir)?.with_seed(config.build.seed);
    let static_dir = collection.root.join("static");
    let content_dir = collection.root.join("content");

    // Build search index.
    let search_index = SearchIndex::build(&collection);
    info!("Search index built with {} documents", search_index.documents.len());

    // Set up live reload.
    let live_reload = Arc::new(LiveReloadState::new());
    let live_reload_for_watcher = Arc::clone(&live_reload);
    let live_reload_for_ws = Arc::clone(&live_reload);

    let state = Arc::new(AppState {
        collection,
        config,
        engine,
        search_index,
        include_drafts,
        port,
    });
    drop(live_reload); // Ownership transferred to routes.

    let rt = rmx::tokio::runtime::Runtime::new()?;
    rt.block_on(async move {
        // Start file watcher in background.
        let watch_paths = vec![content_dir, templates_dir, static_dir.clone()];
        rmx::tokio::spawn(async move {
            let watcher = FileWatcher::new(watch_paths);
            watcher.watch(live_reload_for_watcher).await;
        });

        let app = router(state, &static_dir, Some(live_reload_for_ws));

        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let listener = bind_listener(addr)?;

        info!("Listening on {}", server_url(port));
        info!("Live reload enabled at ws://localhost:{}/livereload", port);
        info!("Press Ctrl+C to stop");

        // Opened only once the socket is listening, so the browser cannot
        // arrive before the server is ready to answer.
        if open
            && let Err(e) = browser::open(&server_url(port))
        {
            info!("{e}");
        }

        // Set up graceful shutdown with ctrlc.
        let shutdown_signal = async {
            let (tx, rx) = oneshot::channel::<()>();

            // Use Mutex to allow sending only once from the handler.
            let tx = std::sync::Mutex::new(Some(tx));

            // Set up the ctrlc handler.
            let _ = rmx::ctrlc::set_handler(move || {
                println!(); // Move to new line after ^C
                info!("Received Ctrl+C, shutting down...");
                if let Some(tx) = tx.lock().unwrap().take() {
                    let _ = tx.send(());
                }
            });

            rx.await.ok();
        };

        rmx::axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal)
            .await
            .map_err(|e| Error::server(e.to_string()))?;

        info!("Server stopped");
        Ok(())
    })
}

/// Resolve a request path against a root directory.
///
/// Returns `None` if the path escapes the root,
/// whether by a `..` component, an absolute component,
/// or a symlink pointing outside.
fn resolve_under_root(root: &std::path::Path, request_path: &str) -> Option<PathBuf> {
    let mut resolved = root.to_path_buf();
    for component in request_path.split('/') {
        match component {
            "" | "." => continue,
            ".." => return None,
            name => resolved.push(name),
        }
    }

    // `canonicalize` resolves symlinks, so this also rejects a link out of the
    // tree. It requires the file to exist; a missing file is a 404 either way.
    let canonical_root = root.canonicalize().ok()?;
    let canonical = resolved.canonicalize().ok()?;
    canonical.starts_with(&canonical_root).then_some(canonical)
}

/// Serve a file from the collection's `static` directory.
async fn handle_static(
    State(root): State<Arc<PathBuf>>,
    AxumPath(path): AxumPath<String>,
) -> Response {
    let Some(file) = resolve_under_root(&root, &path) else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let Ok(bytes) = rmx::tokio::fs::read(&file).await else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let content_type = file
        .extension()
        .and_then(|ext| ext.to_str())
        .map(crate::remote::mime_from_extension)
        .unwrap_or(rmx::mime::APPLICATION_OCTET_STREAM);

    ([(CONTENT_TYPE, content_type.as_ref())], bytes).into_response()
}

/// Handle index page.
async fn handle_index(State(state): State<Arc<AppState>>) -> Response {
    let documents: Vec<&Document> = if state.include_drafts {
        state.collection.all_sorted()
    } else {
        state.collection.published()
    };

    let ctx = state.engine.index_context(&documents, &state.config);
    match state.engine.render_first(&["index.html", "default.html"], &ctx) {
        Ok(html) => {
            let html = inject_script(&html, state.port);
            Html(html).into_response()
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {}", e))
                .into_response()
        }
    }
}

/// Handle document page.
async fn handle_document(
    State(state): State<Arc<AppState>>,
    AxumPath(slug): AxumPath<String>,
) -> Response {
    let doc = state.collection.by_slug(&slug);

    match doc {
        Some(doc) => {
            if doc.frontmatter.draft && !state.include_drafts {
                return StatusCode::NOT_FOUND.into_response();
            }

            let html_content = render_markdown(&doc.content);
            let ctx = state
                .engine
                .document_context(doc, &state.config, &html_content);

            let template = doc
                .frontmatter
                .template
                .as_deref()
                .unwrap_or(&state.config.content.default_template);

            match state.engine.render(template, &ctx) {
                Ok(html) => {
                    let html = inject_script(&html, state.port);
                    Html(html).into_response()
                }
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {}", e))
                        .into_response()
                }
            }
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

/// Handle tag page.
async fn handle_tag(
    State(state): State<Arc<AppState>>,
    AxumPath(tag): AxumPath<String>,
) -> Response {
    let documents: Vec<&Document> = state
        .collection
        .by_tag(&tag)
        .into_iter()
        .filter(|d| state.include_drafts || !d.frontmatter.draft)
        .collect();

    if documents.is_empty() {
        return StatusCode::NOT_FOUND.into_response();
    }

    let ctx = state.engine.tag_context(&tag, &documents, &state.config);
    match state.engine.render_first(&["tag.html", "default.html"], &ctx) {
        Ok(html) => {
            let html = inject_script(&html, state.port);
            Html(html).into_response()
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {}", e))
                .into_response()
        }
    }
}

/// API: list all documents.
async fn api_documents(State(state): State<Arc<AppState>>) -> Response {
    let export = state.collection.to_export();
    match rmx::serde_json::to_string_pretty(&export) {
        Ok(json) => (
            StatusCode::OK,
            [("Content-Type", "application/json")],
            json,
        )
            .into_response(),
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("JSON error: {}", e))
                .into_response()
        }
    }
}

/// API: get single document.
async fn api_document(
    State(state): State<Arc<AppState>>,
    AxumPath(slug): AxumPath<String>,
) -> Response {
    let doc = state.collection.by_slug(&slug);

    match doc {
        Some(doc) => {
            let export = doc.to_export();
            match rmx::serde_json::to_string_pretty(&export) {
                Ok(json) => (
                    StatusCode::OK,
                    [("Content-Type", "application/json")],
                    json,
                )
                    .into_response(),
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, format!("JSON error: {}", e))
                        .into_response()
                }
            }
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

/// API: search documents.
async fn api_search(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> Response {
    let results = state.search_index.search(&query.q);
    match rmx::serde_json::to_string(&results) {
        Ok(json) => (
            StatusCode::OK,
            [("Content-Type", "application/json")],
            json,
        )
            .into_response(),
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("JSON error: {}", e))
                .into_response()
        }
    }
}

/// API: autocomplete suggestions.
async fn api_suggest(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SuggestQuery>,
) -> Response {
    let suggestions = state.search_index.suggest(&query.q);
    match rmx::serde_json::to_string(&suggestions) {
        Ok(json) => (
            StatusCode::OK,
            [("Content-Type", "application/json")],
            json,
        )
            .into_response(),
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("JSON error: {}", e))
                .into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmx::tempfile::{TempDir, tempdir};

    // `#[tokio::test]` expands to code naming `tokio` relative to this scope,
    // so the name has to be bound here. This is the plain-import route the
    // rustmax guide describes, and for `tokio` it is no different from
    // depending on the crate directly.
    use rmx::tokio;

    /// A collection with one published document, one draft, and a static file.
    fn test_collection() -> TempDir {
        let dir = tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("content")).unwrap();
        std::fs::create_dir_all(dir.path().join("static/css")).unwrap();
        std::fs::write(dir.path().join("anthology.toml"), "[collection]\ntitle = \"Served\"\n")
            .unwrap();
        std::fs::write(
            dir.path().join("content/hello.md"),
            "---\ntitle = \"Hello\"\ndate = \"2024-01-01\"\ntags = [\"greeting\"]\n---\n\nHello from the dev server.\n",
        )
        .unwrap();
        std::fs::write(
            dir.path().join("content/secret.md"),
            "---\ntitle = \"Secret\"\ndraft = true\n---\n\nNot published.\n",
        )
        .unwrap();
        std::fs::write(dir.path().join("static/css/site.css"), "body { color: red }").unwrap();
        std::fs::write(dir.path().join("outside.txt"), "should not be reachable").unwrap();
        dir
    }

    /// Run `f` against a live server serving `dir`, then shut it down.
    ///
    /// The server binds port 0 and reports what it got, so concurrent tests do
    /// not collide on a fixed port.
    async fn with_server<F, Fut>(dir: &TempDir, include_drafts: bool, f: F)
    where
        F: FnOnce(String) -> Fut,
        Fut: Future<Output = ()>,
    {
        let config = Config::load(dir.path()).unwrap();
        let collection = Collection::load(dir.path(), &config).unwrap();
        let engine = TemplateEngine::new(&dir.path().join("templates"))
            .unwrap()
            .with_seed(config.build.seed);
        let search_index = SearchIndex::build(&collection);

        let state = Arc::new(AppState {
            collection,
            config,
            engine,
            search_index,
            include_drafts,
            port: 0,
        });

        let listener = bind_listener(SocketAddr::from(([127, 0, 0, 1], 0))).unwrap();
        let port = listener.local_addr().unwrap().port();
        let app = router(state, &dir.path().join("static"), None);

        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
        let server = rmx::tokio::spawn(async move {
            rmx::axum::serve(listener, app)
                .with_graceful_shutdown(async {
                    shutdown_rx.await.ok();
                })
                .await
                .unwrap();
        });

        f(format!("http://127.0.0.1:{port}")).await;

        shutdown_tx.send(()).ok();
        server.await.unwrap();
    }

    #[tokio::test]
    async fn the_index_lists_published_documents() {
        let dir = test_collection();
        with_server(&dir, false, |base| async move {
            let body = rmx::reqwest::get(&base).await.unwrap().text().await.unwrap();
            assert!(body.contains("Hello"), "{body}");
            assert!(!body.contains("Secret"), "{body}");
        })
        .await;
    }

    #[tokio::test]
    async fn a_draft_is_hidden_unless_drafts_are_included() {
        let dir = test_collection();
        with_server(&dir, false, |base| async move {
            let status = rmx::reqwest::get(format!("{base}/secret/")).await.unwrap().status();
            assert_eq!(status, rmx::reqwest::StatusCode::NOT_FOUND);
        })
        .await;

        with_server(&dir, true, |base| async move {
            let response = rmx::reqwest::get(format!("{base}/secret/")).await.unwrap();
            assert!(response.status().is_success());
            assert!(response.text().await.unwrap().contains("Secret"));
        })
        .await;
    }

    #[tokio::test]
    async fn a_document_renders_its_content() {
        let dir = test_collection();
        with_server(&dir, false, |base| async move {
            let body = rmx::reqwest::get(format!("{base}/hello/"))
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            assert!(body.contains("Hello from the dev server"), "{body}");
        })
        .await;
    }

    #[tokio::test]
    async fn a_static_file_is_served_with_its_content_type() {
        let dir = test_collection();
        with_server(&dir, false, |base| async move {
            let response = rmx::reqwest::get(format!("{base}/static/css/site.css"))
                .await
                .unwrap();

            assert!(response.status().is_success());
            assert_eq!(
                response.headers()[rmx::reqwest::header::CONTENT_TYPE],
                "text/css",
            );
            assert_eq!(response.text().await.unwrap(), "body { color: red }");
        })
        .await;
    }

    #[tokio::test]
    async fn a_missing_static_file_is_a_404() {
        let dir = test_collection();
        with_server(&dir, false, |base| async move {
            let status = rmx::reqwest::get(format!("{base}/static/nope.css"))
                .await
                .unwrap()
                .status();
            assert_eq!(status, rmx::reqwest::StatusCode::NOT_FOUND);
        })
        .await;
    }

    #[tokio::test]
    async fn the_static_handler_does_not_serve_files_outside_its_root() {
        let dir = test_collection();
        with_server(&dir, false, |base| async move {
            // `reqwest` normalizes `..` in a URL, so the traversal is sent
            // percent-encoded, which reaches the handler intact.
            let response = rmx::reqwest::get(format!("{base}/static/%2e%2e/outside.txt"))
                .await
                .unwrap();

            assert_ne!(
                response.status(),
                rmx::reqwest::StatusCode::OK,
                "escaped the static root",
            );
            assert!(!response.text().await.unwrap().contains("should not be reachable"));
        })
        .await;
    }

    #[tokio::test]
    async fn the_search_api_finds_a_document() {
        let dir = test_collection();
        with_server(&dir, false, |base| async move {
            let results: rmx::serde_json::Value =
                rmx::reqwest::get(format!("{base}/api/search?q=server"))
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

            let hits = results.as_array().or_else(|| results["results"].as_array());
            let hits = hits.unwrap_or_else(|| panic!("unexpected shape: {results}"));
            assert!(
                hits.iter().any(|hit| hit.to_string().contains("hello")),
                "{results}",
            );
        })
        .await;
    }

    #[tokio::test]
    async fn a_tag_page_lists_its_documents() {
        let dir = test_collection();
        with_server(&dir, false, |base| async move {
            let body = rmx::reqwest::get(format!("{base}/tags/greeting/"))
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            assert!(body.contains("Hello"), "{body}");
        })
        .await;
    }

    // `bind_listener` hands the socket to Tokio, which needs a reactor.
    #[tokio::test]
    async fn a_bound_socket_can_be_rebound_immediately() {
        // What SO_REUSEADDR buys: binding the same port again right after the
        // previous listener is dropped, rather than waiting out TIME_WAIT.
        let first = bind_listener(SocketAddr::from(([127, 0, 0, 1], 0))).unwrap();
        let port = first.local_addr().unwrap().port();
        drop(first);

        let second = bind_listener(SocketAddr::from(([127, 0, 0, 1], port)));
        assert!(second.is_ok(), "{:?}", second.err());
    }

    #[test]
    fn a_traversal_component_never_resolves_under_the_root() {
        let dir = tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("static")).unwrap();
        std::fs::write(dir.path().join("static/ok.css"), "ok").unwrap();
        std::fs::write(dir.path().join("outside.txt"), "no").unwrap();
        let root = dir.path().join("static");

        assert!(resolve_under_root(&root, "ok.css").is_some());
        assert!(resolve_under_root(&root, "./ok.css").is_some());
        assert!(resolve_under_root(&root, "../outside.txt").is_none());
        assert!(resolve_under_root(&root, "a/../../outside.txt").is_none());
        assert!(resolve_under_root(&root, "missing.css").is_none());
    }
}
