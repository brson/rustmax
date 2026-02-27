Low-level HTTP/1 and HTTP/2 implementation for the [`tokio`] ecosystem.

- Crate [`::hyper`].
- [docs.rs](https://docs.rs/hyper)
- [crates.io](https://crates.io/crates/hyper)
- [GitHub](https://github.com/hyperium/hyper)

---

`hyper` is the HTTP implementation underlying [`axum`], [`reqwest`],
and other Rust HTTP libraries.
Most applications should use those higher-level crates;
use `hyper` directly when you need fine-grained control over HTTP connections
or are building your own HTTP framework.

Core types (re-exported from the [`http`] crate):

- [`Request`] / [`Response`] - HTTP request and response, generic over body type
- [`Method`] - HTTP methods (GET, POST, etc.)
- [`StatusCode`] - HTTP status codes (200, 404, etc.)
- [`Uri`] - request URIs
- [`HeaderMap`] - HTTP header collection

## Examples

Working with HTTP request and response types:

```rust
use hyper::{Request, Response, Method, StatusCode};

let req = Request::builder()
    .method(Method::POST)
    .uri("https://example.com/api")
    .header("Content-Type", "application/json")
    .body(())
    .unwrap();

assert_eq!(req.method(), Method::POST);
assert_eq!(req.uri(), "https://example.com/api");

let resp = Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", "text/plain")
    .body(())
    .unwrap();

assert_eq!(resp.status(), StatusCode::OK);
```

[`tokio`]: crate::tokio
[`axum`]: crate::axum
[`reqwest`]: crate::reqwest
[`http`]: crate::http
[`Request`]: crate::hyper::Request
[`Response`]: crate::hyper::Response
[`Method`]: crate::hyper::Method
[`StatusCode`]: crate::hyper::StatusCode
[`Uri`]: crate::hyper::Uri
[`HeaderMap`]: crate::hyper::HeaderMap
