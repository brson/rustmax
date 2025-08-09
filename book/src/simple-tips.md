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
