//! Syntax highlighting using syntect.

use syntect::highlighting::{ThemeSet, Theme};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

/// Syntax highlighter for code blocks.
pub struct Highlighter {
    syntax_set: SyntaxSet,
    theme: Theme,
}

impl Highlighter {
    /// Create a new highlighter with default theme.
    pub fn new() -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();
        let theme = theme_set.themes["InspiredGitHub"].clone();

        Self { syntax_set, theme }
    }

    /// Highlight code with the given language.
    pub fn highlight(&self, code: &str, lang: &str) -> String {
        let syntax = self.syntax_set
            .find_syntax_by_token(lang)
            .or_else(|| self.syntax_set.find_syntax_by_extension(lang))
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        highlighted_html_for_string(code, &self.syntax_set, syntax, &self.theme)
            .unwrap_or_else(|_| html_escape(code))
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
