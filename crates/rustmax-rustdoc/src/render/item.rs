//! Individual item rendering (struct, function, etc.).

use rmx::prelude::*;
use rmx::tera::Context;
use rustdoc_types::{ItemEnum, VariantKind};

use super::RenderContext;
use super::signature::{render_struct_sig, render_union_sig, render_enum_sig, render_trait_sig, render_type, LinkedRenderer};
use crate::types::RenderableItem;

/// Render a struct page to HTML.
pub fn render_struct(ctx: &RenderContext, item: &RenderableItem) -> AnyResult<String> {
    let ItemEnum::Struct(s) = &item.item.inner else {
        bail!("Expected struct item");
    };

    let mut tera_ctx = Context::new();
    let name = item.item.name.as_deref().unwrap_or("?");

    tera_ctx.insert("crate_name", ctx.crate_name());
    tera_ctx.insert("item_name", name);
    tera_ctx.insert("item_path", &item.path);

    // Path to root (needed for linked rendering).
    let depth = item.path.len().saturating_sub(1);
    let path_to_root = if depth == 0 { String::new() } else { "../".repeat(depth) };

    // Get generics from the item.
    let generics = &s.generics;
    let signature = html_escape_sig(&render_struct_sig(s, name, generics));
    tera_ctx.insert("signature", &signature);

    // Pre-resolve item links for doc markdown.
    let pre_resolved = ctx.resolve_item_links(&item.item.links, depth);

    // Documentation.
    let docs = item.item.docs.as_ref()
        .map(|d| ctx.render_markdown_with_item_links(d, depth, &pre_resolved))
        .unwrap_or_default();
    tera_ctx.insert("docs", &docs);

    // Use linked renderer for field types with links.
    let linked = LinkedRenderer::new(ctx, depth);

    // Fields (for plain structs).
    let mut fields = Vec::new();
    if let rustdoc_types::StructKind::Plain { fields: field_ids, .. } = &s.kind {
        for field_id in field_ids {
            if let Some(field_item) = ctx.krate.index.get(field_id) {
                if let ItemEnum::StructField(ty) = &field_item.inner {
                    let field_links = ctx.resolve_item_links(&field_item.links, depth);
                    fields.push(FieldInfo {
                        name: field_item.name.clone().unwrap_or_default(),
                        type_: linked.render_type(ty),
                        docs: field_item.docs.as_ref()
                            .map(|d| ctx.render_markdown_with_item_links(d, depth, &field_links))
                            .unwrap_or_default(),
                    });
                }
            }
        }
    }
    tera_ctx.insert("fields", &fields);

    // Collect impl blocks for this type.
    let impls = collect_impls(ctx, item.id, depth);
    tera_ctx.insert("impls", &impls);
    tera_ctx.insert("path_to_root", &path_to_root);
    tera_ctx.insert("breadcrumbs", &super::build_breadcrumbs(&item.path, depth));

    // Sidebar HTML.
    let sidebar_html = super::sidebar::render_sidebar(ctx, &item.path, &path_to_root)?;
    tera_ctx.insert("sidebar", &sidebar_html);

    ctx.tera.render("struct.html", &tera_ctx)
        .context("Failed to render struct template")
}

/// Render a union page to HTML.
pub fn render_union(ctx: &RenderContext, item: &RenderableItem) -> AnyResult<String> {
    let ItemEnum::Union(u) = &item.item.inner else {
        bail!("Expected union item");
    };

    let mut tera_ctx = Context::new();
    let name = item.item.name.as_deref().unwrap_or("?");

    tera_ctx.insert("crate_name", ctx.crate_name());
    tera_ctx.insert("item_name", name);
    tera_ctx.insert("item_path", &item.path);

    // Path to root (needed for linked rendering).
    let depth = item.path.len().saturating_sub(1);
    let path_to_root = if depth == 0 { String::new() } else { "../".repeat(depth) };

    // Get generics from the item.
    let generics = &u.generics;
    let signature = html_escape_sig(&render_union_sig(u, name, generics));
    tera_ctx.insert("signature", &signature);

    // Pre-resolve item links for doc markdown.
    let pre_resolved = ctx.resolve_item_links(&item.item.links, depth);

    // Documentation.
    let docs = item.item.docs.as_ref()
        .map(|d| ctx.render_markdown_with_item_links(d, depth, &pre_resolved))
        .unwrap_or_default();
    tera_ctx.insert("docs", &docs);

    // Use linked renderer for field types with links.
    let linked = LinkedRenderer::new(ctx, depth);

    // Fields.
    let mut fields = Vec::new();
    for field_id in &u.fields {
        if let Some(field_item) = ctx.krate.index.get(field_id) {
            if let ItemEnum::StructField(ty) = &field_item.inner {
                let field_links = ctx.resolve_item_links(&field_item.links, depth);
                fields.push(FieldInfo {
                    name: field_item.name.clone().unwrap_or_default(),
                    type_: linked.render_type(ty),
                    docs: field_item.docs.as_ref()
                        .map(|d| ctx.render_markdown_with_item_links(d, depth, &field_links))
                        .unwrap_or_default(),
                });
            }
        }
    }
    tera_ctx.insert("fields", &fields);

    // Collect impl blocks for this type.
    let impls = collect_impls(ctx, item.id, depth);
    tera_ctx.insert("impls", &impls);
    tera_ctx.insert("path_to_root", &path_to_root);
    tera_ctx.insert("breadcrumbs", &super::build_breadcrumbs(&item.path, depth));

    // Sidebar HTML.
    let sidebar_html = super::sidebar::render_sidebar(ctx, &item.path, &path_to_root)?;
    tera_ctx.insert("sidebar", &sidebar_html);

    ctx.tera.render("union.html", &tera_ctx)
        .context("Failed to render union template")
}

/// Render a function page to HTML.
pub fn render_function(ctx: &RenderContext, item: &RenderableItem) -> AnyResult<String> {
    let ItemEnum::Function(func) = &item.item.inner else {
        bail!("Expected function item");
    };

    let mut tera_ctx = Context::new();
    let name = item.item.name.as_deref().unwrap_or("?");

    tera_ctx.insert("crate_name", ctx.crate_name());
    tera_ctx.insert("item_name", name);
    tera_ctx.insert("item_path", &item.path);

    // Path to root (needed for linked rendering).
    let depth = item.path.len().saturating_sub(1);
    let path_to_root = if depth == 0 { String::new() } else { "../".repeat(depth) };

    // Use linked renderer for signature with clickable type names.
    let linked = LinkedRenderer::new(ctx, depth);
    let signature = linked.render_function_sig(func, name);
    tera_ctx.insert("signature", &signature);

    // Pre-resolve item links for doc markdown.
    let pre_resolved = ctx.resolve_item_links(&item.item.links, depth);

    // Documentation.
    let docs = item.item.docs.as_ref()
        .map(|d| ctx.render_markdown_with_item_links(d, depth, &pre_resolved))
        .unwrap_or_default();
    tera_ctx.insert("docs", &docs);

    tera_ctx.insert("path_to_root", &path_to_root);
    tera_ctx.insert("breadcrumbs", &super::build_breadcrumbs(&item.path, depth));

    // Sidebar HTML.
    let sidebar_html = super::sidebar::render_sidebar(ctx, &item.path, &path_to_root)?;
    tera_ctx.insert("sidebar", &sidebar_html);

    ctx.tera.render("function.html", &tera_ctx)
        .context("Failed to render function template")
}

/// Render an enum page to HTML.
pub fn render_enum(ctx: &RenderContext, item: &RenderableItem) -> AnyResult<String> {
    let ItemEnum::Enum(e) = &item.item.inner else {
        bail!("Expected enum item");
    };

    let mut tera_ctx = Context::new();
    let name = item.item.name.as_deref().unwrap_or("?");

    tera_ctx.insert("crate_name", ctx.crate_name());
    tera_ctx.insert("item_name", name);
    tera_ctx.insert("item_path", &item.path);

    // Path to root (needed for linked rendering).
    let depth = item.path.len().saturating_sub(1);
    let path_to_root = if depth == 0 { String::new() } else { "../".repeat(depth) };

    let signature = html_escape_sig(&render_enum_sig(e, name, &e.generics));
    tera_ctx.insert("signature", &signature);

    // Pre-resolve item links for doc markdown.
    let pre_resolved = ctx.resolve_item_links(&item.item.links, depth);

    // Documentation.
    let docs = item.item.docs.as_ref()
        .map(|d| ctx.render_markdown_with_item_links(d, depth, &pre_resolved))
        .unwrap_or_default();
    tera_ctx.insert("docs", &docs);

    // Variants.
    let mut variants = Vec::new();
    for variant_id in &e.variants {
        if let Some(variant_item) = ctx.krate.index.get(variant_id) {
            if let ItemEnum::Variant(v) = &variant_item.inner {
                let fields_str = match &v.kind {
                    VariantKind::Plain => None,
                    VariantKind::Tuple(fields) => {
                        let field_strs: Vec<_> = fields.iter().map(|f| {
                            f.as_ref()
                                .and_then(|id| ctx.krate.index.get(id))
                                .and_then(|item| {
                                    if let ItemEnum::StructField(ty) = &item.inner {
                                        Some(render_type(ty))
                                    } else {
                                        None
                                    }
                                })
                                .unwrap_or_else(|| "_".to_string())
                        }).collect();
                        Some(format!("({})", field_strs.join(", ")))
                    }
                    VariantKind::Struct { fields, .. } => {
                        let field_strs: Vec<_> = fields.iter().filter_map(|id| {
                            ctx.krate.index.get(id).and_then(|item| {
                                if let ItemEnum::StructField(ty) = &item.inner {
                                    let field_name = item.name.as_deref().unwrap_or("?");
                                    Some(format!("{}: {}", field_name, render_type(ty)))
                                } else {
                                    None
                                }
                            })
                        }).collect();
                        Some(format!(" {{ {} }}", field_strs.join(", ")))
                    }
                };

                let variant_links = ctx.resolve_item_links(&variant_item.links, depth);
                variants.push(VariantInfo {
                    name: variant_item.name.clone().unwrap_or_default(),
                    fields: fields_str,
                    docs: variant_item.docs.as_ref()
                        .map(|d| ctx.render_markdown_with_item_links(d, depth, &variant_links))
                        .unwrap_or_default(),
                });
            }
        }
    }
    tera_ctx.insert("variants", &variants);

    // Collect impl blocks for this type.
    let impls = collect_impls(ctx, item.id, depth);
    tera_ctx.insert("impls", &impls);
    tera_ctx.insert("path_to_root", &path_to_root);
    tera_ctx.insert("breadcrumbs", &super::build_breadcrumbs(&item.path, depth));

    // Sidebar HTML.
    let sidebar_html = super::sidebar::render_sidebar(ctx, &item.path, &path_to_root)?;
    tera_ctx.insert("sidebar", &sidebar_html);

    ctx.tera.render("enum.html", &tera_ctx)
        .context("Failed to render enum template")
}

/// Render a trait page to HTML.
pub fn render_trait(ctx: &RenderContext, item: &RenderableItem) -> AnyResult<String> {
    let ItemEnum::Trait(t) = &item.item.inner else {
        bail!("Expected trait item");
    };

    let mut tera_ctx = Context::new();
    let name = item.item.name.as_deref().unwrap_or("?");

    tera_ctx.insert("crate_name", ctx.crate_name());
    tera_ctx.insert("item_name", name);
    tera_ctx.insert("item_path", &item.path);

    // Path to root (needed for linked rendering).
    let depth = item.path.len().saturating_sub(1);
    let path_to_root = if depth == 0 { String::new() } else { "../".repeat(depth) };

    // Use linked renderer for HTML-safe signatures with links.
    let linked = LinkedRenderer::new(ctx, depth);

    // Trait signature (HTML-escaped, no types to link in the signature itself).
    let signature = html_escape_sig(&render_trait_sig(t, name, &t.generics));
    tera_ctx.insert("signature", &signature);

    // Pre-resolve item links for doc markdown.
    let pre_resolved = ctx.resolve_item_links(&item.item.links, depth);

    // Documentation.
    let docs = item.item.docs.as_ref()
        .map(|d| ctx.render_markdown_with_item_links(d, depth, &pre_resolved))
        .unwrap_or_default();
    tera_ctx.insert("docs", &docs);

    // Associated types.
    let mut associated_types = Vec::new();
    let mut required_methods = Vec::new();
    let mut provided_methods = Vec::new();

    for item_id in &t.items {
        if let Some(trait_item) = ctx.krate.index.get(item_id) {
            match &trait_item.inner {
                ItemEnum::AssocType { bounds, .. } => {
                    let bounds_str = if bounds.is_empty() {
                        None
                    } else {
                        Some(bounds.iter().map(|b| format!("{:?}", b)).collect::<Vec<_>>().join(" + "))
                    };
                    let assoc_links = ctx.resolve_item_links(&trait_item.links, depth);
                    associated_types.push(AssocTypeInfo {
                        name: trait_item.name.clone().unwrap_or_default(),
                        bounds: bounds_str,
                        docs: trait_item.docs.as_ref()
                            .map(|d| ctx.render_markdown_with_item_links(d, depth, &assoc_links))
                            .unwrap_or_default(),
                    });
                }
                ItemEnum::Function(f) => {
                    let method_name = trait_item.name.as_deref().unwrap_or("?");
                    let sig = linked.render_function_sig(f, method_name);
                    let method_links = ctx.resolve_item_links(&trait_item.links, depth);
                    let info = MethodInfo {
                        signature: sig,
                        docs: trait_item.docs.as_ref()
                            .map(|d| ctx.render_markdown_with_item_links(d, depth, &method_links))
                            .unwrap_or_default(),
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
    }

    tera_ctx.insert("associated_types", &associated_types);
    tera_ctx.insert("required_methods", &required_methods);
    tera_ctx.insert("provided_methods", &provided_methods);

    // Collect implementors from the impl index.
    let mut implementors = Vec::new();
    if let Some(impls) = ctx.impl_index.trait_impls.get(item.id) {
        for impl_info in impls {
            let for_type = linked.render_type(impl_info.for_type);
            let impl_header = render_impl_header_linked(impl_info.impl_, &for_type, Some(name));
            implementors.push(ImplementorInfo { impl_header });
        }
    }
    tera_ctx.insert("implementors", &implementors);

    tera_ctx.insert("path_to_root", &path_to_root);
    tera_ctx.insert("breadcrumbs", &super::build_breadcrumbs(&item.path, depth));

    // Sidebar HTML.
    let sidebar_html = super::sidebar::render_sidebar(ctx, &item.path, &path_to_root)?;
    tera_ctx.insert("sidebar", &sidebar_html);

    ctx.tera.render("trait.html", &tera_ctx)
        .context("Failed to render trait template")
}

/// Render a type alias page to HTML.
pub fn render_type_alias(ctx: &RenderContext, item: &RenderableItem) -> AnyResult<String> {
    let ItemEnum::TypeAlias(ta) = &item.item.inner else {
        bail!("Expected type alias item");
    };

    let mut tera_ctx = Context::new();
    let name = item.item.name.as_deref().unwrap_or("?");

    tera_ctx.insert("crate_name", ctx.crate_name());
    tera_ctx.insert("item_name", name);
    tera_ctx.insert("item_path", &item.path);

    // Path to root (needed for linked rendering).
    let depth = item.path.len().saturating_sub(1);
    let path_to_root = if depth == 0 { String::new() } else { "../".repeat(depth) };

    // Use linked renderer for type with links.
    let linked = LinkedRenderer::new(ctx, depth);
    let signature = format!("type {} = {}", html_escape_sig(name), linked.render_type(&ta.type_));
    tera_ctx.insert("signature", &signature);

    // Pre-resolve item links for doc markdown.
    let pre_resolved = ctx.resolve_item_links(&item.item.links, depth);

    // Documentation.
    let docs = item.item.docs.as_ref()
        .map(|d| ctx.render_markdown_with_item_links(d, depth, &pre_resolved))
        .unwrap_or_default();
    tera_ctx.insert("docs", &docs);

    tera_ctx.insert("path_to_root", &path_to_root);
    tera_ctx.insert("breadcrumbs", &super::build_breadcrumbs(&item.path, depth));

    // Sidebar HTML.
    let sidebar_html = super::sidebar::render_sidebar(ctx, &item.path, &path_to_root)?;
    tera_ctx.insert("sidebar", &sidebar_html);

    ctx.tera.render("type_alias.html", &tera_ctx)
        .context("Failed to render type alias template")
}

/// Render a constant page to HTML.
pub fn render_constant(ctx: &RenderContext, item: &RenderableItem) -> AnyResult<String> {
    let (type_, value, is_static) = match &item.item.inner {
        ItemEnum::Constant { type_, const_ } => (type_, const_.value.as_deref(), false),
        ItemEnum::Static(s) => (&s.type_, None, true),
        _ => bail!("Expected constant or static item"),
    };

    let mut tera_ctx = Context::new();
    let name = item.item.name.as_deref().unwrap_or("?");

    tera_ctx.insert("crate_name", ctx.crate_name());
    tera_ctx.insert("item_name", name);
    tera_ctx.insert("item_path", &item.path);
    tera_ctx.insert("item_kind", if is_static { "Static" } else { "Constant" });

    // Path to root (needed for linked rendering).
    let depth = item.path.len().saturating_sub(1);
    let path_to_root = if depth == 0 { String::new() } else { "../".repeat(depth) };

    // Use linked renderer for type with links.
    let linked = LinkedRenderer::new(ctx, depth);
    let keyword = if is_static { "static" } else { "const" };
    let signature = if let Some(val) = value {
        format!("{} {}: {} = {}", keyword, html_escape_sig(name), linked.render_type(type_), html_escape_sig(val))
    } else {
        format!("{} {}: {}", keyword, html_escape_sig(name), linked.render_type(type_))
    };
    tera_ctx.insert("signature", &signature);

    // Pre-resolve item links for doc markdown.
    let pre_resolved = ctx.resolve_item_links(&item.item.links, depth);

    // Documentation.
    let docs = item.item.docs.as_ref()
        .map(|d| ctx.render_markdown_with_item_links(d, depth, &pre_resolved))
        .unwrap_or_default();
    tera_ctx.insert("docs", &docs);

    tera_ctx.insert("path_to_root", &path_to_root);
    tera_ctx.insert("breadcrumbs", &super::build_breadcrumbs(&item.path, depth));

    // Sidebar HTML.
    let sidebar_html = super::sidebar::render_sidebar(ctx, &item.path, &path_to_root)?;
    tera_ctx.insert("sidebar", &sidebar_html);

    ctx.tera.render("constant.html", &tera_ctx)
        .context("Failed to render constant template")
}

/// Render a macro page to HTML.
pub fn render_macro(ctx: &RenderContext, item: &RenderableItem) -> AnyResult<String> {
    let macro_def = match &item.item.inner {
        ItemEnum::Macro(m) => Some(m.as_str()),
        _ => None,
    };

    let mut tera_ctx = Context::new();
    let name = item.item.name.as_deref().unwrap_or("?");

    tera_ctx.insert("crate_name", ctx.crate_name());
    tera_ctx.insert("item_name", name);
    tera_ctx.insert("item_path", &item.path);

    // Path to root.
    let depth = item.path.len().saturating_sub(1);
    let path_to_root = if depth == 0 { String::new() } else { "../".repeat(depth) };

    let signature = html_escape_sig(
        macro_def.unwrap_or(&format!("macro_rules! {} {{ ... }}", name))
    );
    tera_ctx.insert("signature", &signature);

    // Pre-resolve item links for doc markdown.
    let pre_resolved = ctx.resolve_item_links(&item.item.links, depth);

    // Documentation.
    let docs = item.item.docs.as_ref()
        .map(|d| ctx.render_markdown_with_item_links(d, depth, &pre_resolved))
        .unwrap_or_default();
    tera_ctx.insert("docs", &docs);
    tera_ctx.insert("path_to_root", &path_to_root);
    tera_ctx.insert("breadcrumbs", &super::build_breadcrumbs(&item.path, depth));

    // Sidebar HTML.
    let sidebar_html = super::sidebar::render_sidebar(ctx, &item.path, &path_to_root)?;
    tera_ctx.insert("sidebar", &sidebar_html);

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
    docs: String,
}

#[derive(serde::Serialize)]
struct AssocTypeInfo {
    name: String,
    bounds: Option<String>,
    docs: String,
}

#[derive(serde::Serialize)]
struct MethodInfo {
    signature: String,
    docs: String,
}

#[derive(serde::Serialize)]
struct ImplementorInfo {
    impl_header: String,
}

#[derive(serde::Serialize)]
struct ImplBlockInfo {
    header: String,
    methods: Vec<MethodInfo>,
}

/// Collect impl blocks for a type (struct or enum).
fn collect_impls(ctx: &RenderContext, type_id: &rustdoc_types::Id, depth: usize) -> Vec<ImplBlockInfo> {
    let linked = LinkedRenderer::new(ctx, depth);
    let mut result = Vec::new();

    if let Some(impls) = ctx.impl_index.type_impls.get(type_id) {
        for impl_info in impls {
            let for_type = linked.render_type(impl_info.for_type);
            let trait_name = impl_info.trait_path.as_deref();
            let header = render_impl_header_linked(impl_info.impl_, &for_type, trait_name);

            // Collect methods from this impl.
            let mut methods = Vec::new();
            for method_id in &impl_info.impl_.items {
                if let Some(method_item) = ctx.krate.index.get(method_id) {
                    if let ItemEnum::Function(f) = &method_item.inner {
                        let method_name = method_item.name.as_deref().unwrap_or("?");
                        let sig = linked.render_function_sig(f, method_name);
                        let method_links = ctx.resolve_item_links(&method_item.links, depth);
                        methods.push(MethodInfo {
                            signature: sig,
                            docs: method_item.docs.as_ref()
                                .map(|d| ctx.render_markdown_with_item_links(d, depth, &method_links))
                                .unwrap_or_default(),
                        });
                    }
                }
            }

            result.push(ImplBlockInfo { header, methods });
        }
    }

    // Sort: inherent impls first, then trait impls alphabetically.
    result.sort_by(|a, b| {
        let a_is_trait = a.header.contains(" for ");
        let b_is_trait = b.header.contains(" for ");
        match (a_is_trait, b_is_trait) {
            (false, true) => std::cmp::Ordering::Less,
            (true, false) => std::cmp::Ordering::Greater,
            _ => a.header.cmp(&b.header),
        }
    });

    result
}

/// Render an impl block header with HTML-escaped content.
/// The for_type should already be HTML (with links).
fn render_impl_header_linked(impl_: &rustdoc_types::Impl, for_type: &str, trait_name: Option<&str>) -> String {
    use super::signature::render_generic_param_def;

    let mut result = String::from("impl");

    // Generics.
    if !impl_.generics.params.is_empty() {
        result.push_str("&lt;");
        let params: Vec<_> = impl_.generics.params.iter()
            .map(|p| html_escape_sig(&render_generic_param_def(p)))
            .collect();
        result.push_str(&params.join(", "));
        result.push_str("&gt;");
    }

    result.push(' ');

    // Trait name if this is a trait impl.
    if let Some(name) = trait_name {
        result.push_str(&html_escape_sig(name));
        result.push_str(" for ");
    }

    result.push_str(for_type);

    result
}

/// HTML-escape a signature string for safe insertion into HTML.
fn html_escape_sig(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
