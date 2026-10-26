Body types and combinators for [`hyper`] and the `http-body` ecosystem.

- Crate [`::http_body_util`].
- [docs.rs](https://docs.rs/http-body-util)
- [crates.io](https://crates.io/crates/http-body-util)
- [GitHub](https://github.com/hyperium/http-body)

---

`http-body-util` provides the concrete request and response body types
that [`hyper`] 1.x leaves out.
`hyper` defines only the `Body` trait
and [`Incoming`], the body type of received messages.
Anything that sends or reads body bytes needs this crate.

The types used most:

- [`Full`] - a body holding a single chunk of data
- [`Empty`] - a body with no data, for `GET` requests and bodiless responses
- [`BodyExt`] - extension methods on any body,
  notably [`collect`] for reading a whole body
  and [`boxed`] for erasing its type
- [`BoxBody`] - a type-erased body,
  for handlers that return different body types from different branches
- [`Limited`] - caps the size of a body, to bound memory spent on untrusted input
- [`StreamBody`] - a body built from a [`Stream`] of frames

## Examples

Creating bodies and reading them back with [`collect`]:

```rust
use bytes::Bytes;
use futures::executor::block_on;
use http_body_util::{BodyExt, Empty, Full};

let body = Full::new(Bytes::from("hello"));
let bytes = block_on(body.collect()).unwrap().to_bytes();
assert_eq!(bytes, "hello");

let empty = Empty::<Bytes>::new();
let bytes = block_on(empty.collect()).unwrap().to_bytes();
assert!(bytes.is_empty());
```

Returning different body types as one [`BoxBody`]:

```rust
use bytes::Bytes;
use futures::executor::block_on;
use http_body_util::{BodyExt, Empty, Full, combinators::BoxBody};
use std::convert::Infallible;

fn body_for(found: bool) -> BoxBody<Bytes, Infallible> {
    if found {
        Full::new(Bytes::from("found")).boxed()
    } else {
        Empty::new().boxed()
    }
}

let bytes = block_on(body_for(true).collect()).unwrap().to_bytes();
assert_eq!(bytes, "found");
```


[`hyper`]: crate::hyper
[`Incoming`]: crate::hyper::body::Incoming
[`Stream`]: crate::futures::Stream
[`Full`]: crate::http_body_util::Full
[`Empty`]: crate::http_body_util::Empty
[`BodyExt`]: crate::http_body_util::BodyExt
[`collect`]: crate::http_body_util::BodyExt::collect
[`boxed`]: crate::http_body_util::BodyExt::boxed
[`BoxBody`]: crate::http_body_util::combinators::BoxBody
[`Limited`]: crate::http_body_util::Limited
[`StreamBody`]: crate::http_body_util::StreamBody
