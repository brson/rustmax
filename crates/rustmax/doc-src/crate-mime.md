MIME type parsing and manipulation.

- Crate [`::mime`].
- [docs.rs](https://docs.rs/mime)
- [crates.io](https://crates.io/crates/mime)
- [GitHub](https://github.com/hyperium/mime)

---

`mime` provides parsing and manipulation of MIME (Multipurpose Internet Mail Extensions) types.
MIME types identify the format of data in HTTP requests and responses,
making them essential for web applications and file handling.

## Examples

Using predefined MIME constants:

```
use mime::{TEXT_PLAIN, APPLICATION_JSON, IMAGE_PNG};

assert_eq!(TEXT_PLAIN.type_(), "text");
assert_eq!(TEXT_PLAIN.subtype(), "plain");

assert_eq!(APPLICATION_JSON.essence_str(), "application/json");
assert_eq!(IMAGE_PNG.essence_str(), "image/png");
```

Parsing MIME types from strings:

```
use mime::Mime;

let mime: Mime = "text/html; charset=utf-8".parse().unwrap();
assert_eq!(mime.type_(), "text");
assert_eq!(mime.subtype(), "html");
assert_eq!(mime.get_param("charset").unwrap(), "utf-8");
```

Comparing MIME types:

```
use mime::{TEXT_HTML, TEXT_PLAIN};

let html_mime = "text/html".parse::<mime::Mime>().unwrap();
assert_eq!(html_mime, TEXT_HTML);
assert_ne!(html_mime, TEXT_PLAIN);
```

[`Mime`]: crate::mime::Mime
[`TEXT_PLAIN`]: crate::mime::TEXT_PLAIN
[`APPLICATION_JSON`]: crate::mime::APPLICATION_JSON