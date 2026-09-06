# Site search

How search works, and what is still wrong with it.


## Architecture

Search is curated, not full-text. The corpus is the topic index, not the site.

```
src/topics/*.toml          hand-written topics: name, aliases, category, brief
  |  rustmax export-search-index          (crates/rustmax-cli/src/topics.rs)
  v
work/search-index.json     115 entries, ~7.8 KB gzipped, copied to out/
  |
  +--> www/search-core.js  matching and ranking, shared by browser and CLI
        |                    |
        |                    +--> www/search.js       dropdown UI on index.html
        |                    +--> www/search-cli.js   node wrapper
        |                           ^
        |                           +-- `rustmax search` spawns node against it
```

`search-core.js` is written in ES5 with `var` so it loads as a plain
`<script>` in the browser and via `require()` in node. `search.js` is
browser-only and uses modern syntax.

Categories are `crate`, `book`, `std`, defined in `src/topics/categories.toml`.


## Paths

Each entry carries a site-relative `path`. It is derived from the topic id
and category by `derive_path` in `topics.rs`, and a topic may override it
with an explicit `path` field in its TOML.

The std overrides exist because rustdoc documents std's re-exported modules
under `core` and `alloc`. There is no `api/std/option/index.html`; the real
page is `api/core/option/index.html`. 17 std topics carry explicit paths for
this reason. Do not "simplify" them back to the derivation.

`rustmax validate-links` checks every path in the search index against the
built site, so a wrong path fails `just doc-build` instead of shipping a
404. It also needs `work/search-index.json` to exist, which `doc-www`
guarantees within `doc-build`.


## Ranking

Match type is the primary signal: exact 1.0, prefix 0.9, word-prefix
substring 0.6. Category weight multiplies it as a tie-breaker only.

The weights (`std` 1.10, `crate` 1.05, `book` 1.00) are deliberately kept
within a ratio of 1.0/0.9 — the smallest gap between two match types — so a
category can never promote a worse match over a better one. If you raise a
weight past that ratio, a prefix match on a std module starts outranking an
exact match on a crate. That was the old behaviour and it was wrong.

The dropdown groups by category in the order categories first appear.
Because results arrive score-sorted, the top-scoring result is always the
first row. Do not reintroduce a fixed category display order: it buries the
best match and makes the weights meaningless.


## Escaping

Topic names and aliases are Rust source fragments, so they contain markup
characters — `Box<dyn Error>`, `&str`, `#[derive(Serialize)]`. Everything
interpolated into `innerHTML` in `search.js` must go through `escapeHtml`.
Without it, `Box<dyn Error>` is parsed as a tag and silently disappears.


## Known gaps

Reviewed 2026-09-06. Fixed then: broken std paths, missing escaping,
display order ignoring score, no path validation, cli tests not running.

Still outstanding, roughly by value:

- **No multi-word matching.** The query is matched as one literal string
  against each alias, so `"hash map"` returns nothing while `"HashMap"`
  works. Needs tokenizing plus a coverage-based score.
- **No tests for `search-core.js`.** There is no JS test runner in the repo
  and no golden-query corpus. The whole matching and ranking algorithm is
  unverified. Worth porting the algorithm to Rust so it can be tested with
  the rest of the suite, keeping the JS as a thin mirror.
- **`rustmax search` needs node** and repo-relative default paths, so it
  only works from a checkout with node installed. CI never runs it.
- **Word-prefix substring matches are unweighted**, so `"cli"` matches
  reqwest via "http client" and `"ser"` matches axum via "web server", each
  scored the same as a full-word hit.
- **Coverage is only the 115 topics** — 69 crates, 11 books, 35 std modules.
  The Rustmax book's own chapters, the tools list, and news posts are not
  searchable. `brief` text is not searched either.
- **No accessibility.** No combobox/listbox roles, no `aria-activedescendant`,
  no label on the input, no live region.
- **No empty state.** Zero results and a failed index fetch both just hide
  the dropdown, so they look identical to the user.
- **Index loads on first focus** with no re-run when it arrives, so
  keystrokes during the fetch silently match nothing.
- **Categories are shown by id** (`crate`, `std`) rather than the display
  names already sitting in `categories.toml`, which the index does not carry.
- No match highlighting, no results page, no typo tolerance, no faceting.
- The relation types sketched in `topic-index-format.md` were never built.
