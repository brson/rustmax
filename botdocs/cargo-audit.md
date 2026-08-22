# cargo audit status

Notes on `cargo audit` (`just maint-audit`) findings that survive a plain
`cargo update`, so future sessions don't re-investigate them.

As of 2026-08-22 the main workspace and the `demoapp` workspace both exit 0.
Two "unmaintained" warnings remain. Neither is a vulnerability, and neither is
fixable from this repo without dropping functionality.

## RUSTSEC-2025-0141 - bincode 1.3.3 unmaintained

Path: `syntect 5.3` -> `bincode 1`, pulled in by `rustmax-cli`,
`rustmax-prebuild`, and `rustmax-rustdoc`.

We already build syntect with `default-features = false`, but the
`default-syntaxes` and `default-themes` features load syntect's packed dumps,
which go through `bincode` 1.x. Dropping them means parsing `.sublime-syntax`
and `.tmTheme` files at runtime instead, which is a large change for no
security benefit. Fix belongs upstream in syntect (move to bincode 2).

## RUSTSEC-2024-0436 - paste 1.0.15 unmaintained

Path: `image` -> `exr` -> `pulp` -> `paste`, and `image` -> `ravif` ->
`rav1e` -> `paste`.

Both come from `image?/default` in the `rmx-feature-default` feature. Turning
off image's `exr` and `avif` support would silence it but would remove formats
that rustmax deliberately exposes. `paste` is a proc macro with no known
vulnerability; leave it.

## Unrelated: cargo deny

`cargo deny check` currently fails to even load the advisory database
("unsupported CVSS version: 4.0") with the installed cargo-deny; that needs a
cargo-deny upgrade. `cargo deny check licenses` also fails, and has been
failing independently of any lockfile update - the rejected crates are mostly
MPL-2.0 (cssparser, selectors, dtoa-short, html2md) plus a few unparsable SPDX
expressions. Use `cargo deny check licenses bans sources` to skip the
advisory-db load.
