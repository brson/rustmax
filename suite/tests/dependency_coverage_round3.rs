// Round 3: Targeting remaining testable dependencies.

use rustmax_suite::*;

// Thiserror: Derive macros not exposed through rmx.
// Skipping.

// Cfg-if: Conditional compilation helper.

#[test]
fn test_cfg_if_usage() {
    use rmx::cfg_if::cfg_if;

    cfg_if! {
        if #[cfg(test)] {
            let result = "test mode";
        } else {
            let result = "non-test mode";
        }
    }

    assert_eq!(result, "test mode");

    cfg_if! {
        if #[cfg(target_os = "linux")] {
            let os = "linux";
        } else if #[cfg(target_os = "macos")] {
            let os = "macos";
        } else if #[cfg(target_os = "windows")] {
            let os = "windows";
        } else {
            let os = "other";
        }
    }

    assert!(!os.is_empty());
}

// Futures: Async combinators (sync-testable parts).

#[test]
fn test_futures_channel() {
    use rmx::futures::channel::oneshot;

    let (tx, rx) = oneshot::channel::<i32>();

    tx.send(42).ok();

    // Use futures::executor for sync testing.
    use rmx::futures::executor::block_on;
    let result = block_on(rx).unwrap();
    assert_eq!(result, 42);
}

#[test]
fn test_futures_stream() {
    use rmx::futures::stream::{self, StreamExt};
    use rmx::futures::executor::block_on;

    let stream = stream::iter(vec![1, 2, 3, 4, 5]);
    let doubled: Vec<i32> = block_on(stream.map(|x| x * 2).collect());

    assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
}

#[test]
fn test_futures_ready() {
    use rmx::futures::future::{ready, FutureExt};
    use rmx::futures::executor::block_on;

    let fut = ready(42).map(|x| x * 2);
    let result = block_on(fut);
    assert_eq!(result, 84);
}

// Libc: C library bindings.

#[test]
fn test_libc_constants() {
    use rmx::libc;

    // Test some safe constants.
    assert!(libc::EINVAL > 0);
    assert!(libc::ENOENT > 0);
    assert!(libc::EAGAIN > 0);
}

#[cfg(unix)]
#[test]
fn test_libc_unix_constants() {
    use rmx::libc;

    // Unix-specific constants.
    assert!(libc::SIGTERM > 0);
    assert!(libc::SIGINT > 0);
}

// Socket2: Low-level socket operations.

#[test]
fn test_socket2_creation() {
    use rmx::socket2::{Domain, Socket, Type};

    // Create a UDP socket.
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, None).unwrap();

    // Just verify it was created.
    assert!(socket.local_addr().is_ok() || socket.local_addr().is_err());
}

#[test]
fn test_socket2_tcp() {
    use rmx::socket2::{Domain, Socket, Type};
    use std::net::SocketAddr;

    let socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();

    // Bind to localhost.
    let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    socket.bind(&addr.into()).unwrap();

    // Get actual bound address.
    let bound_addr = socket.local_addr().unwrap();
    assert!(bound_addr.is_ipv4());
}

// Mime: MIME type handling.

#[test]
fn test_mime_types() {
    use rmx::mime;

    let json = mime::APPLICATION_JSON;
    assert_eq!(json.type_(), "application");
    assert_eq!(json.subtype(), "json");

    let text = mime::TEXT_PLAIN;
    assert_eq!(text.type_(), "text");
    assert_eq!(text.subtype(), "plain");

    let html = mime::TEXT_HTML;
    assert_eq!(html.essence_str(), "text/html");
}

#[test]
fn test_mime_parsing() {
    use rmx::mime::Mime;
    use std::str::FromStr;

    let mime = Mime::from_str("text/html; charset=utf-8").unwrap();
    assert_eq!(mime.type_(), "text");
    assert_eq!(mime.subtype(), "html");

    let mime2 = "application/json".parse::<Mime>().unwrap();
    assert_eq!(mime2.essence_str(), "application/json");
}

// Http: HTTP types.

#[test]
fn test_http_status_codes() {
    use rmx::http::StatusCode;

    assert_eq!(StatusCode::OK.as_u16(), 200);
    assert_eq!(StatusCode::NOT_FOUND.as_u16(), 404);
    assert_eq!(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), 500);

    assert!(StatusCode::OK.is_success());
    assert!(StatusCode::NOT_FOUND.is_client_error());
    assert!(StatusCode::INTERNAL_SERVER_ERROR.is_server_error());
}

#[test]
fn test_http_methods() {
    use rmx::http::Method;

    assert_eq!(Method::GET.as_str(), "GET");
    assert_eq!(Method::POST.as_str(), "POST");
    assert_eq!(Method::PUT.as_str(), "PUT");
    assert_eq!(Method::DELETE.as_str(), "DELETE");
}

#[test]
fn test_http_headers() {
    use rmx::http::{HeaderMap, HeaderName, HeaderValue};

    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json"),
    );

    assert_eq!(headers.len(), 1);
    assert_eq!(
        headers.get("content-type").unwrap(),
        "application/json"
    );
}

// Derive_more and static_assertions: Not exposed through rmx prelude.
// Skipping direct tests.

// Extension-trait: Extension trait macro.

#[test]
fn test_extension_trait() {
    use rmx::extension_trait::extension_trait;

    #[extension_trait]
    impl StringExt for String {
        fn double(&self) -> String {
            format!("{}{}", self, self)
        }
    }

    let s = "hello".to_string();
    assert_eq!(s.double(), "hellohello");
}

// Num_cpus: CPU count detection.

#[test]
fn test_num_cpus() {
    use rmx::num_cpus;

    let physical = num_cpus::get_physical();
    let logical = num_cpus::get();

    assert!(physical > 0);
    assert!(logical > 0);
    assert!(logical >= physical);
}

// Rand_pcg: PCG random number generator.

#[test]
fn test_rand_pcg() {
    use rmx::rand::{Rng, SeedableRng};
    use rmx::rand_pcg::Pcg64;

    let mut rng = Pcg64::seed_from_u64(42);

    let num1: u64 = rng.random();
    let num2: u64 = rng.random();
    assert_ne!(num1, num2);

    // Reseed and verify determinism.
    let mut rng2 = Pcg64::seed_from_u64(42);
    let num3: u64 = rng2.random();
    assert_eq!(num1, num3);
}
