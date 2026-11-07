# Code browser

A static, cross-referenced Rust source browser, built as part of rustmax.
Phases 1 to 4 below are implemented in `crates/rustmax-codebrowser`,
run as `rustmax browse [WORKSPACE] [-o OUT] [--scip INDEX] [--rustdoc]`
or `just browse`.
The rest of this file is the original investigation and design.


## Implementation notes

- Output defaults to `target/rmxbrowse/`. The SCIP index and
  rust-analyzer's log go to `target/rmxbrowse-scip/`. `--scip` reuses an
  index and skips the slow step: on this workspace indexing takes 10-25 s
  and rendering about 3 s.
- Modules: `metadata` (`cargo metadata`), `index` (runs rust-analyzer,
  turns SCIP into symbols, occurrences, and per-file locals), `symbol`
  (parses SCIP symbol strings, builds docs.rs / doc.rust-lang.org search
  URLs), `highlight` (syntect plus link overlay), `files` (walks the
  workspace with `ignore`), `graph` (layered crate graph layout and SVG),
  `summary` (dependency usage, per-file definition counts, file
  ownership by package), `render` (tera pages, symbol shards, search).
- Summary pages: the root page embeds the member crate graph (dev edges
  dashed, laid out in-house since graphviz is not assumed). Package pages
  list their Rust files with definition counts. `_rmx/deps.html` groups
  references to external items by defining crate; its item buttons open
  the same references panel as source pages.
- `--rustdoc` runs `cargo +JSON_TOOLCHAIN rustdoc --document-private-items`
  with JSON output once per lib, proc-macro, and bin target, into
  `target/rmxbrowse-rustdoc/`, reading each JSON immediately because
  `rustmax-cli`'s bin and `rustmax`'s lib both write `rustmax.json`. The
  toolchain pin is `rustmax_rustdoc::JSON_TOOLCHAIN`, kept in sync with the
  justfile's `nightly`. The first run here took about 90 s.
- rustdoc items are joined to SCIP symbols by span (the SCIP definition
  with the item's name inside the item's span), or by display path for
  items from other workspace crates. This gives types their impl blocks,
  traits and trait methods their implementations, and impl methods the
  trait method they implement. Blanket and auto-trait impls are skipped.
  Derived impls point at the `#[derive]` line, so several derives on one
  line collapse to one entry.
- Dependency usage is grouped by the crate rust-analyzer says defines an
  item, so re-exports through a facade (rmx, serde) are attributed to the
  underlying crate, and std items mostly show up under `core` and `alloc`.
- Symbol data goes to `_rmx/sym/N.js` (256 symbols per shard) and the
  search index to `_rmx/search.js`. They are JS callbacks, not JSON,
  because `fetch` does not work from `file://` URLs.
- The highlighter closes and reopens open syntect scopes at each line end,
  so each line is a self-contained `<span class="rb-line" id="Ln">`.
- All browser CSS classes are prefixed `rb-`. syntect emits a class per
  scope atom (`comment line`, `meta function`, `meta path`), so unprefixed
  names like `.line` or `.meta` restyle highlighted code.
- rust-analyzer's symbol format differs between versions. The rustup
  component (1.97.1) writes crate-root items with no module descriptor and
  the crate root as `crate/`; the 2026 standalone release wrote `lib/`.
  `symbol.rs` handles the rustup format.
- External links go through rustdoc search (`?search=Type::method`)
  because SCIP gives the defining path, which is often private.
- Hover cards for external symbols show no docs or signature: rust-analyzer
  emits no `external_symbols` information.
- No TOML syntax in syntect's default set, so manifests render as plain
  text with a package panel (targets, resolved dependencies, features)
  above them.

The goal: run against a cargo workspace, emit a large static site that
mirrors the workspace tree, with every source file and manifest replaced by
an interactive page where identifiers link to their definitions and
references, plus summary pages that show the whole workspace at a glance.

It is code-oriented, not doc-oriented: the primary page is a source file,
not an item.


## Prior art

- **Searchfox** (Mozilla). Static per-file HTML generated offline, plus a
  small server for search and cross-reference queries. Its Rust support
  comes from rust-analyzer's SCIP output, the same route recommended here.
- **Woboq Code Browser**. Clang-based C++ browser that generates a purely
  static site: hyperlinked source, hover tooltips, uses loaded lazily from
  per-symbol files by JS. The closest match to the "massive static site"
  shape.
- **Elixir** (Bootlin) and **LXR / OpenGrok**. The classic hyperlinked
  mirror of a source tree, identifier-based rather than semantic.
- **Kythe** (Google) and **Sourcegraph**. Semantic indexes behind a
  server. SCIP is Sourcegraph's format, and rust-analyzer emits it natively.
- **rustdoc source pages** with `-Zunstable-options
  --generate-link-to-definition`. Jump-to-definition for some items in
  rustdoc's `src/` view. Nightly, partial, no find-references.
- **DXR / rustw / RLS**. Built on rustc's save-analysis, which was removed
  in 2022. A dead end, mentioned because old blog posts point at it.
- **cargo-modules**, **cargo-depgraph**, `cargo tree`. Module and crate
  graphs, i.e. the summary-page half of this tool.


## Data sources

Three inputs, each answering different questions.

### `cargo metadata`

Packages, targets, features, the resolved dependency graph, and manifest
paths. Drives the directory structure, the manifest pages, and the crate
graph. Plain JSON, stable toolchain.

### rust-analyzer SCIP (`rust-analyzer scip .`)

The backbone for source pages. Measured on this workspace (2026-11-07,
rust-analyzer 0.3.3105 standalone):

- 24 s wall clock, 2.6 MB index.
- 48 documents (workspace members only), 25,406 occurrences, 4,137
  definitions, 4,167 symbol infos.
- Every identifier occurrence is resolved, including locals, method calls
  through trait impls (`anyhow ... impl#[Result<T, E>][Context<T, E>]with_context().`),
  macros, and std items.
- External symbols carry crate name and version
  (`rust-analyzer cargo serde_core 1.0.229 lib/de/Deserialize#`), which is
  enough to build a link to the dependency's docs or indexed source.
- Symbol infos carry kind and documentation (signature plus doc markdown),
  enough for hover cards.

Gaps observed:

- `relationships` is empty: no "implements" edges. Trait/impl navigation
  has to come from rustdoc JSON.
- `syntax_kind` is never populated, so highlighting stays with syntect and
  links are overlaid on its output.
- Only workspace members are indexed. Dependencies appear only as symbol
  references, not documents.
- rust-analyzer logged "duplicate symbol" and "enclosing definition with no
  name" bugs on this workspace (functions nested in functions, see
  rust-analyzer #18771). Lookups for those symbols are ambiguous.
- One cfg configuration only. Code under inactive `cfg` gets no links.
- Works on stable via `rustup component add rust-analyzer`; std symbols
  need `rust-src`.

Reading it needs the `scip` crate, which pulls in `protobuf`. Neither is in
rmx.

### rustdoc JSON

What `rustmax-rustdoc` already consumes. Supplies what SCIP lacks: impl
blocks, trait implementors, rendered signatures, and item spans (file,
line, column) that can be joined to SCIP definitions. Nightly-only, and
public items only unless `--document-private-items`.

SCIP symbol descriptors (`lib/parse/load_json().`) map mechanically to
rustdoc paths (`rustmax_rustdoc::parse::load_json`), so item pages in an
existing rustmax-rustdoc build can be linked from source pages.


## Output shape

```
out/
  index.html                         workspace summary
  graph/crates.html                  crate graph
  crates/<pkg>/index.html            per-package summary
  tree/                              mirror of the workspace
    Cargo.toml.html
    crates/rustmax-rustdoc/
      index.html                     directory listing
      Cargo.toml.html
      src/parse.rs.html
  refs/<hh>/<symbol-hash>.json       references per symbol, fetched lazily
  hover/<path>.json                  hover cards per source file
  search-index.json                  symbol names for client-side search
```

Source pages: syntect-highlighted, line anchors (`#L12`), each identifier
wrapped as `<a href="def-location" data-s="symbol-id">`. Clicking a
definition opens a references panel filled from `refs/`. Hover cards come
from a per-file sidecar so the HTML itself stays a few times the source
size.

Manifest pages: dependency names link to the crate page, workspace
inheritance (`version.workspace = true`) is shown resolved, features are
expanded to show what each one enables.

Search stays static: a name-only symbol index like rustdoc's. Full-text
search is out of scope for a static site; Searchfox needs a server for it.


## Summary pages

- Crate graph of workspace members and their direct dependencies, from
  `cargo metadata`. `dot` if present, or a pure-Rust layout (`layout-rs`).
- Per-crate module tree with lines of code and definition counts by kind.
- Dependency usage: for each dependency, which workspace files reference
  its symbols, computed from SCIP occurrences. Answers "where do we
  actually use serde" and is not something rustdoc or docs.rs can show.
- Trait implementors across the workspace, from rustdoc JSON.
- Feature graph per package.


## Fit in rustmax

- A new library crate, e.g. `rustmax-codebrowser`, driven by a CLI
  subcommand along the lines of `rustmax browse <workspace> --out <dir>`.
- Reuse from `rustmax-rustdoc`: tera template setup, `base.html`, the
  syntect highlighter, `rustmax-themes.css` and `rustmax-syntax.css`, the
  comrak markdown renderer for hover docs, and the cross-crate link index.
  Some of this would move into a shared module or crate.
- New dependencies: `scip` and `protobuf`, plus something for
  `cargo metadata` (`cargo_metadata` or a serde struct of our own).
- External rust-analyzer binary, invoked as a subprocess, like the nightly
  rustdoc invocation today.


## Phases

Phases 1 to 4 are done, except that phase 4 does not link to
rustmax-rustdoc pages: those cover public items only and would need a
hosting arrangement between the two sites.

1. Mirror tree: `cargo metadata` plus SCIP to highlighted source pages with
   go-to-definition, line anchors, directory listings, manifest pages.
   Dogfood on this workspace.
2. Find-references panels, hover cards, symbol search.
3. Summary pages: crate graph, module trees, dependency usage.
4. rustdoc JSON join: impl and implementor navigation, links into
   rustmax-rustdoc item pages, external symbols linked to the rmx docs or
   docs.rs at the version SCIP reports.
5. Dependency source: index dependencies as documents too. rust-analyzer
   has no flag for this; one option is a synthetic workspace with vendored
   dependencies as path members. Unexplored.


## Open questions

- Whether to make dependency source browsable at all, or link out. It is
  what turns "big" into "massive".
- How to present inactive-cfg code: highlighted but unlinked and dimmed
  seems honest.
- Incremental rebuilds: SCIP is all-or-nothing per run, but page
  generation could skip files whose source and inbound symbols are
  unchanged.
