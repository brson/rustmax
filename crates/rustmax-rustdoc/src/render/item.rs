//! Individual item rendering (struct, function, etc.).

use rmx::prelude::*;
use rmx::tera::Context;
use rustdoc_types::ItemEnum;

use super::RenderContext;
use super::signature::{render_function_sig, render_struct_sig, render_type};
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

    // Sidebar HTML.
    let sidebar_html = super::sidebar::render_sidebar(ctx, &item.path)?;
    tera_ctx.insert("sidebar", &sidebar_html);

    // Path to root.
    let depth = item.path.len().saturating_sub(1);
    let path_to_root = if depth == 0 { String::new() } else { "../".repeat(depth) };
    tera_ctx.insert("path_to_root", &path_to_root);

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

    // Sidebar HTML.
    let sidebar_html = super::sidebar::render_sidebar(ctx, &item.path)?;
    tera_ctx.insert("sidebar", &sidebar_html);

    // Path to root.
    let depth = item.path.len().saturating_sub(1);
    let path_to_root = if depth == 0 { String::new() } else { "../".repeat(depth) };
    tera_ctx.insert("path_to_root", &path_to_root);

    ctx.tera.render("function.html", &tera_ctx)
        .context("Failed to render function template")
}

#[derive(serde::Serialize)]
struct FieldInfo {
    name: String,
    type_: String,
    docs: String,
}
