//! Shortcode parsing using nom.
//!
//! Shortcodes allow embedding dynamic content in markdown:
//! - `{{< name arg1="value" >}}` - inline shortcode
//! - `{{% name %}}content{{% /name %}}` - block shortcode

use rustmax::prelude::*;
use rustmax::nom::{
    IResult, Parser,
    bytes::complete::{tag, take_until, take_while1, take_while},
    character::complete::{char, multispace0, multispace1, alphanumeric1},
    combinator::{map, recognize},
    sequence::{delimited, preceded, separated_pair},
    multi::many0,
    branch::alt,
};
use std::collections::HashMap;

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
            map(parse_named_arg, |arg| Either::Left(arg)),
            map(parse_value, |v| Either::Right(v)),
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

/// Parse block shortcode opening: `{{% name args %}}`
fn parse_block_open(input: &str) -> IResult<&str, (&str, HashMap<String, String>, Vec<String>)> {
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

    // Build close tag patterns.
    let close_with_spaces = format!("{{{{% /{} %}}}}", name);
    let close_no_spaces = format!("{{{{%/{}}}}}", name);

    // Try with spaces around slash.
    if let Some(close_pos) = after_open.find(&close_with_spaces) {
        let content = &after_open[..close_pos];
        let remaining = &after_open[close_pos + close_with_spaces.len()..];

        return Ok((remaining, Shortcode {
            name: name.to_string(),
            args,
            positional,
            content: Some(content.to_string()),
            is_block: true,
        }));
    }

    // Try without spaces.
    if let Some(close_pos) = after_open.find(&close_no_spaces) {
        let content = &after_open[..close_pos];
        let remaining = &after_open[close_pos + close_no_spaces.len()..];

        return Ok((remaining, Shortcode {
            name: name.to_string(),
            args,
            positional,
            content: Some(content.to_string()),
            is_block: true,
        }));
    }

    Err(rustmax::nom::Err::Error(rustmax::nom::error::Error::new(
        after_open,
        rustmax::nom::error::ErrorKind::Tag
    )))
}

/// Parse any shortcode (inline or block).
pub fn parse_shortcode(input: &str) -> IResult<&str, Shortcode> {
    alt((parse_block_shortcode, parse_inline_shortcode)).parse(input)
}

/// Find and extract all shortcodes from content.
pub fn extract_shortcodes(content: &str) -> Vec<(usize, Shortcode, usize)> {
    let mut results = Vec::new();
    let mut pos = 0;

    while pos < content.len() {
        let remaining = &content[pos..];

        // Look for shortcode start.
        if remaining.starts_with("{{<") || remaining.starts_with("{{%") {
            if let Ok((after, shortcode)) = parse_shortcode(remaining) {
                let end_pos = pos + (remaining.len() - after.len());
                results.push((pos, shortcode, end_pos));
                pos = end_pos;
                continue;
            }
        }

        pos += 1;
    }

    results
}

/// Process content, replacing shortcodes with rendered output.
pub fn process_shortcodes<F>(content: &str, mut renderer: F) -> String
where
    F: FnMut(&Shortcode) -> String,
{
    let shortcodes = extract_shortcodes(content);

    if shortcodes.is_empty() {
        return content.to_string();
    }

    let mut result = String::with_capacity(content.len());
    let mut last_end = 0;

    for (start, shortcode, end) in shortcodes {
        result.push_str(&content[last_end..start]);
        result.push_str(&renderer(&shortcode));
        last_end = end;
    }

    result.push_str(&content[last_end..]);
    result
}

/// Built-in shortcode renderer.
pub fn render_builtin(shortcode: &Shortcode) -> Option<String> {
    match shortcode.name.as_str() {
        "youtube" => {
            let id = shortcode.get("id").or_else(|| shortcode.positional(0))?;
            Some(format!(
                r#"<div class="video-container"><iframe src="https://www.youtube.com/embed/{id}" frameborder="0" allowfullscreen></iframe></div>"#
            ))
        }
        "figure" => {
            let src = shortcode.get("src").or_else(|| shortcode.positional(0))?;
            let alt = shortcode.get("alt").unwrap_or("");
            let caption = shortcode.get("caption").unwrap_or("");
            Some(format!(
                r#"<figure><img src="{src}" alt="{alt}"><figcaption>{caption}</figcaption></figure>"#
            ))
        }
        "note" | "warning" | "tip" => {
            let class = &shortcode.name;
            let content = shortcode.content.as_deref().unwrap_or("");
            Some(format!(r#"<div class="admonition {class}">{content}</div>"#))
        }
        "code" => {
            let lang = shortcode.get("lang").unwrap_or("text");
            let content = shortcode.content.as_deref().unwrap_or("");
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
    fn test_extract_shortcodes() {
        let content = r#"Hello {{< youtube id="xyz" >}} world {{% note %}}test{{% /note %}}"#;
        let codes = extract_shortcodes(content);
        assert_eq!(codes.len(), 2);
        assert_eq!(codes[0].1.name, "youtube");
        assert_eq!(codes[1].1.name, "note");
    }

    #[test]
    fn test_process_shortcodes() {
        let content = r#"Watch: {{< youtube id="abc" >}}"#;
        let result = process_shortcodes(content, |sc| {
            if sc.name == "youtube" {
                format!("[VIDEO:{}]", sc.get("id").unwrap_or("?"))
            } else {
                String::new()
            }
        });
        assert_eq!(result, "Watch: [VIDEO:abc]");
    }

    #[test]
    fn test_render_builtin_youtube() {
        let sc = Shortcode {
            name: "youtube".to_string(),
            args: [("id".to_string(), "xyz123".to_string())].into_iter().collect(),
            positional: vec![],
            content: None,
            is_block: false,
        };
        let html = render_builtin(&sc).unwrap();
        assert!(html.contains("youtube.com/embed/xyz123"));
    }

    #[test]
    fn test_render_builtin_note() {
        let sc = Shortcode {
            name: "note".to_string(),
            args: HashMap::new(),
            positional: vec![],
            content: Some("Important!".to_string()),
            is_block: true,
        };
        let html = render_builtin(&sc).unwrap();
        assert!(html.contains("admonition note"));
        assert!(html.contains("Important!"));
    }
}
