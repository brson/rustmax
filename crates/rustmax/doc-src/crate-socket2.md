Low-level network socket programming beyond `std::net`.

- Crate [`::socket2`].
- [docs.rs](https://docs.rs/socket2)
- [crates.io](https://crates.io/crates/socket2)
- [GitHub](https://github.com/rust-lang/socket2)

---

`socket2` provides direct access to system socket APIs without requiring unsafe code.
Where [`std::net`] offers high-level TCP/UDP types,
`socket2` exposes the full range of socket options, address families,
and configuration that the OS provides.
Common uses include setting socket options like `SO_REUSEADDR` or `TCP_NODELAY`,
configuring keepalive, creating dual-stack IPv6 listeners,
and converting the configured socket into standard library types.

The crate is maintained by the Rust project itself.

## Examples

Creating a TCP listener with socket options,
then converting to a [`std::net::TcpListener`]:

```rust
use socket2::{Socket, Domain, Type, Protocol, SockAddr};
use std::net::SocketAddr;

// Create a TCP socket.
let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;

// Set SO_REUSEADDR so the port can be reused immediately after close.
socket.set_reuse_address(true)?;

// Bind to a local address.
let address: SocketAddr = "127.0.0.1:0".parse().unwrap();
socket.bind(&SockAddr::from(address))?;

// Start listening.
socket.listen(128)?;

// Convert to a std TcpListener for use with normal APIs.
let listener: std::net::TcpListener = socket.into();
let local_addr = listener.local_addr()?;
assert_eq!(local_addr.ip(), std::net::Ipv4Addr::LOCALHOST);
# Ok::<(), std::io::Error>(())
```

[`std::net`]: std::net
[`std::net::TcpListener`]: std::net::TcpListener
