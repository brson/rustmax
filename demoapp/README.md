# Folio

A static site generator for technical writing,
and the demo application for [`rustmax`](../README.md).

Folio turns a directory of markdown documents into a static site
with syntax highlighting, tables of contents, client-side search,
feeds and a sitemap,
and exports the same collection as EPUB, JSON or a tarball.
`folio check` lints documents the way technical writing goes wrong:
it parses every Rust code block with `syn`.

Its only dependency is `rustmax`.
Every crate it uses is reached through that one dependency,
so Folio is also a test that the supercrate is enough for a real program.
See [DESIGN.md](DESIGN.md) for which crates are used where, and which are not and why.

## Quick start

```
cargo run -- init my-site
cargo run -- new "First post" --path my-site
cargo run -- check my-site
cargo run -- build my-site --drafts
```

The site is written to `my-site/output`.

## Commands

| Command | |
| --- | --- |
| `init [PATH]` | Create a collection: `folio.toml`, `content/`, `templates/`, `static/`. |
| `new TITLE` | Create a draft document dated today. |
| `build [PATH]` | Build the site. `--drafts`, `--compress`, `--incremental`, `--progress`. |
| `watch [PATH]` | Build, then rebuild on every change until Ctrl-C. `--drafts`, `--output`. |
| `check [PATH]` | Lint documents: Rust code that does not parse, links and images that lead nowhere, unknown shortcodes and languages, future dates. Fails on errors. |
| `export [PATH] -f FORMAT` | `json`, `rss`, `atom`, `json-feed`, `sitemap`, `epub`, or `tar` of the built output. |
| `index [PATH]` | Rebuild the built site's search index without rebuilding pages. |
| `repl [PATH]` | Query the collection interactively. |
| `files [PATH] -p GLOB` | List files matching a glob. |
| `fetch URL...` | Download markdown documents into `content/`, several at once. |

## A collection

```
my-site/
  folio.toml        configuration (or folio.json5)
  content/          markdown documents
  templates/        Tera templates; a built-in one is used if none match
  static/           copied to the output as is
```

Documents start with TOML frontmatter between `---` lines:

```markdown
---
title = "First post"
date = "2026-10-27"
tags = ["rust"]
draft = false
---

Text, with an excerpt break if you like.

<!--more-->
```

`description`, `author` and `template` are also understood,
and any other key is passed through to the template.

## Configuration

`folio init` writes every setting with its default.
The ones most worth knowing:

```toml
[collection]
title = "My Collection"
base_url = "https://example.com"
folio_version = "0.3"     # Semver requirement on the Folio that may build this.

[build]
output_dir = "output"
features = ["search", "feeds", "sitemap", "shortcodes"]
seed = 1                  # Fixes generated ids; see below.

[highlight]
theme = "github-dark"
```

`features` chooses what a build writes beyond the pages:

| Feature | |
| --- | --- |
| `search` | A search page at `/search/`, with `search-index.json` and `search.js` |
| `feeds` | `rss.xml`, `atom.xml`, `feed.json` |
| `sitemap` | `sitemap.xml` |
| `shortcodes` | Expand shortcodes in documents |
| `compress` | A `.gz` beside each compressible file |
| `drafts` | Build draft documents too |

`build --drafts` and `--compress` add to the configured set.

A build with `seed` set and `SOURCE_DATE_EPOCH` in the environment is reproducible:
the same content produces the same bytes, exports included.

## Links and images

Link to another document by its source file, relative to the one you are writing:

```markdown
See [the setup guide](guide/setup.md#install).
```

The build points the link at that document's page, wherever its `slug` puts it.
Images and other files go in `static/` and are linked from the site root,
as in `![Chart](/img/chart.png)`;
local images get `width` and `height` attributes from the file.
`folio check` reports links and images that lead nowhere.

## Shortcodes

```markdown
{{< toc min="2" max="3" heading="Contents" >}}
{{< figure src="/diagram.png" alt="Diagram" caption="How it fits together" >}}
{{< youtube dQw4w9WgXcQ >}}
{{% note %}}Admonition bodies are **markdown**. Also `warning` and `tip`.{{% /note %}}
{{% code lang="rust" %}}fn main() {}{{% /code %}}
```

Shortcodes inside code blocks and inline code are left alone,
so a document can show the syntax.
An unknown shortcode is left as written and reported by `folio check`.

## Templates

Templates are [Tera](https://keats.github.io/tera/).
Document pages get `title`, `content`, `date`, `tags`, `toc`, `word_count`, `reading_time`
and the site settings;
the index and tag pages get `documents`.
`search` is true when the search page is built, for linking to it,
and a `search.html` template replaces the built-in search page.
`integrity` holds the Subresource Integrity hash of each generated asset,
keyed by file name with punctuation replaced by `_`:

```html
<link rel="stylesheet" href="/toc.css" integrity="{{ integrity.toc_css }}">
```

[`templates/default.html`](templates/default.html) is what `folio init` installs.

## Development

```
cargo test
```

Folio is its own workspace, outside the `rustmax` one;
from the repository root, `just test-demoapp` runs its tests.

- [DESIGN.md](DESIGN.md): design decisions and `rustmax` coverage.
- [ROADMAP.md](ROADMAP.md): what is next.
- [VALIDATION.md](VALIDATION.md): what Folio has found in `rustmax`.
