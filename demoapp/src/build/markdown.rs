//! Markdown rendering with comrak.

use rustmax::comrak::{
    markdown_to_html, Options, ExtensionOptions, ParseOptions, RenderOptions,
};

/// Render markdown to HTML.
pub fn render_markdown(content: &str) -> String {
    let mut options = Options::default();

    // Enable common extensions.
    options.extension = ExtensionOptions {
        strikethrough: true,
        tagfilter: true,
        table: true,
        autolink: true,
        tasklist: true,
        superscript: true,
        header_ids: Some("heading-".to_string()),
        footnotes: true,
        description_lists: true,
        front_matter_delimiter: None,
        multiline_block_quotes: true,
        math_dollars: false,
        math_code: false,
        wikilinks_title_after_pipe: false,
        wikilinks_title_before_pipe: false,
        underline: true,
        subscript: true,
        spoiler: true,
        greentext: false,
        ..Default::default()
    };

    options.parse = ParseOptions {
        smart: true,
        default_info_string: None,
        relaxed_tasklist_matching: true,
        relaxed_autolinks: true,
        ..Default::default()
    };

    options.render = RenderOptions {
        hardbreaks: false,
        github_pre_lang: true,
        escape: false,
        unsafe_: true, // Allow raw HTML.
        ..Default::default()
    };

    markdown_to_html(content, &options)
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
}
