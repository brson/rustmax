# Folio Roadmap

What is worth doing next, and why.
Design rationale lives in [DESIGN.md](DESIGN.md);
what this app has found in `rustmax` lives in [VALIDATION.md](VALIDATION.md).

## Where things stand

Folio builds, searches, lints and exports a collection of markdown
documents, on `rustmax` as its only dependency.
263 tests, no clippy warnings.
53 of the 73 crates `rustmax` re-exports are used by a feature;
the other 20 are listed with reasons in DESIGN.md
and are not targets.

## Next, in order

### 1. A second demo for the profiles this one cannot reach

`rmx-profile-build-script` and `rmx-profile-proc-macro`
have no honest use in a site generator,
and the `syn` bug shows what that costs:
the proc-macro profile was unusable
and nothing in the tree would have noticed.

A small crate that actually writes a derive macro,
and one that actually runs a build script,
would cover `bindgen`, `cc`, `cxx`, `cxx-build`, `proc-macro2` and `quote`
the way Folio covers the rest.
It should be its own thing rather than a corner of this one --
padding a site generator with an FFI layer
is the checklist-filling that makes a demo app useless.

### 2. Attribute macros through the re-export

`#[rmx::derive]` solved derives.
Attribute macros in the same class -- `tokio::test`, `tokio::main` --
still need `use rmx::tokio;` at each use site.
That is documented and it works,
but the diagnostic is unhelpful (see VALIDATION.md).
Folio is the right place to try an `#[rmx::attr(..)]` against a real
test suite if `rustmax` grows one.

### 3. `folio serve`

A local server for the output, rebuilding through `folio watch`.
It would use `hyper`, `hyper-util` and `http-body-util`,
replacing the dev server that left with `axum`,
and live reload would give the watch loop a fourth event to select on.

### 4. Lints worth having

`folio check` parses Rust code blocks, flags unhighlightable
languages and reports unknown shortcodes. The obvious next ones, in the order they would pay off:

- Internal links that resolve to no document.
- Images referencing files that are not in `static/`.
- Frontmatter dates in the future on non-draft documents.

Each is a genuine authoring mistake and none needs a new dependency.

### 5. Things deliberately not planned

- **Plugin system.** Named in the old roadmap for two years without a use
  case behind it. A plugin system with no plugins is architecture for its
  own sake.
- **PDF export.** No crate in `rustmax` does it, and adding a dependency
  outside `rustmax` would give up the property that makes this app
  interesting.
- **Fuzzy search.** The BM25 index with prefix matching and stemming is
  already better than the collections it will be pointed at.
