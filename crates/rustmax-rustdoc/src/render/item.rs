//! Individual item rendering (struct, function, etc.).

use rmx::prelude::*;
use rmx::tera::Context;
use rustdoc_types::{Id, Item, ItemEnum, StructKind};

use super::RenderContext;
use super::signature::{html_escape, render_attribute, LinkedRenderer};
use crate::types::RenderableItem;

/// Fields shared by every item page.
///
/// Filling these in one place keeps the per-item render functions to the parts
/// that actually differ between item kinds.
struct PageBase {
    depth: usize,
}

fn page_base(ctx: &RenderContext, item: &RenderableItem, tera_ctx: &mut Context) -> AnyResult<PageBase> {
    let name = item_name(item);
    let depth = item.path.len().saturating_sub(1);
    let path_to_root = if depth == 0 { String::new() } else { "../".repeat(depth) };

    tera_ctx.insert("crate_name", ctx.crate_name());
    tera_ctx.insert("item_name", name);
    tera_ctx.insert("item_path", &item.path);
    tera_ctx.insert("path_to_root", &path_to_root);
    tera_ctx.insert("breadcrumbs", &super::build_breadcrumbs(&item.path, depth));
    tera_ctx.insert("attrs", &render_attrs(item.item));
    tera_ctx.insert("deprecation", &render_deprecation(item.item));

    let pre_resolved = ctx.resolve_item_links(&item.item.links, depth);
    let docs = item.item.docs.as_ref()
        .map(|d| ctx.render_markdown_with_item_links(d, depth, &pre_resolved))
        .unwrap_or_default();
    tera_ctx.insert("docs", &docs);

    let sidebar_html = super::sidebar::render_sidebar(ctx, &item.path, &path_to_root)?;
    tera_ctx.insert("sidebar", &sidebar_html);

    Ok(PageBase { depth })
}

/// The name to render an item under.
///
/// This is the last segment of the path the page lives at, which for a
/// re-exported item is the re-export name rather than the definition name.
fn item_name<'a>(item: &'a RenderableItem<'a>) -> &'a str {
    item.path.last()
        .map(String::as_str)
        .or(item.item.name.as_deref())
        .unwrap_or("?")
}

/// Render the item's attributes, one per line, as they appear in source.
fn render_attrs(item: &Item) -> String {
    item.attrs.iter()
        .filter_map(render_attribute)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Render a deprecation notice, or the empty string when not deprecated.
fn render_deprecation(item: &Item) -> String {
    let Some(dep) = &item.deprecation else {
        return String::new();
    };
    let mut result = match &dep.since {
        Some(since) => format!("Deprecated since {}", html_escape(since)),
        None => "Deprecated".to_string(),
    };
    if let Some(note) = &dep.note {
        result.push_str(": ");
        result.push_str(&html_escape(note));
    }
    result
}

/// Render a struct page to HTML.
pub fn render_struct(ctx: &RenderContext, item: &RenderableItem) -> AnyResult<String> {
    let ItemEnum::Struct(s) = &item.item.inner else {
        bail!("Expected struct item");
    };

    let mut tera_ctx = Context::new();
    let base = page_base(ctx, item, &mut tera_ctx)?;
    let name = item_name(item);
    let linked = LinkedRenderer::new(ctx, base.depth);

    tera_ctx.insert("signature", &linked.render_struct_sig(s, name, Some(&item.item.visibility)));

    // Named fields for plain structs, positional fields for tuple structs.
    let fields = match &s.kind {
        StructKind::Plain { fields, .. } => collect_fields(ctx, &linked, fields, base.depth),
        StructKind::Tuple(fields) => collect_tuple_fields(ctx, &linked, fields, base.depth),
        StructKind::Unit => Vec::new(),
    };
    tera_ctx.insert("fields", &fields);

    insert_impls(ctx, item.id, base.depth, &mut tera_ctx);

    ctx.tera.render("struct.html", &tera_ctx)
        .context("Failed to render struct template")
}

/// Render a union page to HTML.
pub fn render_union(ctx: &RenderContext, item: &RenderableItem) -> AnyResult<String> {
    let ItemEnum::Union(u) = &item.item.inner else {
        bail!("Expected union item");
    };

    let mut tera_ctx = Context::new();
    let base = page_base(ctx, item, &mut tera_ctx)?;
    let name = item_name(item);
    let linked = LinkedRenderer::new(ctx, base.depth);

    tera_ctx.insert("signature", &linked.render_union_sig(u, name, Some(&item.item.visibility)));
    tera_ctx.insert("fields", &collect_fields(ctx, &linked, &u.fields, base.depth));

    insert_impls(ctx, item.id, base.depth, &mut tera_ctx);

    ctx.tera.render("union.html", &tera_ctx)
        .context("Failed to render union template")
}

/// Render a function page to HTML.
pub fn render_function(ctx: &RenderContext, item: &RenderableItem) -> AnyResult<String> {
    let ItemEnum::Function(func) = &item.item.inner else {
        bail!("Expected function item");
    };

    let mut tera_ctx = Context::new();
    let base = page_base(ctx, item, &mut tera_ctx)?;
    let name = item_name(item);
    let linked = LinkedRenderer::new(ctx, base.depth);

    let signature = linked.render_function_sig(func, name, Some(&item.item.visibility));
    tera_ctx.insert("signature", &signature);

    ctx.tera.render("function.html", &tera_ctx)
        .context("Failed to render function template")
}

/// Render an enum page to HTML.
pub fn render_enum(ctx: &RenderContext, item: &RenderableItem) -> AnyResult<String> {
    let ItemEnum::Enum(e) = &item.item.inner else {
        bail!("Expected enum item");
    };

    let mut tera_ctx = Context::new();
    let base = page_base(ctx, item, &mut tera_ctx)?;
    let name = item_name(item);
    let linked = LinkedRenderer::new(ctx, base.depth);

    tera_ctx.insert("signature", &linked.render_enum_sig(e, name, Some(&item.item.visibility)));

    let mut variants = Vec::new();
    for variant_id in &e.variants {
        let Some(variant_item) = ctx.krate.index.get(variant_id) else { continue };
        let ItemEnum::Variant(v) = &variant_item.inner else { continue };

        let variant_links = ctx.resolve_item_links(&variant_item.links, base.depth);
        variants.push(VariantInfo {
            name: variant_item.name.clone().unwrap_or_default(),
            fields: linked.render_variant_fields(&v.kind),
            discriminant: v.discriminant.as_ref().map(|d| html_escape(&d.expr)),
            docs: variant_item.docs.as_ref()
                .map(|d| ctx.render_markdown_with_item_links(d, base.depth, &variant_links))
                .unwrap_or_default(),
        });
    }
    tera_ctx.insert("variants", &variants);

    insert_impls(ctx, item.id, base.depth, &mut tera_ctx);

    ctx.tera.render("enum.html", &tera_ctx)
        .context("Failed to render enum template")
}

/// Render a trait page to HTML.
pub fn render_trait(ctx: &RenderContext, item: &RenderableItem) -> AnyResult<String> {
    let ItemEnum::Trait(t) = &item.item.inner else {
        bail!("Expected trait item");
    };

    let mut tera_ctx = Context::new();
    let base = page_base(ctx, item, &mut tera_ctx)?;
    let name = item_name(item);
    let linked = LinkedRenderer::new(ctx, base.depth);

    tera_ctx.insert("signature", &linked.render_trait_sig(t, name, Some(&item.item.visibility)));

    let mut associated_types = Vec::new();
    let mut associated_consts = Vec::new();
    let mut required_methods = Vec::new();
    let mut provided_methods = Vec::new();

    for item_id in &t.items {
        let Some(trait_item) = ctx.krate.index.get(item_id) else { continue };
        let member_name = trait_item.name.as_deref().unwrap_or("?").to_string();
        let docs = |ctx: &RenderContext| {
            let links = ctx.resolve_item_links(&trait_item.links, base.depth);
            trait_item.docs.as_ref()
                .map(|d| ctx.render_markdown_with_item_links(d, base.depth, &links))
                .unwrap_or_default()
        };

        match &trait_item.inner {
            ItemEnum::AssocType { generics, bounds, type_ } => {
                associated_types.push(MemberInfo {
                    signature: linked.render_assoc_type_sig(
                        &member_name,
                        generics,
                        bounds,
                        type_.as_ref(),
                    ),
                    name: member_name,
                    docs: docs(ctx),
                });
            }
            ItemEnum::AssocConst { type_, value } => {
                associated_consts.push(MemberInfo {
                    signature: linked.render_assoc_const_sig(&member_name, type_, value.as_deref()),
                    name: member_name,
                    docs: docs(ctx),
                });
            }
            ItemEnum::Function(f) => {
                let info = MemberInfo {
                    signature: linked.render_function_sig(f, &member_name, None),
                    name: member_name,
                    docs: docs(ctx),
                };
                if f.has_body {
                    provided_methods.push(info);
                } else {
                    required_methods.push(info);
                }
            }
            _ => {}
        }
    }

    tera_ctx.insert("associated_types", &associated_types);
    tera_ctx.insert("associated_consts", &associated_consts);
    tera_ctx.insert("required_methods", &required_methods);
    tera_ctx.insert("provided_methods", &provided_methods);

    // Collect implementors from the impl index.
    let mut implementors: Vec<String> = ctx.impl_index.trait_impls.get(item.id)
        .into_iter()
        .flatten()
        .map(|impl_info| linked.render_impl_header(impl_info.impl_))
        .collect();
    implementors.sort_by_key(|header| strip_tags(header));
    tera_ctx.insert("implementors", &implementors);

    ctx.tera.render("trait.html", &tera_ctx)
        .context("Failed to render trait template")
}

/// Render a type alias page to HTML.
pub fn render_type_alias(ctx: &RenderContext, item: &RenderableItem) -> AnyResult<String> {
    let ItemEnum::TypeAlias(ta) = &item.item.inner else {
        bail!("Expected type alias item");
    };

    let mut tera_ctx = Context::new();
    let base = page_base(ctx, item, &mut tera_ctx)?;
    let name = item_name(item);
    let linked = LinkedRenderer::new(ctx, base.depth);

    tera_ctx.insert("signature", &linked.render_type_alias_sig(ta, name, Some(&item.item.visibility)));

    ctx.tera.render("type_alias.html", &tera_ctx)
        .context("Failed to render type alias template")
}

/// Render a constant or static page to HTML.
pub fn render_constant(ctx: &RenderContext, item: &RenderableItem) -> AnyResult<String> {
    let mut tera_ctx = Context::new();
    let base = page_base(ctx, item, &mut tera_ctx)?;
    let name = item_name(item);
    let linked = LinkedRenderer::new(ctx, base.depth);
    let vis = Some(&item.item.visibility);

    let (signature, kind) = match &item.item.inner {
        ItemEnum::Constant { type_, const_ } => (
            linked.render_constant_sig(name, type_, const_.value.as_deref(), vis),
            "Constant",
        ),
        ItemEnum::Static(s) => (linked.render_static_sig(s, name, vis), "Static"),
        _ => bail!("Expected constant or static item"),
    };

    tera_ctx.insert("item_kind", kind);
    tera_ctx.insert("signature", &signature);

    ctx.tera.render("constant.html", &tera_ctx)
        .context("Failed to render constant template")
}

/// Render a macro page to HTML.
pub fn render_macro(ctx: &RenderContext, item: &RenderableItem) -> AnyResult<String> {
    let mut tera_ctx = Context::new();
    page_base(ctx, item, &mut tera_ctx)?;
    let name = item_name(item);

    let signature = match &item.item.inner {
        ItemEnum::Macro(m) => html_escape(m),
        _ => html_escape(&format!("macro_rules! {} {{ ... }}", name)),
    };
    tera_ctx.insert("signature", &signature);

    ctx.tera.render("macro.html", &tera_ctx)
        .context("Failed to render macro template")
}

#[derive(serde::Serialize)]
struct FieldInfo {
    name: String,
    type_: String,
    docs: String,
}

#[derive(serde::Serialize)]
struct VariantInfo {
    name: String,
    fields: Option<String>,
    discriminant: Option<String>,
    docs: String,
}

/// A named member of a trait or impl: associated type, associated const, or method.
#[derive(serde::Serialize)]
struct MemberInfo {
    name: String,
    signature: String,
    docs: String,
}

#[derive(serde::Serialize)]
struct ImplBlockInfo {
    header: String,
    members: Vec<MemberInfo>,
}

/// Drop HTML tags from a rendered fragment.
///
/// Rendered headers carry `<a>` links, so ordering them by their raw HTML puts
/// every linked name ahead of every unlinked one. Sorting on the visible text
/// gives the alphabetical order a reader expects.
fn strip_tags(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(c),
            _ => {}
        }
    }
    result
}

/// Collect named fields with their rendered types and docs.
fn collect_fields(
    ctx: &RenderContext,
    linked: &LinkedRenderer,
    field_ids: &[Id],
    depth: usize,
) -> Vec<FieldInfo> {
    field_ids.iter()
        .filter_map(|id| {
            let field_item = ctx.krate.index.get(id)?;
            let ItemEnum::StructField(ty) = &field_item.inner else { return None };
            let links = ctx.resolve_item_links(&field_item.links, depth);
            Some(FieldInfo {
                name: field_item.name.clone().unwrap_or_default(),
                type_: linked.render_type(ty),
                docs: field_item.docs.as_ref()
                    .map(|d| ctx.render_markdown_with_item_links(d, depth, &links))
                    .unwrap_or_default(),
            })
        })
        .collect()
}

/// Collect tuple-struct fields, which are named by their position.
fn collect_tuple_fields(
    ctx: &RenderContext,
    linked: &LinkedRenderer,
    field_ids: &[Option<Id>],
    depth: usize,
) -> Vec<FieldInfo> {
    field_ids.iter()
        .enumerate()
        .filter_map(|(position, id)| {
            let field_item = ctx.krate.index.get(id.as_ref()?)?;
            let ItemEnum::StructField(ty) = &field_item.inner else { return None };
            let links = ctx.resolve_item_links(&field_item.links, depth);
            Some(FieldInfo {
                name: position.to_string(),
                type_: linked.render_type(ty),
                docs: field_item.docs.as_ref()
                    .map(|d| ctx.render_markdown_with_item_links(d, depth, &links))
                    .unwrap_or_default(),
            })
        })
        .collect()
}

/// Group a type's impl blocks the way rustdoc does and add them to the context.
fn insert_impls(ctx: &RenderContext, type_id: &Id, depth: usize, tera_ctx: &mut Context) {
    let linked = LinkedRenderer::new(ctx, depth);
    let mut inherent = Vec::new();
    let mut trait_impls = Vec::new();
    let mut auto_impls = Vec::new();
    let mut blanket_impls = Vec::new();

    for impl_info in ctx.impl_index.type_impls.get(type_id).into_iter().flatten() {
        let impl_ = impl_info.impl_;
        let block = ImplBlockInfo {
            header: linked.render_impl_header(impl_),
            members: collect_impl_members(ctx, &linked, &impl_.items, depth),
        };

        if impl_.blanket_impl.is_some() {
            blanket_impls.push(block);
        } else if impl_.is_synthetic {
            auto_impls.push(block);
        } else if impl_.trait_.is_some() {
            trait_impls.push(block);
        } else {
            inherent.push(block);
        }
    }

    for group in [&mut inherent, &mut trait_impls, &mut auto_impls, &mut blanket_impls] {
        group.sort_by_key(|block| strip_tags(&block.header));
    }

    tera_ctx.insert("impls", &inherent);
    tera_ctx.insert("trait_impls", &trait_impls);
    tera_ctx.insert("auto_impls", &auto_impls);
    tera_ctx.insert("blanket_impls", &blanket_impls);
}

/// Collect the methods, associated types and associated consts of an impl block.
fn collect_impl_members(
    ctx: &RenderContext,
    linked: &LinkedRenderer,
    item_ids: &[Id],
    depth: usize,
) -> Vec<MemberInfo> {
    let mut members = Vec::new();
    for id in item_ids {
        let Some(member) = ctx.krate.index.get(id) else { continue };
        let name = member.name.as_deref().unwrap_or("?").to_string();
        let signature = match &member.inner {
            ItemEnum::Function(f) => linked.render_function_sig(f, &name, None),
            ItemEnum::AssocType { generics, bounds, type_ } => {
                linked.render_assoc_type_sig(&name, generics, bounds, type_.as_ref())
            }
            ItemEnum::AssocConst { type_, value } => {
                linked.render_assoc_const_sig(&name, type_, value.as_deref())
            }
            _ => continue,
        };
        let links = ctx.resolve_item_links(&member.links, depth);
        members.push(MemberInfo {
            name,
            signature,
            docs: member.docs.as_ref()
                .map(|d| ctx.render_markdown_with_item_links(d, depth, &links))
                .unwrap_or_default(),
        });
    }
    members
}
