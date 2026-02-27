Web application framework built on [`tokio`] and [`hyper`].

- Crate [`::axum`].
- [docs.rs](https://docs.rs/axum)
- [crates.io](https://crates.io/crates/axum)
- [GitHub](https://github.com/tokio-rs/axum)

---

`axum` is a web framework from the [`tokio`] project.
It follows a typical router + middleware design.
It is the preeminent web framework through its association
with `tokio`. The API is async and highly generic,
advanced Rust.

- Handlers are plain async functions, not trait impls
- Request data is accessed through extractor types like [`Path`], [`Query`], and [`Json`]
- Responses are any type that implements [`IntoResponse`]
- Middleware uses the [`tower`] `Service` trait,
  composable with the wider Tower ecosystem
- Shared application state is passed via [`State`]

## Examples

A minimal server:

```rust,no_run
use axum::{Router, routing::get, extract::Path};
use tokio::net::TcpListener;

async fn hello() -> &'static str {
    "Hello, World!"
}

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {name}!")
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/greet/{name}", get(greet));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

Handling JSON request and response bodies:

```rust,no_run
use axum::{Router, routing::post, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateItem {
    name: String,
}

#[derive(Serialize)]
struct Item {
    id: u64,
    name: String,
}

async fn create_item(Json(input): Json<CreateItem>) -> Json<Item> {
    Json(Item { id: 1, name: input.name })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/items", post(create_item));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

[`tokio`]: crate::tokio
[`hyper`]: crate::hyper
[`tower`]: crate::tower
[`Path`]: crate::axum::extract::Path
[`Query`]: crate::axum::extract::Query
[`Json`]: crate::axum::Json
[`State`]: crate::axum::extract::State
[`IntoResponse`]: crate::axum::response::IntoResponse
