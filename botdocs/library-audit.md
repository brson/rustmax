# Library quality audit

The books in `work/library` are rendered by `rmxbook`
(`crates/rustmax-cli/src/rmxbook.rs` and its submodules),
not by mdbook, so every mdbook feature and third-party preprocessor
a book depends on has to be reimplemented or it silently degrades.
The build reports success regardless.

This records an audit on 2026-11-26 against the upstream native builds,
what was fixed, and what is still missing.

## How to re-check

- `just doc-check-library-links` reports broken internal links and anchors per book.
- Fidelity against upstream was measured by fetching each book's pages from
  its `upstream_url` and comparing word, code block, heading, image and table counts.
  That script was ad hoc and not kept; the method is easy to redo.
  Beware version skew: upstream sites often serve a different version from our pin
  (cargo/rustc/rustdoc pins track master while upstream_url is stable,
  rust-cookbook's site lags its repo), and upstream mdbook keeps hidden code lines
  in the HTML, so its word counts run high.

## Link check after the fixes

| Book | Checked | Missing | Bad anchors |
|---|---|---|---|
| reference | 8455 | 1 | 0 |
| rfcs | 10961 | 14 | 118 |
| rustc-dev-guide | 2766 | 0 | 22 |
| rust-cookbook | 952 | 0 | 10 |
| rust-forge | 1189 | 0 | 2 |
| edition-guide, nomicon, unsafe-code-guidelines | | 0 | 1 each |
| all others | | 0 | 0 |

Every one of these was checked against the upstream page and is broken upstream too
(typos like `enumerations.mdr`, anchors to renamed headings, mixed-case fragments).
Before the fixes there were about 1100 broken links and anchors,
plus about 150 broken images.

## What was wrong, and what rmxbook now does

- Assets: no non-markdown files were copied, so every image was broken.
  Everything in the source directory that isn't `.md` is now copied.
- SUMMARY.md: nesting assumed 2-space indents, so in most books the second and later
  siblings nested under the first. Indentation of any width now works, and part titles,
  separators, draft chapters and mdbook's section numbers (`2.3.`) are rendered.
  `no-section-label` is honored.
- `{{#include}}`, `{{#rustdoc_include}}`, `{{#playground}}`, `{{#title}}` and the
  `\{{#...}}` escape are implemented following mdbook's own `links` preprocessor
  (`rmxbook/include.rs`): line ranges, anchors, nesting, indentation.
  rust-cookbook went from 24% of upstream words to 98%; trpl got all its listings and outputs.
- Hidden lines (`# ` in Rust blocks, plus `[output.html.code.hidelines]`) are dropped.
  There is no toggle to show them, unlike mdbook.
- Unlabeled and indented code blocks are plain text, as in mdbook, not Rust.
- Fence attributes are kept as classes, and trpl's `does_not_compile`, `panics` and
  `not_desired_behavior` get a visible note in place of upstream's Ferris icons.
- trpl's `<Listing>` tags become figures with file name and numbered caption
  (`rmxbook/listing.rs`, matching the `trpl-listing` preprocessor), including inside blockquotes.
- GitHub-style `> [!NOTE]` alerts, description lists and smart punctuation
  (on unless `smart-punctuation = false`) via comrak options.
- `## Heading {#custom-id}` attributes become an anchor rather than literal text.
- `README.md` chapters render as `index.html`, and links to them follow.
- `[output.html.redirect]` entries get meta-refresh stub pages
  (fragment-only redirects, which mdbook does in script, are skipped).
- Links between books: rust-lang books link to each other relatively, assuming the
  doc.rust-lang.org layout (`../std/`, `../book/`, `../cargo/`). Any relative link that
  leaves its book is now resolved against the book's `upstream_url`, then pointed at our
  copy if the library has that book, or at the upstream URL if not.
  Raw HTML `href`/`src` attributes are rewritten too.
- The Reference: grammar blocks render with an anchor per production and every
  nonterminal linked to its production, links by production name (`[Name]`,
  `grammar-Name`) resolve, and the generated grammar summary page is filled in
  (`rmxbook/spec/grammar.rs`). Links to std items written rustdoc-style
  (`[bool]`, `proc_macro::TokenStream`, `mod@test`) go to a std docs search.
- The rustc book's lint listing pages are generated from the compiler's
  `declare_lint!` docs, as rustc's `lint-docs` tool does (`rustc_lints.rs`).
- Mermaid blocks render with mermaid from a CDN, on pages that have them.
- A footer on every library page names the upstream book and pinned commit
  and links to the same page upstream.
- The sidebar scrolls to the current chapter, and arrow keys turn pages.
- Wide tables scroll instead of overflowing; a little CSS stands in for per-book
  theme CSS (cargo's `option-anchor`, trpl's `caption`, nomicon's `warning`).

## Still missing

- The Reference's railroad diagrams, "Tests with this rule" lists, and test summary
  page (which needs the rust-lang/rust test suite; the page says so).
- The rustc book's lint listings omit the example compiler output that `lint-docs`
  produces by running the freshly built compiler (about two thirds of upstream's words
  are there). `lints/groups.html` is also generated by running rustc and is mostly empty.
- Rust Forge's `blacksmith` tables (installers and source archives), which need
  release-channel data from the network; the pages carry a notice instead.
- MathJax. Only the mdbook guide (documenting MathJax) and a few RFCs use it.
- Search within books, the print page, playground and copy buttons, a theme toggle.
- Links into a redirected page lose their fragment, as they do upstream.
- No `<link rel="canonical">` to the upstream pages. That would tell search engines
  to prefer doc.rust-lang.org over our copies, which is a decision about whether the
  library should draw search traffic, so it was left alone.
- The sidebar is inlined in every page; in rfcs that is about a third of each page.
