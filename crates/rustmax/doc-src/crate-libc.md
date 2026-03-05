Raw FFI bindings to platform-specific system libraries.

- Crate [`::libc`].
- [docs.rs](https://docs.rs/libc)
- [crates.io](https://crates.io/crates/libc)
- [GitHub](https://github.com/rust-lang/libc)

---

`libc` provides raw bindings to the C standard library
and platform-specific system APIs.
It is the foundational crate for most Rust FFI that
needs to interoperate with system and native libraries.

The crate exports types, constants, and function signatures
matching the platform's C headers.
On Linux this includes POSIX APIs and Linux-specific extensions;
on macOS, the Darwin system APIs; on Windows, a small subset of CRT functions.

Common uses include:
- Calling POSIX functions like [`open`], [`read`], [`write`], [`mmap`]
- Accessing system constants like [`EINVAL`], [`O_RDONLY`], [`SIGTERM`]
- Defining types for FFI function signatures ([`c_int`], [`c_char`], [`size_t`])
- Low-level memory operations like [`malloc`] and [`free`]

Note that basic C type aliases like `c_int` and `c_char`
are available directly in [`std::ffi`].
The [`nix`](https://docs.rs/nix)
and [`rustix`](https://docs.rs/rustix)
crates provide additional high-level safe Unix API bindings.
For Windows, the [`windows`](https://docs.rs/windows)
and [`windows-sys`](https://docs.rs/windows-sys) crates
provide bindings to the Win32 API.

## Examples

Querying system configuration not exposed by `std`:

```rust
let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
assert!(page_size > 0);

let uid = unsafe { libc::getuid() };
let pid = unsafe { libc::getpid() };
assert!(pid > 0);
```

[`open`]: crate::libc::open
[`read`]: crate::libc::read
[`write`]: crate::libc::write
[`mmap`]: crate::libc::mmap
[`EINVAL`]: crate::libc::EINVAL
[`O_RDONLY`]: crate::libc::O_RDONLY
[`SIGTERM`]: crate::libc::SIGTERM
[`c_int`]: crate::libc::c_int
[`c_char`]: crate::libc::c_char
[`size_t`]: crate::libc::size_t
[`malloc`]: crate::libc::malloc
[`free`]: crate::libc::free
[`std`]: crate::std
