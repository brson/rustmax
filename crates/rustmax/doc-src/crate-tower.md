Modular middleware framework for async request/response services.

- Crate [`::tower`].
- [docs.rs](https://docs.rs/tower)
- [crates.io](https://crates.io/crates/tower)
- [GitHub](https://github.com/tower-rs/tower)

---

`tower` defines the [`Service`] and [`Layer`] traits
that form the middleware model for [`axum`], [`hyper`], and the broader [`tokio`] ecosystem.
A [`Service`] is an async function from request to response;
a [`Layer`] wraps a service to add behavior like timeouts, rate limiting, retries, etc.

Built-in middleware:

- [`timeout`] - fails requests exceeding a duration
- [`limit`] - rate limiting and concurrency limiting
- [`retry`] - retries failed requests per a policy
- [`buffer`] - adds a cloneable mpsc queue in front of a service
- [`load_shed`] - rejects requests immediately when the inner service is not ready

In practice, most `axum` users encounter tower through
[`axum::Router::layer`] and [`ServiceBuilder`]
when adding middleware to routes.

## Examples

Building a service with stacked middleware using [`ServiceBuilder`]:

```rust,no_run
use tower::{ServiceBuilder, ServiceExt};
use tower::timeout::TimeoutLayer;
use tower::limit::RateLimitLayer;
use std::time::Duration;
use std::convert::Infallible;

// A simple async service function
async fn handle(request: String) -> Result<String, Infallible> {
    Ok(format!("Hello, {request}!"))
}

#[tokio::main]
async fn main() {
    let service = ServiceBuilder::new()
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .layer(RateLimitLayer::new(100, Duration::from_secs(1)))
        .service_fn(handle);
}
```

Creating a service from an async function with [`service_fn`]:

```rust
use tower::{Service, ServiceExt, service_fn};
use std::convert::Infallible;

async fn my_service(req: String) -> Result<usize, Infallible> {
    Ok(req.len())
}

#[tokio::main]
async fn main() {
    let mut svc = service_fn(my_service);

    let response = svc
        .ready()
        .await
        .unwrap()
        .call("hello".to_string())
        .await
        .unwrap();

    assert_eq!(response, 5);
}
```

[`axum`]: crate::axum
[`hyper`]: crate::hyper
[`tokio`]: crate::tokio
[`Service`]: crate::tower::Service
[`Layer`]: crate::tower::Layer
[`ServiceExt`]: crate::tower::ServiceExt
[`ServiceBuilder`]: crate::tower::ServiceBuilder
[`service_fn`]: crate::tower::service_fn
[`timeout`]: crate::tower::timeout
[`limit`]: crate::tower::limit
[`retry`]: crate::tower::retry
[`buffer`]: crate::tower::buffer
[`load_shed`]: crate::tower::load_shed
[`axum::Router::layer`]: crate::axum::Router::layer
