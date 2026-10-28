//! Syntax highlighting of markdown code blocks, with syntect's CSS classes.

use rmx::prelude::*;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{self, Write};

use comrak::adapters::SyntaxHighlighterAdapter;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;

/// Syntax highlighter using syntect.
pub struct Highlighter {
    syntax_set: SyntaxSet,
}

impl Highlighter {
    pub fn new() -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        Self { syntax_set }
    }

    pub fn highlight(&self, code: &str, lang: &str) -> String {
        let syntax = self.syntax_set
            .find_syntax_by_token(lang)
            .or_else(|| self.syntax_set.find_syntax_by_extension(lang))
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let mut generator = ClassedHTMLGenerator::new_with_class_style(
            syntax,
            &self.syntax_set,
            ClassStyle::Spaced,
        );

        for line in syntect::util::LinesWithEndings::from(code) {
            let _ = generator.parse_html_for_line_which_includes_newline(line);
        }

        generator.finalize()
    }
}

/// Adapter for comrak's syntax highlighting.
pub struct HighlightAdapter<'a> {
    pub highlighter: &'a Highlighter,
}

impl SyntaxHighlighterAdapter for HighlightAdapter<'_> {
    fn write_highlighted(
        &self,
        output: &mut dyn Write,
        lang: Option<&str>,
        code: &str,
    ) -> fmt::Result {
        // Default to rust for unlabeled code blocks, strip modifiers like ",ignore".
        let base_lang = lang
            .map(|l| l.split(',').next().unwrap_or(l).trim())
            .filter(|l| !l.is_empty())
            .unwrap_or("rust");

        if base_lang == "text" {
            write!(output, "{}", html_escape(code))
        } else {
            let highlighted = self.highlighter.highlight(code, base_lang);
            write!(output, "{}", highlighted)
        }
    }

    fn write_pre_tag(
        &self,
        output: &mut dyn Write,
        attributes: HashMap<&'static str, Cow<'_, str>>,
    ) -> fmt::Result {
        let mut attrs = String::new();
        for (key, value) in &attributes {
            attrs.push_str(&format!(" {}=\"{}\"", key, html_escape(value)));
        }
        write!(output, "<pre class=\"highlight\"{}>", attrs)
    }

    fn write_code_tag(
        &self,
        output: &mut dyn Write,
        attributes: HashMap<&'static str, Cow<'_, str>>,
    ) -> fmt::Result {
        // Fence info strings carry modifiers after the language, as in
        // `rust,ignore` or `grammar,expressions`. Only the language belongs in
        // the class.
        let lang = attributes.get("class")
            .and_then(|c| c.strip_prefix("language-"))
            .and_then(|c| c.split(',').next())
            .filter(|c| !c.is_empty())
            .unwrap_or("rust");
        write!(output, "<code class=\"language-{}\">", html_escape(lang))
    }
}

pub fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
