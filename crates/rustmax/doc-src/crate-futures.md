Async programming primitives and utilities.

- Crate [`::futures`].
- [docs.rs](https://docs.rs/futures)
- [crates.io](https://crates.io/crates/futures)
- [GitHub](https://github.com/rust-lang/futures-rs)

---

`futures` provides fundamental traits and utilities for asynchronous programming in Rust.
It defines the core [`Future`] trait and provides essential async utilities
that work with both the standard library and async runtimes like Tokio.

The crate includes several key modules:
[`future`] for working with individual futures,
[`stream`] for handling asynchronous streams of data,
[`sink`] for asynchronous data consumption,
[`executor`] for running futures to completion,
and [`channel`] for async communication primitives.

Key traits include [`Future`] for asynchronous computations,
[`Stream`] for asynchronous iterators,
and [`Sink`] for asynchronous data receivers.
The library also provides combinators for chaining and transforming futures.

The channel module provides essential async communication building blocks:
[`mpsc`] channels for multiple-producer, single-consumer communication,
and [`oneshot`] channels for one-time value passing between tasks.
These are runtime-agnostic and work across different async executors.

This crate serves as the foundation for async Rust,
providing compatibility between different async runtimes
and offering low-level building blocks for async applications.

## Examples

Working with futures and combinators:

```rust
use futures::{future, executor::block_on};

// Create simple futures
let fut1 = future::ready(42);
let fut2 = future::ready("hello");

// Combine futures
let combined = future::join(fut1, fut2);

// Execute the future
let (num, text) = block_on(combined);
assert_eq!(num, 42);
assert_eq!(text, "hello");
```

Using async streams:

```rust
use futures::{stream, StreamExt, executor::block_on};

// Create a stream of numbers
let stream = stream::iter(0..5);

// Transform the stream
let doubled_stream = stream.map(|x| x * 2);
let doubled: Vec<_> = block_on(doubled_stream.collect());

assert_eq!(doubled, vec![0, 2, 4, 6, 8]);
```

Using mpsc channels for async communication:

```rust
use futures::{channel::mpsc, SinkExt, StreamExt, executor::block_on};

let (mut sender, mut receiver) = mpsc::channel::<i32>(10);

// Send some values
block_on(async {
    sender.send(1).await.unwrap();
    sender.send(2).await.unwrap();
    sender.send(3).await.unwrap();
    drop(sender); // Close the channel
});

// Receive values
let received: Vec<i32> = block_on(receiver.collect());
assert_eq!(received, vec![1, 2, 3]);
```

Using oneshot channels for single-value communication:

```rust
use futures::{channel::oneshot, executor::block_on};

let (sender, receiver) = oneshot::channel::<String>();

// Send a value
sender.send("Hello from oneshot!".to_string()).unwrap();

// Receive the value
let message = block_on(receiver).unwrap();
assert_eq!(message, "Hello from oneshot!");
```

[`Future`]: crate::futures::Future
[`Stream`]: crate::futures::Stream
[`Sink`]: crate::futures::Sink
[`future`]: crate::futures::future
[`stream`]: crate::futures::stream
[`sink`]: crate::futures::sink
[`executor`]: crate::futures::executor
[`channel`]: crate::futures::channel
[`mpsc`]: crate::futures::channel::mpsc
[`oneshot`]: crate::futures::channel::oneshot