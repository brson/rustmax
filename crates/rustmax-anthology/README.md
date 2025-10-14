# rustmax-anthology

A self-contained crate for curating and presenting a collection of notable Rust blog posts and essays.

This project takes the concept from [rust-anthology](https://github.com/brson/rust-anthology) and transforms it into a working system that:
- Fetches blog posts from URLs
- Extracts main content from HTML
- Converts to markdown
- Stores everything in git for reproducibility
- Generates a presentation (via mdbook)

## Directory Structure

```
crates/rustmax-anthology/
├── metadata/posts.toml     # Master list of posts with URLs, authors, etc.
├── fetched/                # Fetched content (checked into git)
│   └── [post-id]/
│       ├── raw.html        # Original fetched HTML
│       ├── fetch-info.toml # Fetch metadata
│       ├── extracted.html  # Extracted main content
│       └── content.md      # Final markdown
└── book/                   # mdbook structure for presentation
```

## CLI Commands

### Quick Commands (via just)

From the project root (builds automatically via `cargo run`):

```bash
# List all posts
just anthology-list

# Check status of posts
just anthology-status

# Process a single post (fetch + extract + markdown)
just anthology-process [post-id]

# Process all posts
just anthology-process-all
```

### Direct Commands

Build the CLI:
```bash
cargo build -p rustmax-anthology
```

List all posts:
```bash
./target/debug/anthology --metadata-dir crates/rustmax-anthology/metadata \
                         --fetched-dir crates/rustmax-anthology/fetched \
                         list
```

Check status of posts:
```bash
./target/debug/anthology --metadata-dir crates/rustmax-anthology/metadata \
                         --fetched-dir crates/rustmax-anthology/fetched \
                         status
```

Process a single post (fetch + extract + markdown):
```bash
./target/debug/anthology --metadata-dir crates/rustmax-anthology/metadata \
                         --fetched-dir crates/rustmax-anthology/fetched \
                         process [post-id]
```

Process all posts:
```bash
./target/debug/anthology --metadata-dir crates/rustmax-anthology/metadata \
                         --fetched-dir crates/rustmax-anthology/fetched \
                         process all
```

Individual pipeline steps:
```bash
# Fetch only
./target/debug/anthology --metadata-dir crates/rustmax-anthology/metadata \
                         --fetched-dir crates/rustmax-anthology/fetched \
                         fetch [post-id|all]

# Extract only
./target/debug/anthology --metadata-dir crates/rustmax-anthology/metadata \
                         --fetched-dir crates/rustmax-anthology/fetched \
                         extract [post-id|all]

# Convert to markdown only
./target/debug/anthology --metadata-dir crates/rustmax-anthology/metadata \
                         --fetched-dir crates/rustmax-anthology/fetched \
                         to-markdown [post-id|all]
```

## Adding New Posts

Edit `metadata/posts.toml` and add a new entry:

```toml
[[posts]]
id = "unique-slug"
title = "Post Title"
author = "Author Name"
url = "https://example.com/post"
category = "Category Name"  # optional
extractor = "default"       # or custom extractor name
```

## Custom Extractors

The default extractor tries common HTML patterns (article, main, etc.). For sites that need special handling, add a custom extractor in `src/extractors/custom/`.

## Philosophy

This is a big ugly data transformation project. It embraces the mess:
- Custom hacks per site are expected and welcome
- Everything stored in git for reproducibility
- Focus on getting something working over perfect architecture
- Perfect for automation via AI tools

## Future Work

- mdbook integration for generating final presentation
- More custom extractors for common blog platforms
- Better error handling and retry logic
- Incremental updates (re-fetch only stale content)
- Support for detecting and handling redirects/dead links
