//! Markdown rendering with comrak and syntax highlighting.

use rustmax::comrak::{markdown_to_html, Options};
use rustmax::regex::Regex;
use std::sync::LazyLock;

use super::highlight::{Highlighter, HighlightOptions, themes};

/// Regex to match code blocks in HTML output.
static CODE_BLOCK_RE: LazyLock<Regex> = LazyLock::new(|| {
    // Match <pre lang="..."><code>...</code></pre> or <pre><code class="language-...">...</code></pre>
    // Using alternation to handle both formats.
    Regex::new(r#"<pre(?:\s+lang="([^"]*)")?\s*><code(?:\s+class="language-([^"]*)")?\s*>([\s\S]*?)</code></pre>"#)
        .expect("invalid regex")
});

/// Render markdown to HTML.
pub fn render_markdown(content: &str) -> String {
    render_markdown_internal(content, &Options::default())
}

/// Render markdown to HTML with syntax highlighting.
pub fn render_markdown_highlighted(content: &str, options: &HighlightOptions) -> String {
    let html = render_markdown(content);
    apply_syntax_highlighting(&html, options)
}

/// Render markdown with custom comrak options.
fn render_markdown_internal(content: &str, _base_options: &Options) -> String {
    let mut options = Options::default();

    // Enable common extensions.
    options.extension.strikethrough = true;
    options.extension.tagfilter = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.extension.header_id_prefix = Some("heading-".to_string());
    options.extension.footnotes = true;
    options.extension.description_lists = true;
    options.extension.multiline_block_quotes = true;
    options.extension.underline = true;
    options.extension.subscript = true;
    options.extension.spoiler = true;

    options.parse.smart = true;
    options.parse.relaxed_tasklist_matching = true;
    options.parse.relaxed_autolinks = true;

    options.render.github_pre_lang = true;
    // Allow raw HTML.
    options.render.r#unsafe = true;

    markdown_to_html(content, &options)
}

/// Apply syntax highlighting to code blocks in HTML.
pub fn apply_syntax_highlighting(html: &str, options: &HighlightOptions) -> String {
    let theme = themes::by_name(&options.theme)
        .unwrap_or_else(themes::github_dark);

    let highlighter = Highlighter::new()
        .with_theme(theme)
        .with_line_numbers(options.line_numbers)
        .with_copy_button(options.copy_button);

    CODE_BLOCK_RE.replace_all(html, |caps: &rustmax::regex::Captures| {
        // Get language from either lang attribute or class.
        let lang = caps.get(1)
            .or_else(|| caps.get(2))
            .map(|m| m.as_str())
            .unwrap_or("text");

        // Get code content (may be HTML-escaped).
        let code = caps.get(3).map(|m| m.as_str()).unwrap_or("");

        // Unescape HTML entities in code.
        let code = html_unescape(code);

        // Highlight the code.
        highlighter.highlight(&code, lang)
    }).into_owned()
}

/// Generate CSS for syntax highlighting.
pub fn generate_highlight_css(options: &HighlightOptions) -> String {
    let theme = themes::by_name(&options.theme)
        .unwrap_or_else(themes::github_dark);
    theme.generate_css()
}

/// Unescape HTML entities.
fn html_unescape(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_markdown() {
        let md = "# Hello\n\nThis is a **test**.";
        let html = render_markdown(md);
        assert!(html.contains("<h1"));
        assert!(html.contains("<strong>test</strong>"));
    }

    #[test]
    fn test_code_block() {
        let md = "```rust\nfn main() {}\n```";
        let html = render_markdown(md);
        assert!(html.contains("<code"));
        assert!(html.contains("rust"));
    }

    #[test]
    fn test_table() {
        let md = "| A | B |\n|---|---|\n| 1 | 2 |";
        let html = render_markdown(md);
        assert!(html.contains("<table"));
    }

    #[test]
    fn test_task_list() {
        let md = "- [x] Done\n- [ ] Todo";
        let html = render_markdown(md);
        assert!(html.contains("checked"));
    }

    #[test]
    fn test_highlighted_markdown() {
        let md = "```rust\nfn main() { println!(\"hello\"); }\n```";
        let options = HighlightOptions::default();
        let html = render_markdown_highlighted(md, &options);

        // Should contain highlighted spans.
        assert!(html.contains("highlight"), "Should have highlight wrapper");
    }

    #[test]
    fn test_apply_syntax_highlighting() {
        let html = r#"<pre lang="rust"><code>fn main() {}</code></pre>"#;
        let options = HighlightOptions::default();
        let highlighted = apply_syntax_highlighting(html, &options);

        assert!(highlighted.contains("hl-kw"), "Should have keyword class");
    }

    #[test]
    fn test_generate_highlight_css() {
        let options = HighlightOptions {
            theme: "monokai".to_string(),
            line_numbers: true,
            copy_button: true,
        };
        let css = generate_highlight_css(&options);

        assert!(css.contains(".highlight"));
        assert!(css.contains(".hl-kw"));
        assert!(css.contains("#272822")); // Monokai background
    }

    #[test]
    fn test_html_unescape() {
        assert_eq!(html_unescape("&lt;div&gt;"), "<div>");
        assert_eq!(html_unescape("&amp;&amp;"), "&&");
        assert_eq!(html_unescape("&quot;test&quot;"), "\"test\"");
    }

    #[test]
    fn test_highlighted_multiple_blocks() {
        let md = r#"
```rust
fn foo() {}
```

Some text.

```python
def bar():
    pass
```
"#;
        let options = HighlightOptions::default();
        let html = render_markdown_highlighted(md, &options);

        // Count highlight wrappers.
        let count = html.matches("highlight-wrapper").count();
        assert!(count >= 2, "Should have at least 2 code blocks highlighted, got {}", count);
    }

    #[test]
    fn test_highlighted_with_line_numbers() {
        let md = "```rust\nline 1\nline 2\nline 3\n```";
        let options = HighlightOptions {
            theme: "github-dark".to_string(),
            line_numbers: true,
            copy_button: false,
        };
        let html = render_markdown_highlighted(md, &options);

        assert!(html.contains("line-numbers"));
    }

    #[test]
    fn test_highlighted_without_line_numbers() {
        let md = "```rust\nfn main() {}\n```";
        let options = HighlightOptions {
            theme: "github-dark".to_string(),
            line_numbers: false,
            copy_button: false,
        };
        let html = render_markdown_highlighted(md, &options);

        // Single line code should not have line numbers wrapper.
        // Only multi-line code with line_numbers=true shows numbers.
        assert!(html.contains("highlight"));
    }

    #[test]
    fn test_theme_fallback() {
        let options = HighlightOptions {
            theme: "nonexistent-theme".to_string(),
            line_numbers: true,
            copy_button: true,
        };
        let css = generate_highlight_css(&options);

        // Should fall back to github-dark.
        assert!(css.contains("#0d1117")); // github-dark background
    }
}
