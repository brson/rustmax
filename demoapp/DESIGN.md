# Anthology Design Document

A static site generator and document publishing platform demonstrating the rustmax supercrate.

## Purpose

Anthology serves two goals:
1. **User goal**: Achieve test coverage for rustmax crate APIs
2. **App goal**: Be a production-quality, useful static site generator

## Architecture Overview

```
anthology/
├── src/
│   ├── main.rs          # Entry point, runs CLI
│   ├── lib.rs           # Public module exports
│   ├── error.rs         # Error types (thiserror)
│   ├── cli/
│   │   ├── mod.rs       # Re-exports
│   │   └── commands.rs  # CLI commands (clap)
│   ├── collection/
│   │   ├── mod.rs       # Collection type, queries
│   │   ├── config.rs    # anthology.toml parsing
│   │   ├── document.rs  # Document model, frontmatter
│   │   └── scanner.rs   # Directory walking (walkdir, ignore)
│   ├── build/
│   │   ├── mod.rs       # Build orchestration (rayon)
│   │   ├── markdown.rs  # MD->HTML (comrak)
│   │   ├── template.rs  # Template rendering (tera)
│   │   ├── highlight.rs # Syntax highlighting (regex)
│   │   ├── toc.rs       # Table of contents generation
│   │   ├── cache.rs     # Incremental build cache
│   │   ├── compress.rs  # Asset compression (flate2)
│   │   ├── encoding.rs  # Base64/hex encoding
│   │   ├── rewrite.rs   # URL rewriting
│   │   └── search_js.rs # Client-side search JS
│   ├── serve/
│   │   └── mod.rs       # Dev server with search API (axum, tokio)
│   ├── search/
│   │   └── mod.rs       # Full-text indexing with BM25
│   └── feeds/
│       └── mod.rs       # Atom and JSON Feed generation
└── templates/
    └── default.html     # Built-in default template
```

## Core Data Flow

```
anthology.toml (Config)
        |
        v
content/*.md --> Scanner --> Documents --> Collection
                (walkdir)   (frontmatter)
        |
        v
    Build Pipeline (rayon parallel)
        |
        +---> Markdown -> HTML (comrak)
        |
        +---> Template rendering (tera)
        |
        +---> Static asset copy (walkdir)
        |
        v
    output/ (static HTML site)
```

## Rustmax Crate Coverage

### Currently Used (38 crates)

| Category | Crate | Usage |
|----------|-------|-------|
| CLI | clap | Command-line argument parsing |
| CLI | termcolor | Colored terminal output |
| CLI | rustyline | Interactive REPL |
| CLI | ctrlc | Graceful shutdown handling |
| Web | axum | Development server |
| Web | tower-http | Static file serving (external dep) |
| Web | reqwest | Remote content fetching |
| Async | tokio | Async runtime for server |
| Async | futures | Parallel async operations |
| Concurrency | rayon | Parallel document building |
| Parsing | comrak | Markdown to HTML |
| Parsing | regex | URL rewriting, content transforms |
| Parsing | nom | Shortcode syntax parsing |
| Templates | tera | HTML template rendering |
| Serialization | serde | Document/config serialization |
| Serialization | serde_json | JSON export, search index |
| Serialization | toml | Config and frontmatter parsing |
| Encoding | flate2 | Gzip asset compression |
| Encoding | base64 | Inline image data URLs |
| Encoding | hex | Hash display formatting |
| Encoding | bytes | Binary asset handling |
| Crypto | blake3 | Content hashing for cache keys |
| Crypto | sha2 | Alternative hashing (SHA-256/512) |
| Filesystem | walkdir | Directory traversal |
| Filesystem | ignore | .gitignore-style filtering |
| Filesystem | glob | File pattern matching |
| Filesystem | tempfile | Test fixtures |
| Time | jiff | Date parsing (civil::Date) |
| Time | chrono | Date compatibility layer |
| Text | unicode-segmentation | Word counting, search tokenization |
| Text | memchr | Fast byte/substring searching |
| Collections | itertools | Iterator utilities |
| Config | bitflags | Feature flags and options |
| Random | rand | Random ID generation |
| Concurrency | crossbeam | Work-stealing, channels, scoped threads |
| Logging | log + env_logger | Logging infrastructure |
| Errors | thiserror | Error type definitions |
| Errors | anyhow | Fallback error handling |

### All Targeted Crates Integrated!

38 rustmax crates now in use, covering CLI, web, async, concurrency, parsing, templates, serialization, encoding, crypto, filesystem, time, text, and more.

## Key Design Decisions

### 1. Standalone Project
Anthology is NOT part of the rustmax workspace. It lives in `demoapp/` and references rustmax via path dependency. This keeps the demo isolated.

### 2. Derive Macro Dependencies
serde, clap, and thiserror derive macros emit `::crate_name::` in generated code. These must be direct dependencies even though rustmax re-exports them.

### 3. Sync Build, Async Serve
- Build pipeline uses `rayon` for CPU-bound parallel work
- Dev server uses `tokio` + `axum` for async I/O
- No mixing of async in build (keeps it simple)

### 4. TOML Frontmatter
Uses TOML (not YAML) for frontmatter to match Rust ecosystem conventions:
```markdown
---
title = "Hello World"
date = "2024-01-15"
tags = ["rust"]
---
```

### 5. Content Hashing
Each document has a blake3 hash (`Document.content_hash`) for incremental builds (future feature).

### 6. Template Fallbacks
If a specific template (e.g., `post.html`) doesn't exist, falls back to `default.html`.

## CLI Commands

| Command | Description |
|---------|-------------|
| `init [path]` | Create new collection with example content |
| `build [path]` | Build static site to output/ |
| `build --compress` | Build with gzip compression |
| `serve [path]` | Start dev server on port 3000 |
| `check [path]` | Validate all documents |
| `new <title>` | Create new document |
| `index [path]` | Rebuild search index |
| `export --format` | Export as JSON/RSS/sitemap |
| `fetch <url>` | Fetch remote content |
| `files [pattern]` | List files matching glob pattern |
| `repl` | Interactive REPL for queries |

## Config Format (anthology.toml)

```toml
[collection]
title = "My Collection"
base_url = "https://example.com"
description = ""
author = ""

[build]
output_dir = "output"

[content]
date_format = "%Y-%m-%d"
default_template = "default.html"
excerpt_separator = "<!--more-->"

[server]
port = 3000

[highlight]
enabled = true
theme = "github-dark"  # github, monokai, dracula, one-dark, solarized-light, solarized-dark, nord
line_numbers = true
copy_button = true
```

## Document Format

```markdown
---
title = "Post Title"
date = "2024-01-15"
tags = ["tag1", "tag2"]
draft = false
slug = "custom-url"           # optional
template = "custom.html"      # optional
description = "Summary"       # optional
author = "Name"               # optional
custom_field = "value"        # extra fields allowed
---

Markdown content here.
```

## Error Handling Strategy

- `Error` enum with variants for each failure mode
- `#[from]` conversions for common error types
- Helper constructors: `Error::config()`, `Error::document()`, etc.
- `Result<T>` type alias used throughout

## Testing Strategy

Current tests (200 passing):
- Frontmatter parsing
- No-frontmatter documents
- Word counting
- Search indexing (with stemming, BM25)
- Collection queries
- Syntax highlighting (36 tests)
- Table of contents (24 tests)
- Build cache
- Compression
- Encoding
- Atom and JSON Feed generation (9 tests)
- Client-side search JS (2 tests)

Future tests needed:
- CLI integration tests
- Build output verification
- Server endpoint tests
- Property tests with proptest

## Development Phases

### Phase 1: Foundation (COMPLETE)
- Basic CLI with clap
- Config loading
- Document parsing
- Build pipeline
- Dev server
- Search indexing

### Phase 2: Enhanced Features (COMPLETE)
- [x] Live reload via WebSocket (file watcher + hot CSS)
- [x] REPL mode (rustyline)
- [x] Incremental builds using content_hash
- [x] Asset compression (flate2)

### Phase 3: Advanced (PLANNED)
- [ ] Remote content fetching (reqwest)
- [ ] Custom syntax extensions (nom)
- [ ] Image optimization
- [ ] Plugin system

## Known Issues / Technical Debt

1. **tower-http external**: Not in rustmax, added as direct dependency
2. **Templates are limited**: Few built-in templates

## File Locations

- Config: `anthology.toml` in collection root
- Content: `content/*.md`
- Templates: `templates/*.html`
- Static assets: `static/`
- Output: `output/` (configurable)
- Search index: `search-index.json`
