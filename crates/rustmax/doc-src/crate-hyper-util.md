Runtime glue, a pooling client, and server helpers for [`hyper`].

- Crate [`::hyper_util`].
- [docs.rs](https://docs.rs/hyper-util)
- [crates.io](https://crates.io/crates/hyper-util)
- [GitHub](https://github.com/hyperium/hyper-util)

---

[`hyper`] 1.x is runtime-agnostic:
it reads and writes through its own I/O traits
and spawns tasks through an executor trait,
but provides no implementations of either.
`hyper-util` provides them for [`tokio`],
along with the higher-level pieces that hyper 1.0
moved out of its core.

The parts used most:

- [`TokioIo`] - adapts a `tokio` socket to hyper's I/O traits
- [`TokioExecutor`] - spawns hyper's background tasks on `tokio`
- [`Client`] - a connection-pooling HTTP client
- [`auto::Builder`] - serves a connection as HTTP/1 or HTTP/2,
  whichever the client speaks
- [`TowerToHyperService`] - runs a [`tower`] service as a hyper service

Most applications should use [`reqwest`] as a client,
which is built on `hyper-util`.
Use `hyper-util` directly alongside `hyper`
when building servers or custom clients.
Message bodies come from [`http_body_util`].

## Examples

Serving a connection with `hyper` and requesting it with [`Client`]:

```rust
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::{Request, Response, body::Incoming, server::conn::http1, service::service_fn};
use hyper_util::client::legacy::Client;
use hyper_util::rt::{TokioExecutor, TokioIo};
use std::convert::Infallible;
use tokio::net::TcpListener;

async fn hello(_req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("hello"))))
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        http1::Builder::new()
            .serve_connection(TokioIo::new(stream), service_fn(hello))
            .await
            .unwrap();
    });

    let client = Client::builder(TokioExecutor::new()).build_http::<Full<Bytes>>();
    let uri = format!("http://{addr}/").parse().unwrap();
    let response = client.get(uri).await.unwrap();
    let body = response.into_body().collect().await.unwrap().to_bytes();

    assert_eq!(body, "hello");
}
```


[`hyper`]: crate::hyper
[`tokio`]: crate::tokio
[`tower`]: crate::tower
[`reqwest`]: crate::reqwest
[`http_body_util`]: crate::http_body_util
[`TokioIo`]: crate::hyper_util::rt::TokioIo
[`TokioExecutor`]: crate::hyper_util::rt::TokioExecutor
[`Client`]: crate::hyper_util::client::legacy::Client
[`auto::Builder`]: crate::hyper_util::server::conn::auto::Builder
[`TowerToHyperService`]: crate::hyper_util::service::TowerToHyperService
