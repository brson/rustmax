//! Syntax highlighting with hyperlinks overlaid on identifiers.
//!
//! Output uses syntect's spaced CSS classes, the same as the rest of rustmax,
//! so `rustmax-syntax.css` applies.
//! Each line is rendered as self-contained HTML:
//! scopes still open at the end of a line are closed and reopened on the next,
//! so that lines can be wrapped in their own elements for anchoring.

use rmx::prelude::*;
use std::fmt::Write;
use syntect::parsing::{BasicScopeStackOp, ParseState, Scope, ScopeStack, SyntaxReference, SyntaxSet};

pub struct Highlighter {
    syntax_set: SyntaxSet,
}

/// A hyperlink over a byte range of one line.
pub struct Link {
    pub line: u32,
    pub start: u32,
    pub end: u32,
    /// Attributes of the `<a>` element, already escaped.
    pub attrs: String,
}

impl Highlighter {
    pub fn new() -> Highlighter {
        Highlighter { syntax_set: SyntaxSet::load_defaults_newlines() }
    }

    /// Pick a syntax by file name, then by first line, falling back to plain text.
    pub fn syntax_for(&self, file_name: &str, text: &str) -> &SyntaxReference {
        let ss = &self.syntax_set;
        let ext = file_name.rsplit_once('.').map(|(_, e)| e).unwrap_or(file_name);
        ss.find_syntax_by_extension(ext)
            .or_else(|| ss.find_syntax_by_extension(file_name))
            .or_else(|| ss.find_syntax_by_first_line(text.lines().next().unwrap_or("")))
            .unwrap_or_else(|| ss.find_syntax_plain_text())
    }

    /// Highlight a whole file into one HTML string per line.
    ///
    /// `links` must be sorted by line and start.
    /// Links that overlap an earlier link, or that do not fall on
    /// character boundaries, are dropped.
    pub fn highlight_lines(&self, text: &str, syntax: &SyntaxReference, links: &[Link]) -> AnyResult<Vec<String>> {
        let mut state = ParseState::new(syntax);
        let mut stack = ScopeStack::new();
        // The classes of each open scope, mirroring `stack`.
        let mut open: Vec<String> = Vec::new();
        let mut links = links.iter().peekable();
        let mut out = Vec::new();

        for (line_no, line) in syntect::util::LinesWithEndings::from(text).enumerate() {
            let line_no = line_no as u32;
            let ops = state.parse_line(line, &self.syntax_set)?;
            let content = line.trim_end_matches(['\n', '\r']);

            while links.next_if(|l| l.line < line_no).is_some() {}
            let mut line_links = Vec::new();
            while let Some(link) = links.next_if(|l| l.line == line_no) {
                let (start, end) = (link.start as usize, link.end as usize);
                let after_previous = line_links.last().is_none_or(|prev: &&Link| prev.end <= link.start);
                if after_previous
                    && end <= content.len()
                    && content.is_char_boundary(start)
                    && content.is_char_boundary(end)
                {
                    line_links.push(link);
                }
            }

            let mut html = String::new();
            for classes in &open {
                write!(html, "<span class=\"{classes}\">")?;
            }

            let mut ops = ops.iter().peekable();
            let mut line_links = line_links.into_iter().peekable();
            let mut current: Option<&Link> = None;
            let mut a_open = false;
            let mut pos = 0;
            loop {
                if current.is_some_and(|l| l.end as usize <= pos) {
                    current = None;
                }
                let op_here = ops.peek().is_some_and(|(at, _)| *at <= pos);
                if a_open && (op_here || current.is_none()) {
                    html.push_str("</a>");
                    a_open = false;
                }
                while let Some((_, op)) = ops.next_if(|(at, _)| *at <= pos) {
                    stack.apply_with_hook(op, |basic, _| match basic {
                        BasicScopeStackOp::Push(scope) => {
                            let classes = scope_classes(scope);
                            let _ = write!(html, "<span class=\"{classes}\">");
                            open.push(classes);
                        }
                        BasicScopeStackOp::Pop => {
                            html.push_str("</span>");
                            open.pop();
                        }
                    })?;
                }
                if current.is_none() {
                    current = line_links.next_if(|l| l.start as usize <= pos);
                }
                if pos >= content.len() {
                    break;
                }
                if let Some(link) = current
                    && !a_open
                {
                    write!(html, "<a {}>", link.attrs)?;
                    a_open = true;
                }

                let mut next = content.len();
                if let Some((at, _)) = ops.peek() {
                    next = next.min(*at);
                }
                match current {
                    Some(link) => next = next.min(link.end as usize),
                    None => {
                        if let Some(link) = line_links.peek() {
                            next = next.min(link.start as usize);
                        }
                    }
                }
                escape_into(&mut html, &content[pos..next]);
                pos = next;
            }
            if a_open {
                html.push_str("</a>");
            }
            // Ops at the newline are applied on the next iteration's stack,
            // but none can produce text, so applying them here is equivalent.
            for (_, op) in ops {
                stack.apply_with_hook(op, |basic, _| match basic {
                    BasicScopeStackOp::Push(scope) => open.push(scope_classes(scope)),
                    BasicScopeStackOp::Pop => {
                        open.pop();
                    }
                })?;
            }
            for _ in 0..open.len() {
                html.push_str("</span>");
            }
            out.push(html);
        }
        Ok(out)
    }

    /// Highlight a snippet of Rust, like a signature, as one HTML string.
    pub fn highlight_rust(&self, code: &str) -> AnyResult<String> {
        let syntax = self.syntax_set.find_syntax_by_extension("rs").expect("rust syntax");
        Ok(self.highlight_lines(code, syntax, &[])?.join("\n"))
    }
}

/// The classes syntect's `ClassStyle::Spaced` produces for a scope.
fn scope_classes(scope: Scope) -> String {
    scope.build_string().replace('.', " ")
}

pub fn escape_into(out: &mut String, s: &str) {
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            c => out.push(c),
        }
    }
}

pub fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    escape_into(&mut out, s);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn link(line: u32, start: u32, end: u32) -> Link {
        Link { line, start, end, attrs: "href=\"#x\"".to_string() }
    }

    /// Strip tags, leaving the text, to check nothing was lost or duplicated.
    fn text_of(html: &str) -> String {
        let mut out = String::new();
        let mut in_tag = false;
        for c in html.chars() {
            match c {
                '<' => in_tag = true,
                '>' => in_tag = false,
                c if !in_tag => out.push(c),
                _ => {}
            }
        }
        out.replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&amp;", "&")
    }

    fn balanced(html: &str) -> bool {
        html.matches("<span").count() == html.matches("</span>").count()
            && html.matches("<a ").count() == html.matches("</a>").count()
    }

    #[test]
    fn links_overlay_without_losing_text() {
        let h = Highlighter::new();
        let src = "fn main() {\n    let x = foo::<Vec<u8>>(\"s\");\n}\n";
        let syntax = h.syntax_for("main.rs", src);
        let links = [link(0, 3, 7), link(1, 8, 9), link(1, 12, 15), link(1, 20, 22)];
        let lines = h.highlight_lines(src, syntax, &links).unwrap();
        assert_eq!(lines.len(), 3);
        for (html, text) in lines.iter().zip(src.lines()) {
            assert_eq!(text_of(html), text);
            assert!(balanced(html), "{html}");
        }
        assert!(lines[0].contains("<a href=\"#x\">main</a>"), "{}", lines[0]);
        assert!(text_of(&lines[1]).contains("foo"));
    }

    #[test]
    fn multiline_scopes_are_reopened() {
        let h = Highlighter::new();
        let src = "/* one\ntwo */ fn x() {}\n";
        let syntax = h.syntax_for("a.rs", src);
        let lines = h.highlight_lines(src, syntax, &[]).unwrap();
        assert!(lines[1].starts_with("<span class=\"source rust\"><span class=\"comment"), "{}", lines[1]);
        for html in &lines {
            assert!(balanced(html), "{html}");
        }
    }

    #[test]
    fn bad_links_are_dropped() {
        let h = Highlighter::new();
        let src = "let é = 1;\n";
        let syntax = h.syntax_for("a.rs", src);
        let links = [link(0, 4, 5), link(0, 4, 6), link(0, 5, 6), link(0, 100, 101)];
        let lines = h.highlight_lines(src, syntax, &links).unwrap();
        assert_eq!(text_of(&lines[0]), "let é = 1;");
        assert_eq!(lines[0].matches("<a ").count(), 1);
    }
}
