//! Template rendering with Tera.

use rustmax::prelude::*;
use rustmax::tera::{self, Tera, Context, Value};
use rustmax::jiff::Zoned;
use std::path::Path;
use std::collections::HashMap;

use crate::collection::{Config, Document};
use crate::build::{extract_headings_html, TableOfContents, TocOptions};
use crate::Result;

/// Template engine wrapping Tera.
pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    /// Create a new template engine loading templates from the given directory.
    pub fn new(templates_dir: &Path) -> Result<Self> {
        let pattern = templates_dir.join("**/*.html");
        let pattern_str = pattern.to_string_lossy();

        let mut tera = Tera::new(&pattern_str)?;

        // Register custom filters.
        tera.register_filter("date_format", filter_date_format);
        tera.register_filter("word_count", filter_word_count);
        tera.register_filter("reading_time", filter_reading_time);
        tera.register_filter("truncate_words", filter_truncate_words);

        // Add built-in templates as fallback.
        tera.add_raw_template("_builtin/default.html", BUILTIN_DEFAULT_TEMPLATE)?;

        Ok(Self { tera })
    }

    /// Render a template with the given context.
    pub fn render(&self, template_name: &str, context: &Context) -> Result<String> {
        // Try the requested template, fall back to builtin.
        let result = self.tera.render(template_name, context).or_else(|_| {
            self.tera.render("_builtin/default.html", context)
        })?;
        Ok(result)
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
            ctx.insert(key, &toml_to_tera_value(value));
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

        let docs: Vec<HashMap<&str, Value>> = documents
            .iter()
            .map(|doc| {
                let mut map = HashMap::new();
                map.insert("title", Value::String(doc.frontmatter.title.clone()));
                map.insert("slug", Value::String(doc.slug()));
                map.insert("url", Value::String(doc.url_path()));
                map.insert("draft", Value::Bool(doc.frontmatter.draft));
                map.insert(
                    "tags",
                    Value::Array(
                        doc.frontmatter
                            .tags
                            .iter()
                            .map(|t| Value::String(t.clone()))
                            .collect(),
                    ),
                );
                if let Some(date) = doc.frontmatter.date {
                    map.insert("date", Value::String(date.to_string()));
                }
                map
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

/// Convert TOML value to Tera value.
fn toml_to_tera_value(value: &rustmax::toml::Value) -> Value {
    match value {
        rustmax::toml::Value::String(s) => Value::String(s.clone()),
        rustmax::toml::Value::Integer(i) => Value::Number((*i).into()),
        rustmax::toml::Value::Float(f) => {
            // from_f64 returns None for NaN/Infinity, fall back to 0.
            match tera::Number::from_f64(*f) {
                Some(n) => Value::Number(n),
                None => Value::Number(0.into()),
            }
        }
        rustmax::toml::Value::Boolean(b) => Value::Bool(*b),
        rustmax::toml::Value::Array(arr) => {
            Value::Array(arr.iter().map(toml_to_tera_value).collect())
        }
        rustmax::toml::Value::Table(tbl) => {
            let map: tera::Map<String, Value> = tbl
                .iter()
                .map(|(k, v)| (k.clone(), toml_to_tera_value(v)))
                .collect();
            Value::Object(map)
        }
        rustmax::toml::Value::Datetime(dt) => Value::String(dt.to_string()),
    }
}

/// Filter: format a date string.
fn filter_date_format(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    let date_str = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("date_format expects a string"))?;

    let format = args
        .get("format")
        .and_then(|v| v.as_str())
        .unwrap_or("%B %d, %Y");

    // Parse as jiff Date.
    let date: rustmax::jiff::civil::Date = date_str
        .parse()
        .map_err(|e| tera::Error::msg(format!("invalid date: {}", e)))?;

    Ok(Value::String(date.strftime(format).to_string()))
}

/// Filter: count words in text.
fn filter_word_count(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    use rustmax::unicode_segmentation::UnicodeSegmentation;

    let text = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("word_count expects a string"))?;

    let count = text.unicode_words().count();
    Ok(Value::Number(count.into()))
}

/// Filter: estimate reading time.
fn filter_reading_time(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    use rustmax::unicode_segmentation::UnicodeSegmentation;

    let text = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("reading_time expects a string"))?;

    let words = text.unicode_words().count();
    let minutes = (words / 200).max(1);
    Ok(Value::Number(minutes.into()))
}

/// Filter: truncate to N words.
fn filter_truncate_words(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    use rustmax::unicode_segmentation::UnicodeSegmentation;

    let text = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("truncate_words expects a string"))?;

    let count = args
        .get("count")
        .and_then(|v| v.as_u64())
        .unwrap_or(50) as usize;

    let words: Vec<&str> = text.unicode_words().take(count).collect();
    let truncated = words.join(" ");

    if text.unicode_words().count() > count {
        Ok(Value::String(format!("{}...", truncated)))
    } else {
        Ok(Value::String(truncated))
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
