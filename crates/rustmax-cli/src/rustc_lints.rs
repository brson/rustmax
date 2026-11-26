//! Generation of the rustc book's lint listing pages.
//!
//! The rustc book's `lints/listing/*-by-default.md` pages are stubs in the
//! source tree. rustc's build fills them in with its `lint-docs` tool
//! (`src/tools/lint-docs`), which collects the doc comment of every
//! `declare_lint!` in the compiler. This does the same.
//!
//! `lint-docs` also compiles each lint's example with the freshly built
//! compiler and pastes the diagnostics in place of a `{{produces}}` marker.
//! We have no compiler build to run, so those markers are dropped.

use rmx::prelude::*;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Level {
    Allow,
    Warn,
    Deny,
}

struct Lint {
    name: String,
    doc: Vec<String>,
    level: Level,
}

/// Lints that have been renamed, which keep an entry pointing at the new name.
///
/// Copied from `lint-docs`, which hard-codes the list the same way.
const RENAMES: &[(Level, &[(&str, &str)])] = &[
    (
        Level::Allow,
        &[
            ("single-use-lifetime", "single-use-lifetimes"),
            ("elided-lifetime-in-path", "elided-lifetimes-in-paths"),
            ("async-idents", "keyword-idents"),
            ("disjoint-capture-migration", "rust-2021-incompatible-closure-captures"),
            ("keyword-idents", "keyword-idents-2018"),
            ("or-patterns-back-compat", "rust-2021-incompatible-or-patterns"),
        ],
    ),
    (
        Level::Warn,
        &[
            ("bare-trait-object", "bare-trait-objects"),
            ("unstable-name-collision", "unstable-name-collisions"),
            ("unused-doc-comment", "unused-doc-comments"),
            ("redundant-semicolon", "redundant-semicolons"),
            ("overlapping-patterns", "overlapping-range-endpoints"),
            ("non-fmt-panic", "non-fmt-panics"),
            ("unused-tuple-struct-fields", "dead-code"),
            ("static-mut-ref", "static-mut-refs"),
        ],
    ),
    (Level::Deny, &[("exceeding-bitshifts", "arithmetic-overflow")]),
];

/// Write the three listing pages into `listing_dir` from the compiler
/// sources under `compiler_dir`.
pub fn generate(compiler_dir: &Path, listing_dir: &Path) -> AnyResult<()> {
    let mut lints = Vec::new();
    collect(compiler_dir, &mut lints)?;
    if lints.is_empty() {
        bail!("no lints found under {}", compiler_dir.display());
    }

    for (level, names) in RENAMES {
        for (from, to) in *names {
            lints.push(Lint {
                name: from.replace('-', "_"),
                doc: vec![format!("The lint `{from}` has been renamed to [`{to}`](#{to}).")],
                level: *level,
            });
        }
    }

    for (level, file, header) in [
        (Level::Allow, "allowed-by-default.md", ALLOWED_HEADER),
        (Level::Warn, "warn-by-default.md", WARN_HEADER),
        (Level::Deny, "deny-by-default.md", DENY_HEADER),
    ] {
        let mut these: Vec<&Lint> = lints.iter().filter(|l| l.level == level).collect();
        these.sort_by(|a, b| a.name.cmp(&b.name));

        let mut page = String::from(header);
        for lint in &these {
            writeln!(page, "* [`{}`](#{})", lint.name, lint.name.replace('_', "-"))?;
        }
        page.push('\n');
        for lint in &these {
            write!(page, "## {}\n\n", lint.name.replace('_', "-"))?;
            for line in &lint.doc {
                if line.trim() == "{{produces}}" {
                    continue;
                }
                page.push_str(line);
                page.push('\n');
            }
            page.push('\n');
        }
        fs::write(listing_dir.join(file), page)?;
    }

    Ok(())
}

fn collect(dir: &Path, lints: &mut Vec<Lint>) -> AnyResult<()> {
    let mut entries = fs::read_dir(dir)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            collect(&path, lints)?;
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            let contents = fs::read_to_string(&path)?;
            lints.extend(
                lints_in(&contents).with_context(|| format!("reading lints in {}", path.display()))?,
            );
        }
    }
    Ok(())
}

/// The documented lints declared in one source file.
fn lints_in(contents: &str) -> AnyResult<Vec<Lint>> {
    let mut lints = Vec::new();
    let mut lines = contents.lines();

    while lines.by_ref().any(|line| line.trim().starts_with("declare_lint!")) {
        let mut doc = Vec::new();
        let name = loop {
            let Some(line) = lines.next() else {
                bail!("unexpected end of file in a lint declaration");
            };
            let line = line.trim();
            if let Some(text) = line.strip_prefix("/// ") {
                doc.push(text.to_string());
            } else if line == "///" {
                doc.push(String::new());
            } else if let Some(text) = line.strip_prefix("#[doc = \"") {
                doc.push(unescape(text.trim_end_matches(']').trim_end_matches('"'))?);
            } else if line.starts_with("// ") || line.starts_with("#[") {
                // Comments, and attributes like `#[allow]` or the
                // bootstrap-dependent `#[cfg_attr(..., doc = ...)]`, which
                // only matter to a particular compiler stage.
            } else {
                let name = line
                    .split(' ')
                    .next_back()
                    .and_then(|n| n.strip_suffix(','))
                    .filter(|n| {
                        !n.is_empty()
                            && n.chars().all(|c| c.is_uppercase() || c.is_ascii_digit() || c == '_')
                    })
                    .ok_or_else(|| anyhow!("expected a lint name, got `{line}`"))?;
                break name.to_lowercase();
            }
        };

        let level = loop {
            let Some(line) = lines.next() else {
                bail!("expected the level of lint `{name}`");
            };
            match line.trim() {
                l if l.starts_with("// ") => {}
                "Allow," => break Level::Allow,
                "Warn," => break Level::Warn,
                "Deny," => break Level::Deny,
                other => bail!("unexpected level `{other}` for lint `{name}`"),
            }
        };

        // `lint-docs` leaves undocumented lints out, and this one on purpose.
        if doc.is_empty() || name == "deprecated_in_future" {
            continue;
        }
        lints.push(Lint { name, doc, level });
    }

    Ok(lints)
}

/// Unescape the contents of a Rust string literal.
fn unescape(s: &str) -> AnyResult<String> {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some('0') => out.push('\0'),
            Some(c @ ('\\' | '"' | '\'')) => out.push(c),
            Some('u') => {
                let hex: String = chars
                    .by_ref()
                    .skip_while(|&c| c == '{')
                    .take_while(|&c| c != '}')
                    .collect();
                let code = u32::from_str_radix(&hex, 16)?;
                out.push(char::from_u32(code).ok_or_else(|| anyhow!("bad escape \\u{{{hex}}}"))?);
            }
            other => bail!("unsupported escape \\{other:?}"),
        }
    }
    Ok(out)
}

const ALLOWED_HEADER: &str = "# Allowed-by-default Lints

These lints are all set to the 'allow' level by default. As such, they won't show up
unless you set them to a higher lint level with a flag or attribute.

";

const WARN_HEADER: &str = "# Warn-by-default Lints

These lints are all set to the 'warn' level by default.

";

const DENY_HEADER: &str = "# Deny-by-default Lints

These lints are all set to the 'deny' level by default.

";

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &str = r#"
declare_lint! {
    /// The `unused_thing` lint detects unused things.
    ///
    /// ### Example
    ///
    /// ```rust
    #[doc = "    println!(\"{:?}\", '\u{202E}');"]
    /// ```
    ///
    /// {{produces}}
    // A comment for tidy.
    pub UNUSED_THING,
    Warn,
    "detects unused things"
}

declare_lint! {
    pub UNDOCUMENTED,
    Allow,
    "no docs"
}
"#;

    #[test]
    fn test_lints_are_read_from_declarations() {
        let lints = lints_in(SOURCE).unwrap();
        assert_eq!(lints.len(), 1);
        let lint = &lints[0];
        assert_eq!(lint.name, "unused_thing");
        assert!(lint.level == Level::Warn);
        assert_eq!(lint.doc[0], "The `unused_thing` lint detects unused things.");
        assert!(lint.doc.contains(&"    println!(\"{:?}\", '\u{202E}');".to_string()));
    }

    #[test]
    fn test_pages_drop_the_produces_marker() {
        let dir = rmx::tempfile::tempdir().unwrap();
        let compiler = dir.path().join("compiler");
        fs::create_dir_all(&compiler).unwrap();
        fs::write(compiler.join("lints.rs"), SOURCE).unwrap();
        generate(&compiler, dir.path()).unwrap();

        let warn = fs::read_to_string(dir.path().join("warn-by-default.md")).unwrap();
        assert!(warn.contains("* [`unused_thing`](#unused-thing)"), "{warn}");
        assert!(warn.contains("## unused-thing\n\nThe `unused_thing` lint"), "{warn}");
        assert!(!warn.contains("{{produces}}"), "{warn}");
        assert!(warn.contains("## static-mut-ref\n\nThe lint `static-mut-ref` has been renamed"), "{warn}");
    }
}
