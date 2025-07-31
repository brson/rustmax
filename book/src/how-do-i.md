# How Do I … in Rust?

<!-- note the organization here is similar but not
     identical to crates/rmx/src-doc/root-docs.md -->

## Discovery

### … find a crate for a given purpose?

### … find the latest version of a crate?

## Project organization

### … organize a Rust workspace?

## Conveniences

### … define "extension" methods on a type in another crate?

### … guarantee a trait is object-safe?

```
static_assertions::assert_obj_safe!(MyTrait);
```

## Error handling and debugging

### … handle errors simply and correctly?

### … structure errors in a public API?

### … set up basic logging?

## Collections

### … create a fast `HashMap`?

### … convert from slices to fixed-length arrays?

### … Implement an `impl Iterator` with `todo!`?

```
  fn merge_create_accounts_results(                      
      accounts: &[tb::Account],
      results: Vec<tb::CreateAccountsResult>,
  ) -> impl Iterator<Item = (u128, Option<tb::Account>)> + use<'_> {
      todo!(); // optional
      std::iter::empty() // satisfies the type checker
  }  
```

## Numerics

### … convert between numeric types ideomatically?

### … perform math ideomatically?

### … convert between ints and bytes?

## Encoding, serialization, parsing

### … serialize to and from JSON?

### … decide what format to use with `serde`?

## Time

### … parse and render standard time formats?

## Random numbers

### … generate a strong random number?

### … generate a strong random number from a seed?

### … generate a fast random number from a seed?

## Cryptography

### … calculate a cryptographic content hash?

## Parallelism and Concurrency

### … initialize a global value?

todo `LazyLock`, `OnceLock`, and `Once`.

### … send messages to/from async code?

todo futures::channels

## Asynchronous I/O

### … set up the `tokio` event loop?

### … stub an unwritten `async fn`?

## Networking and web

### … make a synchronous HTTP request?

### … configure a basic HTTP server?

## Text / unicode

## Terminal / CLI

### … set up a simple CLI parser with subcommands?

### … display colors in the terminal?

### … read line-based input from the terminal?

## System / OS

### … read environment variables?

### … work with a temporary file?

### … work with multiple files in a temporary directory?

## Testing

### … create a custom test harness?

### … create a custom table-based test harness?

## Build scripts

### … write build scripts ideomatically?

### … link to a native static library?

### … compile and link a C source file?

## FFI / interop

### … create Rust bindings to a C/C++ program?

## Procedural macros

