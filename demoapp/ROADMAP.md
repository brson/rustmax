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

### Partially Used
- [ ] bytes - Could use for binary asset handling

### Not Yet Used (High Priority)
- [ ] reqwest - Remote content fetching
- [ ] flate2 - Asset compression

### Not Yet Used (Medium Priority)
- [ ] regex - URL rewriting, content transforms
- [ ] nom - Custom syntax parsing
- [ ] chrono - Alternative dates (compatibility)
- [ ] base64 - Inline image encoding
- [ ] hex - Hash display
- [ ] sha2 - Alternative hashing

### Not Yet Used (Lower Priority)
- [ ] memchr - Fast searching
- [ ] bitflags - Feature flags
- [ ] rand - Random IDs
- [ ] crossbeam - Advanced concurrency
- [ ] futures - Async utilities

## Feature Roadmap

### v0.2 - Developer Experience
- [ ] Live reload with WebSocket (tokio, futures)
- [ ] REPL mode for quick queries (rustyline)
- [ ] Graceful shutdown (ctrlc)
- [ ] Better error messages with source locations

### v0.3 - Build Optimization
- [ ] Incremental builds using content_hash
- [ ] Asset compression (flate2)
- [ ] Parallel template compilation
- [ ] Build cache persistence

### v0.4 - Content Features
- [ ] Remote content fetching (reqwest)
- [ ] Custom shortcodes (nom)
- [ ] Image optimization
- [ ] Table of contents generation
- [ ] Syntax highlighting themes

### v0.5 - Search Enhancement
- [ ] Stemming for better matches
- [ ] Fuzzy search
- [ ] Search API endpoint
- [ ] Client-side search JS

### v0.6 - Export Formats
- [ ] EPUB export
- [ ] PDF generation
- [ ] Atom feed
- [ ] JSON Feed

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

### Next Session Priorities
1. Add reqwest for remote content fetching
2. Add flate2 for asset compression
3. Add regex for URL rewriting
4. Improve search with stemming
