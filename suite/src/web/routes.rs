// Web route definitions.

use rmx::prelude::*;
use rmx::serde::{Deserialize};
use rmx::axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use crate::infrastructure::state::{AppState, MetricType};

// Build the application router.
pub fn build_router(state: AppState) -> Router {
    Router::new()
        // Core API routes.
        .route("/", get(index_handler))
        .route("/health", get(health_handler))
        .route("/metrics", get(metrics_handler))
        // Temporarily disable POST handlers to focus on GET handlers
        // .route("/api/scan", post(scan_handler))
        // .route("/api/analyze", post(analyze_handler))
        // .route("/api/test", post(test_handler))
        // .route("/api/build", post(build_handler))
        // .route("/api/format", post(format_handler))

        // WebSocket endpoint (disabled for now).
        // .route("/ws", get(crate::web::websocket::websocket_handler))

        // Static assets.
        .route("/static/*path", get(static_handler))

        // Attach shared state.
        .with_state(state)

        // Add middleware (disabled for now).
        // .layer(rmx::tower::ServiceBuilder::new()
        //     .layer(rmx::axum::middleware::from_fn(logging_middleware))
        //     .layer(rmx::tower::limit::ConcurrencyLimitLayer::new(100))
        //     .layer(rmx::tower::timeout::TimeoutLayer::new(std::time::Duration::from_secs(30))))
}

// Middleware for request logging (disabled for now).
/*
async fn logging_middleware(
    req: rmx::axum::http::Request<rmx::axum::body::Body>,
    next: rmx::axum::middleware::Next,
) -> impl IntoResponse {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = std::time::Instant::now();

    let response = next.run(req).await;

    let elapsed = start.elapsed();
    rmx::log::info!(
        "{} {} {} - {:?}",
        method,
        uri,
        response.status(),
        elapsed
    );

    response
}
*/

// Index page handler.
async fn index_handler(State(state): State<AppState>) -> impl IntoResponse {
    state.increment_metric(MetricType::RequestTotal).await;
    state.increment_metric(MetricType::RequestSuccess).await;

    Html(r#"
<!DOCTYPE html>
<html>
<head>
    <title>Rustmax Suite - Developer Tools Hub</title>
    <style>
        body {
            font-family: system-ui, -apple-system, sans-serif;
            max-width: 1200px;
            margin: 0 auto;
            padding: 2rem;
            background: #f5f5f5;
        }
        h1 {
            color: #333;
            border-bottom: 2px solid #ff6b35;
            padding-bottom: 0.5rem;
        }
        .section {
            background: white;
            border-radius: 8px;
            padding: 1.5rem;
            margin: 1rem 0;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        .endpoint {
            background: #f8f9fa;
            border-left: 3px solid #007bff;
            padding: 0.5rem 1rem;
            margin: 0.5rem 0;
            font-family: monospace;
        }
        .status {
            display: inline-block;
            padding: 0.25rem 0.5rem;
            border-radius: 4px;
            background: #28a745;
            color: white;
            font-size: 0.875rem;
        }
    </style>
</head>
<body>
    <h1>🚀 Rustmax Suite - Developer Tools Hub</h1>

    <div class="section">
        <h2>Status</h2>
        <p><span class="status">ONLINE</span> Server is running</p>
        <p>Version: <code>v0.1.0</code></p>
    </div>

    <div class="section">
        <h2>Available Endpoints</h2>
        <div class="endpoint">GET /health - Health check</div>
        <div class="endpoint">GET /metrics - Application metrics</div>
        <div class="endpoint">POST /api/scan - Scan project directory</div>
        <div class="endpoint">POST /api/analyze - Analyze dependencies</div>
        <div class="endpoint">POST /api/test - Run tests</div>
        <div class="endpoint">POST /api/build - Build project</div>
        <div class="endpoint">POST /api/format - Format code</div>
        <div class="endpoint">GET /ws - WebSocket connection</div>
    </div>

    <div class="section">
        <h2>Quick Links</h2>
        <ul>
            <li><a href="/metrics">View Metrics</a></li>
            <li><a href="/health">Health Check</a></li>
        </ul>
    </div>

    <script>
        // Auto-refresh metrics every 5 seconds
        if (window.location.pathname === '/metrics') {
            setTimeout(() => window.location.reload(), 5000);
        }
    </script>
</body>
</html>
    "#)
}

// Health check handler.
async fn health_handler(State(state): State<AppState>) -> impl IntoResponse {
    state.increment_metric(MetricType::RequestTotal).await;
    state.increment_metric(MetricType::RequestSuccess).await;

    let uptime = state.uptime_secs().await;

    Json(rmx::serde_json::json!({
        "status": "healthy",
        "uptime_seconds": uptime,
        "version": crate::VERSION,
    }))
}

// Metrics handler.
async fn metrics_handler(State(state): State<AppState>) -> impl IntoResponse {
    state.increment_metric(MetricType::RequestTotal).await;
    state.increment_metric(MetricType::RequestSuccess).await;

    let metrics = state.metrics_snapshot().await;

    // Return as plain text for now to avoid serialization issues
    format!("Uptime: {}s, Requests: {}/{}/{}, Operations: {}, Cache: {}/{}",
           metrics.uptime_secs,
           metrics.requests_total,
           metrics.requests_success,
           metrics.requests_failed,
           metrics.operations_total,
           metrics.cache_hits,
           metrics.cache_misses)
}

// Project scan handler.
async fn scan_handler(
    State(state): State<AppState>,
    Json(payload): Json<ScanRequest>,
) -> impl IntoResponse {
    state.increment_metric(MetricType::RequestTotal).await;

    match crate::services::scanner::scan_directory(&payload.path, payload.max_depth, &state).await {
        Ok(_) => {
            state.increment_metric(MetricType::RequestSuccess).await;
            (StatusCode::OK, Json(rmx::serde_json::json!({
                "status": "success",
                "message": format!("Scanned directory: {}", payload.path)
            })))
        }
        Err(e) => {
            state.increment_metric(MetricType::RequestFailed).await;
            (StatusCode::INTERNAL_SERVER_ERROR, Json(rmx::serde_json::json!({
                "status": "error",
                "message": e.to_string()
            })))
        }
    }
}

// Dependency analysis handler.
async fn analyze_handler(
    State(state): State<AppState>,
    Json(payload): Json<AnalyzeRequest>,
) -> impl IntoResponse {
    state.increment_metric(MetricType::RequestTotal).await;

    match crate::services::analyzer::analyze_dependencies(&payload.path, payload.check_updates, &state).await {
        Ok(_) => {
            state.increment_metric(MetricType::RequestSuccess).await;
            (StatusCode::OK, Json(rmx::serde_json::json!({
                "status": "success",
                "message": format!("Analyzed: {}", payload.path)
            })))
        }
        Err(e) => {
            state.increment_metric(MetricType::RequestFailed).await;
            (StatusCode::INTERNAL_SERVER_ERROR, Json(rmx::serde_json::json!({
                "status": "error",
                "message": e.to_string()
            })))
        }
    }
}

// Test runner handler.
async fn test_handler(
    State(state): State<AppState>,
    Json(payload): Json<TestRequest>,
) -> impl IntoResponse {
    state.increment_metric(MetricType::RequestTotal).await;

    match crate::services::runner::run_tests(payload.pattern.as_deref(), payload.parallel, &state).await {
        Ok(_) => {
            state.increment_metric(MetricType::RequestSuccess).await;
            (StatusCode::OK, Json(rmx::serde_json::json!({
                "status": "success",
                "message": "Tests completed"
            })))
        }
        Err(e) => {
            state.increment_metric(MetricType::RequestFailed).await;
            (StatusCode::INTERNAL_SERVER_ERROR, Json(rmx::serde_json::json!({
                "status": "error",
                "message": e.to_string()
            })))
        }
    }
}

// Build handler.
async fn build_handler(
    State(state): State<AppState>,
    Json(payload): Json<BuildRequest>,
) -> impl IntoResponse {
    state.increment_metric(MetricType::RequestTotal).await;

    match crate::services::builder::build_project(payload.release, payload.features, &state).await {
        Ok(_) => {
            state.increment_metric(MetricType::RequestSuccess).await;
            (StatusCode::OK, Json(rmx::serde_json::json!({
                "status": "success",
                "message": "Build completed"
            })))
        }
        Err(e) => {
            state.increment_metric(MetricType::RequestFailed).await;
            (StatusCode::INTERNAL_SERVER_ERROR, Json(rmx::serde_json::json!({
                "status": "error",
                "message": e.to_string()
            })))
        }
    }
}

// Format handler.
async fn format_handler(
    State(state): State<AppState>,
    Json(payload): Json<FormatRequest>,
) -> impl IntoResponse {
    state.increment_metric(MetricType::RequestTotal).await;

    match crate::services::formatter::format_code(&payload.path, payload.check, &state).await {
        Ok(_) => {
            state.increment_metric(MetricType::RequestSuccess).await;
            (StatusCode::OK, Json(rmx::serde_json::json!({
                "status": "success",
                "message": format!("Formatted: {}", payload.path)
            })))
        }
        Err(e) => {
            state.increment_metric(MetricType::RequestFailed).await;
            (StatusCode::INTERNAL_SERVER_ERROR, Json(rmx::serde_json::json!({
                "status": "error",
                "message": e.to_string()
            })))
        }
    }
}

// Static file handler.
async fn static_handler(Path(path): Path<String>) -> impl IntoResponse {
    // In a real application, serve static files from a directory.
    // For now, return a simple 404.
    (StatusCode::NOT_FOUND, "Static file not found")
}

// Request types.
#[derive(Debug)]
struct ScanRequest {
    path: String,
    max_depth: Option<usize>,
}

#[derive(Debug)]
struct AnalyzeRequest {
    path: String,
    check_updates: bool,
}

#[derive(Debug)]
struct TestRequest {
    pattern: Option<String>,
    parallel: bool,
}

#[derive(Debug)]
struct BuildRequest {
    release: bool,
    features: Vec<String>,
}

#[derive(Debug)]
struct FormatRequest {
    path: String,
    check: bool,
}