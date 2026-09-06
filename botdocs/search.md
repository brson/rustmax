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
  +--> www/search-core.js            matching and ranking, browser
  |      +--> www/search.js          dropdown UI on index.html
  |
  +--> crates/rustmax-cli/src/search.rs   matching and ranking, CLI
         +--> `rustmax search`
```

There are two implementations of one algorithm, one per language. The CLI
used to shell out to node against `www/search-cli.js`; it no longer does,
so nothing in the product needs node.

`search-core.js` is written in ES5 with `var` so it loads as a plain
`<script>` in the browser and via `require()` under node. `search.js` is
browser-only and uses modern syntax.

Categories are `crate`, `book`, `std`, defined in `src/topics/categories.toml`.


## Parity between the two implementations

`crates/rustmax-cli/tests/search-corpus.json` is the contract. It carries a
small fixture index built to exercise the awkward cases — cross-category
ties, the stable-sort order of equal scores, the word-boundary rule, the
20-result cap, aliases containing markup — plus the expected results for 42
queries.

Both sides check themselves against it, and both run under `cargo test`:

- `search.rs`'s own tests compare the Rust results to the corpus.
- `the_javascript_implementation_matches_the_corpus` shells out to node to
  run `tests/search-parity.js`, which does the same for `search-core.js`.
  It skips when node is absent. Both CI runners ship node, so CI enforces it.

Verified when this was set up: changing a category weight in either
implementation alone fails that implementation's check.

To change the algorithm deliberately: change both sides, run
`just gen-search-corpus` (needs node), and review the corpus diff. Do not
regenerate to silence a failure — a diff there is exactly the drift the
corpus exists to catch.

Two things make parity practical, and both are worth preserving:

- **The index is ASCII.** Case folding is the one place where JS and Rust
  can genuinely disagree, and ASCII lowercasing is identical in both. A test
  asserts the corpus index stays ASCII. Adding a non-ASCII alias reopens
  the question.
- **Both languages sort stably and use IEEE 754 doubles**, so equal scores
  keep their input order on both sides and arithmetic agrees bit for bit.

One wrinkle: `serde_json` is built here without `float_roundtrip`, so its
fast float parser can land one ULP off a literal in the corpus. Scores are
therefore compared to a tolerance of 1e-9, while ids, order, match types
and matched aliases are compared exactly. Ordering is the observable
behaviour, and a score change large enough to matter shows up as a
reordering.


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
Then: the CLI's node dependency, replaced by the Rust port above.

Still outstanding, roughly by value:

- **No multi-word matching.** The query is matched as one literal string
  against each alias, so `"hash map"` returns nothing while `"HashMap"`
  works. Needs tokenizing plus a coverage-based score.
- **The corpus only pins behaviour, not quality.** It is generated from the
  current implementation, so it locks in the present ranking, including its
  flaws. It catches drift; it does not say the ranking is good.
- **`rustmax search` still defaults to a repo-relative index path**
  (`work/search-index.json`), so it only really works from a checkout.
  Embedding the index in the binary would fix that.
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
