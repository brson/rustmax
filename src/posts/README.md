# Posts

This directory contains markdown posts for the Rustmax feed.

## File Format

Posts should be named with the pattern: `YYYY-MM-DD-slug.md`

Example: `2025-10-12-awesome-crate.md`

## Frontmatter

Each post must include YAML frontmatter with the following fields:

```yaml
---
title: "Post Title"
date: 2025-10-12
category: crates  # or tips, news
summary: "Brief one-liner for RSS and teasers"
---
```

## Content

After the frontmatter, write the post content in markdown.

## Categories

- `crates`: Posts about specific crates
- `tips`: Tips and tricks for Rust programming
- `news`: News and updates about Rustmax

## Output

Posts are generated to:
- Individual pages: `/feed/slug.html`
- Feed index: `/feed/index.html`
- RSS feed: `/feed/rss.xml`
- Latest post fragment for homepage: `/latest-post.html`
