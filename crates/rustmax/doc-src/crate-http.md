Shared type definitions for the HTTP protocol.

- Crate [`::http`].
- [docs.rs](https://docs.rs/http)
- [crates.io](https://crates.io/crates/http)
- [GitHub](https://github.com/hyperium/http)

---

`http` provides the common vocabulary types for HTTP
used across the Rust ecosystem.
[`hyper`], [`axum`], [`reqwest`], and other HTTP libraries
all build on these types, making them interoperable.

This crate defines types only, with no I/O or protocol logic.

- [`Request`] / [`Response`] - HTTP messages, generic over body type
- [`Method`] - HTTP methods (GET, POST, PUT, DELETE, etc.)
- [`StatusCode`] - HTTP status codes (200, 404, 500, etc.)
- [`Uri`] - request URIs with access to scheme, authority, path, and query
- [`HeaderMap`] - efficient multi-map of header name-value pairs
- [`HeaderName`] / [`HeaderValue`] - typed header components

## Examples

Building a request and inspecting its parts:

```rust
use http::{Request, Method, StatusCode, Response};

let req = Request::builder()
    .method(Method::GET)
    .uri("https://example.com/path?q=1")
    .header("Accept", "application/json")
    .body(())
    .unwrap();

assert_eq!(req.method(), Method::GET);
assert_eq!(req.uri().path(), "/path");
assert_eq!(req.uri().query(), Some("q=1"));
assert_eq!(
    req.headers()["Accept"],
    "application/json",
);

let resp = Response::builder()
    .status(StatusCode::NOT_FOUND)
    .header("Content-Type", "text/plain")
    .body(())
    .unwrap();

assert_eq!(resp.status(), StatusCode::NOT_FOUND);
assert_eq!(resp.status().as_u16(), 404);
```

[`hyper`]: crate::hyper
[`axum`]: crate::axum
[`reqwest`]: crate::reqwest
[`Request`]: crate::http::Request
[`Response`]: crate::http::Response
[`Method`]: crate::http::Method
[`StatusCode`]: crate::http::StatusCode
[`Uri`]: crate::http::Uri
[`HeaderMap`]: crate::http::HeaderMap
[`HeaderName`]: crate::http::HeaderName
[`HeaderValue`]: crate::http::HeaderValue
