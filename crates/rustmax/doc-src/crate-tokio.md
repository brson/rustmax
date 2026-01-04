Asynchronous runtime for writing reliable network applications.

- Crate [`::tokio`].
- [docs.rs](https://docs.rs/tokio)
- [crates.io](https://crates.io/crates/tokio)
- [GitHub](https://github.com/tokio-rs/tokio)

---

`tokio` is an asynchronous runtime for the Rust programming language.
It provides the building blocks needed for writing network applications
without compromising speed.

At a high level, Tokio provides:
- A multi-threaded, work-stealing based task scheduler
- A reactor backed by the operating system's event queue (epoll, kqueue, IOCP, etc.)
- Asynchronous TCP and UDP sockets
- Asynchronous filesystem operations
- Timer and timeout utilities

Tokio is built using the async/await syntax introduced in Rust 1.39.
The runtime schedules asynchronous tasks cooperatively,
allowing thousands of tasks to run concurrently on a small number of threads.

The [`Runtime`] provides the execution environment for async code.
Most applications will use the [`tokio::main`] macro to set up the runtime,
though it can also be constructed manually for more control.

Tokio's networking primitives like [`TcpListener`] and [`TcpStream`]
are designed to work seamlessly with async/await,
making it straightforward to write servers and clients.

## Examples

Creating a simple TCP echo server:

```rust,no_run
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            handle_client(socket).await;
        });
    }
}

async fn handle_client(mut socket: TcpStream) {
    let mut buf = [0; 1024];

    loop {
        match socket.read(&mut buf).await {
            Ok(0) => break, // Connection closed
            Ok(n) => {
                if socket.write_all(&buf[0..n]).await.is_err() {
                    break;
                }
            }
            Err(_) => break,
        }
    }
}
```

Using timers and timeouts:

```rust
use tokio::time::{sleep, timeout, Duration};

#[tokio::main]
async fn main() {
    // Sleep briefly
    sleep(Duration::from_millis(10)).await;

    // Timeout an operation
    let result = timeout(Duration::from_millis(50), slow_operation()).await;

    match result {
        Ok(value) => println!("Operation completed: {:?}", value),
        Err(_) => println!("Operation timed out"),
    }
}

async fn slow_operation() -> &'static str {
    sleep(Duration::from_millis(100)).await;
    "done"
}
```

Spawning concurrent tasks:

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    let handle1 = task::spawn(async {
        // Do some work
        println!("Task 1 completed");
        42
    });

    let handle2 = task::spawn(async {
        // Do some other work
        println!("Task 2 completed");
        "hello"
    });

    // Wait for both tasks to complete
    let (result1, result2) = tokio::join!(handle1, handle2);

    println!("Results: {:?}, {:?}", result1.unwrap(), result2.unwrap());
}
```

[`Runtime`]: crate::tokio::runtime::Runtime
[`tokio::main`]: crate::tokio::main
[`TcpListener`]: crate::tokio::net::TcpListener
[`TcpStream`]: crate::tokio::net::TcpStream