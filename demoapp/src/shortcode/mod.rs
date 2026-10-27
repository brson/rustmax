//! Shortcodes: small directives embedded in markdown, parsed with nom.
//!
//! - `{{< name arg="value" >}}` is an inline shortcode.
//! - `{{% name %}}content{{% /name %}}` is a block shortcode.
//!
//! Shortcodes are expanded to HTML before the markdown is rendered.
//! Anything inside a fenced code block or an inline code span is left alone,
//! so a document can show shortcode syntax without running it.
//! A shortcode Folio does not know is also left as written,
//! and `folio check` reports it.

use rmx::prelude::*;
use rmx::memchr::{memchr_iter, memmem, memrchr};
use rmx::nom::{
    IResult, Parser,
    bytes::complete::{tag, take_until, take_while1, take_while},
    character::complete::{char, multispace0, multispace1, alphanumeric1},
    combinator::{map, recognize},
    sequence::{delimited, preceded, separated_pair},
    multi::many0,
    branch::alt,
};
use std::collections::HashMap;
use std::ops::Range;

use crate::build::{extract_headings_html, html_escape, render_markdown, TableOfContents, TocOptions};

/// The shortcodes Folio renders.
pub const BUILTIN: &[&str] = &["youtube", "figure", "note", "warning", "tip", "code", "toc"];

/// A parsed shortcode.
#[derive(Debug, Clone, PartialEq)]
pub struct Shortcode {
    /// Shortcode name (e.g., "youtube", "note").
    pub name: String,
    /// Named arguments.
    pub args: HashMap<String, String>,
    /// Positional arguments.
    pub positional: Vec<String>,
    /// Inner content for block shortcodes.
    pub content: Option<String>,
    /// Whether this is an inline or block shortcode.
    pub is_block: bool,
}

impl Shortcode {
    /// Get an argument by name.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.args.get(key).map(|s| s.as_str())
    }

    /// Get a positional argument by index.
    pub fn positional(&self, index: usize) -> Option<&str> {
        self.positional.get(index).map(|s| s.as_str())
    }

    /// Whether Folio knows how to render this shortcode.
    pub fn is_builtin(&self) -> bool {
        BUILTIN.contains(&self.name.as_str())
    }
}

/// Parse a shortcode name (alphanumeric + underscores/hyphens).
fn parse_name(input: &str) -> IResult<&str, &str> {
    recognize((
        alphanumeric1,
        take_while(|c: char| c.is_alphanumeric() || c == '_' || c == '-')
    )).parse(input)
}

/// Parse a quoted string value.
fn parse_quoted_string(input: &str) -> IResult<&str, &str> {
    alt((
        delimited(char('"'), take_until("\""), char('"')),
        delimited(char('\''), take_until("'"), char('\'')),
    )).parse(input)
}

/// Parse an unquoted value (no spaces).
fn parse_unquoted_value(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| !c.is_whitespace() && c != '"' && c != '\'' && c != '>' && c != '%').parse(input)
}

/// Parse a value (quoted or unquoted).
fn parse_value(input: &str) -> IResult<&str, &str> {
    alt((parse_quoted_string, parse_unquoted_value)).parse(input)
}

/// Parse a named argument (key="value" or key=value).
fn parse_named_arg(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(
        parse_name,
        (multispace0, char('='), multispace0),
        parse_value
    ).parse(input)
}

/// Helper enum for parsing.
enum Either<L, R> {
    Left(L),
    Right(R),
}

/// Parse arguments (mix of named and positional).
fn parse_args(input: &str) -> IResult<&str, (HashMap<String, String>, Vec<String>)> {
    let mut named = HashMap::new();
    let mut positional = Vec::new();

    let (remaining, args) = many0(preceded(
        multispace1,
        alt((
            map(parse_named_arg, Either::Left),
            map(parse_value, Either::Right),
        ))
    )).parse(input)?;

    for arg in args {
        match arg {
            Either::Left((k, v)) => { named.insert(k.to_string(), v.to_string()); }
            Either::Right(v) => { positional.push(v.to_string()); }
        }
    }

    Ok((remaining, (named, positional)))
}

/// Parse an inline shortcode: `{{< name args >}}`
fn parse_inline_shortcode(input: &str) -> IResult<&str, Shortcode> {
    let (remaining, (_, _, name, args, _, _)) = (
        tag("{{<"),
        multispace0,
        parse_name,
        parse_args,
        multispace0,
        tag(">}}"),
    ).parse(input)?;

    Ok((remaining, Shortcode {
        name: name.to_string(),
        args: args.0,
        positional: args.1,
        content: None,
        is_block: false,
    }))
}

/// A parsed block-shortcode opening tag: name, named arguments, positional
/// arguments.
type BlockOpen<'a> = (&'a str, HashMap<String, String>, Vec<String>);

/// Parse block shortcode opening: `{{% name args %}}`
fn parse_block_open(input: &str) -> IResult<&str, BlockOpen<'_>> {
    let (remaining, (_, _, name, args, _, _)) = (
        tag("{{%"),
        multispace0,
        parse_name,
        parse_args,
        multispace0,
        tag("%}}"),
    ).parse(input)?;

    Ok((remaining, (name, args.0, args.1)))
}

/// Parse a complete block shortcode.
fn parse_block_shortcode(input: &str) -> IResult<&str, Shortcode> {
    let (after_open, (name, args, positional)) = parse_block_open(input)?;

    // The closing tag may be written with or without spaces around the slash.
    let closes = [
        format!("{{{{% /{} %}}}}", name),
        format!("{{{{%/{}%}}}}", name),
    ];

    for close in &closes {
        if let Some(close_pos) = memmem::find(after_open.as_bytes(), close.as_bytes()) {
            let content = &after_open[..close_pos];
            let remaining = &after_open[close_pos + close.len()..];

            return Ok((remaining, Shortcode {
                name: name.to_string(),
                args,
                positional,
                content: Some(content.to_string()),
                is_block: true,
            }));
        }
    }

    Err(rmx::nom::Err::Error(rmx::nom::error::Error::new(
        after_open,
        rmx::nom::error::ErrorKind::Tag
    )))
}

/// Parse any shortcode (inline or block).
pub fn parse_shortcode(input: &str) -> IResult<&str, Shortcode> {
    alt((parse_block_shortcode, parse_inline_shortcode)).parse(input)
}

/// A shortcode found in a document.
#[derive(Debug, Clone, PartialEq)]
pub struct Found {
    /// Byte range of the shortcode in the source, tags included.
    pub span: Range<usize>,
    pub shortcode: Shortcode,
}

/// Find every shortcode in a markdown document, in source order.
///
/// Shortcodes inside fenced code blocks and inline code spans are skipped.
pub fn find_shortcodes(markdown: &str) -> Vec<Found> {
    let bytes = markdown.as_bytes();
    let fences = fenced_ranges(markdown);
    let finder = memmem::Finder::new("{{");

    let mut found = Vec::new();
    let mut pos = 0;

    while let Some(offset) = finder.find(&bytes[pos..]) {
        let start = pos + offset;
        // `{` is ASCII, so `start + 1` is always a char boundary.
        pos = start + 1;

        if fences.iter().any(|fence| fence.contains(&start)) || in_inline_code(bytes, start) {
            continue;
        }

        if let Ok((rest, shortcode)) = parse_shortcode(&markdown[start..]) {
            let end = markdown.len() - rest.len();
            found.push(Found { span: start..end, shortcode });
            pos = end;
        }
    }

    found
}

/// Byte ranges of the fenced code blocks in a markdown document.
///
/// An unclosed fence runs to the end of the document, as in CommonMark.
fn fenced_ranges(markdown: &str) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();
    // Start offset, fence character and fence length of the open block.
    let mut open: Option<(usize, u8, usize)> = None;
    let mut offset = 0;

    for line in markdown.split_inclusive('\n') {
        let trimmed = line.trim_start_matches(' ');
        let indent = line.len() - trimmed.len();
        let fence = if indent <= 3 { fence_run(trimmed) } else { None };

        match (open, fence) {
            (None, Some((ch, len))) => open = Some((offset, ch, len)),
            (Some((start, ch, len)), Some((close_ch, close_len)))
                if close_ch == ch && close_len >= len && trimmed[close_len..].trim().is_empty() =>
            {
                ranges.push(start..offset + line.len());
                open = None;
            }
            _ => {}
        }

        offset += line.len();
    }

    if let Some((start, _, _)) = open {
        ranges.push(start..markdown.len());
    }

    ranges
}

/// The fence character and run length if `line` opens or closes a code fence.
fn fence_run(line: &str) -> Option<(u8, usize)> {
    let ch = *line.as_bytes().first()?;
    if ch != b'`' && ch != b'~' {
        return None;
    }
    let len = line.bytes().take_while(|&b| b == ch).count();
    (len >= 3).then_some((ch, len))
}

/// Whether `pos` falls inside an inline code span on its line.
///
/// An odd number of backticks earlier on the line means a span is open.
fn in_inline_code(bytes: &[u8], pos: usize) -> bool {
    let line_start = memrchr(b'\n', &bytes[..pos]).map_or(0, |i| i + 1);
    memchr_iter(b'`', &bytes[line_start..pos]).count() % 2 == 1
}

/// Markdown with its shortcodes expanded.
///
/// A `toc` shortcode cannot be rendered until the markdown has been, since it
/// has to link to the heading ids the renderer assigns. It is left as a marker
/// for [`Expanded::finish`] to fill in.
#[derive(Debug, Clone)]
pub struct Expanded {
    pub markdown: String,
    tocs: Vec<TocOptions>,
}

impl Expanded {
    /// Fill in the tables of contents in the rendered HTML.
    pub fn finish(&self, html: &str) -> String {
        if self.tocs.is_empty() {
            return html.to_string();
        }

        let headings = extract_headings_html(html);
        let mut html = html.to_string();

        for (index, options) in self.tocs.iter().enumerate() {
            let toc = TableOfContents::from_headings(&options.filter_headings(&headings));
            // The title is not a heading, so it does not end up in the page's
            // own table of contents.
            let rendered = if toc.is_empty() {
                String::new()
            } else if options.include_title {
                format!(
                    "<nav class=\"toc\">\n<p class=\"toc-title\">{}</p>\n{}</nav>\n",
                    html_escape(&options.title),
                    toc.to_html_list(),
                )
            } else {
                format!("<nav class=\"toc\">\n{}</nav>\n", toc.to_html_list())
            };
            html = html.replace(&toc_marker(index), &rendered);
        }

        html
    }
}

fn toc_marker(index: usize) -> String {
    format!("<!--folio:toc:{index}-->")
}

/// Expand every shortcode in a markdown document.
pub fn expand(markdown: &str) -> Expanded {
    let mut tocs = Vec::new();
    let mut out = String::with_capacity(markdown.len());
    let mut last_end = 0;

    for Found { span, shortcode } in find_shortcodes(markdown) {
        out.push_str(&markdown[last_end..span.start]);

        let rendered = if shortcode.name == "toc" {
            tocs.push(toc_options(&shortcode));
            Some(toc_marker(tocs.len() - 1))
        } else {
            render(&shortcode)
        };

        match rendered {
            Some(html) => out.push_str(&html),
            None => out.push_str(&markdown[span.clone()]),
        }
        last_end = span.end;
    }

    out.push_str(&markdown[last_end..]);
    Expanded { markdown: out, tocs }
}

fn toc_options(shortcode: &Shortcode) -> TocOptions {
    let defaults = TocOptions::default();
    TocOptions {
        min_level: shortcode.get("min").and_then(|s| s.parse().ok()).unwrap_or(defaults.min_level),
        max_level: shortcode.get("max").and_then(|s| s.parse().ok()).unwrap_or(defaults.max_level),
        include_title: shortcode.get("title") != Some("false"),
        title: shortcode.get("heading").map(str::to_string).unwrap_or(defaults.title),
    }
}

/// Render a built-in shortcode, other than `toc`, to HTML.
///
/// Returns `None` for a shortcode Folio does not know,
/// or one missing an argument it needs.
pub fn render(shortcode: &Shortcode) -> Option<String> {
    match shortcode.name.as_str() {
        "youtube" => {
            let id = html_escape(shortcode.get("id").or_else(|| shortcode.positional(0))?);
            Some(format!(
                r#"<div class="video-container"><iframe src="https://www.youtube.com/embed/{id}" frameborder="0" allowfullscreen></iframe></div>"#
            ))
        }
        "figure" => {
            let src = html_escape(shortcode.get("src").or_else(|| shortcode.positional(0))?);
            let alt = html_escape(shortcode.get("alt").unwrap_or(""));
            let caption = match shortcode.get("caption") {
                Some(caption) => format!("<figcaption>{}</figcaption>", html_escape(caption)),
                None => String::new(),
            };
            Some(format!(r#"<figure><img src="{src}" alt="{alt}">{caption}</figure>"#))
        }
        "note" | "warning" | "tip" => {
            let class = &shortcode.name;
            // The body is markdown in its own right, shortcodes and all.
            let body = expand(shortcode.content.as_deref().unwrap_or(""));
            let html = body.finish(&render_markdown(&body.markdown));
            Some(format!("<div class=\"admonition {class}\">\n{html}</div>\n"))
        }
        "code" => {
            let lang = html_escape(shortcode.get("lang").unwrap_or("text"));
            let content = html_escape(shortcode.content.as_deref().unwrap_or("").trim_matches('\n'));
            Some(format!(r#"<pre><code class="language-{lang}">{content}</code></pre>"#))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_inline_shortcode() {
        let (_, sc) = parse_inline_shortcode(r#"{{< youtube id="abc123" >}}"#).unwrap();
        assert_eq!(sc.name, "youtube");
        assert_eq!(sc.get("id"), Some("abc123"));
        assert!(!sc.is_block);
    }

    #[test]
    fn test_parse_inline_positional() {
        let (_, sc) = parse_inline_shortcode(r#"{{< youtube abc123 >}}"#).unwrap();
        assert_eq!(sc.name, "youtube");
        assert_eq!(sc.positional(0), Some("abc123"));
    }

    #[test]
    fn test_parse_block_shortcode() {
        let (_, sc) = parse_block_shortcode(r#"{{% note %}}This is important!{{% /note %}}"#).unwrap();
        assert_eq!(sc.name, "note");
        assert_eq!(sc.content, Some("This is important!".to_string()));
        assert!(sc.is_block);
    }

    #[test]
    fn finds_shortcodes_with_their_spans() {
        let content = r#"Hello {{< youtube id="xyz" >}} world {{% note %}}test{{% /note %}}"#;
        let found = find_shortcodes(content);
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].shortcode.name, "youtube");
        assert_eq!(&content[found[0].span.clone()], r#"{{< youtube id="xyz" >}}"#);
        assert_eq!(found[1].shortcode.name, "note");
    }

    #[test]
    fn non_ascii_text_around_shortcodes_does_not_panic() {
        let content = "Ünïcødé — {{< youtube abc >}} …and {{ not one }} ✓";
        let found = find_shortcodes(content);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].shortcode.positional(0), Some("abc"));
    }

    #[test]
    fn shortcodes_in_fenced_code_are_left_alone() {
        let content = "```markdown\n{{< youtube abc >}}\n```\n\n~~~~\n{{< youtube def >}}\n~~~~\n\n{{< youtube ghi >}}\n";
        let found = find_shortcodes(content);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].shortcode.positional(0), Some("ghi"));
    }

    #[test]
    fn a_shorter_fence_does_not_close_a_longer_one() {
        let content = "````\n```\n{{< youtube abc >}}\n```\n````\n";
        assert!(find_shortcodes(content).is_empty());
    }

    #[test]
    fn shortcodes_in_inline_code_are_left_alone() {
        let content = "Write `{{< youtube id >}}` to embed, like {{< youtube abc >}}.";
        let found = find_shortcodes(content);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].shortcode.positional(0), Some("abc"));
    }

    #[test]
    fn unknown_shortcodes_are_left_as_written() {
        let content = "before {{< mystery x=1 >}} after";
        assert_eq!(expand(content).markdown, content);
    }

    #[test]
    fn render_youtube() {
        let (_, sc) = parse_shortcode(r#"{{< youtube id="xyz123" >}}"#).unwrap();
        let html = render(&sc).unwrap();
        assert!(html.contains("youtube.com/embed/xyz123"));
    }

    #[test]
    fn arguments_are_escaped() {
        let (_, sc) = parse_shortcode(r#"{{< figure src="a.png" alt='say "hi"' caption="<b>" >}}"#).unwrap();
        let html = render(&sc).unwrap();
        assert!(html.contains(r#"alt="say &quot;hi&quot;""#), "{html}");
        assert!(html.contains("<figcaption>&lt;b&gt;</figcaption>"), "{html}");
    }

    #[test]
    fn admonition_bodies_are_rendered_as_markdown() {
        let (_, sc) = parse_shortcode("{{% note %}}This is **important**.{{% /note %}}").unwrap();
        let html = render(&sc).unwrap();
        assert!(html.contains("admonition note"));
        assert!(html.contains("<strong>important</strong>"), "{html}");
    }

    #[test]
    fn code_bodies_are_escaped() {
        let (_, sc) = parse_shortcode("{{% code lang=\"rust\" %}}\nlet v: Vec<u8> = vec![];\n{{% /code %}}").unwrap();
        let html = render(&sc).unwrap();
        assert_eq!(html, r#"<pre><code class="language-rust">let v: Vec&lt;u8&gt; = vec![];</code></pre>"#);
    }

    fn render_page(markdown: &str) -> String {
        let expanded = expand(markdown);
        expanded.finish(&render_markdown(&expanded.markdown))
    }

    #[test]
    fn toc_links_to_the_rendered_heading_ids() {
        let html = render_page("# Title\n\n{{< toc >}}\n\n## Section 1\n\nSome content.\n\n## Section 2\n");

        assert!(html.contains("<nav class=\"toc\">"), "{html}");
        assert!(html.contains("Table of Contents"));
        // Each link must land on an id the page actually has.
        for id in ["title", "section-1", "section-2"] {
            assert!(html.contains(&format!("id=\"{id}\"")), "{id} missing: {html}");
            assert!(html.contains(&format!("href=\"#{id}\"")), "{id} not linked: {html}");
        }
        assert!(!html.contains("folio:toc"));
    }

    #[test]
    fn toc_respects_level_bounds() {
        let html = render_page("# Title\n\n{{< toc min=\"2\" max=\"3\" >}}\n\n## Section 1\n\n### Subsection\n\n#### Deep\n");
        let nav = &html[html.find("<nav").unwrap()..html.find("</nav>").unwrap()];

        assert!(nav.contains("Section 1"));
        assert!(nav.contains("Subsection"));
        assert!(!nav.contains("Title"));
        assert!(!nav.contains("Deep"));
    }

    #[test]
    fn toc_title_can_be_renamed_or_dropped() {
        let html = render_page("{{< toc heading=\"Contents\" >}}\n\n## A\n");
        assert!(html.contains("<p class=\"toc-title\">Contents</p>"), "{html}");

        let html = render_page("{{< toc title=\"false\" >}}\n\n## A\n");
        assert!(!html.contains("toc-title"), "{html}");
    }
}
