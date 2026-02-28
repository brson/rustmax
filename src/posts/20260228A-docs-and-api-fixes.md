---
title: "Documentation and API doc fixes"
category: news
summary: "New crate docs for the web stack and CLI crates, plus many API doc generator fixes."
---

A big round of crate documentation and API doc generator improvements.
It's becoming more viable to use as a real doc tool.

**New crate documentation and examples** -
Added doc pages for axum, tower, hyper, http, ctrlc, termcolor, and tera.

**API doc generator fixes** -
Getting the custom rustdoc renderer to behave as expected
and/or similar to rustdoc is a slog -
what rustdoc actually does to Rust's ASTs in a sensible way is pretty grungy.
And our implementation has grown organically;
fix one link, break another.
Complicated by my insistence on rendering fewer duplicate pages that rustdoc,
so deciding where the "real" docs live and routing links correctly is tough.
