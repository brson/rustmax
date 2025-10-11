Compile-time assertions about constants, types, and more.

- Crate [`::static_assertions`].
- [docs.rs](https://docs.rs/static_assertions)
- [crates.io](https://crates.io/crates/static_assertions)
- [GitHub](https://github.com/nvzqz/static-assertions)

---

`static_assertions` provides macros for compile-time assertions
that verify assumptions about constants, types, and traits.

All assertions execute at compile time with zero runtime cost.
When an assertion fails, compilation stops with a clear error message,
catching bugs early in the development process.

The crate is particularly valuable for library authors
who need to enforce API contracts at compile time,
such as ensuring types implement required traits,
verifying type sizes match platform expectations,
or validating constant expressions.

Key macros include [`const_assert!`] for constant boolean expressions,
[`assert_impl_all!`] to verify trait implementations,
[`assert_eq_size!`] to check type sizes,
and [`assert_fields!`] to confirm struct field presence.

## Examples

Assert constant boolean expressions:

```rust
use static_assertions::const_assert;

const BUFFER_SIZE: usize = 1024;
const MAX_SIZE: usize = 4096;

// Ensure buffer size is reasonable.
const_assert!(BUFFER_SIZE > 0);
const_assert!(BUFFER_SIZE <= MAX_SIZE);
const_assert!(BUFFER_SIZE.is_power_of_two());
```

Verify types implement required traits:

```rust
use static_assertions::assert_impl_all;

struct Config {
    name: String,
    enabled: bool,
}

// Ensure Config can be sent between threads and cloned.
assert_impl_all!(Config: Send, Sync, Clone);

impl Clone for Config {
    fn clone(&self) -> Self {
        Config {
            name: self.name.clone(),
            enabled: self.enabled,
        }
    }
}
```

Check type sizes match expectations:

```rust
use static_assertions::assert_eq_size;

#[repr(C)]
struct Header {
    magic: u32,
    version: u32,
}

// Ensure header is exactly 8 bytes for binary format compatibility.
assert_eq_size!(Header, [u8; 8]);
```

[`const_assert!`]: https://docs.rs/static_assertions/latest/static_assertions/macro.const_assert.html
[`assert_impl_all!`]: https://docs.rs/static_assertions/latest/static_assertions/macro.assert_impl_all.html
[`assert_eq_size!`]: https://docs.rs/static_assertions/latest/static_assertions/macro.assert_eq_size.html
[`assert_fields!`]: https://docs.rs/static_assertions/latest/static_assertions/macro.assert_fields.html
