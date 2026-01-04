//! Development server with live reload.

mod livereload;

pub use livereload::{LiveReloadState, ChangeType, FileWatcher, live_reload_script, inject_script};

use rustmax::prelude::*;
use rustmax::axum::{
    Router,
    routing::get,
    response::{Html, IntoResponse, Response},
    extract::{State, Path as AxumPath, Query},
    http::StatusCode,
};
use tower_http::services::ServeDir;
use rustmax::tokio::net::TcpListener;
use rustmax::tokio::sync::oneshot;
use rustmax::log::info;
use serde::Deserialize;
use std::sync::Arc;

use crate::collection::{Collection, Config, Document};
use crate::build::{render_markdown, TemplateEngine};
use crate::search::SearchIndex;
use crate::{Error, Result};

/// Shared server state.
struct AppState {
    collection: Collection,
    config: Config,
    engine: TemplateEngine,
    search_index: SearchIndex,
    include_drafts: bool,
    port: u16,
}

/// Query parameters for search endpoint.
#[derive(Deserialize)]
struct SearchQuery {
    q: String,
}

/// Query parameters for suggest endpoint.
#[derive(Deserialize)]
struct SuggestQuery {
    q: String,
}

/// Start the development server.
pub fn serve(
    collection: Collection,
    config: Config,
    port: u16,
    include_drafts: bool,
) -> Result<()> {
    let templates_dir = collection.root.join("templates");
    let engine = TemplateEngine::new(&templates_dir)?;
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

    let rt = rustmax::tokio::runtime::Runtime::new()?;
    rt.block_on(async move {
        // Start file watcher in background.
        let watch_paths = vec![content_dir, templates_dir, static_dir.clone()];
        rustmax::tokio::spawn(async move {
            let watcher = FileWatcher::new(watch_paths);
            watcher.watch(live_reload_for_watcher).await;
        });

        // Build the live reload WebSocket route.
        let reload_routes = Router::new()
            .route("/livereload", get(livereload::ws_handler))
            .with_state(live_reload_for_ws);

        let mut app = Router::new()
            .route("/", get(handle_index))
            .route("/{slug}/", get(handle_document))
            .route("/tags/{tag}/", get(handle_tag))
            .route("/api/documents", get(api_documents))
            .route("/api/documents/{slug}", get(api_document))
            .route("/api/search", get(api_search))
            .route("/api/search/suggest", get(api_suggest))
            .with_state(state)
            .merge(reload_routes);

        // Serve static files if directory exists.
        if static_dir.exists() {
            app = app.nest_service("/static", ServeDir::new(&static_dir));
        }

        let addr = format!("0.0.0.0:{}", port);
        info!("Listening on http://localhost:{}", port);
        info!("Live reload enabled at ws://localhost:{}/livereload", port);
        info!("Press Ctrl+C to stop");

        let listener = TcpListener::bind(&addr).await.map_err(|e| {
            Error::server(format!("failed to bind to {}: {}", addr, e))
        })?;

        // Set up graceful shutdown with ctrlc.
        let shutdown_signal = async {
            let (tx, rx) = oneshot::channel::<()>();

            // Use Mutex to allow sending only once from the handler.
            let tx = std::sync::Mutex::new(Some(tx));

            // Set up the ctrlc handler.
            let _ = rustmax::ctrlc::set_handler(move || {
                println!(); // Move to new line after ^C
                info!("Received Ctrl+C, shutting down...");
                if let Some(tx) = tx.lock().unwrap().take() {
                    let _ = tx.send(());
                }
            });

            rx.await.ok();
        };

        rustmax::axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal)
            .await
            .map_err(|e| Error::server(e.to_string()))?;

        info!("Server stopped");
        Ok(())
    })
}

/// Handle index page.
async fn handle_index(State(state): State<Arc<AppState>>) -> Response {
    let documents: Vec<&Document> = if state.include_drafts {
        state.collection.all_sorted()
    } else {
        state.collection.published()
    };

    let ctx = state.engine.index_context(&documents, &state.config);
    match state.engine.render("index.html", &ctx) {
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
    let doc = state
        .collection
        .documents
        .iter()
        .find(|d| d.slug() == slug);

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
    match state.engine.render("tag.html", &ctx) {
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
    match rustmax::serde_json::to_string_pretty(&export) {
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
    let doc = state
        .collection
        .documents
        .iter()
        .find(|d| d.slug() == slug);

    match doc {
        Some(doc) => {
            let export = doc.to_export();
            match rustmax::serde_json::to_string_pretty(&export) {
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
    match rustmax::serde_json::to_string(&results) {
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
    match rustmax::serde_json::to_string(&suggestions) {
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
