//! `folio serve`: serve the built site locally, rebuilding and reloading as it changes.
//!
//! The server is `hyper` 1, with `hyper-util`'s connection builder speaking
//! HTTP/1 and HTTP/2, on a `tokio` runtime. Rebuilding is `folio watch`'s loop,
//! run on its own thread.
//!
//! Live reload is a server-sent event stream at [`RELOAD_PATH`], which sends
//! `reload` after every successful build. HTML pages get a one-line script
//! that listens to it, added as they are served, so the files on disk stay
//! exactly what `folio build` writes.

use rmx::bytes::Bytes;
use rmx::crossbeam::channel::{self, Receiver};
use rmx::futures::StreamExt;
use rmx::http::{Method, Request, Response, StatusCode, header};
use rmx::http_body_util::{BodyExt, Full, StreamBody, combinators::BoxBody};
use rmx::hyper::body::{Frame, Incoming};
use rmx::hyper::service::service_fn;
use rmx::hyper_util::rt::{TokioExecutor, TokioIo};
use rmx::hyper_util::server::conn::auto;
use rmx::log::{debug, info};
use rmx::mime::{self, Mime};
use rmx::prelude::*;
use rmx::tokio::{self, net::TcpListener, sync::watch};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use crate::collection::Config;
use crate::watch::WatchOptions;
use crate::Result;

/// Where pages listen for rebuilds.
pub const RELOAD_PATH: &str = "/_folio/reload";

/// Added to the end of every HTML page served.
const RELOAD_SCRIPT: &str =
    r#"<script>new EventSource("/_folio/reload").onmessage = () => location.reload();</script>"#;

/// How to serve a collection.
#[derive(Debug, Clone)]
pub struct ServeOptions {
    pub watch: WatchOptions,
    /// Address to listen on. Port 0 picks a free one.
    pub addr: SocketAddr,
}

type Body = BoxBody<Bytes, Infallible>;

/// What every request handler needs.
struct Site {
    output_dir: PathBuf,
    /// Counts successful builds. Reload streams wait for it to change.
    builds: watch::Receiver<u64>,
}

/// Serve a collection until Ctrl-C.
pub fn serve(root: &Path, options: &ServeOptions) -> Result<()> {
    let (stop_tx, stop_rx) = channel::bounded(1);
    rmx::ctrlc::set_handler(move || {
        let _ = stop_tx.try_send(());
    })?;

    serve_until(root, options, stop_rx, |addr| info!("Serving http://{addr}/"))?;
    info!("Stopped serving");
    Ok(())
}

/// Serve a collection until `stop` receives.
///
/// `ready` is called with the bound address once the server is listening,
/// before the first build.
pub fn serve_until(
    root: &Path,
    options: &ServeOptions,
    stop: Receiver<()>,
    ready: impl FnOnce(SocketAddr),
) -> Result<()> {
    let config = Config::load(root)?;
    let output_dir = options
        .watch
        .output
        .clone()
        .unwrap_or_else(|| root.join(&config.build.output_dir));

    let runtime = tokio::runtime::Runtime::new()?;
    let listener = runtime.block_on(TcpListener::bind(options.addr))?;
    ready(listener.local_addr()?);

    let (builds_tx, builds_rx) = watch::channel(0u64);
    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let site = Arc::new(Site { output_dir, builds: builds_rx });
    let server = runtime.spawn(accept(listener, site, shutdown_rx));

    // Blocks this thread until `stop`, rebuilding as things change.
    let watched = crate::watch::watch_until(root, &options.watch, stop, || {
        builds_tx.send_modify(|count| *count += 1);
    });

    // Dropping the sender ends every reload stream, so their connections close.
    drop(builds_tx);
    // The receiver is gone only if the server has already stopped.
    let _ = shutdown_tx.send(true);
    runtime.block_on(server).expect("the accept loop does not panic")?;
    watched
}

async fn accept(listener: TcpListener, site: Arc<Site>, mut shutdown: watch::Receiver<bool>) -> Result<()> {
    loop {
        let (stream, peer) = tokio::select! {
            accepted = listener.accept() => accepted?,
            _ = shutdown.changed() => return Ok(()),
        };

        let site = site.clone();
        tokio::spawn(async move {
            let service = service_fn(move |request| respond(site.clone(), request));
            if let Err(e) = auto::Builder::new(TokioExecutor::new())
                .serve_connection(TokioIo::new(stream), service)
                .await
            {
                // A browser closing a connection mid-response is routine.
                debug!("connection from {peer}: {e}");
            }
        });
    }
}

async fn respond(site: Arc<Site>, request: Request<Incoming>) -> std::result::Result<Response<Body>, Infallible> {
    if request.method() != Method::GET && request.method() != Method::HEAD {
        return Ok(text(StatusCode::METHOD_NOT_ALLOWED, "Only GET and HEAD are served."));
    }

    let path = request.uri().path();
    debug!("{} {path}", request.method());
    if path == RELOAD_PATH {
        return Ok(reload_stream(site.builds.clone()));
    }

    Ok(match locate(&site.output_dir, path) {
        Located::File(file) => match tokio::fs::read(&file).await {
            Ok(contents) => file_response(&file, contents),
            Err(_) => not_found(&site.output_dir).await,
        },
        Located::Directory => {
            let location = format!("{path}/");
            let mut response = text(StatusCode::MOVED_PERMANENTLY, "");
            response.headers_mut().insert(header::LOCATION, location.parse().expect("a path is a valid header"));
            response
        }
        Located::Forbidden => text(StatusCode::BAD_REQUEST, "Paths may not leave the site."),
        Located::Missing => not_found(&site.output_dir).await,
    })
}

/// What a request path names in the output directory.
#[derive(Debug, PartialEq, Eq)]
pub enum Located {
    File(PathBuf),
    /// A directory named without its trailing slash.
    ///
    /// Pages link relatively, so they have to be served from the slash form.
    Directory,
    /// A path that would leave the output directory.
    Forbidden,
    Missing,
}

/// Find the file a request path names, serving a directory's `index.html`.
pub fn locate(output_dir: &Path, path: &str) -> Located {
    let decoded = crate::site::percent_decode(path);
    let relative = Path::new(decoded.trim_start_matches('/'));
    if !relative.components().all(|c| matches!(c, Component::Normal(_))) {
        return Located::Forbidden;
    }

    let target = output_dir.join(relative);
    if target.is_file() {
        Located::File(target)
    } else if target.is_dir() && target.join("index.html").is_file() {
        if decoded.ends_with('/') {
            Located::File(target.join("index.html"))
        } else {
            Located::Directory
        }
    } else {
        Located::Missing
    }
}

/// The content type to serve a file with, from its extension.
pub fn content_type(path: &Path) -> Mime {
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    match extension.as_str() {
        "html" => mime::TEXT_HTML_UTF_8,
        "css" => mime::TEXT_CSS_UTF_8,
        "js" => mime::APPLICATION_JAVASCRIPT_UTF_8,
        "json" => mime::APPLICATION_JSON,
        "xml" => mime::TEXT_XML,
        "txt" | "md" => mime::TEXT_PLAIN_UTF_8,
        "svg" => mime::IMAGE_SVG,
        "png" => mime::IMAGE_PNG,
        "jpg" | "jpeg" => mime::IMAGE_JPEG,
        "gif" => mime::IMAGE_GIF,
        "webp" => "image/webp".parse().expect("valid MIME type"),
        "woff2" => "font/woff2".parse().expect("valid MIME type"),
        "epub" => "application/epub+zip".parse().expect("valid MIME type"),
        _ => mime::APPLICATION_OCTET_STREAM,
    }
}

fn file_response(file: &Path, contents: Vec<u8>) -> Response<Body> {
    let mime = content_type(file);
    let body = if mime == mime::TEXT_HTML_UTF_8 {
        with_reload_script(contents)
    } else {
        contents
    };

    let mut response = Response::new(Full::new(Bytes::from(body)).boxed());
    let headers = response.headers_mut();
    headers.insert(header::CONTENT_TYPE, mime.as_ref().parse().expect("a MIME type is a valid header"));
    // Every response is from the latest build; a cached one would not be.
    headers.insert(header::CACHE_CONTROL, "no-store".parse().expect("valid header"));
    response
}

/// Add the live-reload script to an HTML page, before `</body>` if it has one.
pub fn with_reload_script(mut html: Vec<u8>) -> Vec<u8> {
    let at = rmx::memchr::memmem::rfind(&html, b"</body>").unwrap_or(html.len());
    html.splice(at..at, RELOAD_SCRIPT.bytes());
    html
}

/// A stream that sends `reload` every time a build finishes.
fn reload_stream(mut builds: watch::Receiver<u64>) -> Response<Body> {
    // Only builds after the page connected matter.
    builds.mark_unchanged();

    let first = Bytes::from_static(b": connected\n\n");
    let events = rmx::futures::stream::once(async move { first }).chain(rmx::futures::stream::unfold(
        builds,
        |mut builds| async move {
            builds.changed().await.ok()?;
            Some((Bytes::from_static(b"data: reload\n\n"), builds))
        },
    ));
    let body = StreamBody::new(events.map(|bytes| Ok(Frame::data(bytes))));

    let mut response = Response::new(BodyExt::boxed(body));
    let headers = response.headers_mut();
    headers.insert(header::CONTENT_TYPE, "text/event-stream".parse().expect("valid header"));
    headers.insert(header::CACHE_CONTROL, "no-store".parse().expect("valid header"));
    response
}

async fn not_found(output_dir: &Path) -> Response<Body> {
    let page = output_dir.join("404.html");
    match tokio::fs::read(&page).await {
        Ok(contents) => {
            let mut response = file_response(&page, contents);
            *response.status_mut() = StatusCode::NOT_FOUND;
            response
        }
        Err(_) => text(StatusCode::NOT_FOUND, "Not found."),
    }
}

fn text(status: StatusCode, message: &'static str) -> Response<Body> {
    let mut response = Response::new(Full::new(Bytes::from_static(message.as_bytes())).boxed());
    *response.status_mut() = status;
    response
        .headers_mut()
        .insert(header::CONTENT_TYPE, mime::TEXT_PLAIN_UTF_8.as_ref().parse().expect("valid header"));
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmx::tempfile::tempdir;
    use std::fs;

    #[test]
    fn paths_locate_files_and_directory_indexes() {
        let dir = tempdir().unwrap();
        let out = dir.path();
        fs::create_dir_all(out.join("post")).unwrap();
        fs::write(out.join("index.html"), "home").unwrap();
        fs::write(out.join("post/index.html"), "post").unwrap();
        fs::write(out.join("a b.css"), "css").unwrap();

        assert_eq!(locate(out, "/"), Located::File(out.join("index.html")));
        assert_eq!(locate(out, "/post/"), Located::File(out.join("post/index.html")));
        assert_eq!(locate(out, "/post"), Located::Directory);
        assert_eq!(locate(out, "/a%20b.css"), Located::File(out.join("a b.css")));
        assert_eq!(locate(out, "/nope/"), Located::Missing);
    }

    #[test]
    fn paths_cannot_leave_the_output_directory() {
        let dir = tempdir().unwrap();
        assert_eq!(locate(dir.path(), "/../secret"), Located::Forbidden);
        assert_eq!(locate(dir.path(), "/post/%2e%2e/%2e%2e/secret"), Located::Forbidden);
    }

    #[test]
    fn the_reload_script_goes_before_the_end_of_the_body() {
        let html = with_reload_script(b"<html><body><p>x</p></body></html>".to_vec());
        let html = String::from_utf8(html).unwrap();
        assert!(html.ends_with(&format!("{RELOAD_SCRIPT}</body></html>")), "{html}");

        let fragment = String::from_utf8(with_reload_script(b"<p>x</p>".to_vec())).unwrap();
        assert!(fragment.ends_with(RELOAD_SCRIPT));
    }

    #[test]
    fn content_types_follow_the_extension() {
        assert_eq!(content_type(Path::new("a/index.html")), mime::TEXT_HTML_UTF_8);
        assert_eq!(content_type(Path::new("search.js")), mime::APPLICATION_JAVASCRIPT_UTF_8);
        assert_eq!(content_type(Path::new("photo.JPG")), mime::IMAGE_JPEG);
        assert_eq!(content_type(Path::new("feed.json")), mime::APPLICATION_JSON);
        assert_eq!(content_type(Path::new("archive.bin")), mime::APPLICATION_OCTET_STREAM);
    }
}
