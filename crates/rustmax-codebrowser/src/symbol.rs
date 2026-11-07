//! Parsing of SCIP symbol strings, and links to external documentation.
//!
//! A global SCIP symbol looks like
//! `rust-analyzer cargo serde_core 1.0.229 de/Deserialize#`:
//! scheme, package manager, package name, version, then descriptors.
//! See <https://github.com/sourcegraph/scip/blob/main/scip.proto>.
//!
//! rust-analyzer writes items in the crate root with no module descriptor,
//! and the crate root itself as the namespace `crate/`.

/// A parsed global symbol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScipSymbol {
    pub package: String,
    pub version: String,
    pub descriptors: Vec<Descriptor>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Descriptor {
    pub name: String,
    pub suffix: Suffix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suffix {
    Namespace,
    Type,
    Term,
    Method,
    TypeParameter,
    Parameter,
    Meta,
    Macro,
}

/// The crates whose documentation lives on doc.rust-lang.org.
const SYSROOT_CRATES: &[&str] = &["std", "core", "alloc", "proc_macro", "test"];

impl ScipSymbol {
    /// Parse a global symbol, returning `None` for locals and malformed symbols.
    pub fn parse(symbol: &str) -> Option<ScipSymbol> {
        if symbol.starts_with("local ") {
            return None;
        }
        let mut parts = symbol.splitn(5, ' ');
        let _scheme = parts.next()?;
        let _manager = parts.next()?;
        let package = parts.next()?.to_string();
        let version = parts.next()?.to_string();
        let descriptors = parse_descriptors(parts.next()?)?;
        Some(ScipSymbol { package, version, descriptors })
    }

    /// The package name as a Rust identifier.
    pub fn crate_ident(&self) -> String {
        self.package.replace('-', "_")
    }

    /// The last named descriptor, which is what the symbol is called.
    pub fn name(&self) -> &str {
        self.descriptors
            .iter()
            .rev()
            .find(|d| !matches!(d.suffix, Suffix::TypeParameter | Suffix::Parameter))
            .filter(|d| !(d.suffix == Suffix::Namespace && d.name == "crate"))
            .map(|d| d.name.as_str())
            .unwrap_or(&self.package)
    }

    /// A Rust-like path for display and search, e.g. `serde_core::de::Deserialize`.
    ///
    /// rust-analyzer encodes impl blocks as a type named `impl`
    /// followed by the self type and trait as type parameters;
    /// the self type stands in for the impl.
    pub fn display_path(&self) -> String {
        let mut segments = vec![self.crate_ident()];
        let mut descriptors = self.path_descriptors().iter().peekable();
        while let Some(d) = descriptors.next() {
            match d.suffix {
                Suffix::TypeParameter | Suffix::Parameter => {}
                Suffix::Type if d.name == "impl" => {
                    if let Some(self_ty) = descriptors.next_if(|d| d.suffix == Suffix::TypeParameter) {
                        segments.push(self_ty.name.clone());
                    }
                    while descriptors.next_if(|d| d.suffix == Suffix::TypeParameter).is_some() {}
                }
                _ => segments.push(d.name.clone()),
            }
        }
        segments.join("::")
    }

    /// A link to documentation for an item defined outside the workspace.
    ///
    /// Items are linked through rustdoc's search,
    /// since SCIP gives the defining module path,
    /// which is often private and re-exported elsewhere,
    /// and does not say whether a type is a struct, enum, or trait.
    pub fn docs_url(&self) -> String {
        let base = if SYSROOT_CRATES.contains(&self.package.as_str()) {
            format!("https://doc.rust-lang.org/{}/", self.package)
        } else {
            format!("https://docs.rs/{}/{}/{}/", self.package, self.version, self.crate_ident())
        };
        match self.search_query() {
            Some(query) => format!("{base}?search={}", url_encode(&query)),
            None => base,
        }
    }

    /// The descriptors below the crate root.
    fn path_descriptors(&self) -> &[Descriptor] {
        match self.descriptors.first() {
            Some(d) if d.suffix == Suffix::Namespace && d.name == "crate" => &self.descriptors[1..],
            _ => &self.descriptors,
        }
    }

    /// The rustdoc search query for this symbol, `None` for the crate root.
    ///
    /// Members of types are qualified by the type, e.g. `Path::new`.
    fn search_query(&self) -> Option<String> {
        let named: Vec<&Descriptor> = self
            .path_descriptors()
            .iter()
            .filter(|d| !matches!(d.suffix, Suffix::TypeParameter | Suffix::Parameter))
            .collect();
        let (last, rest) = named.split_last()?;
        match rest.last() {
            Some(parent) if parent.suffix == Suffix::Type && parent.name != "impl" => {
                Some(format!("{}::{}", parent.name, last.name))
            }
            _ => Some(last.name.clone()),
        }
    }
}

fn parse_descriptors(mut s: &str) -> Option<Vec<Descriptor>> {
    let mut out = Vec::new();
    while !s.is_empty() {
        let (descriptor, rest) = match s.as_bytes()[0] {
            b'[' => {
                let (name, rest) = parse_name(&s[1..])?;
                let rest = rest.strip_prefix(']')?;
                (Descriptor { name, suffix: Suffix::TypeParameter }, rest)
            }
            b'(' => {
                let (name, rest) = parse_name(&s[1..])?;
                let rest = rest.strip_prefix(')')?;
                (Descriptor { name, suffix: Suffix::Parameter }, rest)
            }
            _ => {
                let (name, rest) = parse_name(s)?;
                let (suffix, rest) = match rest.as_bytes().first()? {
                    b'/' => (Suffix::Namespace, &rest[1..]),
                    b'#' => (Suffix::Type, &rest[1..]),
                    b'.' => (Suffix::Term, &rest[1..]),
                    b':' => (Suffix::Meta, &rest[1..]),
                    b'!' => (Suffix::Macro, &rest[1..]),
                    b'(' => {
                        let close = rest.find(')')?;
                        let rest = rest[close + 1..].strip_prefix('.')?;
                        (Suffix::Method, rest)
                    }
                    _ => return None,
                };
                (Descriptor { name, suffix }, rest)
            }
        };
        out.push(descriptor);
        s = rest;
    }
    Some(out)
}

/// Parse a simple or backtick-escaped name, returning it and the remaining input.
fn parse_name(s: &str) -> Option<(String, &str)> {
    if let Some(mut rest) = s.strip_prefix('`') {
        let mut name = String::new();
        loop {
            let tick = rest.find('`')?;
            name.push_str(&rest[..tick]);
            rest = &rest[tick + 1..];
            match rest.strip_prefix('`') {
                Some(r) => {
                    name.push('`');
                    rest = r;
                }
                None => return Some((name, rest)),
            }
        }
    }
    let end = s
        .find(|c: char| !(c.is_alphanumeric() || matches!(c, '_' | '+' | '-' | '$')))
        .unwrap_or(s.len());
    if end == 0 {
        return None;
    }
    Some((s[..end].to_string(), &s[end..]))
}

fn url_encode(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-' | b'.' | b':') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_type() {
        let s = ScipSymbol::parse("rust-analyzer cargo serde_core 1.0.229 de/Deserialize#").unwrap();
        assert_eq!(s.package, "serde_core");
        assert_eq!(s.version, "1.0.229");
        assert_eq!(s.display_path(), "serde_core::de::Deserialize");
        assert_eq!(s.name(), "Deserialize");
        assert_eq!(
            s.docs_url(),
            "https://docs.rs/serde_core/1.0.229/serde_core/?search=Deserialize",
        );
    }

    #[test]
    fn parses_impl_method() {
        let s = ScipSymbol::parse(
            "rust-analyzer cargo anyhow 1.0.104 context/impl#[`Result<T, E>`][`Context<T, E>`]with_context().",
        )
        .unwrap();
        assert_eq!(s.name(), "with_context");
        assert_eq!(s.display_path(), "anyhow::context::Result<T, E>::with_context");
        assert!(s.docs_url().ends_with("?search=with_context"));
    }

    #[test]
    fn parses_std_member() {
        let s = ScipSymbol::parse(
            "rust-analyzer cargo std https://github.com/rust-lang/rust/library/std path/Path#new().",
        )
        .unwrap();
        assert_eq!(s.docs_url(), "https://doc.rust-lang.org/std/?search=Path::new");
    }

    #[test]
    fn crate_root_links_to_docs_root() {
        let s = ScipSymbol::parse("rust-analyzer cargo rustdoc-types 0.56.0 crate/").unwrap();
        assert_eq!(s.docs_url(), "https://docs.rs/rustdoc-types/0.56.0/rustdoc_types/");
        assert_eq!(s.display_path(), "rustdoc_types");
        assert_eq!(s.name(), "rustdoc-types");
    }

    #[test]
    fn parses_crate_root_item() {
        let s = ScipSymbol::parse("rust-analyzer cargo anyhow 1.0.104 Result#").unwrap();
        assert_eq!(s.display_path(), "anyhow::Result");
        assert_eq!(s.docs_url(), "https://docs.rs/anyhow/1.0.104/anyhow/?search=Result");
    }

    #[test]
    fn parses_inherent_impl_method() {
        let s = ScipSymbol::parse("rust-analyzer cargo proc-macro2 1.0.107 impl#[Ident]new().").unwrap();
        assert_eq!(s.display_path(), "proc_macro2::Ident::new");
    }

    #[test]
    fn rejects_locals() {
        assert_eq!(ScipSymbol::parse("local 3"), None);
    }

    #[test]
    fn parses_macro_and_escapes() {
        let s = ScipSymbol::parse("rust-analyzer cargo alloc https://x macros/format!").unwrap();
        assert_eq!(s.descriptors.last().unwrap().suffix, Suffix::Macro);
        let s = ScipSymbol::parse("a b c d `we``ird`#").unwrap();
        assert_eq!(s.descriptors[0].name, "we`ird");
    }
}
