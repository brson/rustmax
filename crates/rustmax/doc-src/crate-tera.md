Text template engine based on Jinja2/Django.

- Crate [`::tera`].
- [docs.rs](https://docs.rs/tera)
- [crates.io](https://crates.io/crates/tera)
- [GitHub](https://github.com/Keats/tera)

---

`tera` is a template engine inspired by Jinja2 and the Django template language.
It compiles templates to an internal representation and renders them
with a [`Context`] holding the variables.
Templates support inheritance, includes, for loops, conditionals,
filters, and custom functions.

The main entry point is the [`Tera`] struct, which loads and caches templates.
For one-off rendering without managing a template collection,
[`Tera::one_off`] renders a template string directly.

## Examples

Rendering a template from a string:

```rust
use tera::{Tera, Context};

let mut tera = Tera::default();
tera.add_raw_template("greeting", "Hello, {{ name }}!").unwrap();

let mut ctx = Context::new();
ctx.insert("name", "Rust");

let output = tera.render("greeting", &ctx).unwrap();
assert_eq!(output, "Hello, Rust!");
```

Using loops and conditionals:

```rust
use tera::{Tera, Context};

let template = "{% for item in items %}{% if item.active %}{{ item.label }}, {% endif %}{% endfor %}";

let mut tera = Tera::default();
tera.add_raw_template("list", template).unwrap();

let mut ctx = Context::new();
ctx.insert("items", &vec![
    serde_json::json!({"label": "alpha", "active": true}),
    serde_json::json!({"label": "beta", "active": false}),
    serde_json::json!({"label": "gamma", "active": true}),
]);

let output = tera.render("list", &ctx).unwrap();
assert_eq!(output, "alpha, gamma, ");
```

Quick one-off rendering without creating a [`Tera`] instance:

```rust
use tera::{Tera, Context};

let mut ctx = Context::new();
ctx.insert("count", &3);

let output = Tera::one_off("There are {{ count }} lights.", &ctx, false).unwrap();
assert_eq!(output, "There are 3 lights.");
```

[`Tera`]: crate::tera::Tera
[`Tera::one_off`]: crate::tera::Tera::one_off
[`Context`]: crate::tera::Context
