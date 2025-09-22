Efficient byte buffer management.

- Crate [`::bytes`].
- [docs.rs](https://docs.rs/bytes)
- [crates.io](https://crates.io/crates/bytes)
- [GitHub](https://github.com/tokio-rs/bytes)

---

`bytes` provides a robust and performant way to work with byte buffers
without unnecessary allocations.

The crate is built around two primary types:
[`Bytes`] is an immutable, reference-counted byte buffer
that enables zero-copy cloning and slicing.
[`BytesMut`] is its mutable counterpart that can be efficiently
converted to `Bytes` when you're done modifying it.

The [`Buf`] and [`BufMut`] traits provide a cursor-based API
for reading and writing bytes to buffers.
These traits are implemented by various types including
`Bytes`, `BytesMut`, and standard types like `Vec<u8>` and `&[u8]`.

This crate is particularly useful in network programming
where efficient buffer management is critical for performance.
It's a foundational component of the Tokio ecosystem
and is used extensively in async I/O operations.

## Examples

Creating and sharing byte buffers efficiently:

```rust,ignore
use bytes::{Bytes, BytesMut};

// Create a mutable buffer
let mut buf = BytesMut::with_capacity(1024);
buf.extend_from_slice(b"hello ");
buf.extend_from_slice(b"world");

// Convert to immutable Bytes (zero-copy)
let bytes: Bytes = buf.freeze();

// Clone is cheap (reference counted)
let clone = bytes.clone();

// Slicing is also zero-copy
let slice = bytes.slice(0..5);
assert_eq!(&slice[..], b"hello");
```

Using the `Buf` trait for reading:

```rust,ignore
use bytes::Buf;

fn read_u32(buf: &mut impl Buf) -> u32 {
    buf.get_u32()
}

let mut data = &b"\x00\x00\x00\x42rest"[..];
let value = read_u32(&mut data);
assert_eq!(value, 0x42);
assert_eq!(data, b"rest");
```

[`Bytes`]: crate::bytes::Bytes
[`BytesMut`]: crate::bytes::BytesMut
[`Buf`]: crate::bytes::Buf
[`BufMut`]: crate::bytes::BufMut