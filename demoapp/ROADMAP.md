# Anthology Roadmap

Progress tracking for rustmax crate coverage.

## Crate Coverage Status

### Fully Integrated
- [x] clap - CLI parsing
- [x] serde - Serialization framework
- [x] serde_json - JSON export
- [x] toml - Config and frontmatter
- [x] thiserror - Error definitions
- [x] anyhow - Fallback errors
- [x] log - Logging facade
- [x] env_logger - Log output
- [x] axum - Web server
- [x] tokio - Async runtime
- [x] rayon - Parallel builds
- [x] comrak - Markdown parsing
- [x] tera - Templates
- [x] walkdir - Directory traversal
- [x] ignore - Gitignore patterns
- [x] blake3 - Content hashing
- [x] jiff - Date handling
- [x] unicode-segmentation - Text processing
- [x] termcolor - Colored output
- [x] rustyline - REPL mode
- [x] ctrlc - Graceful server shutdown
- [x] tempfile - Test fixtures
- [x] glob - File pattern matching
- [x] itertools - Collection operations
- [x] reqwest - Remote content fetching
- [x] flate2 - Asset compression
- [x] regex - URL rewriting, content transforms
- [x] futures - Parallel async operations
- [x] bytes - Binary asset handling
- [x] base64 - Inline image encoding
- [x] hex - Hash display
- [x] rand - Random ID generation
- [x] sha2 - Alternative hashing (SHA-256/512)
- [x] memchr - Fast byte/substring searching
- [x] chrono - Date compatibility layer
- [x] nom - Shortcode syntax parsing
- [x] bitflags - Feature flags and options
- [x] crossbeam - Work-stealing, channels, scoped threads

### Recently Integrated (v0.7)
- [x] image - Image optimization and format conversion
- [x] zip - EPUB export (ZIP-based format)
- [x] notify - Native file watching for live reload
- [x] indicatif - Progress bars for CLI builds

### Recently Integrated (v0.8)
- [x] url - URL parsing and validation
- [x] mime - MIME type detection
- [x] proptest - Property-based testing

## Feature Roadmap

### v0.2 - Developer Experience (COMPLETE)
- [x] Live reload with WebSocket (tokio, futures)
- [x] REPL mode for quick queries (rustyline)
- [x] Graceful shutdown (ctrlc)
- [ ] Better error messages with source locations

### v0.3 - Build Optimization (COMPLETE)
- [x] Incremental builds using content_hash
- [x] Asset compression (flate2)
- [ ] Parallel template compilation
- [x] Build cache persistence

### v0.4 - Content Features (COMPLETE)
- [x] Remote content fetching (reqwest)
- [x] Custom shortcodes (nom)
- [x] Image optimization (image crate)
- [x] Table of contents generation
- [x] Syntax highlighting themes

### v0.5 - Search Enhancement (COMPLETE)
- [x] Stemming for better matches
- [x] Search API endpoint
- [x] Client-side search JS
- [ ] Fuzzy search

### v0.6 - Export Formats (COMPLETE)
- [x] Atom feed
- [x] JSON Feed
- [x] EPUB export (zip crate)
- [ ] PDF generation

## Implementation Notes

### Adding rustyline REPL
```rust
// In cli/repl.rs
use rustmax::rustyline::{DefaultEditor, Result};

pub fn run_repl(collection: &Collection) -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    loop {
        let line = rl.readline("anthology> ")?;
        // Parse and execute commands
    }
}
```

### Adding reqwest for remote content
```rust
// In collection/remote.rs
use rustmax::reqwest;

pub async fn fetch_remote(url: &str) -> Result<String> {
    let response = reqwest::get(url).await?;
    Ok(response.text().await?)
}
```

### Adding flate2 compression
```rust
// In build/compress.rs
use rustmax::flate2::write::GzEncoder;
use rustmax::flate2::Compression;

pub fn compress_assets(output_dir: &Path) -> Result<()> {
    // Compress HTML, CSS, JS files
}
```

### Adding tempfile for tests
```rust
// In tests/
use rustmax::tempfile::tempdir;

#[test]
fn test_build_output() {
    let dir = tempdir().unwrap();
    // Set up test collection
    // Run build
    // Verify output
}
```

## Test Coverage Goals

| Module | Current | Target |
|--------|---------|--------|
| collection/document | 3 tests | 10 tests |
| collection/scanner | 0 tests | 5 tests |
| collection/config | 0 tests | 5 tests |
| build/markdown | 0 tests | 5 tests |
| build/template | 0 tests | 5 tests |
| search | 1 test | 5 tests |
| cli integration | 0 tests | 10 tests |
| serve | 0 tests | 5 tests |

## Session Notes

### 2024-01-XX: Initial Implementation
- Created full project structure
- Implemented all core commands
- 9 tests passing
- Build and serve working

### 2026-01-03: Enhanced Features
- Added 17 integration tests using tempfile
- Implemented REPL mode with rustyline (commands: list, drafts, show, search, tags, by-tag, files, stats, recent)
- Added ctrlc handler for graceful server shutdown
- Added glob-based file filtering (CLI and REPL)
- Refactored with itertools for cleaner collection operations
- 26 total tests passing
- 24 rustmax crates now integrated

### 2026-01-03: Remote Content and Compression
- Added reqwest for remote content fetching (fetch command)
- Added flate2 for gzip asset compression (build --compress)
- Added regex for URL rewriting and content transforms
- Added futures for parallel async operations
- New modules: remote/, build/compress.rs, build/rewrite.rs
- 38 total tests passing (21 unit + 17 integration)
- 28 rustmax crates now integrated

### 2026-01-03: Encoding and Utilities
- Added bytes for binary asset handling (AssetBuffer)
- Added base64 for inline image encoding (data URLs)
- Added hex for hash display formatting
- Added rand for random ID generation
- New modules: build/encoding.rs, util/
- 52 total tests passing (35 unit + 17 integration)
- 32 rustmax crates now integrated

### 2026-01-03: Crypto, Text, and Time
- Added sha2 for SHA-256/512 hashing
- Added memchr for fast byte/substring searching
- Added chrono for date compatibility with jiff
- New modules: crypto/, text/, time/
- 82 total tests passing (65 unit + 17 integration)
- 35 rustmax crates now integrated

### 2026-01-03: Shortcodes and Feature Flags
- Added nom for shortcode parsing (inline and block shortcodes)
- Added bitflags for BuildFeatures, ContentFlags, ServerFlags
- New modules: shortcode/, features/
- 97 total tests passing (80 unit + 17 integration)
- 37 rustmax crates now integrated

### 2026-01-03: Advanced Concurrency
- Added crossbeam for work-stealing, scoped threads, channels
- New module: concurrency/ with TaskPool, Pipeline, parallel_map, parallel_for
- Progress reporting system with ProgressReporter
- Fan-out pattern for distributing work
- 106 total tests passing (89 unit + 17 integration)
- 38 rustmax crates now integrated (all targeted crates complete!)

### 2026-01-03: Live Reload
- Added WebSocket-based live reload for development server
- File watcher detects content, template, and CSS changes
- Hot CSS reload without full page refresh
- JavaScript injected into pages during dev serving
- 116 total tests passing (99 unit + 17 integration)

### 2026-01-03: Enhanced Search
- Added Porter stemmer for English word stemming
- Added stop word filtering (100+ common words)
- BM25-like scoring with term frequency and document length normalization
- Title boost for better relevance ranking
- Prefix matching for partial queries
- Autocomplete suggestions via `suggest()` method
- 123 total tests passing (106 unit + 17 integration)

### 2026-01-03: Incremental Builds
- Added build cache for tracking document content hashes
- Detects template changes and forces full rebuild when needed
- Prunes cache entries for deleted documents
- CLI: `anthology build --incremental` or `anthology build -i`
- 130 total tests passing (113 unit + 17 integration)

### 2026-01-03: Syntax Highlighting
- Added comprehensive syntax highlighting system with regex-based tokenizer
- Supports 14 languages: Rust, JavaScript, Python, Go, Bash, SQL, JSON, YAML, TOML, HTML, CSS, Markdown, C, C++
- 8 built-in color themes: GitHub, GitHub Dark, Monokai, Dracula, One Dark, Solarized Light/Dark, Nord
- Configurable line numbers and copy-to-clipboard button
- Integrated with markdown pipeline (auto-highlights code blocks)
- Generates highlight.css in build output
- New config section: `[highlight]` with theme, line_numbers, copy_button options
- 166 total tests passing (149 unit + 17 integration)

### 2026-01-03: Table of Contents
- Added TOC generation from markdown headings
- Hierarchical structure with proper nesting (h1 > h2 > h3, etc.)
- Custom heading IDs via `{#custom-id}` syntax
- Automatic unique ID generation from heading text
- `{{< toc >}}` shortcode for inline TOC insertion
- Shortcode options: min/max level, title visibility
- Template variables: `toc`, `toc_list`, `has_toc`
- Generates toc.css with styling
- 190 total tests passing (173 unit + 17 integration)

### 2026-01-03: Search API and Feed Generation
- Added search API endpoint to dev server (`/api/search?q=query`, `/api/search/suggest?q=prefix`)
- Created client-side search.js for static sites (BM25 scoring, stemming, prefix matching)
- Auto-generates search-index.json during build
- Added Atom 1.0 feed generation (`anthology export --format atom`)
- Added JSON Feed 1.1 generation (`anthology export --format json-feed`)
- New module: `feeds/` with Atom and JSON Feed support
- New module: `build/search_js.rs` for client-side search assets
- 200 total tests passing (183 unit + 17 integration)

### 2026-01-04: New Crate Integration (v0.7)
- Added native file watching with `notify` crate (replaces polling in dev server)
- Added progress bars with `indicatif` crate (`anthology build --progress`)
- Added image optimization with `image` crate (resize, thumbnails, WebP conversion)
- Added EPUB export with `zip` crate (`anthology export --format epub`)
- New modules: `build/progress.rs`, `build/images.rs`, `export/epub.rs`
- Updated serve/livereload.rs to use notify for instant file change detection
- 222 total tests passing (205 unit + 17 integration)
- 42 rustmax crates now integrated (all targeted crates complete!)

### v0.7 - New Crate Integration (COMPLETE)
- [x] Native file watching with `notify` crate (replaced polling)
- [x] Progress bars with `indicatif` crate (build progress, spinners)
- [x] Image optimization with `image` crate (resize, format conversion, WebP)
- [x] EPUB export with `zip` crate (package HTML into EPUB3)

### 2026-01-04: URL, MIME, and Property Testing (v0.8)
- Added URL parsing and validation with `url` crate (parse_url, filename_from_url, domain_from_url)
- Added MIME type detection with `mime` crate (mime_from_extension, mime_from_url)
- Added property-based testing with `proptest` crate (document and search tests)
- New functions in remote/mod.rs for URL handling
- 13 proptest properties verifying hash, stemmer, and document parsing invariants
- 241 total tests passing (224 unit + 17 integration)
- 45 rustmax crates now integrated

### Future Considerations
- Plugin system for custom build steps
- PDF export
- Fuzzy search
- Better error messages with source locations
