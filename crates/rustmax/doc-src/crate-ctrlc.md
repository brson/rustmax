Simple handling of Ctrl+C for CLI programs.

- Crate [`::ctrlc`].
- [docs.rs](https://docs.rs/ctrlc)
- [crates.io](https://crates.io/crates/ctrlc)
- [GitHub](https://github.com/Detegr/rust-ctrlc)

---

`ctrlc` provides a simple, cross-platform way to set a handler
for the Ctrl+C signal (`SIGINT` on Unix, `CTRL_C_EVENT` on Windows).
It spawns a dedicated signal-handling thread
and invokes a user-provided closure when the signal is received.

The typical pattern is to pair a handler with an [`AtomicBool`]
that the main loop checks to know when to shut down gracefully.

## Examples

Graceful shutdown with an atomic flag:

```rust,no_run
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

let running = Arc::new(AtomicBool::new(true));
let r = running.clone();

ctrlc::set_handler(move || {
    r.store(false, Ordering::SeqCst);
}).expect("Error setting Ctrl-C handler");

while running.load(Ordering::SeqCst) {
    // Do work...
    break; // (break immediately for doctest)
}

println!("Shutting down.");
```

[`AtomicBool`]: std::sync::atomic::AtomicBool
