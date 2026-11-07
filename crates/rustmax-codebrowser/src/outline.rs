//! The per-file outline of item definitions shown beside source pages.
//!
//! Items are nested by their SCIP descriptors: members sit under the type,
//! trait, module, or function that contains them, and methods sit under
//! their impl. SCIP has no definitions for impl blocks themselves,
//! so an impl entry is synthesized at its first member,
//! and adjacent impl blocks for the same type and trait merge into one.

use crate::highlight::escape;
use crate::index::{Document, Symbol, Target};
use crate::symbol::{Descriptor, Suffix};
use std::fmt::Write;

#[derive(Debug, PartialEq)]
pub struct Item {
    /// A short keyword like `fn` or `struct`.
    pub keyword: &'static str,
    pub name: String,
    /// Zero-based line of the definition.
    pub line: u32,
    pub children: Vec<Item>,
}

/// The keyword shown for a SCIP symbol kind, `None` for kinds left out of the outline.
fn keyword(kind: &str) -> Option<&'static str> {
    Some(match kind {
        "Module" => "mod",
        "Struct" => "struct",
        "Enum" => "enum",
        "Union" => "union",
        "Trait" => "trait",
        "TypeAlias" | "AssociatedType" => "type",
        "Function" | "Method" | "StaticMethod" | "TraitMethod" => "fn",
        "Constant" | "AssociatedConst" => "const",
        "StaticVariable" => "static",
        "Macro" => "macro",
        _ => return None,
    })
}

/// Build the outline of one document.
pub fn build(document: &Document, symbols: &[Symbol]) -> Vec<Item> {
    let entries = document.occurrences.iter().filter_map(|occ| {
        let Target::Global(id) = occ.target else { return None };
        if !occ.is_def {
            return None;
        }
        let symbol = &symbols[id as usize];
        let keyword = keyword(symbol.kind.as_deref()?)?;
        let descriptors = &symbol.scip.as_ref()?.descriptors;
        Some((occ.line, keyword, descriptors.as_slice()))
    });
    nest(entries)
}

/// Nest `(line, keyword, descriptors)` entries, given in line order.
fn nest<'a>(entries: impl Iterator<Item = (u32, &'static str, &'a [Descriptor])>) -> Vec<Item> {
    // Items in line order, each with its parent's position.
    let mut flat: Vec<(Item, Option<usize>)> = Vec::new();
    // The containers enclosing the current position, innermost last.
    let mut stack: Vec<(&[Descriptor], usize)> = Vec::new();

    fn enter(stack: &mut Vec<(&[Descriptor], usize)>, container: &[Descriptor]) -> Option<usize> {
        match stack.iter().rposition(|(key, _)| *key == container) {
            Some(pos) => {
                stack.truncate(pos + 1);
                Some(stack[pos].1)
            }
            None => {
                stack.clear();
                None
            }
        }
    }

    for (line, keyword, descriptors) in entries {
        let Some((own, container)) = descriptors.split_last() else { continue };
        let parent = match impl_start(container) {
            Some(start) if !stack.iter().any(|(key, _)| *key == container) => {
                let parent = enter(&mut stack, &container[..start]);
                let impl_item = Item {
                    keyword: "impl",
                    name: impl_name(&container[start + 1..]),
                    line,
                    children: Vec::new(),
                };
                flat.push((impl_item, parent));
                stack.push((container, flat.len() - 1));
                Some(flat.len() - 1)
            }
            _ => enter(&mut stack, container),
        };
        let item = Item { keyword, name: own.name.clone(), line, children: Vec::new() };
        flat.push((item, parent));
        stack.push((descriptors, flat.len() - 1));
    }

    let mut children: Vec<Vec<usize>> = vec![Vec::new(); flat.len()];
    let mut roots = Vec::new();
    for (i, (_, parent)) in flat.iter().enumerate() {
        match parent {
            Some(p) => children[*p].push(i),
            None => roots.push(i),
        }
    }
    let mut slots: Vec<Option<Item>> = flat.into_iter().map(|(item, _)| Some(item)).collect();
    fn assemble(i: usize, slots: &mut [Option<Item>], children: &[Vec<usize>]) -> Item {
        let mut item = slots[i].take().expect("each item has one parent");
        item.children = children[i].iter().map(|&c| assemble(c, slots, children)).collect();
        item
    }
    roots.into_iter().map(|i| assemble(i, &mut slots, &children)).collect()
}

/// The position of the `impl` descriptor if `container` is an impl block.
///
/// rust-analyzer writes an impl as a type named `impl`
/// followed by the self type and optional trait as type parameters.
fn impl_start(container: &[Descriptor]) -> Option<usize> {
    let start = container.iter().rposition(|d| d.suffix == Suffix::Type && d.name == "impl")?;
    let params = &container[start + 1..];
    (!params.is_empty() && params.iter().all(|d| d.suffix == Suffix::TypeParameter)).then_some(start)
}

/// `Type`, or `Trait for Type`, from an impl's type parameters.
fn impl_name(params: &[Descriptor]) -> String {
    match params {
        [self_ty, tr, ..] => format!("{} for {}", tr.name, self_ty.name),
        [self_ty] => self_ty.name.clone(),
        [] => unreachable!("impl_start requires a self type"),
    }
}

/// Render the outline as nested lists linking to line anchors.
pub fn render_html(items: &[Item]) -> String {
    let mut out = String::new();
    render_list(&mut out, items);
    out
}

fn render_list(out: &mut String, items: &[Item]) {
    out.push_str("<ul>");
    for item in items {
        let link = format!(
            "<a href=\"#L{}\"><span class=\"rb-kw\">{}</span> {}</a>",
            item.line + 1,
            item.keyword,
            escape(&item.name),
        );
        if item.children.is_empty() {
            let _ = write!(out, "<li>{link}</li>");
        } else {
            let _ = write!(out, "<li><details open><summary>{link}</summary>");
            render_list(out, &item.children);
            out.push_str("</details></li>");
        }
    }
    out.push_str("</ul>");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol::ScipSymbol;

    fn entry(line: u32, keyword: &'static str, symbol: &str) -> (u32, &'static str, Vec<Descriptor>) {
        let symbol = ScipSymbol::parse(&format!("rust-analyzer cargo demo 0.1.0 {symbol}")).unwrap();
        (line, keyword, symbol.descriptors)
    }

    fn show(items: &[Item], depth: usize, out: &mut Vec<String>) {
        for item in items {
            out.push(format!("{}{} {} {}", "  ".repeat(depth), item.line, item.keyword, item.name));
            show(&item.children, depth + 1, out);
        }
    }

    #[test]
    fn nests_members_under_containers() {
        let entries = [
            entry(0, "struct", "m/Index#"),
            entry(4, "fn", "m/impl#[Index]load()."),
            entry(8, "fn", "m/impl#[Index]save()."),
            entry(12, "fn", "m/impl#[Index][Display]fmt()."),
            entry(16, "trait", "m/Store#"),
            entry(17, "fn", "m/Store#get()."),
            entry(20, "fn", "m/helper()."),
            entry(21, "fn", "m/helper().inner()."),
            entry(24, "mod", "m/tests/"),
            entry(25, "fn", "m/tests/works()."),
            entry(28, "fn", "m/tests/impl#[Fixture]new()."),
            entry(30, "fn", "m/after()."),
        ];
        let items = nest(entries.iter().map(|(l, k, d)| (*l, *k, d.as_slice())));
        let mut lines = Vec::new();
        show(&items, 0, &mut lines);
        assert_eq!(
            lines,
            [
                "0 struct Index",
                "4 impl Index",
                "  4 fn load",
                "  8 fn save",
                "12 impl Display for Index",
                "  12 fn fmt",
                "16 trait Store",
                "  17 fn get",
                "20 fn helper",
                "  21 fn inner",
                "24 mod tests",
                "  25 fn works",
                "  28 impl Fixture",
                "    28 fn new",
                "30 fn after",
            ],
        );
    }

    #[test]
    fn renders_links_to_lines() {
        let items = [Item {
            keyword: "impl",
            name: "Wrap<T>".to_string(),
            line: 2,
            children: vec![Item { keyword: "fn", name: "new".to_string(), line: 3, children: Vec::new() }],
        }];
        let html = render_html(&items);
        assert!(html.contains("<summary><a href=\"#L3\"><span class=\"rb-kw\">impl</span> Wrap&lt;T&gt;</a></summary>"));
        assert!(html.contains("<li><a href=\"#L4\"><span class=\"rb-kw\">fn</span> new</a></li>"));
    }
}
