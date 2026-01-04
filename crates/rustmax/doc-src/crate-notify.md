Cross-platform filesystem notification library.

- Crate [`::notify`].
- [docs.rs](https://docs.rs/notify)
- [crates.io](https://crates.io/crates/notify)
- [GitHub](https://github.com/notify-rs/notify)

---

`notify` provides filesystem watching capabilities,
detecting when files or directories are created, modified, deleted, or renamed.
It uses platform-native APIs (inotify on Linux, FSEvents on macOS, ReadDirectoryChangesW on Windows).

The main entry point is [`recommended_watcher`] which creates a watcher
appropriate for the current platform.
Events are delivered through a channel or callback.

## Examples

Watch a directory for changes:

```rust,no_run
use notify::{recommended_watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

let (tx, rx) = channel();

let mut watcher = recommended_watcher(move |res| {
    tx.send(res).unwrap();
}).unwrap();

watcher.watch(std::path::Path::new("."), RecursiveMode::Recursive).unwrap();

loop {
    match rx.recv_timeout(Duration::from_secs(1)) {
        Ok(Ok(event)) => println!("Change: {:?}", event),
        Ok(Err(e)) => println!("Watch error: {:?}", e),
        Err(_) => {} // Timeout, continue
    }
}
```

[`recommended_watcher`]: crate::notify::recommended_watcher
[`Watcher`]: crate::notify::Watcher
[`RecursiveMode`]: crate::notify::RecursiveMode
