//! Syntax highlighting using syntect with CSS classes.

use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;

/// Syntax highlighter for code blocks.
pub struct Highlighter {
    syntax_set: SyntaxSet,
}

impl Highlighter {
    /// Create a new highlighter.
    pub fn new() -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        Self { syntax_set }
    }

    /// Highlight code with the given language, returning HTML with CSS classes.
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

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}
