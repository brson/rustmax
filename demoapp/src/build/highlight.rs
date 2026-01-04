//! Syntax highlighting for code blocks.
//!
//! Provides regex-based tokenization for multiple programming languages
//! with configurable color themes.

use rustmax::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

/// Token types for syntax highlighting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    Keyword,
    String,
    Comment,
    Number,
    Operator,
    Punctuation,
    Function,
    Type,
    Variable,
    Attribute,
    Macro,
    Constant,
    Builtin,
    Plain,
}

impl TokenType {
    /// CSS class name for this token type.
    pub fn css_class(&self) -> &'static str {
        match self {
            TokenType::Keyword => "hl-kw",
            TokenType::String => "hl-str",
            TokenType::Comment => "hl-cmt",
            TokenType::Number => "hl-num",
            TokenType::Operator => "hl-op",
            TokenType::Punctuation => "hl-punc",
            TokenType::Function => "hl-fn",
            TokenType::Type => "hl-type",
            TokenType::Variable => "hl-var",
            TokenType::Attribute => "hl-attr",
            TokenType::Macro => "hl-mac",
            TokenType::Constant => "hl-const",
            TokenType::Builtin => "hl-bi",
            TokenType::Plain => "hl-pl",
        }
    }
}

/// A token in highlighted code.
#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub token_type: TokenType,
}

/// Color theme for syntax highlighting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub background: String,
    pub foreground: String,
    pub line_number: String,
    pub line_number_bg: String,
    pub selection: String,
    pub colors: ThemeColors,
}

/// Token colors for a theme.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub keyword: String,
    pub string: String,
    pub comment: String,
    pub number: String,
    pub operator: String,
    pub punctuation: String,
    pub function: String,
    pub r#type: String,
    pub variable: String,
    pub attribute: String,
    pub r#macro: String,
    pub constant: String,
    pub builtin: String,
}

impl Theme {
    /// Get color for a token type.
    pub fn color_for(&self, token_type: TokenType) -> &str {
        match token_type {
            TokenType::Keyword => &self.colors.keyword,
            TokenType::String => &self.colors.string,
            TokenType::Comment => &self.colors.comment,
            TokenType::Number => &self.colors.number,
            TokenType::Operator => &self.colors.operator,
            TokenType::Punctuation => &self.colors.punctuation,
            TokenType::Function => &self.colors.function,
            TokenType::Type => &self.colors.r#type,
            TokenType::Variable => &self.colors.variable,
            TokenType::Attribute => &self.colors.attribute,
            TokenType::Macro => &self.colors.r#macro,
            TokenType::Constant => &self.colors.constant,
            TokenType::Builtin => &self.colors.builtin,
            TokenType::Plain => &self.foreground,
        }
    }

    /// Generate CSS for this theme.
    pub fn generate_css(&self) -> String {
        format!(
            r#".highlight {{
  background: {bg};
  color: {fg};
  padding: 1em;
  border-radius: 6px;
  overflow-x: auto;
  font-family: 'SF Mono', Menlo, Monaco, Consolas, monospace;
  font-size: 0.9em;
  line-height: 1.5;
}}
.highlight pre {{
  margin: 0;
  background: transparent;
}}
.highlight code {{
  background: transparent;
}}
.hl-kw {{ color: {kw}; font-weight: bold; }}
.hl-str {{ color: {str}; }}
.hl-cmt {{ color: {cmt}; font-style: italic; }}
.hl-num {{ color: {num}; }}
.hl-op {{ color: {op}; }}
.hl-punc {{ color: {punc}; }}
.hl-fn {{ color: {fn_}; }}
.hl-type {{ color: {type_}; }}
.hl-var {{ color: {var}; }}
.hl-attr {{ color: {attr}; }}
.hl-mac {{ color: {mac}; }}
.hl-const {{ color: {const_}; }}
.hl-bi {{ color: {bi}; }}
.hl-pl {{ color: {fg}; }}
.highlight .line-numbers {{
  background: {ln_bg};
  color: {ln};
  padding: 1em 0.5em;
  margin-right: 1em;
  border-right: 1px solid {ln};
  user-select: none;
  text-align: right;
  min-width: 2em;
  display: inline-block;
}}
.highlight .copy-btn {{
  position: absolute;
  top: 0.5em;
  right: 0.5em;
  background: {ln_bg};
  color: {fg};
  border: 1px solid {ln};
  border-radius: 4px;
  padding: 0.25em 0.5em;
  cursor: pointer;
  opacity: 0;
  transition: opacity 0.2s;
}}
.highlight:hover .copy-btn {{
  opacity: 1;
}}
.highlight-wrapper {{
  position: relative;
}}
"#,
            bg = self.background,
            fg = self.foreground,
            kw = self.colors.keyword,
            str = self.colors.string,
            cmt = self.colors.comment,
            num = self.colors.number,
            op = self.colors.operator,
            punc = self.colors.punctuation,
            fn_ = self.colors.function,
            type_ = self.colors.r#type,
            var = self.colors.variable,
            attr = self.colors.attribute,
            mac = self.colors.r#macro,
            const_ = self.colors.constant,
            bi = self.colors.builtin,
            ln = self.line_number,
            ln_bg = self.line_number_bg,
        )
    }
}

/// Built-in themes.
pub mod themes {
    use super::*;

    /// GitHub light theme.
    pub fn github() -> Theme {
        Theme {
            name: "github".to_string(),
            background: "#f6f8fa".to_string(),
            foreground: "#24292e".to_string(),
            line_number: "#6a737d".to_string(),
            line_number_bg: "#f0f3f6".to_string(),
            selection: "#c8c8fa".to_string(),
            colors: ThemeColors {
                keyword: "#d73a49".to_string(),
                string: "#032f62".to_string(),
                comment: "#6a737d".to_string(),
                number: "#005cc5".to_string(),
                operator: "#d73a49".to_string(),
                punctuation: "#24292e".to_string(),
                function: "#6f42c1".to_string(),
                r#type: "#005cc5".to_string(),
                variable: "#e36209".to_string(),
                attribute: "#22863a".to_string(),
                r#macro: "#6f42c1".to_string(),
                constant: "#005cc5".to_string(),
                builtin: "#005cc5".to_string(),
            },
        }
    }

    /// GitHub dark theme.
    pub fn github_dark() -> Theme {
        Theme {
            name: "github-dark".to_string(),
            background: "#0d1117".to_string(),
            foreground: "#c9d1d9".to_string(),
            line_number: "#484f58".to_string(),
            line_number_bg: "#161b22".to_string(),
            selection: "#388bfd66".to_string(),
            colors: ThemeColors {
                keyword: "#ff7b72".to_string(),
                string: "#a5d6ff".to_string(),
                comment: "#8b949e".to_string(),
                number: "#79c0ff".to_string(),
                operator: "#ff7b72".to_string(),
                punctuation: "#c9d1d9".to_string(),
                function: "#d2a8ff".to_string(),
                r#type: "#79c0ff".to_string(),
                variable: "#ffa657".to_string(),
                attribute: "#7ee787".to_string(),
                r#macro: "#d2a8ff".to_string(),
                constant: "#79c0ff".to_string(),
                builtin: "#79c0ff".to_string(),
            },
        }
    }

    /// Monokai theme.
    pub fn monokai() -> Theme {
        Theme {
            name: "monokai".to_string(),
            background: "#272822".to_string(),
            foreground: "#f8f8f2".to_string(),
            line_number: "#90908a".to_string(),
            line_number_bg: "#2f302a".to_string(),
            selection: "#49483e".to_string(),
            colors: ThemeColors {
                keyword: "#f92672".to_string(),
                string: "#e6db74".to_string(),
                comment: "#75715e".to_string(),
                number: "#ae81ff".to_string(),
                operator: "#f92672".to_string(),
                punctuation: "#f8f8f2".to_string(),
                function: "#a6e22e".to_string(),
                r#type: "#66d9ef".to_string(),
                variable: "#fd971f".to_string(),
                attribute: "#a6e22e".to_string(),
                r#macro: "#a6e22e".to_string(),
                constant: "#ae81ff".to_string(),
                builtin: "#66d9ef".to_string(),
            },
        }
    }

    /// Dracula theme.
    pub fn dracula() -> Theme {
        Theme {
            name: "dracula".to_string(),
            background: "#282a36".to_string(),
            foreground: "#f8f8f2".to_string(),
            line_number: "#6272a4".to_string(),
            line_number_bg: "#21222c".to_string(),
            selection: "#44475a".to_string(),
            colors: ThemeColors {
                keyword: "#ff79c6".to_string(),
                string: "#f1fa8c".to_string(),
                comment: "#6272a4".to_string(),
                number: "#bd93f9".to_string(),
                operator: "#ff79c6".to_string(),
                punctuation: "#f8f8f2".to_string(),
                function: "#50fa7b".to_string(),
                r#type: "#8be9fd".to_string(),
                variable: "#ffb86c".to_string(),
                attribute: "#50fa7b".to_string(),
                r#macro: "#50fa7b".to_string(),
                constant: "#bd93f9".to_string(),
                builtin: "#8be9fd".to_string(),
            },
        }
    }

    /// One Dark theme.
    pub fn one_dark() -> Theme {
        Theme {
            name: "one-dark".to_string(),
            background: "#282c34".to_string(),
            foreground: "#abb2bf".to_string(),
            line_number: "#4b5263".to_string(),
            line_number_bg: "#21252b".to_string(),
            selection: "#3e4451".to_string(),
            colors: ThemeColors {
                keyword: "#c678dd".to_string(),
                string: "#98c379".to_string(),
                comment: "#5c6370".to_string(),
                number: "#d19a66".to_string(),
                operator: "#56b6c2".to_string(),
                punctuation: "#abb2bf".to_string(),
                function: "#61afef".to_string(),
                r#type: "#e5c07b".to_string(),
                variable: "#e06c75".to_string(),
                attribute: "#d19a66".to_string(),
                r#macro: "#e06c75".to_string(),
                constant: "#d19a66".to_string(),
                builtin: "#56b6c2".to_string(),
            },
        }
    }

    /// Solarized light theme.
    pub fn solarized_light() -> Theme {
        Theme {
            name: "solarized-light".to_string(),
            background: "#fdf6e3".to_string(),
            foreground: "#657b83".to_string(),
            line_number: "#93a1a1".to_string(),
            line_number_bg: "#eee8d5".to_string(),
            selection: "#eee8d5".to_string(),
            colors: ThemeColors {
                keyword: "#859900".to_string(),
                string: "#2aa198".to_string(),
                comment: "#93a1a1".to_string(),
                number: "#d33682".to_string(),
                operator: "#657b83".to_string(),
                punctuation: "#657b83".to_string(),
                function: "#268bd2".to_string(),
                r#type: "#b58900".to_string(),
                variable: "#cb4b16".to_string(),
                attribute: "#859900".to_string(),
                r#macro: "#cb4b16".to_string(),
                constant: "#d33682".to_string(),
                builtin: "#268bd2".to_string(),
            },
        }
    }

    /// Solarized dark theme.
    pub fn solarized_dark() -> Theme {
        Theme {
            name: "solarized-dark".to_string(),
            background: "#002b36".to_string(),
            foreground: "#839496".to_string(),
            line_number: "#586e75".to_string(),
            line_number_bg: "#073642".to_string(),
            selection: "#073642".to_string(),
            colors: ThemeColors {
                keyword: "#859900".to_string(),
                string: "#2aa198".to_string(),
                comment: "#586e75".to_string(),
                number: "#d33682".to_string(),
                operator: "#839496".to_string(),
                punctuation: "#839496".to_string(),
                function: "#268bd2".to_string(),
                r#type: "#b58900".to_string(),
                variable: "#cb4b16".to_string(),
                attribute: "#859900".to_string(),
                r#macro: "#cb4b16".to_string(),
                constant: "#d33682".to_string(),
                builtin: "#268bd2".to_string(),
            },
        }
    }

    /// Nord theme.
    pub fn nord() -> Theme {
        Theme {
            name: "nord".to_string(),
            background: "#2e3440".to_string(),
            foreground: "#d8dee9".to_string(),
            line_number: "#4c566a".to_string(),
            line_number_bg: "#3b4252".to_string(),
            selection: "#434c5e".to_string(),
            colors: ThemeColors {
                keyword: "#81a1c1".to_string(),
                string: "#a3be8c".to_string(),
                comment: "#616e88".to_string(),
                number: "#b48ead".to_string(),
                operator: "#81a1c1".to_string(),
                punctuation: "#eceff4".to_string(),
                function: "#88c0d0".to_string(),
                r#type: "#8fbcbb".to_string(),
                variable: "#d8dee9".to_string(),
                attribute: "#8fbcbb".to_string(),
                r#macro: "#bf616a".to_string(),
                constant: "#b48ead".to_string(),
                builtin: "#81a1c1".to_string(),
            },
        }
    }

    /// Get theme by name.
    pub fn by_name(name: &str) -> Option<Theme> {
        match name.to_lowercase().as_str() {
            "github" | "github-light" => Some(github()),
            "github-dark" => Some(github_dark()),
            "monokai" => Some(monokai()),
            "dracula" => Some(dracula()),
            "one-dark" | "onedark" => Some(one_dark()),
            "solarized-light" | "solarized" => Some(solarized_light()),
            "solarized-dark" => Some(solarized_dark()),
            "nord" => Some(nord()),
            _ => None,
        }
    }

    /// List all available theme names.
    pub fn available() -> Vec<&'static str> {
        vec![
            "github",
            "github-dark",
            "monokai",
            "dracula",
            "one-dark",
            "solarized-light",
            "solarized-dark",
            "nord",
        ]
    }
}

/// Language definition for syntax highlighting.
#[derive(Debug)]
pub struct Language {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub keywords: &'static [&'static str],
    pub types: &'static [&'static str],
    pub builtins: &'static [&'static str],
    pub constants: &'static [&'static str],
    pub string_delimiters: &'static [char],
    pub single_line_comment: Option<&'static str>,
    pub multi_line_comment: Option<(&'static str, &'static str)>,
    pub has_attributes: bool,
    pub has_macros: bool,
}

/// Built-in language definitions.
pub mod languages {
    use super::*;

    pub const RUST: Language = Language {
        name: "rust",
        aliases: &["rs"],
        keywords: &[
            "as", "async", "await", "break", "const", "continue", "crate", "dyn",
            "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in",
            "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
            "self", "Self", "static", "struct", "super", "trait", "true", "type",
            "unsafe", "use", "where", "while",
        ],
        types: &[
            "bool", "char", "str", "u8", "u16", "u32", "u64", "u128", "usize",
            "i8", "i16", "i32", "i64", "i128", "isize", "f32", "f64",
            "String", "Vec", "Option", "Result", "Box", "Rc", "Arc", "Cell",
            "RefCell", "HashMap", "HashSet", "BTreeMap", "BTreeSet",
        ],
        builtins: &[
            "println", "print", "eprintln", "eprint", "format", "panic", "assert",
            "assert_eq", "assert_ne", "debug_assert", "debug_assert_eq",
            "todo", "unimplemented", "unreachable",
        ],
        constants: &["None", "Some", "Ok", "Err", "true", "false"],
        string_delimiters: &['"'],
        single_line_comment: Some("//"),
        multi_line_comment: Some(("/*", "*/")),
        has_attributes: true,
        has_macros: true,
    };

    pub const JAVASCRIPT: Language = Language {
        name: "javascript",
        aliases: &["js", "jsx", "ts", "typescript", "tsx"],
        keywords: &[
            "async", "await", "break", "case", "catch", "class", "const",
            "continue", "debugger", "default", "delete", "do", "else", "export",
            "extends", "finally", "for", "function", "if", "import", "in",
            "instanceof", "let", "new", "return", "static", "super", "switch",
            "this", "throw", "try", "typeof", "var", "void", "while", "with",
            "yield", "of", "from", "as",
        ],
        types: &[
            "any", "boolean", "number", "string", "void", "never", "unknown",
            "object", "Array", "Object", "String", "Number", "Boolean", "Function",
            "Promise", "Map", "Set", "WeakMap", "WeakSet", "Symbol", "BigInt",
        ],
        builtins: &[
            "console", "window", "document", "fetch", "setTimeout", "setInterval",
            "clearTimeout", "clearInterval", "JSON", "Math", "Date", "RegExp",
            "parseInt", "parseFloat", "isNaN", "isFinite", "encodeURI", "decodeURI",
        ],
        constants: &["true", "false", "null", "undefined", "NaN", "Infinity"],
        string_delimiters: &['"', '\'', '`'],
        single_line_comment: Some("//"),
        multi_line_comment: Some(("/*", "*/")),
        has_attributes: false,
        has_macros: false,
    };

    pub const PYTHON: Language = Language {
        name: "python",
        aliases: &["py", "python3"],
        keywords: &[
            "and", "as", "assert", "async", "await", "break", "class", "continue",
            "def", "del", "elif", "else", "except", "finally", "for", "from",
            "global", "if", "import", "in", "is", "lambda", "nonlocal", "not",
            "or", "pass", "raise", "return", "try", "while", "with", "yield",
        ],
        types: &[
            "int", "float", "str", "bool", "list", "dict", "set", "tuple",
            "bytes", "bytearray", "memoryview", "range", "frozenset", "type",
            "object", "complex",
        ],
        builtins: &[
            "print", "len", "range", "open", "input", "type", "isinstance",
            "hasattr", "getattr", "setattr", "delattr", "callable", "iter",
            "next", "enumerate", "zip", "map", "filter", "sorted", "reversed",
            "min", "max", "sum", "abs", "round", "pow", "divmod", "hex", "oct",
            "bin", "chr", "ord", "repr", "format", "vars", "dir", "help",
        ],
        constants: &["True", "False", "None"],
        string_delimiters: &['"', '\''],
        single_line_comment: Some("#"),
        multi_line_comment: None,
        has_attributes: false,
        has_macros: false,
    };

    pub const GO: Language = Language {
        name: "go",
        aliases: &["golang"],
        keywords: &[
            "break", "case", "chan", "const", "continue", "default", "defer",
            "else", "fallthrough", "for", "func", "go", "goto", "if", "import",
            "interface", "map", "package", "range", "return", "select", "struct",
            "switch", "type", "var",
        ],
        types: &[
            "bool", "byte", "complex64", "complex128", "error", "float32",
            "float64", "int", "int8", "int16", "int32", "int64", "rune",
            "string", "uint", "uint8", "uint16", "uint32", "uint64", "uintptr",
        ],
        builtins: &[
            "append", "cap", "close", "complex", "copy", "delete", "imag",
            "len", "make", "new", "panic", "print", "println", "real", "recover",
        ],
        constants: &["true", "false", "nil", "iota"],
        string_delimiters: &['"', '`'],
        single_line_comment: Some("//"),
        multi_line_comment: Some(("/*", "*/")),
        has_attributes: false,
        has_macros: false,
    };

    pub const BASH: Language = Language {
        name: "bash",
        aliases: &["sh", "shell", "zsh"],
        keywords: &[
            "if", "then", "else", "elif", "fi", "case", "esac", "for", "while",
            "until", "do", "done", "in", "function", "select", "time", "coproc",
            "local", "readonly", "export", "declare", "typeset", "return",
            "break", "continue", "exit", "trap", "source",
        ],
        types: &[],
        builtins: &[
            "echo", "printf", "read", "cd", "pwd", "pushd", "popd", "dirs",
            "set", "unset", "shift", "test", "eval", "exec", "alias", "unalias",
            "type", "which", "command", "builtin", "enable", "help", "hash",
            "true", "false", ":", ".", "[", "[[",
        ],
        constants: &["true", "false"],
        string_delimiters: &['"', '\''],
        single_line_comment: Some("#"),
        multi_line_comment: None,
        has_attributes: false,
        has_macros: false,
    };

    pub const SQL: Language = Language {
        name: "sql",
        aliases: &["mysql", "postgresql", "sqlite"],
        keywords: &[
            "SELECT", "FROM", "WHERE", "AND", "OR", "NOT", "IN", "LIKE", "BETWEEN",
            "ORDER", "BY", "ASC", "DESC", "LIMIT", "OFFSET", "GROUP", "HAVING",
            "JOIN", "LEFT", "RIGHT", "INNER", "OUTER", "FULL", "CROSS", "ON",
            "AS", "DISTINCT", "ALL", "INSERT", "INTO", "VALUES", "UPDATE", "SET",
            "DELETE", "CREATE", "TABLE", "INDEX", "VIEW", "DROP", "ALTER", "ADD",
            "PRIMARY", "KEY", "FOREIGN", "REFERENCES", "UNIQUE", "DEFAULT",
            "NULL", "CONSTRAINT", "CHECK", "CASE", "WHEN", "THEN", "ELSE", "END",
            "UNION", "EXCEPT", "INTERSECT", "EXISTS", "COALESCE", "CAST",
        ],
        types: &[
            "INT", "INTEGER", "BIGINT", "SMALLINT", "TINYINT", "DECIMAL", "NUMERIC",
            "FLOAT", "REAL", "DOUBLE", "VARCHAR", "CHAR", "TEXT", "BLOB", "DATE",
            "TIME", "DATETIME", "TIMESTAMP", "BOOLEAN", "BOOL", "JSON", "UUID",
        ],
        builtins: &[
            "COUNT", "SUM", "AVG", "MIN", "MAX", "ABS", "ROUND", "FLOOR", "CEIL",
            "CONCAT", "SUBSTRING", "LENGTH", "UPPER", "LOWER", "TRIM", "LTRIM",
            "RTRIM", "REPLACE", "NOW", "CURDATE", "CURTIME", "DATE_FORMAT",
        ],
        constants: &["TRUE", "FALSE", "NULL"],
        string_delimiters: &['\''],
        single_line_comment: Some("--"),
        multi_line_comment: Some(("/*", "*/")),
        has_attributes: false,
        has_macros: false,
    };

    pub const JSON: Language = Language {
        name: "json",
        aliases: &["json5"],
        keywords: &[],
        types: &[],
        builtins: &[],
        constants: &["true", "false", "null"],
        string_delimiters: &['"'],
        single_line_comment: None,
        multi_line_comment: None,
        has_attributes: false,
        has_macros: false,
    };

    pub const YAML: Language = Language {
        name: "yaml",
        aliases: &["yml"],
        keywords: &[],
        types: &[],
        builtins: &[],
        constants: &["true", "false", "null", "~", "yes", "no", "on", "off"],
        string_delimiters: &['"', '\''],
        single_line_comment: Some("#"),
        multi_line_comment: None,
        has_attributes: false,
        has_macros: false,
    };

    pub const TOML: Language = Language {
        name: "toml",
        aliases: &[],
        keywords: &[],
        types: &[],
        builtins: &[],
        constants: &["true", "false"],
        string_delimiters: &['"', '\''],
        single_line_comment: Some("#"),
        multi_line_comment: None,
        has_attributes: false,
        has_macros: false,
    };

    pub const HTML: Language = Language {
        name: "html",
        aliases: &["htm", "xml", "svg"],
        keywords: &[],
        types: &[],
        builtins: &[],
        constants: &[],
        string_delimiters: &['"', '\''],
        single_line_comment: None,
        multi_line_comment: Some(("<!--", "-->")),
        has_attributes: true,
        has_macros: false,
    };

    pub const CSS: Language = Language {
        name: "css",
        aliases: &["scss", "sass", "less"],
        keywords: &[
            "@import", "@media", "@keyframes", "@font-face", "@supports",
            "@page", "@namespace", "@charset", "@layer", "@property",
        ],
        types: &[],
        builtins: &[
            "rgb", "rgba", "hsl", "hsla", "url", "calc", "var", "min", "max",
            "clamp", "attr", "counter", "linear-gradient", "radial-gradient",
            "conic-gradient", "translate", "rotate", "scale", "skew",
        ],
        constants: &[
            "inherit", "initial", "unset", "revert", "none", "auto", "transparent",
        ],
        string_delimiters: &['"', '\''],
        single_line_comment: None,
        multi_line_comment: Some(("/*", "*/")),
        has_attributes: false,
        has_macros: false,
    };

    pub const MARKDOWN: Language = Language {
        name: "markdown",
        aliases: &["md"],
        keywords: &[],
        types: &[],
        builtins: &[],
        constants: &[],
        string_delimiters: &[],
        single_line_comment: None,
        multi_line_comment: None,
        has_attributes: false,
        has_macros: false,
    };

    pub const C: Language = Language {
        name: "c",
        aliases: &["h"],
        keywords: &[
            "auto", "break", "case", "char", "const", "continue", "default",
            "do", "double", "else", "enum", "extern", "float", "for", "goto",
            "if", "inline", "int", "long", "register", "restrict", "return",
            "short", "signed", "sizeof", "static", "struct", "switch", "typedef",
            "union", "unsigned", "void", "volatile", "while", "_Alignas",
            "_Alignof", "_Atomic", "_Bool", "_Complex", "_Generic", "_Imaginary",
            "_Noreturn", "_Static_assert", "_Thread_local",
        ],
        types: &[
            "int", "char", "float", "double", "void", "long", "short", "signed",
            "unsigned", "size_t", "ptrdiff_t", "intptr_t", "uintptr_t",
            "int8_t", "int16_t", "int32_t", "int64_t",
            "uint8_t", "uint16_t", "uint32_t", "uint64_t",
            "bool", "FILE", "NULL",
        ],
        builtins: &[
            "printf", "scanf", "malloc", "free", "realloc", "calloc",
            "memcpy", "memmove", "memset", "memcmp", "strlen", "strcpy",
            "strcat", "strcmp", "strncpy", "strncat", "strncmp",
            "fopen", "fclose", "fread", "fwrite", "fprintf", "fscanf",
            "exit", "abort", "atexit", "system",
        ],
        constants: &["NULL", "true", "false", "EOF"],
        string_delimiters: &['"'],
        single_line_comment: Some("//"),
        multi_line_comment: Some(("/*", "*/")),
        has_attributes: false,
        has_macros: true,
    };

    pub const CPP: Language = Language {
        name: "cpp",
        aliases: &["c++", "cc", "cxx", "hpp", "hxx"],
        keywords: &[
            "alignas", "alignof", "and", "and_eq", "asm", "auto", "bitand",
            "bitor", "bool", "break", "case", "catch", "char", "char8_t",
            "char16_t", "char32_t", "class", "compl", "concept", "const",
            "consteval", "constexpr", "constinit", "const_cast", "continue",
            "co_await", "co_return", "co_yield", "decltype", "default", "delete",
            "do", "double", "dynamic_cast", "else", "enum", "explicit", "export",
            "extern", "false", "float", "for", "friend", "goto", "if", "inline",
            "int", "long", "mutable", "namespace", "new", "noexcept", "not",
            "not_eq", "nullptr", "operator", "or", "or_eq", "private", "protected",
            "public", "register", "reinterpret_cast", "requires", "return",
            "short", "signed", "sizeof", "static", "static_assert", "static_cast",
            "struct", "switch", "template", "this", "thread_local", "throw",
            "true", "try", "typedef", "typeid", "typename", "union", "unsigned",
            "using", "virtual", "void", "volatile", "wchar_t", "while", "xor",
            "xor_eq", "override", "final",
        ],
        types: &[
            "int", "char", "float", "double", "void", "long", "short", "signed",
            "unsigned", "bool", "wchar_t", "char8_t", "char16_t", "char32_t",
            "size_t", "ptrdiff_t", "nullptr_t", "string", "vector", "map", "set",
            "unordered_map", "unordered_set", "array", "deque", "list", "queue",
            "stack", "pair", "tuple", "optional", "variant", "any", "span",
            "unique_ptr", "shared_ptr", "weak_ptr",
        ],
        builtins: &[
            "std", "cout", "cin", "cerr", "endl", "make_unique", "make_shared",
            "move", "forward", "swap", "begin", "end", "size", "empty",
        ],
        constants: &["nullptr", "true", "false", "NULL"],
        string_delimiters: &['"'],
        single_line_comment: Some("//"),
        multi_line_comment: Some(("/*", "*/")),
        has_attributes: true,
        has_macros: true,
    };

    /// Get language by name or alias.
    pub fn by_name(name: &str) -> Option<&'static Language> {
        let name_lower = name.to_lowercase();
        let all = [
            &RUST, &JAVASCRIPT, &PYTHON, &GO, &BASH, &SQL, &JSON, &YAML, &TOML,
            &HTML, &CSS, &MARKDOWN, &C, &CPP,
        ];

        for lang in all {
            if lang.name == name_lower {
                return Some(lang);
            }
            for alias in lang.aliases {
                if *alias == name_lower {
                    return Some(lang);
                }
            }
        }
        None
    }

    /// List all supported language names.
    pub fn supported() -> Vec<&'static str> {
        vec![
            "rust", "javascript", "python", "go", "bash", "sql", "json",
            "yaml", "toml", "html", "css", "markdown", "c", "cpp",
        ]
    }
}

/// Syntax highlighter.
pub struct Highlighter {
    theme: Theme,
    show_line_numbers: bool,
    show_copy_button: bool,
}

impl Highlighter {
    /// Create a new highlighter with default theme.
    pub fn new() -> Self {
        Self {
            theme: themes::github_dark(),
            show_line_numbers: true,
            show_copy_button: true,
        }
    }

    /// Set the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Set whether to show line numbers.
    pub fn with_line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    /// Set whether to show copy button.
    pub fn with_copy_button(mut self, show: bool) -> Self {
        self.show_copy_button = show;
        self
    }

    /// Get the theme.
    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    /// Highlight code for a given language.
    pub fn highlight(&self, code: &str, language: &str) -> String {
        let tokens = tokenize(code, language);
        self.render_tokens(&tokens, code)
    }

    /// Render tokens to HTML.
    fn render_tokens(&self, tokens: &[Token], original: &str) -> String {
        let mut html = String::new();

        // Wrapper for positioning.
        html.push_str("<div class=\"highlight-wrapper\">\n");

        if self.show_copy_button {
            html.push_str("<button class=\"copy-btn\" onclick=\"navigator.clipboard.writeText(this.parentElement.querySelector('code').textContent)\">Copy</button>\n");
        }

        html.push_str("<div class=\"highlight\">\n<pre><code>");

        let lines: Vec<&str> = original.lines().collect();
        let line_count = lines.len();

        if self.show_line_numbers && line_count > 1 {
            // Build line with numbers.
            for (i, _line) in lines.iter().enumerate() {
                if i > 0 {
                    html.push('\n');
                }
                html.push_str(&format!("<span class=\"line-numbers\">{:>width$}</span>",
                    i + 1, width = line_count.to_string().len()));
            }
            html.push_str("\n</code></pre>\n<pre><code>");
        }

        // Render highlighted tokens.
        for token in tokens {
            let escaped = html_escape(&token.text);
            if token.token_type == TokenType::Plain {
                html.push_str(&escaped);
            } else {
                html.push_str(&format!(
                    "<span class=\"{}\">{}</span>",
                    token.token_type.css_class(),
                    escaped
                ));
            }
        }

        html.push_str("</code></pre>\n</div>\n</div>");
        html
    }

    /// Generate CSS for the current theme.
    pub fn generate_css(&self) -> String {
        self.theme.generate_css()
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}

/// Tokenize code for a given language.
pub fn tokenize(code: &str, language: &str) -> Vec<Token> {
    let lang = match languages::by_name(language) {
        Some(l) => l,
        None => return vec![Token { text: code.to_string(), token_type: TokenType::Plain }],
    };

    let mut tokens = Vec::new();
    let chars: Vec<char> = code.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        // Check for multi-line comment.
        if let Some((start, end)) = lang.multi_line_comment {
            if starts_with_at(&chars, i, start) {
                let start_idx = i;
                i += start.len();
                while i < chars.len() && !starts_with_at(&chars, i, end) {
                    i += 1;
                }
                if starts_with_at(&chars, i, end) {
                    i += end.len();
                }
                let text: String = chars[start_idx..i].iter().collect();
                tokens.push(Token { text, token_type: TokenType::Comment });
                continue;
            }
        }

        // Check for single-line comment.
        if let Some(comment) = lang.single_line_comment {
            if starts_with_at(&chars, i, comment) {
                let start_idx = i;
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
                let text: String = chars[start_idx..i].iter().collect();
                tokens.push(Token { text, token_type: TokenType::Comment });
                continue;
            }
        }

        // Check for string.
        if lang.string_delimiters.contains(&chars[i]) {
            let delim = chars[i];
            let start_idx = i;
            i += 1;
            while i < chars.len() {
                if chars[i] == '\\' && i + 1 < chars.len() {
                    i += 2; // Skip escaped char.
                } else if chars[i] == delim {
                    i += 1;
                    break;
                } else if chars[i] == '\n' && delim != '`' {
                    break; // Unterminated string.
                } else {
                    i += 1;
                }
            }
            let text: String = chars[start_idx..i].iter().collect();
            tokens.push(Token { text, token_type: TokenType::String });
            continue;
        }

        // Check for attribute (Rust #[...]).
        if lang.has_attributes && chars[i] == '#' && i + 1 < chars.len() && chars[i + 1] == '[' {
            let start_idx = i;
            let mut depth = 0;
            while i < chars.len() {
                if chars[i] == '[' {
                    depth += 1;
                } else if chars[i] == ']' {
                    depth -= 1;
                    if depth == 0 {
                        i += 1;
                        break;
                    }
                }
                i += 1;
            }
            let text: String = chars[start_idx..i].iter().collect();
            tokens.push(Token { text, token_type: TokenType::Attribute });
            continue;
        }

        // Check for macro (Rust word!).
        if lang.has_macros && chars[i].is_alphabetic() {
            let start_idx = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            if i < chars.len() && chars[i] == '!' {
                i += 1;
                let text: String = chars[start_idx..i].iter().collect();
                tokens.push(Token { text, token_type: TokenType::Macro });
                continue;
            }
            // Reset - not a macro.
            i = start_idx;
        }

        // Check for number.
        if chars[i].is_ascii_digit() || (chars[i] == '.' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit()) {
            let start_idx = i;
            // Handle hex, octal, binary.
            if chars[i] == '0' && i + 1 < chars.len() {
                match chars[i + 1] {
                    'x' | 'X' => {
                        i += 2;
                        while i < chars.len() && chars[i].is_ascii_hexdigit() {
                            i += 1;
                        }
                    }
                    'b' | 'B' => {
                        i += 2;
                        while i < chars.len() && (chars[i] == '0' || chars[i] == '1') {
                            i += 1;
                        }
                    }
                    'o' | 'O' => {
                        i += 2;
                        while i < chars.len() && chars[i] >= '0' && chars[i] <= '7' {
                            i += 1;
                        }
                    }
                    _ => {
                        // Regular number.
                        while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.' || chars[i] == '_' || chars[i] == 'e' || chars[i] == 'E') {
                            i += 1;
                        }
                    }
                }
            } else {
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.' || chars[i] == '_' || chars[i] == 'e' || chars[i] == 'E') {
                    i += 1;
                }
            }
            // Handle type suffix (u32, f64, etc.).
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let text: String = chars[start_idx..i].iter().collect();
            tokens.push(Token { text, token_type: TokenType::Number });
            continue;
        }

        // Check for identifier/keyword.
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let start_idx = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let text: String = chars[start_idx..i].iter().collect();

            let token_type = if lang.keywords.contains(&text.as_str()) {
                TokenType::Keyword
            } else if lang.types.contains(&text.as_str()) {
                TokenType::Type
            } else if lang.builtins.contains(&text.as_str()) {
                TokenType::Builtin
            } else if lang.constants.contains(&text.as_str()) {
                TokenType::Constant
            } else if i < chars.len() && chars[i] == '(' {
                TokenType::Function
            } else if text.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                // PascalCase likely a type.
                TokenType::Type
            } else {
                TokenType::Plain
            };

            tokens.push(Token { text, token_type });
            continue;
        }

        // Check for operators.
        static OPERATORS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
            vec![
                "===", "!==", "..=", "...", "=>", "->", "::", "<<", ">>", "<=", ">=",
                "==", "!=", "&&", "||", "+=", "-=", "*=", "/=", "%=", "&=", "|=",
                "^=", "..", "+", "-", "*", "/", "%", "&", "|", "^", "!", "~",
                "<", ">", "=", "?", ":",
            ]
        });

        let mut found_op = false;
        for op in OPERATORS.iter() {
            if starts_with_at(&chars, i, op) {
                tokens.push(Token {
                    text: op.to_string(),
                    token_type: TokenType::Operator,
                });
                i += op.len();
                found_op = true;
                break;
            }
        }
        if found_op {
            continue;
        }

        // Check for punctuation.
        if "(){}[];,.".contains(chars[i]) {
            tokens.push(Token {
                text: chars[i].to_string(),
                token_type: TokenType::Punctuation,
            });
            i += 1;
            continue;
        }

        // Plain text (whitespace, unknown).
        tokens.push(Token {
            text: chars[i].to_string(),
            token_type: TokenType::Plain,
        });
        i += 1;
    }

    // Merge adjacent plain tokens.
    merge_plain_tokens(tokens)
}

/// Check if chars starting at index match the pattern.
fn starts_with_at(chars: &[char], i: usize, pattern: &str) -> bool {
    let pattern_chars: Vec<char> = pattern.chars().collect();
    if i + pattern_chars.len() > chars.len() {
        return false;
    }
    for (j, pc) in pattern_chars.iter().enumerate() {
        if chars[i + j] != *pc {
            return false;
        }
    }
    true
}

/// Merge adjacent plain tokens for efficiency.
fn merge_plain_tokens(tokens: Vec<Token>) -> Vec<Token> {
    let mut result = Vec::new();
    let mut current_plain = String::new();

    for token in tokens {
        if token.token_type == TokenType::Plain {
            current_plain.push_str(&token.text);
        } else {
            if !current_plain.is_empty() {
                result.push(Token {
                    text: std::mem::take(&mut current_plain),
                    token_type: TokenType::Plain,
                });
            }
            result.push(token);
        }
    }

    if !current_plain.is_empty() {
        result.push(Token {
            text: current_plain,
            token_type: TokenType::Plain,
        });
    }

    result
}

/// HTML-escape a string.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Highlight options for configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightOptions {
    pub theme: String,
    pub line_numbers: bool,
    pub copy_button: bool,
}

impl Default for HighlightOptions {
    fn default() -> Self {
        Self {
            theme: "github-dark".to_string(),
            line_numbers: true,
            copy_button: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_rust_basic() {
        let code = "fn main() { println!(\"hello\"); }";
        let tokens = tokenize(code, "rust");

        let kw_count = tokens.iter().filter(|t| t.token_type == TokenType::Keyword).count();
        assert!(kw_count >= 1, "Should have at least one keyword");

        let str_count = tokens.iter().filter(|t| t.token_type == TokenType::String).count();
        assert_eq!(str_count, 1, "Should have one string");

        let mac_count = tokens.iter().filter(|t| t.token_type == TokenType::Macro).count();
        assert_eq!(mac_count, 1, "Should have one macro");
    }

    #[test]
    fn test_tokenize_rust_comment() {
        let code = "// this is a comment\nfn test() {}";
        let tokens = tokenize(code, "rust");

        let cmt_count = tokens.iter().filter(|t| t.token_type == TokenType::Comment).count();
        assert_eq!(cmt_count, 1, "Should have one comment");
    }

    #[test]
    fn test_tokenize_rust_numbers() {
        let code = "let x = 42; let y = 3.14; let z = 0xFF;";
        let tokens = tokenize(code, "rust");

        let num_count = tokens.iter().filter(|t| t.token_type == TokenType::Number).count();
        assert_eq!(num_count, 3, "Should have three numbers");
    }

    #[test]
    fn test_tokenize_rust_attribute() {
        let code = "#[derive(Debug)]\nstruct Foo;";
        let tokens = tokenize(code, "rust");

        let attr_count = tokens.iter().filter(|t| t.token_type == TokenType::Attribute).count();
        assert_eq!(attr_count, 1, "Should have one attribute");
    }

    #[test]
    fn test_tokenize_javascript() {
        let code = "const x = () => { console.log('hello'); };";
        let tokens = tokenize(code, "javascript");

        let kw_count = tokens.iter().filter(|t| t.token_type == TokenType::Keyword).count();
        assert!(kw_count >= 1);

        let str_count = tokens.iter().filter(|t| t.token_type == TokenType::String).count();
        assert_eq!(str_count, 1);
    }

    #[test]
    fn test_tokenize_python() {
        let code = "def hello():\n    print('world')";
        let tokens = tokenize(code, "python");

        let kw_count = tokens.iter().filter(|t| t.token_type == TokenType::Keyword).count();
        assert!(kw_count >= 1);
    }

    #[test]
    fn test_tokenize_unknown_language() {
        let code = "some random code";
        let tokens = tokenize(code, "unknown-lang");

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Plain);
    }

    #[test]
    fn test_theme_github() {
        let theme = themes::github();
        assert_eq!(theme.name, "github");
        assert!(!theme.background.is_empty());
    }

    #[test]
    fn test_theme_monokai() {
        let theme = themes::monokai();
        assert_eq!(theme.name, "monokai");
        assert!(theme.background.starts_with('#'));
    }

    #[test]
    fn test_theme_by_name() {
        assert!(themes::by_name("github").is_some());
        assert!(themes::by_name("monokai").is_some());
        assert!(themes::by_name("dracula").is_some());
        assert!(themes::by_name("nonexistent").is_none());
    }

    #[test]
    fn test_theme_generate_css() {
        let theme = themes::github_dark();
        let css = theme.generate_css();

        assert!(css.contains(".highlight"));
        assert!(css.contains(".hl-kw"));
        assert!(css.contains(".hl-str"));
        assert!(css.contains(theme.background.as_str()));
    }

    #[test]
    fn test_highlighter_basic() {
        let highlighter = Highlighter::new();
        let html = highlighter.highlight("fn main() {}", "rust");

        assert!(html.contains("highlight"));
        assert!(html.contains("hl-kw"));
    }

    #[test]
    fn test_highlighter_with_theme() {
        let highlighter = Highlighter::new()
            .with_theme(themes::monokai());

        assert_eq!(highlighter.theme().name, "monokai");
    }

    #[test]
    fn test_highlighter_line_numbers() {
        let code = "line 1\nline 2\nline 3";
        let highlighter = Highlighter::new().with_line_numbers(true);
        let html = highlighter.highlight(code, "text");

        assert!(html.contains("line-numbers"));
    }

    #[test]
    fn test_highlighter_copy_button() {
        let highlighter = Highlighter::new().with_copy_button(true);
        let html = highlighter.highlight("code", "rust");

        assert!(html.contains("copy-btn"));
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("<script>"), "&lt;script&gt;");
        assert_eq!(html_escape("a & b"), "a &amp; b");
        assert_eq!(html_escape("\"quoted\""), "&quot;quoted&quot;");
    }

    #[test]
    fn test_languages_by_name() {
        assert!(languages::by_name("rust").is_some());
        assert!(languages::by_name("rs").is_some()); // Alias
        assert!(languages::by_name("javascript").is_some());
        assert!(languages::by_name("js").is_some()); // Alias
        assert!(languages::by_name("python").is_some());
        assert!(languages::by_name("py").is_some()); // Alias
    }

    #[test]
    fn test_languages_supported() {
        let supported = languages::supported();
        assert!(supported.contains(&"rust"));
        assert!(supported.contains(&"javascript"));
        assert!(supported.contains(&"python"));
    }

    #[test]
    fn test_themes_available() {
        let available = themes::available();
        assert!(available.contains(&"github"));
        assert!(available.contains(&"monokai"));
        assert!(available.contains(&"dracula"));
        assert!(available.contains(&"nord"));
    }

    #[test]
    fn test_tokenize_multiline_comment() {
        let code = "/* multi\nline\ncomment */ fn test() {}";
        let tokens = tokenize(code, "rust");

        let cmt = tokens.iter().find(|t| t.token_type == TokenType::Comment).unwrap();
        assert!(cmt.text.contains("multi"));
        assert!(cmt.text.contains("line"));
    }

    #[test]
    fn test_tokenize_operators() {
        let code = "a + b - c * d / e == f != g && h || i";
        let tokens = tokenize(code, "rust");

        let op_count = tokens.iter().filter(|t| t.token_type == TokenType::Operator).count();
        assert!(op_count >= 8, "Should have multiple operators");
    }

    #[test]
    fn test_tokenize_types() {
        let code = "let x: String = Vec::new();";
        let tokens = tokenize(code, "rust");

        let type_tokens: Vec<_> = tokens.iter()
            .filter(|t| t.token_type == TokenType::Type)
            .collect();
        assert!(type_tokens.len() >= 2, "Should recognize String and Vec as types");
    }

    #[test]
    fn test_tokenize_go() {
        let code = "func main() { fmt.Println(\"hello\") }";
        let tokens = tokenize(code, "go");

        let kw_count = tokens.iter().filter(|t| t.token_type == TokenType::Keyword).count();
        assert!(kw_count >= 1);
    }

    #[test]
    fn test_tokenize_sql() {
        let code = "SELECT * FROM users WHERE id = 1;";
        let tokens = tokenize(code, "sql");

        let kw_count = tokens.iter().filter(|t| t.token_type == TokenType::Keyword).count();
        assert!(kw_count >= 3, "Should have SELECT, FROM, WHERE keywords");
    }

    #[test]
    fn test_tokenize_bash() {
        let code = "#!/bin/bash\nif [ -f file ]; then echo 'exists'; fi";
        let tokens = tokenize(code, "bash");

        let kw_count = tokens.iter().filter(|t| t.token_type == TokenType::Keyword).count();
        assert!(kw_count >= 2);
    }

    #[test]
    fn test_highlight_options_default() {
        let opts = HighlightOptions::default();
        assert_eq!(opts.theme, "github-dark");
        assert!(opts.line_numbers);
        assert!(opts.copy_button);
    }

    #[test]
    fn test_tokenize_escaped_string() {
        let code = r#"let s = "hello \"world\"";"#;
        let tokens = tokenize(code, "rust");

        let str_tokens: Vec<_> = tokens.iter()
            .filter(|t| t.token_type == TokenType::String)
            .collect();
        assert_eq!(str_tokens.len(), 1);
        assert!(str_tokens[0].text.contains("\\\""));
    }

    #[test]
    fn test_token_css_class() {
        assert_eq!(TokenType::Keyword.css_class(), "hl-kw");
        assert_eq!(TokenType::String.css_class(), "hl-str");
        assert_eq!(TokenType::Comment.css_class(), "hl-cmt");
        assert_eq!(TokenType::Function.css_class(), "hl-fn");
    }
}
