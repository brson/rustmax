//! Template rendering with Tera.

use rustmax::prelude::*;
use rustmax::tera::{self, Tera, Context, Kwargs, State};
use rustmax::jiff::Zoned;
use rustmax::serde::Serialize;
use rustmax::walkdir::WalkDir;
use std::path::Path;

use crate::collection::{Config, Document};
use crate::build::{extract_headings_html, TableOfContents, TocOptions};
use crate::Result;

/// Name of the template used when the collection defines none.
const BUILTIN_TEMPLATE_NAME: &str = "_builtin/default.html";

/// Template engine wrapping Tera.
pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    /// Create a new template engine loading templates from the given directory.
    ///
    /// Templates are named by their path relative to `templates_dir`,
    /// so `templates/partials/head.html` is `partials/head.html`.
    pub fn new(templates_dir: &Path) -> Result<Self> {
        let mut tera = Tera::new();

        // Register custom filters. Tera resolves filter names while parsing a
        // template, so this has to happen before any template is added.
        tera.register_filter("date_format", filter_date_format);
        tera.register_filter("word_count", filter_word_count);
        tera.register_filter("reading_time", filter_reading_time);
        tera.register_filter("truncate_words", filter_truncate_words);

        // A collection need not have a templates directory;
        // it then renders with the built-in template below.
        if templates_dir.exists() {
            load_templates(&mut tera, templates_dir)?;
        }

        // Add built-in templates as fallback.
        tera.add_raw_template(BUILTIN_TEMPLATE_NAME, BUILTIN_DEFAULT_TEMPLATE)?;

        Ok(Self { tera })
    }

    /// Render `template_name`, or the built-in template if the collection
    /// does not define it.
    pub fn render(&self, template_name: &str, context: &Context) -> Result<String> {
        self.render_first(&[template_name], context)
    }

    /// Render the first of `candidates` that the collection defines,
    /// or the built-in template if it defines none of them.
    ///
    /// A template that exists but fails to render is an error;
    /// falling back to the built-in template would hide the failure.
    pub fn render_first(&self, candidates: &[&str], context: &Context) -> Result<String> {
        let name = candidates
            .iter()
            .copied()
            .find(|name| self.tera.contains_template(name))
            .unwrap_or(BUILTIN_TEMPLATE_NAME);

        Ok(self.tera.render(name, context)?)
    }

    /// Build template context for a document.
    pub fn document_context(
        &self,
        doc: &Document,
        config: &Config,
        html_content: &str,
    ) -> Context {
        let mut ctx = Context::new();

        // Site info.
        ctx.insert("site_title", &config.collection.title);
        ctx.insert("site_description", &config.collection.description);
        ctx.insert("site_author", &config.collection.author);
        ctx.insert("base_url", &config.collection.base_url);

        // Document info.
        ctx.insert("title", &doc.frontmatter.title);
        ctx.insert("content", html_content);
        ctx.insert("slug", &doc.slug());
        ctx.insert("url", &doc.url_path());
        ctx.insert("tags", &doc.frontmatter.tags);
        ctx.insert("draft", &doc.frontmatter.draft);
        ctx.insert("word_count", &doc.word_count());
        ctx.insert("reading_time", &doc.reading_time());

        if let Some(date) = doc.frontmatter.date {
            ctx.insert("date", &date.to_string());
        }

        if let Some(ref desc) = doc.frontmatter.description {
            ctx.insert("description", desc);
        }

        if let Some(ref author) = doc.frontmatter.author {
            ctx.insert("author", author);
        } else {
            ctx.insert("author", &config.collection.author);
        }

        // Extra frontmatter fields.
        for (key, value) in &doc.frontmatter.extra {
            ctx.insert(key.clone(), value);
        }

        // Generate table of contents.
        let toc_options = TocOptions::default();
        let headings = extract_headings_html(html_content);
        let filtered = toc_options.filter_headings(&headings);
        let toc = TableOfContents::from_headings(&filtered);
        ctx.insert("toc", &toc.to_html());
        ctx.insert("toc_list", &toc.to_html_list());
        ctx.insert("has_toc", &!toc.is_empty());

        // Build metadata.
        let now = Zoned::now();
        ctx.insert("build_time", &now.strftime("%Y-%m-%dT%H:%M:%S%z").to_string());

        ctx
    }

    /// Build template context for index page.
    pub fn index_context(&self, documents: &[&Document], config: &Config) -> Context {
        let mut ctx = Context::new();

        ctx.insert("site_title", &config.collection.title);
        ctx.insert("site_description", &config.collection.description);
        ctx.insert("site_author", &config.collection.author);
        ctx.insert("base_url", &config.collection.base_url);
        ctx.insert("title", &config.collection.title);

        let docs: Vec<DocumentSummary> = documents
            .iter()
            .map(|doc| DocumentSummary {
                title: doc.frontmatter.title.clone(),
                slug: doc.slug(),
                url: doc.url_path(),
                draft: doc.frontmatter.draft,
                tags: doc.frontmatter.tags.clone(),
                date: doc.frontmatter.date.map(|date| date.to_string()),
            })
            .collect();

        ctx.insert("documents", &docs);
        ctx.insert("is_index", &true);

        ctx
    }

    /// Build template context for tag page.
    pub fn tag_context(
        &self,
        tag: &str,
        documents: &[&Document],
        config: &Config,
    ) -> Context {
        let mut ctx = self.index_context(documents, config);
        ctx.insert("tag", tag);
        ctx.insert("title", &format!("Tag: {}", tag));
        ctx.insert("is_tag_page", &true);
        ctx
    }
}

/// Add every `.html` file under `templates_dir` to `tera`.
fn load_templates(tera: &mut Tera, templates_dir: &Path) -> Result<()> {
    for entry in WalkDir::new(templates_dir) {
        let entry = entry?;
        let path = entry.path();

        if !entry.file_type().is_file() {
            continue;
        }
        if path.extension().is_none_or(|ext| ext != "html") {
            continue;
        }

        let name = path
            .strip_prefix(templates_dir)
            .expect("walkdir entry is under the templates directory")
            .components()
            .map(|c| c.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/");

        tera.add_template_file(path, Some(&name))?;
    }

    Ok(())
}

/// A document as seen by index and tag templates.
#[derive(Serialize)]
struct DocumentSummary {
    title: String,
    slug: String,
    url: String,
    draft: bool,
    tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,
}

/// Filter: format a date string.
fn filter_date_format(value: &str, kwargs: Kwargs, _state: &State) -> tera::TeraResult<String> {
    let format = kwargs.get::<&str>("format")?.unwrap_or("%B %d, %Y");

    // Parse as jiff Date.
    let date: rustmax::jiff::civil::Date = value
        .parse()
        .map_err(|e| tera::Error::message(format!("invalid date: {}", e)))?;

    Ok(date.strftime(format).to_string())
}

/// Filter: count words in text.
fn filter_word_count(value: &str, _kwargs: Kwargs, _state: &State) -> usize {
    use rustmax::unicode_segmentation::UnicodeSegmentation;

    value.unicode_words().count()
}

/// Filter: estimate reading time.
fn filter_reading_time(value: &str, _kwargs: Kwargs, _state: &State) -> usize {
    use rustmax::unicode_segmentation::UnicodeSegmentation;

    let words = value.unicode_words().count();
    (words / 200).max(1)
}

/// Filter: truncate to N words.
fn filter_truncate_words(value: &str, kwargs: Kwargs, _state: &State) -> tera::TeraResult<String> {
    use rustmax::unicode_segmentation::UnicodeSegmentation;

    let count = kwargs.get::<usize>("count")?.unwrap_or(50);

    let words: Vec<&str> = value.unicode_words().take(count).collect();
    let truncated = words.join(" ");

    if value.unicode_words().count() > count {
        Ok(format!("{}...", truncated))
    } else {
        Ok(truncated)
    }
}

const BUILTIN_DEFAULT_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{% if title %}{{ title }} - {% endif %}{{ site_title }}</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            line-height: 1.6;
            max-width: 800px;
            margin: 0 auto;
            padding: 2rem;
            color: #333;
        }
        h1, h2, h3 { color: #111; }
        a { color: #0066cc; }
        pre {
            background: #f5f5f5;
            padding: 1rem;
            overflow-x: auto;
            border-radius: 4px;
        }
        code {
            background: #f5f5f5;
            padding: 0.2rem 0.4rem;
            border-radius: 2px;
        }
        pre code { background: none; padding: 0; }
        .meta { color: #666; font-size: 0.9rem; }
        .tags a {
            background: #eee;
            padding: 0.2rem 0.5rem;
            border-radius: 3px;
            text-decoration: none;
            font-size: 0.85rem;
        }
        .document-list { list-style: none; padding: 0; }
        .document-list li { margin-bottom: 1.5rem; }
        .document-list h2 { margin-bottom: 0.25rem; }
    </style>
</head>
<body>
    <header>
        <nav><a href="/">{{ site_title }}</a></nav>
    </header>
    <main>
        {% if is_index %}
        <h1>{{ site_title }}</h1>
        {% if site_description %}<p>{{ site_description }}</p>{% endif %}
        <ul class="document-list">
        {% for doc in documents %}
            <li>
                <h2><a href="{{ doc.url }}">{{ doc.title }}</a></h2>
                {% if doc.date %}<p class="meta">{{ doc.date }}</p>{% endif %}
            </li>
        {% endfor %}
        </ul>
        {% else %}
        <article>
            <h1>{{ title }}</h1>
            {% if date %}<p class="meta">{{ date }}{% if reading_time %} &middot; {{ reading_time }} min read{% endif %}</p>{% endif %}
            {% if tags %}<p class="tags">{% for tag in tags %}<a href="/tags/{{ tag }}/">{{ tag }}</a> {% endfor %}</p>{% endif %}
            {{ content | safe }}
        </article>
        {% endif %}
    </main>
</body>
</html>
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use rustmax::tempfile::tempdir;
    use std::fs;

    #[test]
    fn templates_are_named_relative_to_the_templates_dir() {
        let dir = tempdir().unwrap();
        let templates = dir.path().join("templates");
        fs::create_dir_all(templates.join("partials")).unwrap();
        fs::write(templates.join("page.html"), "<p>{{ title }}</p>").unwrap();
        fs::write(templates.join("partials/head.html"), "<title>{{ title }}</title>").unwrap();
        fs::write(templates.join("notes.txt"), "not a template").unwrap();

        let engine = TemplateEngine::new(&templates).unwrap();

        let mut ctx = Context::new();
        ctx.insert("title", "Hello");

        assert_eq!(engine.render("page.html", &ctx).unwrap(), "<p>Hello</p>");
        assert_eq!(
            engine.render("partials/head.html", &ctx).unwrap(),
            "<title>Hello</title>"
        );
    }

    #[test]
    fn custom_filters_are_available_to_loaded_templates() {
        let dir = tempdir().unwrap();
        let templates = dir.path().join("templates");
        fs::create_dir_all(&templates).unwrap();
        fs::write(
            templates.join("page.html"),
            r#"{{ date | date_format(format="%Y") }} {{ text | word_count }}"#,
        )
        .unwrap();

        let engine = TemplateEngine::new(&templates).unwrap();

        let mut ctx = Context::new();
        ctx.insert("date", "2026-08-22");
        ctx.insert("text", "one two three");

        assert_eq!(engine.render("page.html", &ctx).unwrap(), "2026 3");
    }

    #[test]
    fn a_missing_templates_dir_falls_back_to_the_builtin_template() {
        let dir = tempdir().unwrap();
        let engine = TemplateEngine::new(&dir.path().join("templates")).unwrap();

        let mut ctx = Context::new();
        ctx.insert("site_title", "Demo");
        ctx.insert("title", "Hello");
        ctx.insert("content", "<p>Body</p>");

        let html = engine.render("page.html", &ctx).unwrap();
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("<p>Body</p>"));
    }
}
