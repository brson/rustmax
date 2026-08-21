//! Type signature formatting.
//!
//! Everything here renders to HTML: type names that resolve to a documented
//! item become links, and all other text is escaped. There is deliberately no
//! second plain-text renderer, so signatures cannot drift between the two.

use rustdoc_types::*;

use super::RenderContext;

/// Renders signatures as HTML, linking type references where possible.
pub struct LinkedRenderer<'a, 'ctx> {
    ctx: &'a RenderContext<'ctx>,
    current_depth: usize,
}

impl<'a, 'ctx> LinkedRenderer<'a, 'ctx> {
    /// Create a new linked renderer.
    pub fn new(ctx: &'a RenderContext<'ctx>, current_depth: usize) -> Self {
        Self { ctx, current_depth }
    }

    // --- Types ---

    /// Render a type with HTML links.
    pub fn render_type(&self, ty: &Type) -> String {
        match ty {
            Type::ResolvedPath(path) => self.render_resolved_path(path),
            Type::DynTrait(dyn_trait) => self.render_dyn_trait(dyn_trait),
            Type::Generic(name) => html_escape(name),
            Type::Primitive(name) => html_escape(name),
            Type::FunctionPointer(fp) => self.render_fn_pointer(fp),
            Type::Tuple(types) => {
                let inner: Vec<_> = types.iter().map(|t| self.render_type(t)).collect();
                // A one-tuple needs the trailing comma to stay a tuple.
                if inner.len() == 1 {
                    format!("({},)", inner[0])
                } else {
                    format!("({})", inner.join(", "))
                }
            }
            Type::Slice(inner) => format!("[{}]", self.render_type(inner)),
            Type::Array { type_, len } => {
                format!("[{}; {}]", self.render_type(type_), html_escape(len))
            }
            Type::Pat { type_, __pat_unstable_do_not_use: pat } => {
                format!("{} is {}", self.render_type(type_), html_escape(pat))
            }
            Type::ImplTrait(bounds) => {
                let bounds_str: Vec<_> = bounds.iter().map(|b| self.render_generic_bound(b)).collect();
                if bounds_str.is_empty() {
                    "impl Sized".to_string()
                } else {
                    format!("impl {}", bounds_str.join(" + "))
                }
            }
            Type::Infer => "_".to_string(),
            Type::RawPointer { is_mutable, type_ } => {
                let mutability = if *is_mutable { "mut" } else { "const" };
                format!("*{} {}", mutability, self.render_type(type_))
            }
            Type::BorrowedRef { lifetime, is_mutable, type_ } => {
                let mut result = "&amp;".to_string();
                if let Some(lt) = lifetime {
                    result.push_str(&html_escape(lt));
                    result.push(' ');
                }
                if *is_mutable {
                    result.push_str("mut ");
                }
                result.push_str(&self.render_type(type_));
                result
            }
            Type::QualifiedPath { name, args, self_type, trait_ } => {
                // Rustdoc emits an empty trait path when the trait is elided,
                // as in `Self::Item`. Only spell out `<T as Trait>::Assoc`
                // when there is actually a trait to name.
                let named_trait = trait_.as_ref().filter(|t| !t.path.is_empty());
                let mut result = match named_trait {
                    Some(trait_path) => format!(
                        "&lt;{} as {}&gt;::",
                        self.render_type(self_type),
                        self.render_resolved_path(trait_path),
                    ),
                    None => format!("{}::", self.render_type(self_type)),
                };
                result.push_str(&html_escape(name));
                if let Some(args) = args {
                    result.push_str(&self.render_generic_args(args));
                }
                result
            }
        }
    }

    fn render_resolved_path(&self, path: &Path) -> String {
        // Use only the last segment as the display name. Rustdoc JSON
        // stores source-level paths like "super::join_handle::JoinHandle"
        // but we want to show just "JoinHandle".
        let simple_name = last_path_segment(&path.path);
        let name = html_escape(simple_name);
        let args = path.args.as_ref()
            .map(|a| self.render_generic_args(a))
            .unwrap_or_default();

        // Try to resolve to a link.
        if let Some(url) = self.ctx.resolve_item_url(&path.id, self.current_depth) {
            format!("<a href=\"{}\">{}</a>{}", url, name, args)
        } else {
            format!("{}{}", name, args)
        }
    }

    fn render_generic_args(&self, args: &GenericArgs) -> String {
        match args {
            GenericArgs::AngleBracketed { args, constraints } => {
                if args.is_empty() && constraints.is_empty() {
                    return String::new();
                }
                let mut parts: Vec<String> = args.iter().map(|a| self.render_generic_arg(a)).collect();
                for constraint in constraints {
                    parts.push(self.render_assoc_item_constraint(constraint));
                }
                format!("&lt;{}&gt;", parts.join(", "))
            }
            GenericArgs::Parenthesized { inputs, output } => {
                let inputs_str: Vec<_> = inputs.iter().map(|t| self.render_type(t)).collect();
                let mut result = format!("({})", inputs_str.join(", "));
                if let Some(output) = output {
                    result.push_str(" -&gt; ");
                    result.push_str(&self.render_type(output));
                }
                result
            }
            GenericArgs::ReturnTypeNotation => "(..)".to_string(),
        }
    }

    fn render_generic_arg(&self, arg: &GenericArg) -> String {
        match arg {
            GenericArg::Lifetime(lt) => html_escape(lt),
            GenericArg::Type(ty) => self.render_type(ty),
            GenericArg::Const(c) => html_escape(&constant_value(c)),
            GenericArg::Infer => "_".to_string(),
        }
    }

    fn render_assoc_item_constraint(&self, constraint: &AssocItemConstraint) -> String {
        let mut result = html_escape(&constraint.name);
        if let Some(ref args) = constraint.args {
            result.push_str(&self.render_generic_args(args));
        }
        match &constraint.binding {
            AssocItemConstraintKind::Equality(term) => {
                result.push_str(" = ");
                result.push_str(&self.render_term(term));
            }
            AssocItemConstraintKind::Constraint(bounds) => {
                if !bounds.is_empty() {
                    result.push_str(": ");
                    result.push_str(&self.render_bounds(bounds));
                }
            }
        }
        result
    }

    fn render_term(&self, term: &Term) -> String {
        match term {
            Term::Type(ty) => self.render_type(ty),
            Term::Constant(c) => html_escape(&constant_value(c)),
        }
    }

    // --- Bounds and generics ---

    /// Render a `+`-joined bound list.
    pub fn render_bounds(&self, bounds: &[GenericBound]) -> String {
        bounds.iter()
            .map(|b| self.render_generic_bound(b))
            .collect::<Vec<_>>()
            .join(" + ")
    }

    fn render_generic_bound(&self, bound: &GenericBound) -> String {
        match bound {
            GenericBound::TraitBound { trait_, generic_params, modifier } => {
                let mut result = String::new();
                match modifier {
                    TraitBoundModifier::None => {}
                    TraitBoundModifier::Maybe => result.push('?'),
                    TraitBoundModifier::MaybeConst => result.push_str("~const "),
                }
                result.push_str(&self.render_for_binder(generic_params));
                result.push_str(&self.render_resolved_path(trait_));
                result
            }
            GenericBound::Outlives(lt) => html_escape(lt),
            GenericBound::Use(args) => {
                let args_str: Vec<_> = args.iter().map(|a| {
                    let name = match a {
                        PreciseCapturingArg::Lifetime(lt) => lt,
                        PreciseCapturingArg::Param(p) => p,
                    };
                    html_escape(name)
                }).collect();
                format!("use&lt;{}&gt;", args_str.join(", "))
            }
        }
    }

    /// Render a `for<'a, T> ` higher-ranked binder, or nothing if empty.
    fn render_for_binder(&self, params: &[GenericParamDef]) -> String {
        if params.is_empty() {
            return String::new();
        }
        let rendered: Vec<_> = params.iter().map(|p| self.render_generic_param_def(p)).collect();
        format!("for&lt;{}&gt; ", rendered.join(", "))
    }

    /// Render one generic parameter declaration.
    pub fn render_generic_param_def(&self, param: &GenericParamDef) -> String {
        match &param.kind {
            GenericParamDefKind::Lifetime { outlives } => {
                let mut result = html_escape(&param.name);
                if !outlives.is_empty() {
                    result.push_str(": ");
                    let lts: Vec<_> = outlives.iter().map(|l| html_escape(l)).collect();
                    result.push_str(&lts.join(" + "));
                }
                result
            }
            GenericParamDefKind::Type { bounds, default, .. } => {
                let mut result = html_escape(&param.name);
                if !bounds.is_empty() {
                    result.push_str(": ");
                    result.push_str(&self.render_bounds(bounds));
                }
                if let Some(default) = default {
                    result.push_str(" = ");
                    result.push_str(&self.render_type(default));
                }
                result
            }
            GenericParamDefKind::Const { type_, default } => {
                // The `const` keyword is part of the parameter declaration.
                let mut result = format!("const {}: {}", html_escape(&param.name), self.render_type(type_));
                if let Some(default) = default {
                    result.push_str(" = ");
                    result.push_str(&html_escape(default));
                }
                result
            }
        }
    }

    /// Render the `<...>` parameter list of a declaration, or nothing if empty.
    ///
    /// Synthetic parameters are skipped: rustdoc invents one per `impl Trait`
    /// argument, and those are already spelled out at the argument position.
    pub fn render_generic_params(&self, generics: &Generics) -> String {
        let rendered: Vec<_> = generics.params.iter()
            .filter(|p| !is_synthetic(p))
            .map(|p| self.render_generic_param_def(p))
            .collect();
        if rendered.is_empty() {
            return String::new();
        }
        format!("&lt;{}&gt;", rendered.join(", "))
    }

    /// Render a `where` clause, or nothing if there are no predicates.
    pub fn render_where_clause(&self, generics: &Generics) -> String {
        if generics.where_predicates.is_empty() {
            return String::new();
        }
        let predicates: Vec<_> = generics.where_predicates.iter()
            .map(|p| self.render_where_predicate(p))
            .collect();
        format!("\nwhere\n    {},", predicates.join(",\n    "))
    }

    fn render_where_predicate(&self, pred: &WherePredicate) -> String {
        match pred {
            WherePredicate::BoundPredicate { type_, bounds, generic_params } => {
                format!(
                    "{}{}: {}",
                    self.render_for_binder(generic_params),
                    self.render_type(type_),
                    self.render_bounds(bounds),
                )
            }
            WherePredicate::LifetimePredicate { lifetime, outlives } => {
                let lts: Vec<_> = outlives.iter().map(|s| html_escape(s)).collect();
                format!("{}: {}", html_escape(lifetime), lts.join(" + "))
            }
            WherePredicate::EqPredicate { lhs, rhs } => {
                format!("{} = {}", self.render_type(lhs), self.render_term(rhs))
            }
        }
    }

    // --- Function-like signatures ---

    fn render_dyn_trait(&self, dyn_trait: &DynTrait) -> String {
        let mut parts: Vec<String> = dyn_trait.traits.iter().map(|pt| {
            format!(
                "{}{}",
                self.render_for_binder(&pt.generic_params),
                self.render_resolved_path(&pt.trait_),
            )
        }).collect();
        if let Some(lt) = &dyn_trait.lifetime {
            parts.push(html_escape(lt));
        }
        format!("dyn {}", parts.join(" + "))
    }

    fn render_fn_pointer(&self, fp: &FunctionPointer) -> String {
        let mut result = self.render_for_binder(&fp.generic_params);
        result.push_str(&render_fn_header(&fp.header));
        result.push_str("fn(");
        let params: Vec<_> = fp.sig.inputs.iter()
            .map(|(name, ty)| self.render_param(name, ty))
            .collect();
        result.push_str(&params.join(", "));
        result.push(')');

        if let Some(ref output) = fp.sig.output {
            result.push_str(" -&gt; ");
            result.push_str(&self.render_type(output));
        }

        result
    }

    /// Render one parameter of a function or function pointer.
    ///
    /// `self` receivers are printed in shorthand form, and the placeholder
    /// names rustdoc uses for unnamed function-pointer parameters are dropped.
    fn render_param(&self, name: &str, ty: &Type) -> String {
        if name == "self"
            && let Some(shorthand) = render_self_receiver(ty)
        {
            return shorthand;
        }
        if name.is_empty() || name == "_" {
            return self.render_type(ty);
        }
        format!("{}: {}", html_escape(name), self.render_type(ty))
    }

    /// Render a function signature with links.
    pub fn render_function_sig(&self, func: &Function, name: &str, vis: Option<&Visibility>) -> String {
        let mut result = render_visibility(vis);
        result.push_str(&render_fn_header(&func.header));
        result.push_str("fn ");
        result.push_str(&html_escape(name));
        result.push_str(&self.render_generic_params(&func.generics));

        result.push('(');
        let params: Vec<_> = func.sig.inputs.iter()
            .map(|(name, ty)| self.render_param(name, ty))
            .collect();
        result.push_str(&params.join(", "));
        if func.sig.is_c_variadic {
            if !params.is_empty() {
                result.push_str(", ");
            }
            result.push_str("...");
        }
        result.push(')');

        if let Some(ref output) = func.sig.output {
            result.push_str(" -&gt; ");
            result.push_str(&self.render_type(output));
        }

        result.push_str(&self.render_where_clause(&func.generics));
        result
    }

    // --- Item declarations ---

    /// Render a struct declaration.
    pub fn render_struct_sig(&self, s: &Struct, name: &str, vis: Option<&Visibility>) -> String {
        let mut result = render_visibility(vis);
        result.push_str("struct ");
        result.push_str(&html_escape(name));
        result.push_str(&self.render_generic_params(&s.generics));

        match &s.kind {
            StructKind::Unit => {
                result.push_str(&self.render_where_clause(&s.generics));
                result.push(';');
            }
            StructKind::Tuple(fields) => {
                let rendered: Vec<_> = fields.iter()
                    .map(|f| match f {
                        Some(id) => self.render_tuple_field(id),
                        // A `None` entry is a field stripped from the public docs.
                        None => "/* private field */".to_string(),
                    })
                    .collect();
                result.push_str(&format!("({})", rendered.join(", ")));
                result.push_str(&self.render_where_clause(&s.generics));
                result.push(';');
            }
            StructKind::Plain { fields, has_stripped_fields } => {
                result.push_str(&self.render_where_clause(&s.generics));
                result.push_str(&self.render_field_block(fields, *has_stripped_fields));
            }
        }

        result
    }

    /// Render a union declaration.
    pub fn render_union_sig(&self, u: &Union, name: &str, vis: Option<&Visibility>) -> String {
        let mut result = render_visibility(vis);
        result.push_str("union ");
        result.push_str(&html_escape(name));
        result.push_str(&self.render_generic_params(&u.generics));
        result.push_str(&self.render_where_clause(&u.generics));
        result.push_str(&self.render_field_block(&u.fields, u.has_stripped_fields));
        result
    }

    /// Render an enum declaration.
    pub fn render_enum_sig(&self, e: &Enum, name: &str, vis: Option<&Visibility>) -> String {
        let mut result = render_visibility(vis);
        result.push_str("enum ");
        result.push_str(&html_escape(name));
        result.push_str(&self.render_generic_params(&e.generics));
        result.push_str(&self.render_where_clause(&e.generics));
        result
    }

    /// Render a trait declaration.
    pub fn render_trait_sig(&self, t: &Trait, name: &str, vis: Option<&Visibility>) -> String {
        let mut result = render_visibility(vis);
        if t.is_unsafe {
            result.push_str("unsafe ");
        }
        if t.is_auto {
            result.push_str("auto ");
        }
        result.push_str("trait ");
        result.push_str(&html_escape(name));
        result.push_str(&self.render_generic_params(&t.generics));

        if !t.bounds.is_empty() {
            result.push_str(": ");
            result.push_str(&self.render_bounds(&t.bounds));
        }

        result.push_str(&self.render_where_clause(&t.generics));
        result
    }

    /// Render a type alias declaration.
    pub fn render_type_alias_sig(&self, ta: &TypeAlias, name: &str, vis: Option<&Visibility>) -> String {
        let mut result = render_visibility(vis);
        result.push_str("type ");
        result.push_str(&html_escape(name));
        result.push_str(&self.render_generic_params(&ta.generics));
        result.push_str(&self.render_where_clause(&ta.generics));
        result.push_str(" = ");
        result.push_str(&self.render_type(&ta.type_));
        result.push(';');
        result
    }

    /// Render a `const` declaration.
    pub fn render_constant_sig(
        &self,
        name: &str,
        type_: &Type,
        value: Option<&str>,
        vis: Option<&Visibility>,
    ) -> String {
        let mut result = render_visibility(vis);
        result.push_str(&format!("const {}: {}", html_escape(name), self.render_type(type_)));
        if let Some(value) = value {
            result.push_str(" = ");
            result.push_str(&html_escape(value));
        }
        result.push(';');
        result
    }

    /// Render a `static` declaration.
    pub fn render_static_sig(&self, s: &Static, name: &str, vis: Option<&Visibility>) -> String {
        let mut result = render_visibility(vis);
        result.push_str("static ");
        if s.is_mutable {
            result.push_str("mut ");
        }
        result.push_str(&format!("{}: {}", html_escape(name), self.render_type(&s.type_)));
        result.push(';');
        result
    }

    /// Render an associated type declaration, as it appears in a trait.
    pub fn render_assoc_type_sig(
        &self,
        name: &str,
        generics: &Generics,
        bounds: &[GenericBound],
        default: Option<&Type>,
    ) -> String {
        let mut result = format!("type {}", html_escape(name));
        result.push_str(&self.render_generic_params(generics));
        if !bounds.is_empty() {
            result.push_str(": ");
            result.push_str(&self.render_bounds(bounds));
        }
        if let Some(default) = default {
            result.push_str(" = ");
            result.push_str(&self.render_type(default));
        }
        result.push(';');
        result
    }

    /// Render an associated const declaration, as it appears in a trait or impl.
    pub fn render_assoc_const_sig(&self, name: &str, type_: &Type, value: Option<&str>) -> String {
        let mut result = format!("const {}: {}", html_escape(name), self.render_type(type_));
        if let Some(value) = value {
            result.push_str(" = ");
            result.push_str(&html_escape(value));
        }
        result.push(';');
        result
    }

    /// Render an impl block header, e.g. `impl<T> Trait<T> for Type where ...`.
    pub fn render_impl_header(&self, impl_: &Impl) -> String {
        let mut result = String::new();
        if impl_.is_unsafe {
            result.push_str("unsafe ");
        }
        result.push_str("impl");
        result.push_str(&self.render_generic_params(&impl_.generics));
        result.push(' ');

        if let Some(trait_) = &impl_.trait_ {
            if impl_.is_negative {
                result.push('!');
            }
            result.push_str(&self.render_resolved_path(trait_));
            result.push_str(" for ");
        }

        result.push_str(&self.render_type(&impl_.for_));
        result.push_str(&self.render_where_clause(&impl_.generics));
        result
    }

    /// Render the fields of an enum variant, e.g. `(u32, String)` or `{ a: u32 }`.
    pub fn render_variant_fields(&self, kind: &VariantKind) -> Option<String> {
        match kind {
            VariantKind::Plain => None,
            VariantKind::Tuple(fields) => {
                let rendered: Vec<_> = fields.iter()
                    .map(|f| match f {
                        Some(id) => self.render_field_type(id),
                        None => "/* private field */".to_string(),
                    })
                    .collect();
                Some(format!("({})", rendered.join(", ")))
            }
            VariantKind::Struct { fields, has_stripped_fields } => {
                Some(self.render_field_block(fields, *has_stripped_fields))
            }
        }
    }

    /// Render a `{ name: Type, ... }` block for the given field ids.
    fn render_field_block(&self, fields: &[Id], has_stripped_fields: bool) -> String {
        let mut parts: Vec<String> = Vec::new();
        for id in fields {
            let Some(item) = self.ctx.krate.index.get(id) else { continue };
            let ItemEnum::StructField(ty) = &item.inner else { continue };
            parts.push(format!(
                "{}{}: {}",
                render_visibility(Some(&item.visibility)),
                html_escape(item.name.as_deref().unwrap_or("_")),
                self.render_type(ty),
            ));
        }
        if has_stripped_fields {
            parts.push("/* private fields */".to_string());
        }
        if parts.is_empty() {
            return " {}".to_string();
        }
        format!(" {{ {} }}", parts.join(", "))
    }

    /// Render a tuple-struct field, including its visibility.
    fn render_tuple_field(&self, id: &Id) -> String {
        match self.ctx.krate.index.get(id) {
            Some(item) => match &item.inner {
                ItemEnum::StructField(ty) => format!(
                    "{}{}",
                    render_visibility(Some(&item.visibility)),
                    self.render_type(ty),
                ),
                _ => "_".to_string(),
            },
            None => "_".to_string(),
        }
    }

    /// Render the type of a struct field by id.
    fn render_field_type(&self, id: &Id) -> String {
        self.ctx.krate.index.get(id)
            .and_then(|item| match &item.inner {
                ItemEnum::StructField(ty) => Some(self.render_type(ty)),
                _ => None,
            })
            // A field whose item is missing from the index was stripped.
            .unwrap_or_else(|| "_".to_string())
    }
}

/// Render a `self` receiver in its shorthand form.
///
/// Returns `None` when the receiver has a type that has to be spelled out,
/// such as `self: Rc<Self>`.
fn render_self_receiver(ty: &Type) -> Option<String> {
    match ty {
        Type::Generic(name) if name == "Self" => Some("self".to_string()),
        Type::BorrowedRef { lifetime, is_mutable, type_ } => {
            let Type::Generic(name) = &**type_ else { return None };
            if name != "Self" {
                return None;
            }
            let mut result = "&amp;".to_string();
            if let Some(lt) = lifetime {
                result.push_str(&html_escape(lt));
                result.push(' ');
            }
            if *is_mutable {
                result.push_str("mut ");
            }
            result.push_str("self");
            Some(result)
        }
        _ => None,
    }
}

/// Render the qualifiers that precede `fn`, in declaration order.
fn render_fn_header(header: &FunctionHeader) -> String {
    let mut result = String::new();
    if header.is_const {
        result.push_str("const ");
    }
    if header.is_async {
        result.push_str("async ");
    }
    if header.is_unsafe {
        result.push_str("unsafe ");
    }
    result.push_str(&render_abi(&header.abi));
    result
}

/// Render an ABI as it is written in source, e.g. `extern "C" `.
///
/// The implicit Rust ABI renders as nothing.
fn render_abi(abi: &Abi) -> String {
    let (name, unwind) = match abi {
        Abi::Rust => return String::new(),
        Abi::C { unwind } => ("C", *unwind),
        Abi::Cdecl { unwind } => ("cdecl", *unwind),
        Abi::Stdcall { unwind } => ("stdcall", *unwind),
        Abi::Fastcall { unwind } => ("fastcall", *unwind),
        Abi::Aapcs { unwind } => ("aapcs", *unwind),
        Abi::Win64 { unwind } => ("win64", *unwind),
        Abi::SysV64 { unwind } => ("sysv64", *unwind),
        Abi::System { unwind } => ("system", *unwind),
        Abi::Other(other) => return format!("extern &quot;{}&quot; ", html_escape(other)),
    };
    let suffix = if unwind { "-unwind" } else { "" };
    format!("extern &quot;{}{}&quot; ", name, suffix)
}

/// Render a visibility qualifier, including the trailing space.
fn render_visibility(vis: Option<&Visibility>) -> String {
    match vis {
        Some(Visibility::Public) => "pub ".to_string(),
        Some(Visibility::Crate) => "pub(crate) ".to_string(),
        Some(Visibility::Restricted { path, .. }) => {
            format!("pub(in {}) ", html_escape(path))
        }
        Some(Visibility::Default) | None => String::new(),
    }
}

/// Render an attribute in source form, e.g. `#[repr(C)]`.
///
/// Returns `None` for attributes that carry no information for a reader of the
/// docs, and for the ones rustdoc only exposes as an opaque debug string.
pub fn render_attribute(attr: &Attribute) -> Option<String> {
    let rendered = match attr {
        Attribute::NonExhaustive => "#[non_exhaustive]".to_string(),
        Attribute::MustUse { reason: None } => "#[must_use]".to_string(),
        Attribute::MustUse { reason: Some(reason) } => {
            format!("#[must_use = \"{}\"]", reason)
        }
        Attribute::MacroExport => "#[macro_export]".to_string(),
        Attribute::ExportName(name) => format!("#[export_name = \"{}\"]", name),
        Attribute::LinkSection(name) => format!("#[link_section = \"{}\"]", name),
        Attribute::NoMangle => "#[no_mangle]".to_string(),
        Attribute::TargetFeature { enable } => {
            let features: Vec<_> = enable.iter()
                .map(|f| format!("enable = \"{}\"", f))
                .collect();
            format!("#[target_feature({})]", features.join(", "))
        }
        Attribute::Repr(repr) => return render_repr(repr),
        Attribute::AutomaticallyDerived | Attribute::Other(_) => return None,
    };
    Some(html_escape(&rendered))
}

fn render_repr(repr: &AttributeRepr) -> Option<String> {
    let mut parts: Vec<String> = Vec::new();
    match repr.kind {
        // `Rust` is the default and only worth printing when something else in
        // the attribute is.
        ReprKind::Rust => {}
        ReprKind::C => parts.push("C".to_string()),
        ReprKind::Transparent => parts.push("transparent".to_string()),
        ReprKind::Simd => parts.push("simd".to_string()),
    }
    if let Some(int) = &repr.int {
        parts.push(int.clone());
    }
    if let Some(align) = repr.align {
        parts.push(format!("align({})", align));
    }
    if let Some(packed) = repr.packed {
        parts.push(format!("packed({})", packed));
    }
    if parts.is_empty() {
        return None;
    }
    Some(html_escape(&format!("#[repr({})]", parts.join(", "))))
}

/// A synthetic parameter is one rustdoc invented for an `impl Trait` argument.
fn is_synthetic(param: &GenericParamDef) -> bool {
    matches!(param.kind, GenericParamDefKind::Type { is_synthetic: true, .. })
}

fn constant_value(c: &Constant) -> String {
    c.value.clone().unwrap_or_else(|| c.expr.clone())
}

/// The final segment of a `::`-separated path.
///
/// This runs for every type reference in every signature, so it scans bytes
/// backwards for a colon rather than paying for a substring search over `"::"`.
pub fn last_path_segment(path: &str) -> &str {
    match path.as_bytes().iter().rposition(|&b| b == b':') {
        Some(i) => &path[i + 1..],
        None => path,
    }
}

/// HTML-escape a string.
pub fn html_escape(s: &str) -> String {
    // Most strings have nothing to escape, and this runs on every identifier
    // in every signature, so take one pass to check before doing any work.
    if !s.contains(['&', '<', '>', '"']) {
        return s.to_string();
    }
    let mut escaped = String::with_capacity(s.len() + 16);
    for c in s.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            _ => escaped.push(c),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RenderConfig;
    use std::path::PathBuf;

    const ROOT: u32 = 0;
    const TUP: u32 = 1;
    const FIELD_0: u32 = 2;
    const FIELD_1: u32 = 3;

    fn item(id: u32, name: Option<&str>, vis: Visibility, inner: ItemEnum) -> Item {
        Item {
            id: Id(id),
            crate_id: 0,
            name: name.map(|n| n.to_string()),
            span: None,
            visibility: vis,
            docs: None,
            links: Default::default(),
            attrs: vec![],
            deprecation: None,
            inner,
        }
    }

    fn no_generics() -> Generics {
        Generics { params: vec![], where_predicates: vec![] }
    }

    /// A crate holding a tuple struct `Tup(pub u32, String)`.
    fn test_crate() -> Crate {
        let mut krate = Crate {
            root: Id(ROOT),
            crate_version: None,
            includes_private: false,
            index: Default::default(),
            paths: Default::default(),
            external_crates: Default::default(),
            target: Target { triple: String::new(), target_features: vec![] },
            format_version: 0,
        };

        krate.index.insert(Id(ROOT), item(ROOT, Some("mycrate"), Visibility::Public,
            ItemEnum::Module(Module { is_crate: true, items: vec![Id(TUP)], is_stripped: false })));
        krate.paths.insert(Id(ROOT), ItemSummary {
            crate_id: 0,
            path: vec!["mycrate".to_string()],
            kind: ItemKind::Module,
        });

        krate.index.insert(Id(TUP), item(TUP, Some("Tup"), Visibility::Public,
            ItemEnum::Struct(Struct {
                kind: StructKind::Tuple(vec![Some(Id(FIELD_0)), Some(Id(FIELD_1))]),
                generics: no_generics(),
                impls: vec![],
            })));
        krate.paths.insert(Id(TUP), ItemSummary {
            crate_id: 0,
            path: vec!["mycrate".to_string(), "Tup".to_string()],
            kind: ItemKind::Struct,
        });

        krate.index.insert(Id(FIELD_0), item(FIELD_0, Some("0"), Visibility::Public,
            ItemEnum::StructField(Type::Primitive("u32".to_string()))));
        krate.index.insert(Id(FIELD_1), item(FIELD_1, Some("1"), Visibility::Default,
            ItemEnum::StructField(Type::Primitive("bool".to_string()))));

        krate
    }

    fn config() -> RenderConfig {
        RenderConfig { output_dir: PathBuf::from("/tmp/test"), ..Default::default() }
    }

    /// Run `f` with a renderer over [`test_crate`].
    fn with_renderer(f: impl FnOnce(&LinkedRenderer)) {
        let krate = test_crate();
        let config = config();
        let ctx = RenderContext::new(&krate, &config).unwrap();
        let renderer = LinkedRenderer::new(&ctx, 1);
        f(&renderer);
    }

    fn simple_fn(inputs: Vec<(&str, Type)>, header: FunctionHeader) -> Function {
        Function {
            sig: FunctionSignature {
                inputs: inputs.into_iter().map(|(n, t)| (n.to_string(), t)).collect(),
                output: None,
                is_c_variadic: false,
            },
            generics: no_generics(),
            header,
            has_body: true,
        }
    }

    fn header(is_const: bool, is_async: bool, is_unsafe: bool, abi: Abi) -> FunctionHeader {
        FunctionHeader { is_const, is_unsafe, is_async, abi }
    }

    fn self_ty() -> Type {
        Type::Generic("Self".to_string())
    }

    fn ref_self(is_mutable: bool) -> Type {
        Type::BorrowedRef {
            lifetime: None,
            is_mutable,
            type_: Box::new(self_ty()),
        }
    }

    #[test]
    fn test_render_abi() {
        assert_eq!(render_abi(&Abi::Rust), "");
        assert_eq!(render_abi(&Abi::C { unwind: false }), "extern &quot;C&quot; ");
        assert_eq!(render_abi(&Abi::C { unwind: true }), "extern &quot;C-unwind&quot; ");
        assert_eq!(render_abi(&Abi::System { unwind: false }), "extern &quot;system&quot; ");
        assert_eq!(
            render_abi(&Abi::Other("ptx-kernel".to_string())),
            "extern &quot;ptx-kernel&quot; ",
        );
    }

    #[test]
    fn test_render_fn_header_order() {
        // rustc requires `const` before `unsafe` before `extern`.
        assert_eq!(
            render_fn_header(&header(true, false, true, Abi::C { unwind: false })),
            "const unsafe extern &quot;C&quot; ",
        );
        assert_eq!(render_fn_header(&header(false, true, false, Abi::Rust)), "async ");
        assert_eq!(render_fn_header(&header(false, false, false, Abi::Rust)), "");
    }

    #[test]
    fn test_self_receivers() {
        with_renderer(|r| {
            let f = simple_fn(vec![("self", self_ty())], header(false, false, false, Abi::Rust));
            assert_eq!(r.render_function_sig(&f, "take", None), "fn take(self)");

            let f = simple_fn(vec![("self", ref_self(false))], header(false, false, false, Abi::Rust));
            assert_eq!(r.render_function_sig(&f, "get", None), "fn get(&amp;self)");

            let f = simple_fn(vec![("self", ref_self(true))], header(false, false, false, Abi::Rust));
            assert_eq!(r.render_function_sig(&f, "set", None), "fn set(&amp;mut self)");
        });
    }

    #[test]
    fn test_boxed_receiver_is_spelled_out() {
        with_renderer(|r| {
            // A receiver that is not `Self` or `&Self` keeps its type annotation.
            let boxed = Type::ResolvedPath(Path {
                path: "Box".to_string(),
                id: Id(999),
                args: Some(Box::new(GenericArgs::AngleBracketed {
                    args: vec![GenericArg::Type(self_ty())],
                    constraints: vec![],
                })),
            });
            let f = simple_fn(vec![("self", boxed)], header(false, false, false, Abi::Rust));
            assert_eq!(
                r.render_function_sig(&f, "boxed", None),
                "fn boxed(self: Box&lt;Self&gt;)",
            );
        });
    }

    #[test]
    fn test_unnamed_fn_pointer_params() {
        with_renderer(|r| {
            // Rustdoc names unnamed function-pointer parameters `_`.
            let fp = Type::FunctionPointer(Box::new(FunctionPointer {
                sig: FunctionSignature {
                    inputs: vec![("_".to_string(), Type::Primitive("u32".to_string()))],
                    output: Some(Type::Primitive("bool".to_string())),
                    is_c_variadic: false,
                },
                generic_params: vec![],
                header: header(false, false, true, Abi::C { unwind: false }),
            }));
            assert_eq!(
                r.render_type(&fp),
                "unsafe extern &quot;C&quot; fn(u32) -&gt; bool",
            );
        });
    }

    #[test]
    fn test_const_generic_param() {
        with_renderer(|r| {
            let param = GenericParamDef {
                name: "N".to_string(),
                kind: GenericParamDefKind::Const {
                    type_: Type::Primitive("usize".to_string()),
                    default: Some("4".to_string()),
                },
            };
            assert_eq!(r.render_generic_param_def(&param), "const N: usize = 4");
        });
    }

    #[test]
    fn test_synthetic_params_are_skipped() {
        with_renderer(|r| {
            // Rustdoc invents a parameter per `impl Trait` argument; it is
            // already visible at the argument position.
            let generics = Generics {
                params: vec![
                    GenericParamDef {
                        name: "T".to_string(),
                        kind: GenericParamDefKind::Type {
                            bounds: vec![], default: None, is_synthetic: false,
                        },
                    },
                    GenericParamDef {
                        name: "impl Display".to_string(),
                        kind: GenericParamDefKind::Type {
                            bounds: vec![], default: None, is_synthetic: true,
                        },
                    },
                ],
                where_predicates: vec![],
            };
            assert_eq!(r.render_generic_params(&generics), "&lt;T&gt;");
        });
    }

    #[test]
    fn test_precise_capturing_bound() {
        with_renderer(|r| {
            let ty = Type::ImplTrait(vec![GenericBound::Use(vec![
                PreciseCapturingArg::Lifetime("'a".to_string()),
                PreciseCapturingArg::Param("T".to_string()),
            ])]);
            assert_eq!(r.render_type(&ty), "impl use&lt;'a, T&gt;");
        });
    }

    #[test]
    fn test_qualified_path_with_elided_trait() {
        with_renderer(|r| {
            // Rustdoc leaves the trait path empty for `Self::Item`.
            let elided = Type::QualifiedPath {
                name: "Item".to_string(),
                args: None,
                self_type: Box::new(self_ty()),
                trait_: Some(Path { path: String::new(), id: Id(999), args: None }),
            };
            assert_eq!(r.render_type(&elided), "Self::Item");

            let spelled = Type::QualifiedPath {
                name: "Item".to_string(),
                args: None,
                self_type: Box::new(Type::Generic("I".to_string())),
                trait_: Some(Path {
                    path: "Iterator".to_string(),
                    id: Id(998),
                    args: None,
                }),
            };
            assert_eq!(r.render_type(&spelled), "&lt;I as Iterator&gt;::Item");
        });
    }

    #[test]
    fn test_tuple_struct_renders_field_types() {
        with_renderer(|r| {
            let krate = test_crate();
            let ItemEnum::Struct(s) = &krate.index.get(&Id(TUP)).unwrap().inner else {
                panic!("expected struct");
            };
            // Field types, not field ids; the private field keeps no `pub`.
            assert_eq!(
                r.render_struct_sig(s, "Tup", Some(&Visibility::Public)),
                "pub struct Tup(pub u32, bool);",
            );
        });
    }

    #[test]
    fn test_static_mut() {
        with_renderer(|r| {
            let s = Static {
                type_: Type::Primitive("u32".to_string()),
                is_mutable: true,
                expr: "0".to_string(),
                is_unsafe: false,
            };
            assert_eq!(
                r.render_static_sig(&s, "COUNTER", Some(&Visibility::Public)),
                "pub static mut COUNTER: u32;",
            );
        });
    }

    #[test]
    fn test_type_alias_keeps_generics() {
        with_renderer(|r| {
            let ta = TypeAlias {
                type_: Type::Generic("T".to_string()),
                generics: Generics {
                    params: vec![GenericParamDef {
                        name: "T".to_string(),
                        kind: GenericParamDefKind::Type {
                            bounds: vec![], default: None, is_synthetic: false,
                        },
                    }],
                    where_predicates: vec![],
                },
            };
            assert_eq!(
                r.render_type_alias_sig(&ta, "Alias", Some(&Visibility::Public)),
                "pub type Alias&lt;T&gt; = T;",
            );
        });
    }

    #[test]
    fn test_impl_header_keeps_trait_args_and_where_clause() {
        with_renderer(|r| {
            let impl_ = Impl {
                is_unsafe: false,
                generics: Generics {
                    params: vec![GenericParamDef {
                        name: "T".to_string(),
                        kind: GenericParamDefKind::Type {
                            bounds: vec![], default: None, is_synthetic: false,
                        },
                    }],
                    where_predicates: vec![WherePredicate::BoundPredicate {
                        type_: Type::Generic("T".to_string()),
                        bounds: vec![GenericBound::TraitBound {
                            trait_: Path { path: "Clone".to_string(), id: Id(997), args: None },
                            generic_params: vec![],
                            modifier: TraitBoundModifier::None,
                        }],
                        generic_params: vec![],
                    }],
                },
                provided_trait_methods: vec![],
                trait_: Some(Path {
                    path: "Complex".to_string(),
                    id: Id(996),
                    args: Some(Box::new(GenericArgs::AngleBracketed {
                        args: vec![GenericArg::Type(Type::Generic("T".to_string()))],
                        constraints: vec![],
                    })),
                }),
                for_: Type::Primitive("u32".to_string()),
                items: vec![],
                is_negative: false,
                is_synthetic: false,
                blanket_impl: None,
            };
            assert_eq!(
                r.render_impl_header(&impl_),
                "impl&lt;T&gt; Complex&lt;T&gt; for u32\nwhere\n    T: Clone,",
            );
        });
    }

    #[test]
    fn test_negative_and_unsafe_impl_header() {
        with_renderer(|r| {
            let mut impl_ = Impl {
                is_unsafe: true,
                generics: no_generics(),
                provided_trait_methods: vec![],
                trait_: Some(Path { path: "Send".to_string(), id: Id(995), args: None }),
                for_: Type::Primitive("u32".to_string()),
                items: vec![],
                is_negative: true,
                is_synthetic: false,
                blanket_impl: None,
            };
            assert_eq!(r.render_impl_header(&impl_), "unsafe impl !Send for u32");

            impl_.is_unsafe = false;
            impl_.is_negative = false;
            assert_eq!(r.render_impl_header(&impl_), "impl Send for u32");
        });
    }

    #[test]
    fn test_render_attribute() {
        assert_eq!(render_attribute(&Attribute::NonExhaustive).as_deref(), Some("#[non_exhaustive]"));
        assert_eq!(
            render_attribute(&Attribute::Repr(AttributeRepr {
                kind: ReprKind::C, align: Some(8), packed: None, int: None,
            })).as_deref(),
            Some("#[repr(C, align(8))]"),
        );
        assert_eq!(
            render_attribute(&Attribute::Repr(AttributeRepr {
                kind: ReprKind::Rust, align: None, packed: None, int: Some("u8".to_string()),
            })).as_deref(),
            Some("#[repr(u8)]"),
        );
        // A plain `#[repr(Rust)]` is the default and carries no information.
        assert_eq!(
            render_attribute(&Attribute::Repr(AttributeRepr {
                kind: ReprKind::Rust, align: None, packed: None, int: None,
            })),
            None,
        );
        // Opaque debug strings are not worth showing.
        assert_eq!(render_attribute(&Attribute::Other("#[attr = Optimize(Speed)]".to_string())), None);
    }

    #[test]
    fn test_render_visibility() {
        assert_eq!(render_visibility(Some(&Visibility::Public)), "pub ");
        assert_eq!(render_visibility(Some(&Visibility::Crate)), "pub(crate) ");
        assert_eq!(render_visibility(Some(&Visibility::Default)), "");
        assert_eq!(render_visibility(None), "");
    }
}
