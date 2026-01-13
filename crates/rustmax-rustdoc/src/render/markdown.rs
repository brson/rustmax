//! Markdown to HTML rendering with syntax highlighting.

use comrak::{markdown_to_html_with_plugins, Options, Plugins};
use comrak::adapters::SyntaxHighlighterAdapter;

use super::highlight::Highlighter;

/// Render markdown to HTML with syntax highlighting.
pub fn render_markdown(md: &str, highlighter: &Highlighter) -> String {
    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;
    options.render.unsafe_ = true; // Allow raw HTML in docs.

    let adapter = HighlightAdapter { highlighter };
    let mut plugins = Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    markdown_to_html_with_plugins(md, &options, &plugins)
}

struct HighlightAdapter<'a> {
    highlighter: &'a Highlighter,
}

impl SyntaxHighlighterAdapter for HighlightAdapter<'_> {
    fn write_highlighted(
        &self,
        output: &mut dyn std::io::Write,
        lang: Option<&str>,
        code: &str,
    ) -> std::io::Result<()> {
        let lang = lang.unwrap_or("rust");
        let highlighted = self.highlighter.highlight(code, lang);
        write!(output, "{}", highlighted)
    }

    fn write_pre_tag(
        &self,
        output: &mut dyn std::io::Write,
        attributes: std::collections::HashMap<String, String>,
    ) -> std::io::Result<()> {
        let mut attrs = String::new();
        for (key, value) in &attributes {
            attrs.push_str(&format!(" {}=\"{}\"", key, html_escape(value)));
        }
        write!(output, "<pre class=\"highlight\"{}>", attrs)
    }

    fn write_code_tag(
        &self,
        output: &mut dyn std::io::Write,
        attributes: std::collections::HashMap<String, String>,
    ) -> std::io::Result<()> {
        let lang = attributes.get("class")
            .and_then(|c| c.strip_prefix("language-"))
            .unwrap_or("rust");
        write!(output, "<code class=\"language-{}\">", html_escape(lang))
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
