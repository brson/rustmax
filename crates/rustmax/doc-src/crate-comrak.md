CommonMark and GitHub Flavored Markdown parser.

- Crate [`::comrak`].
- [docs.rs](https://docs.rs/comrak)
- [crates.io](https://crates.io/crates/comrak)
- [GitHub](https://github.com/kivikakk/comrak)

---

`comrak` is a 100% CommonMark-compatible Markdown parser
that also supports GitHub Flavored Markdown (GFM) extensions.
It can render Markdown to HTML, CommonMark, or manipulate the AST directly.

The main entry point is [`markdown_to_html`] for simple conversions,
or [`parse_document`] for AST manipulation.
Options are configured via [`Options`] which controls
extensions like tables, strikethrough, and autolinks.

## Examples

Basic Markdown to HTML conversion:

```rust
use comrak::{markdown_to_html, Options};

let markdown = "# Hello\n\nThis is **bold** and *italic*.";
let html = markdown_to_html(markdown, &Options::default());

assert!(html.contains("<h1>"));
assert!(html.contains("<strong>bold</strong>"));
assert!(html.contains("<em>italic</em>"));
```

Enabling GitHub Flavored Markdown extensions:

```rust
use comrak::{markdown_to_html, Options, ExtensionOptions};

let mut options = Options::default();
options.extension.strikethrough = true;
options.extension.table = true;
options.extension.autolink = true;

let markdown = "~~deleted~~ and https://example.com";
let html = markdown_to_html(markdown, &options);

assert!(html.contains("<del>deleted</del>"));
assert!(html.contains("<a href="));
```

[`markdown_to_html`]: crate::comrak::markdown_to_html
[`parse_document`]: crate::comrak::parse_document
[`Options`]: crate::comrak::Options
