## Assert `Sync` / `Sync`

```
struct DbPathGen(());

const _ASSERT_SEND_SYNC: () = assert_send_sync::<DbPathGen>();
const fn assert_send_sync<T: Send + Sync>() { }
```

Also in `rustmax::extras::assert_send_sync`.


## Copy a directory recursively

There's no standard way to recursively copy a dir!

Here take this:

```
    pub fn copy_dir_recursive(
        src: &std::path::Path,
        dst: &std::path::Path,
    ) -> std::io::Result<()> {
        std::fs::create_dir_all(dst)?;

        for entry in std::fs::read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if file_type.is_dir() {
                copy_dir_recursive(&src_path, &dst_path)?;
            } else {
                std::fs::copy(&src_path, &dst_path)?;
            }
        }

        Ok(())
    }
```

It's also at `rustmax::extras::copy_dir_recursive`.


## There's an `Either` hiding in the `futures` crate!

The `Either` type, most known from Haskell, is a useful
reusable abstraction for the common need to create a lightweight type
that is a named union of two other types, typically looks like:

```
enum Either<A, B> {
  Left(A),
  Right(B),
}
```

Rust doesn't provide this type because it instead provides `Result`,
which is the most common use-case for the pattern, and the remaining
use-cases seem marginal for such a simple type - official guidelines
encouraging writing your own `Either` type for specific uses, with more
specific names.

Sometimes you want one though!

There's one in the futures crate: [`futures::future::Either`].

Although `Either` is a `Future` and provided for awaiting pairs of futures,
its definition is exactly as above.

[`futures::future::Either`]: https://docs.rs/futures/latest/futures/future/enum.Either.html


## Import traits with `as _`

Most of the time we need traits to call a method,
and for this we need the trait to be in scope,
but it doesn't need to be namable.

```rust
use std::io::{self, Seek as _};

let hash = read_hash(&mut reader)?;
reader.seek(io::SeekFrom::Start(0))?;
```




## Set up the `mold` linker

Linking is one of the most time-consuming
stages of a Rust build,
and it has to be redone every time you test your program.

On Linux the [`mold`] linker is faster than
the the default linker.
Setting it up manually is not difficult,
but just hard enough that I have to look it up
every time.

The Rust Max CLI tool can do it instead:

`rustmax install-tool mold`

[`mold`]: https://github.com/rui314/mold


## Use `rustfmt::skip`

Sometimes exact formatting is important to make code beautiful.
Don't be afraid annotate with `#[rustfmt::skip]`.


## Use `Option` with `?`

The `?` operator works with `Option` to early-return on `None`,
making option-heavy code much cleaner.

```rust
fn find_user_email(id: u32) -> Option<String> {
    let user = database.find_user(id)?;  // Return None if user not found
    let profile = user.get_profile()?;   // Return None if no profile
    let email = profile.email?;          // Return None if no email
    Some(email)
}
```

An alternative to `match` or `if let` statements, `?` lets you chain
operations that might fail, automatically propagating `None` up the call stack.


## Put common development commands in a `justfile`

Almost every project has a handful of commands the developer(s)
uses frequently. Put these in a `justfile` so the menu of
commands for this project is always obvious, which
can be extra helpful after years away from a project.

`just` runs commands listed in a file named `justfile`.
The `justfile` lives your project's root directory,
and is configured with a `make`-like syntax:

```just
default:
    just --list

install-tools:
    cargo install mdbook
    cargo install mdbook-yapp

clean: doc-clean
    cargo clean

doc-clean:
    rm -rf out
```

It's a simple idea, but suprisingly useful. And don't worry that it looks like
a `Makefile` &mdash; it is much more fun and sensible in use than `make`.

When you come back to a project and see there's a justfile you
know to run `just --list` and you'll immediately see what
was on the previous maintainer's mind.

```
$ just --list
Available recipes:
    build
    check
    clean
    default
    doc-book
    doc-build
    doc-clean
    doc-crates
    install-tools
    lint
    maint-audit
    maint-duplicates
    maint-lock-minimum-versions # useful prior to running `cargo audit`
    maint-outdated
    maint-upgrade
    prebuild
    publish
    publish-dry
    replace-version old new
    test
    test-min-version-build
```


## Merge message streams with `futures::stream::select`

An async event loop often needs to receive messages from multiple sources.
Use `futures::stream::select` to merge two streams of the same type into one.

A common pattern: an event loop that handles both external messages
and internally-generated events (like async task completions):

```rust
use futures::channel::mpsc;
use futures::stream::StreamExt;

async fn run_event_loop(
    mut rx_external: mpsc::Receiver<Msg>,
) {
    // Create channel for loop to send messages to itself.
    let (tx_self, rx_self) = mpsc::channel::<Msg>(10);

    // Merge both streams.
    let mut rx_combined = futures::stream::select(rx_external, rx_self);

    while let Some(msg) = rx_combined.next().await {
        match msg {
            Msg::StartTask => {
                // Spawn async task that reports completion.
                let mut tx_self = tx_self.clone();
                tokio::spawn(async move {
                    do_work().await;
                    tx_self.send(Msg::TaskComplete).await.ok();
                });
            }
            Msg::TaskComplete => {
                // Handle completion sent from spawned task.
            }
        }
    }
}
```
