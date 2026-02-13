//! Type signature formatting.

use rustdoc_types::*;

/// Render a type to its string representation.
pub fn render_type(ty: &Type) -> String {
    match ty {
        Type::ResolvedPath(path) => render_resolved_path(path),
        Type::DynTrait(dyn_trait) => render_dyn_trait(dyn_trait),
        Type::Generic(name) => name.clone(),
        Type::Primitive(name) => name.clone(),
        Type::FunctionPointer(fp) => render_fn_pointer(fp),
        Type::Tuple(types) => {
            let inner: Vec<_> = types.iter().map(render_type).collect();
            format!("({})", inner.join(", "))
        }
        Type::Slice(inner) => format!("[{}]", render_type(inner)),
        Type::Array { type_, len } => format!("[{}; {}]", render_type(type_), len),
        Type::Pat { type_, __pat_unstable_do_not_use: pat } => {
            format!("{}: {}", render_type(type_), pat)
        }
        Type::ImplTrait(bounds) => {
            let bounds_str: Vec<_> = bounds.iter().map(render_generic_bound).collect();
            format!("impl {}", bounds_str.join(" + "))
        }
        Type::Infer => "_".to_string(),
        Type::RawPointer { is_mutable, type_ } => {
            let mutability = if *is_mutable { "mut" } else { "const" };
            format!("*{} {}", mutability, render_type(type_))
        }
        Type::BorrowedRef { lifetime, is_mutable, type_ } => {
            let mut result = "&".to_string();
            if let Some(lt) = lifetime {
                result.push_str(lt);
                result.push(' ');
            }
            if *is_mutable {
                result.push_str("mut ");
            }
            result.push_str(&render_type(type_));
            result
        }
        Type::QualifiedPath { name, args, self_type, trait_ } => {
            let mut result = format!("<{}", render_type(self_type));
            if let Some(trait_path) = trait_ {
                result.push_str(" as ");
                result.push_str(&render_resolved_path(trait_path));
            }
            result.push_str(">::");
            result.push_str(name);
            if let Some(args) = args {
                result.push_str(&render_generic_args(args));
            }
            result
        }
    }
}

fn render_resolved_path(path: &Path) -> String {
    let mut result = path.path.clone();
    if let Some(ref args) = path.args {
        result.push_str(&render_generic_args(args));
    }
    result
}

fn render_generic_args(args: &GenericArgs) -> String {
    match args {
        GenericArgs::AngleBracketed { args, constraints } => {
            if args.is_empty() && constraints.is_empty() {
                return String::new();
            }
            let mut parts: Vec<String> = args.iter().map(render_generic_arg).collect();
            for constraint in constraints {
                parts.push(render_assoc_item_constraint(constraint));
            }
            format!("<{}>", parts.join(", "))
        }
        GenericArgs::Parenthesized { inputs, output } => {
            let inputs_str: Vec<_> = inputs.iter().map(render_type).collect();
            let mut result = format!("({})", inputs_str.join(", "));
            if let Some(output) = output {
                result.push_str(" -> ");
                result.push_str(&render_type(output));
            }
            result
        }
        GenericArgs::ReturnTypeNotation => "(..)".to_string(),
    }
}

fn render_generic_arg(arg: &GenericArg) -> String {
    match arg {
        GenericArg::Lifetime(lt) => lt.clone(),
        GenericArg::Type(ty) => render_type(ty),
        GenericArg::Const(c) => render_constant(c),
        GenericArg::Infer => "_".to_string(),
    }
}

fn render_assoc_item_constraint(constraint: &AssocItemConstraint) -> String {
    let mut result = constraint.name.clone();
    if let Some(ref args) = constraint.args {
        result.push_str(&render_generic_args(args));
    }
    match &constraint.binding {
        AssocItemConstraintKind::Equality(term) => {
            result.push_str(" = ");
            result.push_str(&render_term(term));
        }
        AssocItemConstraintKind::Constraint(bounds) => {
            if !bounds.is_empty() {
                result.push_str(": ");
                let bounds_str: Vec<_> = bounds.iter().map(render_generic_bound).collect();
                result.push_str(&bounds_str.join(" + "));
            }
        }
    }
    result
}

fn render_term(term: &Term) -> String {
    match term {
        Term::Type(ty) => render_type(ty),
        Term::Constant(c) => render_constant(c),
    }
}

fn render_constant(c: &Constant) -> String {
    c.value.clone().unwrap_or_else(|| c.expr.clone())
}

fn render_generic_bound(bound: &GenericBound) -> String {
    match bound {
        GenericBound::TraitBound { trait_, generic_params, modifier } => {
            let mut result = String::new();
            match modifier {
                TraitBoundModifier::None => {}
                TraitBoundModifier::Maybe => result.push('?'),
                TraitBoundModifier::MaybeConst => result.push_str("~const "),
            }
            if !generic_params.is_empty() {
                result.push_str("for<");
                let params: Vec<_> = generic_params.iter().map(render_generic_param_def).collect();
                result.push_str(&params.join(", "));
                result.push_str("> ");
            }
            result.push_str(&render_resolved_path(trait_));
            result
        }
        GenericBound::Outlives(lt) => lt.clone(),
        GenericBound::Use(args) => {
            let args_str: Vec<_> = args.iter().map(|a| format!("{:?}", a)).collect();
            format!("use<{}>", args_str.join(", "))
        }
    }
}

pub fn render_generic_param_def(param: &GenericParamDef) -> String {
    let mut result = param.name.clone();
    match &param.kind {
        GenericParamDefKind::Lifetime { outlives } => {
            if !outlives.is_empty() {
                result.push_str(": ");
                result.push_str(&outlives.join(" + "));
            }
        }
        GenericParamDefKind::Type { bounds, default, is_synthetic: _ } => {
            if !bounds.is_empty() {
                result.push_str(": ");
                let bounds_str: Vec<_> = bounds.iter().map(render_generic_bound).collect();
                result.push_str(&bounds_str.join(" + "));
            }
            if let Some(default) = default {
                result.push_str(" = ");
                result.push_str(&render_type(default));
            }
        }
        GenericParamDefKind::Const { type_, default } => {
            result.push_str(": ");
            result.push_str(&render_type(type_));
            if let Some(default) = default {
                result.push_str(" = ");
                result.push_str(default);
            }
        }
    }
    result
}

fn render_dyn_trait(dyn_trait: &DynTrait) -> String {
    let mut result = "dyn ".to_string();
    let traits: Vec<_> = dyn_trait.traits.iter().map(|pt| {
        let mut s = render_resolved_path(&pt.trait_);
        if !pt.generic_params.is_empty() {
            s = format!("for<{}> {}", pt.generic_params.iter().map(render_generic_param_def).collect::<Vec<_>>().join(", "), s);
        }
        s
    }).collect();
    result.push_str(&traits.join(" + "));
    if let Some(lt) = &dyn_trait.lifetime {
        result.push_str(" + ");
        result.push_str(lt);
    }
    result
}

fn render_fn_pointer(fp: &FunctionPointer) -> String {
    let mut result = String::new();

    if !fp.generic_params.is_empty() {
        result.push_str("for<");
        let params: Vec<_> = fp.generic_params.iter().map(render_generic_param_def).collect();
        result.push_str(&params.join(", "));
        result.push_str("> ");
    }

    if fp.header.is_unsafe {
        result.push_str("unsafe ");
    }
    if fp.header.is_async {
        result.push_str("async ");
    }
    if fp.header.is_const {
        result.push_str("const ");
    }
    if fp.header.abi != Abi::Rust {
        result.push_str(&format!("extern {:?} ", fp.header.abi));
    }

    result.push_str("fn(");
    let params: Vec<_> = fp.sig.inputs.iter().map(|(name, ty)| {
        if name.is_empty() {
            render_type(ty)
        } else {
            format!("{}: {}", name, render_type(ty))
        }
    }).collect();
    result.push_str(&params.join(", "));
    result.push(')');

    if let Some(ref output) = fp.sig.output {
        result.push_str(" -> ");
        result.push_str(&render_type(output));
    }

    result
}

fn render_where_predicate(pred: &WherePredicate) -> String {
    match pred {
        WherePredicate::BoundPredicate { type_, bounds, generic_params } => {
            let mut result = String::new();
            if !generic_params.is_empty() {
                result.push_str("for<");
                let params: Vec<_> = generic_params.iter().map(render_generic_param_def).collect();
                result.push_str(&params.join(", "));
                result.push_str("> ");
            }
            result.push_str(&render_type(type_));
            result.push_str(": ");
            let bounds_str: Vec<_> = bounds.iter().map(render_generic_bound).collect();
            result.push_str(&bounds_str.join(" + "));
            result
        }
        WherePredicate::LifetimePredicate { lifetime, outlives } => {
            format!("{}: {}", lifetime, outlives.join(" + "))
        }
        WherePredicate::EqPredicate { lhs, rhs } => {
            format!("{} = {}", render_type(lhs), render_term(rhs))
        }
    }
}

/// Render struct definition.
pub fn render_struct_sig(s: &Struct, name: &str, generics: &Generics) -> String {
    let mut result = String::from("struct ");
    result.push_str(name);

    if !generics.params.is_empty() {
        result.push('<');
        let params: Vec<_> = generics.params.iter().map(render_generic_param_def).collect();
        result.push_str(&params.join(", "));
        result.push('>');
    }

    match &s.kind {
        StructKind::Unit => {}
        StructKind::Tuple(fields) => {
            result.push('(');
            let field_strs: Vec<_> = fields.iter().map(|f| {
                f.as_ref().map(|id| format!("{}", id.0)).unwrap_or_else(|| "_".to_string())
            }).collect();
            result.push_str(&field_strs.join(", "));
            result.push(')');
        }
        StructKind::Plain { fields: _, has_stripped_fields: _ } => {
            result.push_str(" { ... }");
        }
    }

    if !generics.where_predicates.is_empty() {
        result.push_str("\nwhere\n    ");
        let predicates: Vec<_> = generics.where_predicates.iter()
            .map(render_where_predicate)
            .collect();
        result.push_str(&predicates.join(",\n    "));
    }

    result
}

/// Render union definition.
pub fn render_union_sig(_u: &Union, name: &str, generics: &Generics) -> String {
    let mut result = String::from("union ");
    result.push_str(name);

    if !generics.params.is_empty() {
        result.push('<');
        let params: Vec<_> = generics.params.iter().map(render_generic_param_def).collect();
        result.push_str(&params.join(", "));
        result.push('>');
    }

    result.push_str(" { ... }");

    if !generics.where_predicates.is_empty() {
        result.push_str("\nwhere\n    ");
        let predicates: Vec<_> = generics.where_predicates.iter()
            .map(render_where_predicate)
            .collect();
        result.push_str(&predicates.join(",\n    "));
    }

    result
}

/// Render enum definition.
pub fn render_enum_sig(_e: &Enum, name: &str, generics: &Generics) -> String {
    let mut result = String::from("enum ");
    result.push_str(name);

    if !generics.params.is_empty() {
        result.push('<');
        let params: Vec<_> = generics.params.iter().map(render_generic_param_def).collect();
        result.push_str(&params.join(", "));
        result.push('>');
    }

    if !generics.where_predicates.is_empty() {
        result.push_str("\nwhere\n    ");
        let predicates: Vec<_> = generics.where_predicates.iter()
            .map(render_where_predicate)
            .collect();
        result.push_str(&predicates.join(",\n    "));
    }

    result
}

/// Render trait definition.
pub fn render_trait_sig(t: &Trait, name: &str, generics: &Generics) -> String {
    let mut result = String::new();

    if t.is_unsafe {
        result.push_str("unsafe ");
    }
    if t.is_auto {
        result.push_str("auto ");
    }

    result.push_str("trait ");
    result.push_str(name);

    if !generics.params.is_empty() {
        result.push('<');
        let params: Vec<_> = generics.params.iter().map(render_generic_param_def).collect();
        result.push_str(&params.join(", "));
        result.push('>');
    }

    if !t.bounds.is_empty() {
        result.push_str(": ");
        let bounds: Vec<_> = t.bounds.iter().map(render_generic_bound).collect();
        result.push_str(&bounds.join(" + "));
    }

    if !generics.where_predicates.is_empty() {
        result.push_str("\nwhere\n    ");
        let predicates: Vec<_> = generics.where_predicates.iter()
            .map(render_where_predicate)
            .collect();
        result.push_str(&predicates.join(",\n    "));
    }

    result
}

// --- Linked signature rendering (with HTML links) ---

use super::RenderContext;

/// Renderer that outputs HTML with links for type references.
pub struct LinkedRenderer<'a, 'ctx> {
    ctx: &'a RenderContext<'ctx>,
    current_depth: usize,
}

impl<'a, 'ctx> LinkedRenderer<'a, 'ctx> {
    /// Create a new linked renderer.
    pub fn new(ctx: &'a RenderContext<'ctx>, current_depth: usize) -> Self {
        Self { ctx, current_depth }
    }

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
                format!("({})", inner.join(", "))
            }
            Type::Slice(inner) => format!("[{}]", self.render_type(inner)),
            Type::Array { type_, len } => format!("[{}; {}]", self.render_type(type_), html_escape(len)),
            Type::Pat { type_, __pat_unstable_do_not_use: pat } => {
                format!("{}: {}", self.render_type(type_), html_escape(pat))
            }
            Type::ImplTrait(bounds) => {
                let bounds_str: Vec<_> = bounds.iter().map(|b| self.render_generic_bound(b)).collect();
                format!("impl {}", bounds_str.join(" + "))
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
                let mut result = format!("&lt;{}", self.render_type(self_type));
                if let Some(trait_path) = trait_ {
                    result.push_str(" as ");
                    result.push_str(&self.render_resolved_path(trait_path));
                }
                result.push_str("&gt;::");
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
        let simple_name = path.path.rsplit("::").next().unwrap_or(&path.path);
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
            GenericArg::Const(c) => html_escape(&render_constant(c)),
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
                    let bounds_str: Vec<_> = bounds.iter().map(|b| self.render_generic_bound(b)).collect();
                    result.push_str(&bounds_str.join(" + "));
                }
            }
        }
        result
    }

    fn render_term(&self, term: &Term) -> String {
        match term {
            Term::Type(ty) => self.render_type(ty),
            Term::Constant(c) => html_escape(&render_constant(c)),
        }
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
                if !generic_params.is_empty() {
                    result.push_str("for&lt;");
                    let params: Vec<_> = generic_params.iter()
                        .map(|p| html_escape(&render_generic_param_def_plain(p)))
                        .collect();
                    result.push_str(&params.join(", "));
                    result.push_str("&gt; ");
                }
                result.push_str(&self.render_resolved_path(trait_));
                result
            }
            GenericBound::Outlives(lt) => html_escape(lt),
            GenericBound::Use(args) => {
                let args_str: Vec<_> = args.iter().map(|a| format!("{:?}", a)).collect();
                format!("use&lt;{}&gt;", args_str.join(", "))
            }
        }
    }

    fn render_dyn_trait(&self, dyn_trait: &DynTrait) -> String {
        let mut result = "dyn ".to_string();
        let traits: Vec<_> = dyn_trait.traits.iter().map(|pt| {
            let mut s = self.render_resolved_path(&pt.trait_);
            if !pt.generic_params.is_empty() {
                let params: Vec<_> = pt.generic_params.iter()
                    .map(|p| html_escape(&render_generic_param_def_plain(p)))
                    .collect();
                s = format!("for&lt;{}&gt; {}", params.join(", "), s);
            }
            s
        }).collect();
        result.push_str(&traits.join(" + "));
        if let Some(lt) = &dyn_trait.lifetime {
            result.push_str(" + ");
            result.push_str(&html_escape(lt));
        }
        result
    }

    fn render_fn_pointer(&self, fp: &FunctionPointer) -> String {
        let mut result = String::new();

        if !fp.generic_params.is_empty() {
            result.push_str("for&lt;");
            let params: Vec<_> = fp.generic_params.iter()
                .map(|p| html_escape(&render_generic_param_def_plain(p)))
                .collect();
            result.push_str(&params.join(", "));
            result.push_str("&gt; ");
        }

        if fp.header.is_unsafe {
            result.push_str("unsafe ");
        }
        if fp.header.is_async {
            result.push_str("async ");
        }
        if fp.header.is_const {
            result.push_str("const ");
        }
        if fp.header.abi != Abi::Rust {
            result.push_str(&format!("extern {:?} ", fp.header.abi));
        }

        result.push_str("fn(");
        let params: Vec<_> = fp.sig.inputs.iter().map(|(name, ty)| {
            if name.is_empty() {
                self.render_type(ty)
            } else {
                format!("{}: {}", html_escape(name), self.render_type(ty))
            }
        }).collect();
        result.push_str(&params.join(", "));
        result.push(')');

        if let Some(ref output) = fp.sig.output {
            result.push_str(" -&gt; ");
            result.push_str(&self.render_type(output));
        }

        result
    }

    /// Render a function signature with links.
    pub fn render_function_sig(&self, func: &Function, name: &str) -> String {
        let mut result = String::new();

        if func.header.is_unsafe {
            result.push_str("unsafe ");
        }
        if func.header.is_async {
            result.push_str("async ");
        }
        if func.header.is_const {
            result.push_str("const ");
        }
        if func.header.abi != Abi::Rust {
            result.push_str(&format!("extern {:?} ", func.header.abi));
        }

        result.push_str("fn ");
        result.push_str(&html_escape(name));

        if !func.generics.params.is_empty() {
            result.push_str("&lt;");
            let params: Vec<_> = func.generics.params.iter()
                .map(|p| html_escape(&render_generic_param_def_plain(p)))
                .collect();
            result.push_str(&params.join(", "));
            result.push_str("&gt;");
        }

        result.push('(');
        let params: Vec<_> = func.sig.inputs.iter().map(|(name, ty)| {
            format!("{}: {}", html_escape(name), self.render_type(ty))
        }).collect();
        result.push_str(&params.join(", "));
        result.push(')');

        if let Some(ref output) = func.sig.output {
            result.push_str(" -&gt; ");
            result.push_str(&self.render_type(output));
        }

        if !func.generics.where_predicates.is_empty() {
            result.push_str("\nwhere\n    ");
            let predicates: Vec<_> = func.generics.where_predicates.iter()
                .map(|p| self.render_where_predicate(p))
                .collect();
            result.push_str(&predicates.join(",\n    "));
        }

        result
    }

    fn render_where_predicate(&self, pred: &WherePredicate) -> String {
        match pred {
            WherePredicate::BoundPredicate { type_, bounds, generic_params } => {
                let mut result = String::new();
                if !generic_params.is_empty() {
                    result.push_str("for&lt;");
                    let params: Vec<_> = generic_params.iter()
                        .map(|p| html_escape(&render_generic_param_def_plain(p)))
                        .collect();
                    result.push_str(&params.join(", "));
                    result.push_str("&gt; ");
                }
                result.push_str(&self.render_type(type_));
                result.push_str(": ");
                let bounds_str: Vec<_> = bounds.iter().map(|b| self.render_generic_bound(b)).collect();
                result.push_str(&bounds_str.join(" + "));
                result
            }
            WherePredicate::LifetimePredicate { lifetime, outlives } => {
                format!("{}: {}", html_escape(lifetime), outlives.iter().map(|s| html_escape(s)).collect::<Vec<_>>().join(" + "))
            }
            WherePredicate::EqPredicate { lhs, rhs } => {
                format!("{} = {}", self.render_type(lhs), self.render_term(rhs))
            }
        }
    }
}

/// Render a generic param def without HTML escaping (plain text).
fn render_generic_param_def_plain(param: &GenericParamDef) -> String {
    render_generic_param_def(param)
}

/// HTML-escape a string.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
