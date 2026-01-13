//! Individual item rendering (struct, function, etc.).

use rmx::prelude::*;
use rmx::tera::Context;
use rustdoc_types::{ItemEnum, VariantKind};

use super::RenderContext;
use super::signature::{render_function_sig, render_struct_sig, render_enum_sig, render_trait_sig, render_type};
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

    // Get generics from the item.
    let generics = &s.generics;
    let signature = render_struct_sig(s, name, generics);
    tera_ctx.insert("signature", &signature);

    // Documentation.
    let docs = item.item.docs.as_ref()
        .map(|d| ctx.render_markdown(d))
        .unwrap_or_default();
    tera_ctx.insert("docs", &docs);

    // Fields (for plain structs).
    let mut fields = Vec::new();
    if let rustdoc_types::StructKind::Plain { fields: field_ids, .. } = &s.kind {
        for field_id in field_ids {
            if let Some(field_item) = ctx.krate.index.get(field_id) {
                if let ItemEnum::StructField(ty) = &field_item.inner {
                    fields.push(FieldInfo {
                        name: field_item.name.clone().unwrap_or_default(),
                        type_: render_type(ty),
                        docs: field_item.docs.as_ref()
                            .map(|d| ctx.render_markdown(d))
                            .unwrap_or_default(),
                    });
                }
            }
        }
    }
    tera_ctx.insert("fields", &fields);

    // Path to root.
    let depth = item.path.len().saturating_sub(1);
    let path_to_root = if depth == 0 { String::new() } else { "../".repeat(depth) };
    tera_ctx.insert("path_to_root", &path_to_root);

    // Sidebar HTML.
    let sidebar_html = super::sidebar::render_sidebar(ctx, &item.path, &path_to_root)?;
    tera_ctx.insert("sidebar", &sidebar_html);

    ctx.tera.render("struct.html", &tera_ctx)
        .context("Failed to render struct template")
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

    let signature = render_function_sig(func, name);
    tera_ctx.insert("signature", &signature);

    // Documentation.
    let docs = item.item.docs.as_ref()
        .map(|d| ctx.render_markdown(d))
        .unwrap_or_default();
    tera_ctx.insert("docs", &docs);

    // Path to root.
    let depth = item.path.len().saturating_sub(1);
    let path_to_root = if depth == 0 { String::new() } else { "../".repeat(depth) };
    tera_ctx.insert("path_to_root", &path_to_root);

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

    let signature = render_enum_sig(e, name, &e.generics);
    tera_ctx.insert("signature", &signature);

    // Documentation.
    let docs = item.item.docs.as_ref()
        .map(|d| ctx.render_markdown(d))
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

                variants.push(VariantInfo {
                    name: variant_item.name.clone().unwrap_or_default(),
                    fields: fields_str,
                    docs: variant_item.docs.as_ref()
                        .map(|d| ctx.render_markdown(d))
                        .unwrap_or_default(),
                });
            }
        }
    }
    tera_ctx.insert("variants", &variants);

    // Path to root.
    let depth = item.path.len().saturating_sub(1);
    let path_to_root = if depth == 0 { String::new() } else { "../".repeat(depth) };
    tera_ctx.insert("path_to_root", &path_to_root);

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

    let signature = render_trait_sig(t, name, &t.generics);
    tera_ctx.insert("signature", &signature);

    // Documentation.
    let docs = item.item.docs.as_ref()
        .map(|d| ctx.render_markdown(d))
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
                    associated_types.push(AssocTypeInfo {
                        name: trait_item.name.clone().unwrap_or_default(),
                        bounds: bounds_str,
                        docs: trait_item.docs.as_ref()
                            .map(|d| ctx.render_markdown(d))
                            .unwrap_or_default(),
                    });
                }
                ItemEnum::Function(f) => {
                    let method_name = trait_item.name.as_deref().unwrap_or("?");
                    let sig = render_function_sig(f, method_name);
                    let info = MethodInfo {
                        signature: sig,
                        docs: trait_item.docs.as_ref()
                            .map(|d| ctx.render_markdown(d))
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

    // Path to root.
    let depth = item.path.len().saturating_sub(1);
    let path_to_root = if depth == 0 { String::new() } else { "../".repeat(depth) };
    tera_ctx.insert("path_to_root", &path_to_root);

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

    let signature = format!("type {} = {}", name, render_type(&ta.type_));
    tera_ctx.insert("signature", &signature);

    // Documentation.
    let docs = item.item.docs.as_ref()
        .map(|d| ctx.render_markdown(d))
        .unwrap_or_default();
    tera_ctx.insert("docs", &docs);

    // Path to root.
    let depth = item.path.len().saturating_sub(1);
    let path_to_root = if depth == 0 { String::new() } else { "../".repeat(depth) };
    tera_ctx.insert("path_to_root", &path_to_root);

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

    let keyword = if is_static { "static" } else { "const" };
    let signature = if let Some(val) = value {
        format!("{} {}: {} = {}", keyword, name, render_type(type_), val)
    } else {
        format!("{} {}: {}", keyword, name, render_type(type_))
    };
    tera_ctx.insert("signature", &signature);

    // Documentation.
    let docs = item.item.docs.as_ref()
        .map(|d| ctx.render_markdown(d))
        .unwrap_or_default();
    tera_ctx.insert("docs", &docs);

    // Path to root.
    let depth = item.path.len().saturating_sub(1);
    let path_to_root = if depth == 0 { String::new() } else { "../".repeat(depth) };
    tera_ctx.insert("path_to_root", &path_to_root);

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

    let signature = macro_def.unwrap_or(&format!("macro_rules! {} {{ ... }}", name)).to_string();
    tera_ctx.insert("signature", &signature);

    // Documentation.
    let docs = item.item.docs.as_ref()
        .map(|d| ctx.render_markdown(d))
        .unwrap_or_default();
    tera_ctx.insert("docs", &docs);

    // Path to root.
    let depth = item.path.len().saturating_sub(1);
    let path_to_root = if depth == 0 { String::new() } else { "../".repeat(depth) };
    tera_ctx.insert("path_to_root", &path_to_root);

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
