Quasi-quoting for generating Rust source code as tokens.

- Crate [`::quote`].
- [docs.rs](https://docs.rs/quote)
- [crates.io](https://crates.io/crates/quote)
- [GitHub](https://github.com/dtolnay/quote)

---

`quote` provides the [`quote!`] macro,
which turns Rust syntax into a [`proc_macro2::TokenStream`].
It is one of the three foundational crates for writing procedural macros,
alongside [`syn`] for parsing and [`proc_macro2`] for token representation.

The `quote!` macro supports interpolation of variables with `#var`
and repetition with `#( ... )*`, mirroring the patterns of `macro_rules!`.

## Example

Generating a token stream with interpolation:

```rust
use quote::quote;
use proc_macro2::Ident;

let name = Ident::new("Greeter", proc_macro2::Span::call_site());
let field_name = Ident::new("message", proc_macro2::Span::call_site());

let tokens = quote! {
    struct #name {
        #field_name: String,
    }
};

let code = tokens.to_string();
assert!(code.contains("struct Greeter"));
assert!(code.contains("message : String"));
```

[`quote!`]: crate::quote::quote
[`proc_macro2::TokenStream`]: crate::proc_macro2::TokenStream
[`syn`]: crate::syn
[`proc_macro2`]: crate::proc_macro2
