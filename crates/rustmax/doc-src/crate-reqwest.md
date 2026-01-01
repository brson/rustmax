HTTP client for making web requests.

- Crate [`::reqwest`].
- [docs.rs](https://docs.rs/reqwest)
- [crates.io](https://crates.io/crates/reqwest)
- [GitHub](https://github.com/seanmonstar/reqwest)

---

`reqwest` is a high-level HTTP client library built on top of [`hyper`].
It provides both asynchronous and blocking APIs for making HTTP requests,
with support for JSON, form data, cookies, proxies, and TLS.

The async API uses [`Client`] as the main entry point.
For quick one-off requests, convenience functions like [`get`] are available.
The blocking API lives in the [`blocking`] module.

Common features include:
- Automatic handling of redirects and cookies
- JSON serialization/deserialization via serde
- Request timeouts and connection pooling
- TLS/HTTPS support via native-tls or rustls
- Multipart form data for file uploads
- Streaming request and response bodies

## Examples

Making a simple GET request (blocking):

```rust,no_run
use reqwest::blocking;

let body = blocking::get("https://httpbin.org/get")
    .expect("request failed")
    .text()
    .expect("failed to read body");
println!("Response: {}", body);
```

Making an async GET request:

```rust,no_run
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let resp = client.get("https://httpbin.org/get")
        .send()
        .await?;

    println!("Status: {}", resp.status());
    println!("Body: {}", resp.text().await?);
    Ok(())
}
```

Posting JSON data:

```rust,no_run
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    name: String,
    count: u32,
}

#[derive(Deserialize, Debug)]
struct Response {
    json: Payload,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let payload = Payload {
        name: "test".to_string(),
        count: 42,
    };

    let resp: Response = client.post("https://httpbin.org/post")
        .json(&payload)
        .send()
        .await?
        .json()
        .await?;

    println!("Response: {:?}", resp);
    Ok(())
}
```

[`hyper`]: crate::hyper
[`Client`]: crate::reqwest::Client
[`get`]: crate::reqwest::get
[`blocking`]: crate::reqwest::blocking
