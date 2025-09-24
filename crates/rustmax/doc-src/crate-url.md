URL parsing and manipulation.

- Crate [`::url`].
- [docs.rs](https://docs.rs/url)
- [crates.io](https://crates.io/crates/url)
- [GitHub](https://github.com/servo/rust-url)

---

`url` implements the WHATWG URL Standard for parsing and manipulating URLs.
It provides a robust, spec-compliant URL parser that handles all the edge cases
of URL parsing, including internationalized domain names, percent encoding,
and proper handling of relative URLs.

## Examples

Basic URL parsing and components:

```
use url::Url;

let parsed = Url::parse("https://example.com:8080/path?query=value#fragment").unwrap();
assert_eq!(parsed.scheme(), "https");
assert_eq!(parsed.host_str(), Some("example.com"));
assert_eq!(parsed.port(), Some(8080));
assert_eq!(parsed.path(), "/path");
assert_eq!(parsed.query(), Some("query=value"));
assert_eq!(parsed.fragment(), Some("fragment"));
```

[`Url`]: crate::url::Url
[`parse`]: crate::url::Url::parse
[`join`]: crate::url::Url::join