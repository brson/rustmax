Concurrency tools to supplement `std::sync`, including fast channels.

- Crate [`::crossbeam`].
- [docs.rs](https://docs.rs/crossbeam)
- [crates.io](https://crates.io/crates/crossbeam)
- [GitHub](https://github.com/crossbeam-rs/crossbeam)

---

`crossbeam` provides a collection of tools for concurrent programming in Rust.
It supplements the standard library's [`std::sync`] and [`std::thread`] modules
with high-performance primitives and convenient abstractions.

The crate's primary features include:
- Fast multi-producer multi-consumer channels via [`crossbeam::channel`]
- Scoped threads that can borrow from the parent scope via [`crossbeam::scope`]
- Lock-free and wait-free data structures
- Utilities like [`Backoff`] for optimizing spin loops
- Epoch-based memory reclamation for concurrent data structures

The [`channel`] module provides bounded and unbounded channels
that are faster than [`std::sync::mpsc`] and support multiple producers
and multiple consumers. Channels can be selected over using the [`select!`] macro,
enabling patterns like waiting on multiple channels or implementing timeouts.

The [`scope`] function allows spawning threads that can safely access
non-`'static` data from the parent thread's stack.
This is more ergonomic than [`std::thread::spawn`],
which requires all data to be `'static` or moved into the thread.

## Examples

Using scoped threads to borrow data from the parent scope:

```rust
use crossbeam;

let values = vec![1, 2, 3, 4];
let mut total = 0;

crossbeam::scope(|scope| {
    scope.spawn(|_| {
        // Can safely borrow from parent scope
        for &val in &values {
            println!("Processing: {}", val);
        }
    });
}).unwrap();

assert_eq!(values, vec![1, 2, 3, 4]);
```

Communicating between threads using channels:

```rust
use crossbeam::channel;
use std::thread;

let (sender, receiver) = channel::unbounded();

thread::spawn(move || {
    for i in 0..5 {
        sender.send(i).unwrap();
    }
});

let mut sum = 0;
for received in receiver {
    sum += received;
}

assert_eq!(sum, 10); // 0 + 1 + 2 + 3 + 4
```

Using bounded channels for backpressure:

```rust
use crossbeam::channel;
use std::thread;

let (sender, receiver) = channel::bounded(2);

thread::spawn(move || {
    for i in 0..5 {
        sender.send(i).unwrap(); // Blocks when buffer is full
    }
});

thread::sleep(std::time::Duration::from_millis(10));

let values: Vec<_> = receiver.iter().collect();
assert_eq!(values, vec![0, 1, 2, 3, 4]);
```

Selecting over multiple channels:

```rust
use crossbeam::channel;
use std::thread;
use std::time::Duration;

let (s1, r1) = channel::unbounded();
let (s2, r2) = channel::unbounded();

thread::spawn(move || {
    s1.send("hello").unwrap();
});

thread::spawn(move || {
    thread::sleep(Duration::from_millis(10));
    s2.send("world").unwrap();
});

crossbeam::select! {
    recv(r1) -> msg => assert_eq!(msg, Ok("hello")),
    recv(r2) -> msg => assert_eq!(msg, Ok("world")),
}
```

[`std::sync`]: crate::std::sync
[`std::thread`]: crate::std::thread
[`std::sync::mpsc`]: crate::std::sync::mpsc
[`std::thread::spawn`]: crate::std::thread::spawn
[`crossbeam::channel`]: crate::crossbeam::channel
[`crossbeam::scope`]: crate::crossbeam::scope
[`channel`]: crate::crossbeam::channel
[`scope`]: crate::crossbeam::scope
[`select!`]: crate::crossbeam::select
[`Backoff`]: crate::crossbeam::utils::Backoff
