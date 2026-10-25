# cargo audit status

Notes on `cargo audit` (`just maint-audit`) findings that survive a plain
`cargo update`, so future sessions don't re-investigate them.

As of 2026-10-25, after `cargo update` in both workspaces, neither the main
workspace nor `demoapp` has any vulnerabilities. That update fixed advisories
in rustls, rustls-webpki, webpki-root-certs, quinn-proto and ctutils, plus zip
in `demoapp`, whose lockfile was still on zip 8. Two "unmaintained" warnings
remain (only paste in `demoapp`). Neither is a vulnerability, and neither is
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

## cargo deny

With cargo-deny 0.20.2 the advisory database loads again. `cargo deny check
advisories` fails on the two unmaintained crates above, because cargo-deny
treats "unmaintained" as an error. `bans` and `sources` pass.

`cargo deny check licenses` fails on license policy, not on security:

- MPL-2.0: cssparser, cssparser-macros, dtoa-short, selectors (via scraper).
- GPL-3.0+: html2md, a direct dependency of `rustmax-anthology`.
- Apache-2.0 WITH LLVM-exception: ar_archive_writer.
- CDLA-Permissive-2.0: webpki-root-certs.
- Unicode-DFS-2016: finl_unicode. NCSA: libfuzzer-sys. bzip2-1.0.6: libbz2-rs-sys.

Expanding the allow list in `deny.toml` is a policy call for the maintainer.
