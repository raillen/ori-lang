use crate::hir::*;
use ori_ast::common::Visibility;
use ori_ast::expr::{Arg, ClosureBody, ClosureExpr, Expr, FStrPart};
use ori_ast::item::{Item, SourceFile};
use ori_ast::stmt::{Block, MatchCase, Stmt};
use ori_diagnostics::{DiagnosticSink, FileId, Span};
use ori_types::{
    expand_ty_aliases, lower_type_with_aliases, DefId, DefKind, DefMap, FuncSig, ImplSig, ReExport,
    TraitSig, TypeAliasSig, Ty,
};
use smol_str::SmolStr;
use std::collections::{HashMap, HashSet};

/// Maps an Ori stdlib qualified path to the C function name used at link time.
fn stdlib_c_name(ori_path: &str) -> Option<&'static str> {
    match ori_path {
        // ori.io
        "ori.io.print" | "ori.io.println" => Some("ori_io_print"),
        "ori.io.eprint" | "ori.io.eprintln" => Some("ori_io_eprint"),
        "ori.io.read_line" => Some("ori_io_read_line"),
        // ori.string
        "ori.string.len" => Some("ori_string_len"),
        "ori.string.concat" => Some("ori_string_concat"),
        "ori.string.split" => Some("ori_string_split"),
        "ori.string.slice" => Some("ori_string_slice"),
        "ori.string.contains" => Some("ori_string_contains"),
        "ori.string.starts_with" => Some("ori_string_starts_with"),
        "ori.string.ends_with" => Some("ori_string_ends_with"),
        "ori.string.trim" => Some("ori_string_trim"),
        "ori.string.to_upper" => Some("ori_string_to_upper"),
        "ori.string.to_lower" => Some("ori_string_to_lower"),
        "ori.string.replace" => Some("ori_string_replace"),
        "ori.string.chars" => Some("ori_string_chars"),
        // builtin conversion functions
        "string" => Some("ori_to_string"),
        "int" => Some("ori_to_int"),
        "float" => Some("ori_to_float"),
        "len" => Some("ori_len"),
        // list operations (used as method calls: list.push, list.get, etc.)
        "ori.list.new" | "list.new" => Some("ori_list_new"),
        "ori.list.push" | "list.push" => Some("ori_list_push"),
        "ori.list.get" | "list.get" => Some("ori_list_get"),
        "ori.list.set" | "list.set" => Some("ori_list_set"),
        "ori.list.len" | "list.len" => Some("ori_list_len"),
        "ori.list.free" | "list.free" => Some("ori_list_free"),
        "ori.set.new" | "set.new" => Some("ori_set_new"),
        "ori.set.add" | "set.add" => Some("ori_set_add"),
        "ori.set.contains" | "set.contains" => Some("ori_set_contains"),
        "ori.set.len" | "set.len" => Some("ori_set_len"),
        "ori.set.free" | "set.free" => Some("ori_set_free"),
        "ori.map.new" | "map.new" => Some("ori_map_new"),
        "ori.map.set" | "map.set" => Some("ori_map_set"),
        "ori.map.get" | "map.get" => Some("ori_map_get"),
        "ori.map.contains" | "map.contains" => Some("ori_map_contains"),
        "ori.map.len" | "map.len" => Some("ori_map_len"),
        "ori.map.free" | "map.free" => Some("ori_map_free"),
        // ori.math
        "ori.math.sqrt" => Some("ori_math_sqrt"),
        "ori.math.abs" => Some("ori_math_abs"),
        "ori.math.min" => Some("ori_math_min"),
        "ori.math.max" => Some("ori_math_max"),
        _ => None,
    }
}

fn stdlib_c_func_ty(c_name: &str) -> Ty {
    let (params, ret) = match c_name {
        "ori_io_print" | "ori_io_eprint" => (vec![Ty::String], Ty::Void),
        "ori_io_read_line" => (vec![], Ty::String),
        "ori_string_len" | "ori_len" => (vec![Ty::String], Ty::Int),
        "ori_string_concat" => (vec![Ty::String, Ty::String], Ty::String),
        "ori_string_split" => (vec![Ty::String, Ty::String], Ty::List(Box::new(Ty::String))),
        "ori_string_slice" => (vec![Ty::String, Ty::Int, Ty::Int], Ty::String),
        "ori_string_contains" | "ori_string_starts_with" | "ori_string_ends_with" => {
            (vec![Ty::String, Ty::String], Ty::Bool)
        }
        "ori_string_trim" | "ori_string_to_upper" | "ori_string_to_lower" => {
            (vec![Ty::String], Ty::String)
        }
        "ori_string_replace" => (vec![Ty::String, Ty::String, Ty::String], Ty::String),
        "ori_string_chars" => (vec![Ty::String], Ty::List(Box::new(Ty::String))),
        "ori_to_string" => (vec![Ty::Int], Ty::String),
        "ori_list_new" => (vec![], Ty::List(Box::new(Ty::Infer(0)))),
        "ori_list_push" => (
            vec![Ty::List(Box::new(Ty::Infer(0))), Ty::Infer(0)],
            Ty::Void,
        ),
        "ori_list_get" => (
            vec![Ty::List(Box::new(Ty::Infer(0))), Ty::Int],
            Ty::Infer(0),
        ),
        "ori_list_set" => (
            vec![Ty::List(Box::new(Ty::Infer(0))), Ty::Int, Ty::Infer(0)],
            Ty::Void,
        ),
        "ori_list_len" => (vec![Ty::List(Box::new(Ty::Infer(0)))], Ty::Int),
        "ori_list_free" => (vec![Ty::List(Box::new(Ty::Infer(0)))], Ty::Void),
        "ori_set_new" => (vec![], Ty::Set(Box::new(Ty::Infer(0)))),
        "ori_set_add" => (
            vec![Ty::Set(Box::new(Ty::Infer(0))), Ty::Infer(0)],
            Ty::Void,
        ),
        "ori_set_contains" => (
            vec![Ty::Set(Box::new(Ty::Infer(0))), Ty::Infer(0)],
            Ty::Bool,
        ),
        "ori_set_len" => (vec![Ty::Set(Box::new(Ty::Infer(0)))], Ty::Int),
        "ori_set_free" => (vec![Ty::Set(Box::new(Ty::Infer(0)))], Ty::Void),
        "ori_map_new" => (
            vec![],
            Ty::Map(Box::new(Ty::Infer(0)), Box::new(Ty::Infer(0))),
        ),
        "ori_map_set" => (
            vec![
                Ty::Map(Box::new(Ty::Infer(0)), Box::new(Ty::Infer(0))),
                Ty::Infer(0),
                Ty::Infer(0),
            ],
            Ty::Void,
        ),
        "ori_map_get" => (
            vec![
                Ty::Map(Box::new(Ty::Infer(0)), Box::new(Ty::Infer(0))),
                Ty::Infer(0),
            ],
            Ty::Infer(0),
        ),
        "ori_map_contains" => (
            vec![
                Ty::Map(Box::new(Ty::Infer(0)), Box::new(Ty::Infer(0))),
                Ty::Infer(0),
            ],
            Ty::Bool,
        ),
        "ori_map_len" => (
            vec![Ty::Map(Box::new(Ty::Infer(0)), Box::new(Ty::Infer(0)))],
            Ty::Int,
        ),
        "ori_map_free" => (
            vec![Ty::Map(Box::new(Ty::Infer(0)), Box::new(Ty::Infer(0)))],
            Ty::Void,
        ),
        "ori_math_sqrt" => (vec![Ty::Float], Ty::Float),
        "ori_math_abs" => (vec![Ty::Int], Ty::Int),
        "ori_math_min" | "ori_math_max" => (vec![Ty::Int, Ty::Int], Ty::Int),
        _ => return Ty::Infer(0),
    };
    Ty::Func {
        params,
        ret: Box::new(ret),
    }
}

// ── Scope stack ───────────────────────────────────────────────────────────────

#[derive(Default)]
struct Scope {
    vars: HashMap<SmolStr, Ty>,
}

struct Lowerer<'a> {
    def_map: &'a DefMap,
    func_sigs: &'a [FuncSig],
    trait_sigs: &'a [TraitSig],
    impl_sigs: &'a [ImplSig],
    namespace: &'a str,
    file_id: FileId,
    sink: &'a mut DiagnosticSink,
    scopes: Vec<Scope>,
    /// `import ori.io as io` → `io` maps to `ori.io`.
    aliases: HashMap<SmolStr, SmolStr>,
    /// Current function's return type (for `?` desugaring).
    ret_ty: Ty,
    closure_counter: usize,
    generated_funcs: Vec<HirFunc>,
    /// `DefId` → `(arity, underlying_ty)` for each `type alias` declaration.
    type_alias_map: HashMap<DefId, (usize, Ty)>,
}

impl<'a> Lowerer<'a> {
    fn new(
        def_map: &'a DefMap,
        func_sigs: &'a [FuncSig],
        trait_sigs: &'a [TraitSig],
        impl_sigs: &'a [ImplSig],
        type_alias_sigs: &[TypeAliasSig],
        namespace: &'a str,
        file_id: FileId,
        sink: &'a mut DiagnosticSink,
    ) -> Self {
        let type_alias_map: HashMap<DefId, (usize, Ty)> = type_alias_sigs
            .iter()
            .map(|s| (s.def_id, (s.type_params.len(), s.ty.clone())))
            .collect();
        Self {
            def_map,
            func_sigs,
            trait_sigs,
            impl_sigs,
            namespace,
            file_id,
            sink,
            scopes: vec![Scope::default()],
            aliases: HashMap::new(),
            ret_ty: Ty::Void,
            closure_counter: 0,
            generated_funcs: Vec::new(),
            type_alias_map,
        }
    }

    /// Resolve `io.print` → `ori.io.print` using the import alias map,
    /// then look up in the stdlib table.
    fn resolve_stdlib(&self, name: &str) -> Option<&'static str> {
        // Direct hit (e.g., builtin `string`, `len`, `int`)
        if let Some(c) = stdlib_c_name(name) {
            return Some(c);
        }
        // Qualified via alias: `io.print` where `io → ori.io`
        let expanded = self.expand_alias(name);
        if expanded != name {
            return stdlib_c_name(&expanded);
        }
        None
    }
    fn push(&mut self) {
        self.scopes.push(Scope::default());
    }
    fn pop(&mut self) {
        self.scopes.pop();
    }
    fn bind(&mut self, name: SmolStr, ty: Ty) {
        if let Some(s) = self.scopes.last_mut() {
            s.vars.insert(name, ty);
        }
    }
    fn lookup_var(&self, name: &str) -> Option<Ty> {
        for s in self.scopes.iter().rev() {
            if let Some(t) = s.vars.get(name) {
                return Some(t.clone());
            }
        }
        None
    }
    fn lookup(&self, name: &str) -> Ty {
        self.lookup_var(name).unwrap_or(Ty::Error)
    }
    fn next_closure_name(&mut self) -> SmolStr {
        let index = self.closure_counter;
        self.closure_counter += 1;
        SmolStr::new(format!("{}.__closure_{}", self.namespace, index))
    }
    fn expand_alias(&self, name: &str) -> SmolStr {
        let mut prefix_end = name.len();
        loop {
            let prefix = &name[..prefix_end];
            if let Some(full_ns) = self.aliases.get(prefix) {
                let suffix = &name[prefix_end..];
                if suffix.is_empty() {
                    return SmolStr::new(full_ns.to_string());
                }
                return SmolStr::new(format!("{}{}", full_ns, suffix));
            }
            if let Some(dot) = name[..prefix_end].rfind('.') {
                prefix_end = dot;
            } else {
                break;
            }
        }
        SmolStr::new(name)
    }
    fn resolve_def_path(&self, name: &str) -> Option<SmolStr> {
        let expanded = self.expand_alias(name);
        if self.def_map.lookup(&expanded).is_some() {
            return Some(expanded);
        }
        let local = SmolStr::new(format!("{}.{}", self.namespace, expanded));
        if self.def_map.lookup(&local).is_some() {
            return Some(local);
        }
        None
    }
    fn resolve_def_id_with_kind(&self, name: &str, kind: DefKind) -> Option<ori_types::DefId> {
        let path = self.resolve_def_path(name)?;
        let id = self.def_map.lookup(&path)?;
        if self.def_map.get(id).kind == kind {
            Some(id)
        } else {
            None
        }
    }
    fn resolve_enum_variant(
        &self,
        q: &ori_ast::common::QualifiedName,
    ) -> Option<(ori_types::DefId, SmolStr)> {
        let enum_path = qualified_prefix(q)?;
        let id = self.resolve_def_id_with_kind(&enum_path, DefKind::Enum)?;
        Some((id, q.last().text.clone()))
    }
    fn ty_for_def_path(&self, path: &str) -> Ty {
        if let Some(id) = self.def_map.lookup(path) {
            match self.def_map.get(id).kind {
                DefKind::Struct | DefKind::Enum | DefKind::TypeAlias => Ty::Named(id, Vec::new()),
                DefKind::Func | DefKind::Extern => {
                    if let Some(sig) = self.func_sigs.iter().find(|sig| sig.def_id == id) {
                        Ty::Func {
                            params: sig.params.clone(),
                            ret: Box::new(sig.return_ty.clone()),
                        }
                    } else {
                        Ty::Infer(0)
                    }
                }
                _ => Ty::Infer(0),
            }
        } else {
            Ty::Error
        }
    }
    fn trait_method_return_ty(&self, trait_def_id: ori_types::DefId, method: &str) -> Option<Ty> {
        self.trait_sigs
            .iter()
            .find(|sig| sig.def_id == trait_def_id)?
            .methods
            .iter()
            .find(|sig| sig.name == method)
            .map(|sig| sig.return_ty.clone())
    }
    fn trait_method_func_for_type(
        &self,
        type_def_id: ori_types::DefId,
        method: &str,
    ) -> Option<(SmolStr, Ty)> {
        let mut matches = Vec::new();
        for impl_sig in self
            .impl_sigs
            .iter()
            .filter(|sig| sig.type_def_id == type_def_id)
        {
            let Some(trait_sig) = self
                .trait_sigs
                .iter()
                .find(|sig| sig.def_id == impl_sig.trait_def_id)
            else {
                continue;
            };
            let Some(method_sig) = trait_sig.methods.iter().find(|sig| sig.name == method) else {
                continue;
            };
            if let Some(impl_method) = impl_sig.methods.iter().find(|sig| sig.name == method) {
                matches.push((
                    self.def_map.get(impl_method.func_def_id).path.clone(),
                    method_sig.return_ty.clone(),
                ));
            } else if method_sig.has_default {
                let trait_path = self.def_map.get(trait_sig.def_id).path.clone();
                matches.push((
                    SmolStr::new(format!("{}.{}", trait_path, method_sig.name)),
                    method_sig.return_ty.clone(),
                ));
            }
        }
        (matches.len() == 1).then(|| matches.remove(0))
    }
    fn lower_ast_ty(&mut self, t: &ori_ast::ty::Type, tp: &[SmolStr]) -> Ty {
        let raw = lower_type_with_aliases(
            t,
            self.namespace,
            tp,
            self.def_map,
            self.file_id,
            self.sink,
            &self.aliases,
        );
        expand_ty_aliases(raw, self.def_map, &self.type_alias_map)
    }
    fn lower_named_args(
        &mut self,
        args: &[ori_ast::expr::Arg],
        tp: &[SmolStr],
    ) -> Vec<(SmolStr, HirExpr)> {
        args.iter()
            .filter_map(|arg| {
                let label = arg.label.as_ref()?;
                let value = match &arg.value {
                    ori_ast::expr::ArgValue::Expr(e) | ori_ast::expr::ArgValue::Spread(e) => e,
                };
                Some((label.text.clone(), self.lower_expr(value, tp)))
            })
            .collect()
    }
    fn lower_call_arg(&mut self, arg: &Arg, tp: &[SmolStr]) -> HirArg {
        let (spread, value) = match &arg.value {
            ori_ast::expr::ArgValue::Expr(e) => (false, self.lower_expr(e, tp)),
            ori_ast::expr::ArgValue::Spread(e) => (true, self.lower_expr(e, tp)),
        };
        HirArg {
            label: arg.label.as_ref().map(|label| label.text.clone()),
            spread,
            value,
        }
    }
    fn lower_call_args(&mut self, args: &[Arg], tp: &[SmolStr]) -> Vec<HirArg> {
        args.iter()
            .map(|arg| self.lower_call_arg(arg, tp))
            .collect()
    }
    fn lower_local_field_path(
        &mut self,
        q: &ori_ast::common::QualifiedName,
        _tp: &[SmolStr],
    ) -> Option<HirExpr> {
        let first = q.parts.first()?;
        let mut ty = self.lookup_var(&first.text)?;
        let mut expr = HirExpr {
            kind: HirExprKind::Var(first.text.clone()),
            ty: ty.clone(),
            span: first.span,
        };
        for field in q.parts.iter().skip(1) {
            let field_ty = match &ty {
                Ty::Named(_, _) => Ty::Infer(0),
                Ty::Tuple(elems) => field
                    .text
                    .parse::<usize>()
                    .ok()
                    .and_then(|idx| elems.get(idx).cloned())
                    .unwrap_or(Ty::Infer(0)),
                _ => Ty::Infer(0),
            };
            expr = HirExpr {
                kind: HirExprKind::Field {
                    object: Box::new(expr),
                    field: field.text.clone(),
                },
                ty: field_ty.clone(),
                span: first.span.cover(field.span),
            };
            ty = field_ty;
        }
        Some(expr)
    }
    fn lower_param(&mut self, p: &ori_ast::item::Param, tp: &[SmolStr]) -> HirParam {
        use ori_ast::item::ParamKind;
        let raw_ty = self.lower_ast_ty(&p.ty, tp);
        let variadic = matches!(p.kind, ParamKind::Variadic);
        let ty = if variadic {
            Ty::List(Box::new(raw_ty.clone()))
        } else {
            raw_ty
        };
        let default = match &p.kind {
            ParamKind::Default(expr) | ParamKind::DefaultAndContract(expr, _) => {
                Some(self.lower_expr(expr, tp))
            }
            ParamKind::Required | ParamKind::Variadic | ParamKind::Contract(_) => None,
        };
        let contract = match &p.kind {
            ParamKind::Contract(expr) | ParamKind::DefaultAndContract(_, expr) => {
                Some(self.lower_param_contract(&p.name.text, &ty, expr, tp))
            }
            ParamKind::Required | ParamKind::Variadic | ParamKind::Default(_) => None,
        };
        HirParam {
            name: p.name.text.clone(),
            ty,
            default,
            contract,
            variadic,
            span: p.span,
        }
    }

    fn lower_param_contract(
        &mut self,
        name: &SmolStr,
        ty: &Ty,
        expr: &Expr,
        tp: &[SmolStr],
    ) -> HirExpr {
        self.push();
        self.bind(SmolStr::new("it"), ty.clone());
        self.bind(name.clone(), ty.clone());
        let contract = self.lower_expr(expr, tp);
        self.pop();
        contract
    }

    fn lower_field_contract(&mut self, ty: &Ty, expr: &Expr, tp: &[SmolStr]) -> HirExpr {
        self.push();
        self.bind(SmolStr::new("it"), ty.clone());
        let contract = self.lower_expr(expr, tp);
        self.pop();
        contract
    }

    fn lower_params(&mut self, params: &[ori_ast::item::Param], tp: &[SmolStr]) -> Vec<HirParam> {
        self.push();
        let lowered: Vec<HirParam> = params
            .iter()
            .map(|p| {
                let param = self.lower_param(p, tp);
                self.bind(param.name.clone(), param.ty.clone());
                param
            })
            .collect();
        self.pop();
        lowered
    }

    fn err_expr(span: Span) -> HirExpr {
        HirExpr {
            kind: HirExprKind::Unit,
            ty: Ty::Error,
            span,
        }
    }
}

// ── Public entry ──────────────────────────────────────────────────────────────

fn lower_trait_sigs(def_map: &DefMap, trait_sigs: &[TraitSig]) -> Vec<HirTrait> {
    trait_sigs
        .iter()
        .map(|sig| HirTrait {
            def_id: sig.def_id,
            name: def_map.get(sig.def_id).path.clone(),
            methods: sig
                .methods
                .iter()
                .map(|method| HirTraitMethod {
                    name: method.name.clone(),
                    params: method.params.clone(),
                    return_ty: method.return_ty.clone(),
                    default_func_name: method.has_default.then(|| {
                        SmolStr::new(format!("{}.{}", def_map.get(sig.def_id).path, method.name))
                    }),
                })
                .collect(),
        })
        .collect()
}

fn lower_impl_sigs(def_map: &DefMap, impl_sigs: &[ImplSig]) -> Vec<HirTraitImpl> {
    impl_sigs
        .iter()
        .map(|sig| HirTraitImpl {
            trait_def_id: sig.trait_def_id,
            type_def_id: sig.type_def_id,
            methods: sig
                .methods
                .iter()
                .map(|method| HirTraitImplMethod {
                    name: method.name.clone(),
                    func_name: def_map.get(method.func_def_id).path.clone(),
                })
                .collect(),
        })
        .collect()
}

pub fn lower(
    file: &SourceFile,
    def_map: &DefMap,
    func_sigs: &[FuncSig],
    trait_sigs: &[TraitSig],
    impl_sigs: &[ImplSig],
    type_alias_sigs: &[TypeAliasSig],
    reexports: &[ReExport],
    namespace: &str,
    file_id: FileId,
    sink: &mut DiagnosticSink,
) -> HirModule {
    let mut l = Lowerer::new(
        def_map,
        func_sigs,
        trait_sigs,
        impl_sigs,
        type_alias_sigs,
        namespace,
        file_id,
        sink,
    );

    // Build alias map from imports: `import ori.io as io` → `io → ori.io`
    l.aliases = ori_types::resolve::import_aliases(file, reexports);
    let mut structs = Vec::new();
    let mut enums = Vec::new();
    let traits = lower_trait_sigs(def_map, trait_sigs);
    let trait_impls = lower_impl_sigs(def_map, impl_sigs);
    let mut funcs = Vec::new();
    let mut consts = Vec::new();
    let mut externs = Vec::new();

    for item in &file.items {
        match &item.item {
            Item::Struct(s) => {
                let tp: Vec<SmolStr> = s.type_params.iter().map(|p| p.name.text.clone()).collect();
                let fields = s
                    .fields
                    .iter()
                    .map(|f| {
                        let ty = l.lower_ast_ty(&f.ty, &tp);
                        let contract = f
                            .contract
                            .as_ref()
                            .map(|expr| l.lower_field_contract(&ty, expr, &tp));
                        HirField {
                            name: f.name.text.clone(),
                            ty,
                            contract,
                            span: f.span,
                        }
                    })
                    .collect();
                let path = format!("{}.{}", namespace, s.name.text);
                let def_id = def_map.lookup(&path).unwrap_or(ori_types::DefId(u32::MAX));
                structs.push(HirStruct {
                    def_id,
                    name: SmolStr::new(&path),
                    fields,
                    is_public: s.visibility == Visibility::Public,
                    span: s.span,
                });

                for m in &s.methods {
                    let mut all_tp = tp.clone();
                    all_tp.extend(m.type_params.iter().map(|p| p.name.text.clone()));
                    l.aliases.insert(SmolStr::new("Self"), s.name.text.clone());
                    let params = l.lower_params(&m.params, &all_tp);
                    let return_ty = m
                        .return_ty
                        .as_ref()
                        .map(|t| l.lower_ast_ty(t, &all_tp))
                        .unwrap_or(Ty::Void);
                    l.aliases.remove("Self");
                    l.push();
                    for p in &params {
                        l.bind(p.name.clone(), p.ty.clone());
                    }
                    l.ret_ty = return_ty.clone();
                    let body = l.lower_block(&m.body, &all_tp);
                    l.pop();
                    let path = format!("{}.{}.{}", namespace, s.name.text, m.name.text);
                    let def_id = def_map.lookup(&path).unwrap_or(ori_types::DefId(u32::MAX));
                    funcs.push(HirFunc {
                        def_id,
                        name: SmolStr::new(&path),
                        params,
                        return_ty,
                        body,
                        closure_captures: Vec::new(),
                        is_public: m.visibility == Visibility::Public,
                        is_mut: m.is_mut,
                        span: m.span,
                    });
                }
            }
            Item::Enum(e) => {
                let path = format!("{}.{}", namespace, e.name.text);
                let def_id = def_map.lookup(&path).unwrap_or(ori_types::DefId(u32::MAX));
                let tp: Vec<SmolStr> = e.type_params.iter().map(|p| p.name.text.clone()).collect();
                let variants = e
                    .variants
                    .iter()
                    .map(|v| HirVariant {
                        name: v.name.text.clone(),
                        fields: v
                            .fields
                            .iter()
                            .map(|f| HirField {
                                name: f.name.text.clone(),
                                ty: l.lower_ast_ty(&f.ty, &tp),
                                contract: None,
                                span: f.span,
                            })
                            .collect(),
                        span: v.span,
                    })
                    .collect();
                enums.push(HirEnum {
                    def_id,
                    name: SmolStr::new(&path),
                    variants,
                    is_public: e.visibility == Visibility::Public,
                    span: e.span,
                });
            }
            Item::Implement(i) => {
                let type_name = i.for_type.last().text.clone();
                let tp: Vec<SmolStr> = i.type_params.iter().map(|p| p.name.text.clone()).collect();
                for m in &i.methods {
                    let mut all_tp = tp.clone();
                    all_tp.extend(m.type_params.iter().map(|p| p.name.text.clone()));
                    l.aliases.insert(SmolStr::new("Self"), type_name.clone());
                    let params = l.lower_params(&m.params, &all_tp);
                    let return_ty = m
                        .return_ty
                        .as_ref()
                        .map(|t| l.lower_ast_ty(t, &all_tp))
                        .unwrap_or(Ty::Void);
                    l.aliases.remove("Self");
                    l.push();
                    for p in &params {
                        l.bind(p.name.clone(), p.ty.clone());
                    }
                    l.ret_ty = return_ty.clone();
                    let body = l.lower_block(&m.body, &all_tp);
                    l.pop();
                    let path = format!("{}.{}.{}", namespace, type_name, m.name.text);
                    let def_id = def_map.lookup(&path).unwrap_or(ori_types::DefId(u32::MAX));
                    funcs.push(HirFunc {
                        def_id,
                        name: SmolStr::new(&path),
                        params,
                        return_ty,
                        body,
                        closure_captures: Vec::new(),
                        is_public: m.visibility == Visibility::Public,
                        is_mut: m.is_mut,
                        span: m.span,
                    });
                }
            }
            Item::Trait(t) => {
                let tp: Vec<SmolStr> = t.type_params.iter().map(|p| p.name.text.clone()).collect();
                for m in &t.members {
                    if let ori_ast::item::TraitMember::Default(func) = m {
                        let mut all_tp = tp.clone();
                        all_tp.extend(func.type_params.iter().map(|p| p.name.text.clone()));
                        l.aliases.insert(SmolStr::new("Self"), t.name.text.clone());
                        let params = l.lower_params(&func.params, &all_tp);
                        let return_ty = func
                            .return_ty
                            .as_ref()
                            .map(|ty| l.lower_ast_ty(ty, &all_tp))
                            .unwrap_or(Ty::Void);
                        l.aliases.remove("Self");
                        l.push();
                        for p in &params {
                            l.bind(p.name.clone(), p.ty.clone());
                        }
                        l.ret_ty = return_ty.clone();
                        let body = l.lower_block(&func.body, &all_tp);
                        l.pop();
                        let path = format!("{}.{}.{}", namespace, t.name.text, func.name.text);
                        let def_id = def_map.lookup(&path).unwrap_or(ori_types::DefId(u32::MAX));
                        funcs.push(HirFunc {
                            def_id,
                            name: SmolStr::new(&path),
                            params,
                            return_ty,
                            body,
                            closure_captures: Vec::new(),
                            is_public: func.visibility == Visibility::Public,
                            is_mut: func.is_mut,
                            span: func.span,
                        });
                    }
                }
            }
            Item::Func(f) => {
                let tp: Vec<SmolStr> = f.type_params.iter().map(|p| p.name.text.clone()).collect();
                let params = l.lower_params(&f.params, &tp);
                let return_ty = f
                    .return_ty
                    .as_ref()
                    .map(|t| l.lower_ast_ty(t, &tp))
                    .unwrap_or(Ty::Void);
                l.push();
                for p in &params {
                    l.bind(p.name.clone(), p.ty.clone());
                }
                l.ret_ty = return_ty.clone();
                let body = l.lower_block(&f.body, &tp);
                l.pop();
                let path = format!("{}.{}", namespace, f.name.text);
                let def_id = def_map.lookup(&path).unwrap_or(ori_types::DefId(u32::MAX));
                funcs.push(HirFunc {
                    def_id,
                    name: SmolStr::new(&path),
                    params,
                    return_ty,
                    body,
                    closure_captures: Vec::new(),
                    is_public: f.visibility == Visibility::Public,
                    is_mut: f.is_mut,
                    span: f.span,
                });
            }
            Item::Const(c) => {
                let ty = l.lower_ast_ty(&c.ty, &[]);
                let mut value = l.lower_expr(&c.value, &[]);
                apply_expected_expr_ty(&mut value, &ty);
                let path = format!("{}.{}", namespace, c.name.text);
                let def_id = def_map.lookup(&path).unwrap_or(ori_types::DefId(u32::MAX));
                consts.push(HirConst {
                    def_id,
                    name: SmolStr::new(&path),
                    ty,
                    value,
                    mutable: false,
                    is_public: c.visibility == Visibility::Public,
                    span: c.span,
                });
            }
            Item::Var(v) => {
                let ty = l.lower_ast_ty(&v.ty, &[]);
                let mut value = l.lower_expr(&v.value, &[]);
                apply_expected_expr_ty(&mut value, &ty);
                let path = format!("{}.{}", namespace, v.name.text);
                let def_id = def_map.lookup(&path).unwrap_or(ori_types::DefId(u32::MAX));
                consts.push(HirConst {
                    def_id,
                    name: SmolStr::new(&path),
                    ty,
                    value,
                    mutable: true,
                    is_public: v.visibility == Visibility::Public,
                    span: v.span,
                });
            }
            Item::Extern(ext) => {
                let abi = match ext.abi {
                    ori_ast::item::AbiLabel::C => SmolStr::new("C"),
                    ori_ast::item::AbiLabel::Host => SmolStr::new("host"),
                };
                for member in &ext.members {
                    match member {
                        ori_ast::item::ExternMember::Func {
                            name,
                            params,
                            return_ty,
                            span,
                            ..
                        } => {
                            let hir_params = l.lower_params(params, &[]);
                            let ret = return_ty
                                .as_ref()
                                .map(|t| l.lower_ast_ty(t, &[]))
                                .unwrap_or(Ty::Void);
                            externs.push(HirExtern::Func {
                                name: name.text.clone(),
                                params: hir_params,
                                return_ty: ret,
                                abi: abi.clone(),
                                span: *span,
                            });
                        }
                        ori_ast::item::ExternMember::Var { name, ty, span, .. } => {
                            externs.push(HirExtern::Var {
                                name: name.text.clone(),
                                ty: l.lower_ast_ty(ty, &[]),
                                abi: abi.clone(),
                                span: *span,
                            });
                        }
                    }
                }
            }
            _ => {}
        }
    }

    funcs.extend(std::mem::take(&mut l.generated_funcs));

    HirModule {
        namespace: SmolStr::new(namespace),
        structs,
        enums,
        traits,
        trait_impls,
        funcs,
        consts,
        externs,
    }
}

// ── Statement lowering ────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct DefaultParam {
    name: SmolStr,
    ty: Ty,
    default: Option<HirExpr>,
    variadic: bool,
}

pub fn insert_default_arguments(module: &mut HirModule) {
    let defaults: HashMap<SmolStr, Vec<DefaultParam>> = module
        .funcs
        .iter()
        .map(|func| {
            (
                func.name.clone(),
                func.params
                    .iter()
                    .map(|param| DefaultParam {
                        name: param.name.clone(),
                        ty: param.ty.clone(),
                        default: param.default.clone(),
                        variadic: param.variadic,
                    })
                    .collect(),
            )
        })
        .collect();

    if defaults.is_empty() {
        return;
    }

    for func in &mut module.funcs {
        for param in &mut func.params {
            if let Some(default) = &mut param.default {
                insert_default_arguments_expr(default, &defaults);
            }
        }
        insert_default_arguments_block(&mut func.body, &defaults);
    }
    for konst in &mut module.consts {
        insert_default_arguments_expr(&mut konst.value, &defaults);
    }
}

fn insert_default_arguments_block(
    block: &mut HirBlock,
    defaults: &HashMap<SmolStr, Vec<DefaultParam>>,
) {
    for stmt in &mut block.stmts {
        insert_default_arguments_stmt(stmt, defaults);
    }
}

fn insert_default_arguments_stmt(
    stmt: &mut HirStmt,
    defaults: &HashMap<SmolStr, Vec<DefaultParam>>,
) {
    match stmt {
        HirStmt::Let { value, .. } | HirStmt::Using { value, .. } => {
            insert_default_arguments_expr(value, defaults);
        }
        HirStmt::Assign { lvalue, value, .. } => {
            insert_default_arguments_lvalue(lvalue, defaults);
            insert_default_arguments_expr(value, defaults);
        }
        HirStmt::Return(Some(value), _) | HirStmt::Expr(value) => {
            insert_default_arguments_expr(value, defaults);
        }
        HirStmt::Return(None, _) | HirStmt::Break(_) | HirStmt::Continue(_) => {}
        HirStmt::If {
            cond,
            then,
            else_ifs,
            else_,
            ..
        } => {
            insert_default_arguments_expr(cond, defaults);
            insert_default_arguments_block(then, defaults);
            for (cond, block) in else_ifs {
                insert_default_arguments_expr(cond, defaults);
                insert_default_arguments_block(block, defaults);
            }
            if let Some(block) = else_ {
                insert_default_arguments_block(block, defaults);
            }
        }
        HirStmt::While { cond, body, .. } => {
            insert_default_arguments_expr(cond, defaults);
            insert_default_arguments_block(body, defaults);
        }
        HirStmt::Loop { body, .. } => insert_default_arguments_block(body, defaults),
        HirStmt::For { iterable, body, .. } => {
            insert_default_arguments_expr(iterable, defaults);
            insert_default_arguments_block(body, defaults);
        }
        HirStmt::Repeat { count, body, .. } => {
            insert_default_arguments_expr(count, defaults);
            insert_default_arguments_block(body, defaults);
        }
        HirStmt::Match {
            scrutinee, arms, ..
        } => {
            insert_default_arguments_expr(scrutinee, defaults);
            for arm in arms {
                for stmt in &mut arm.body {
                    insert_default_arguments_stmt(stmt, defaults);
                }
            }
        }
        HirStmt::IfSome {
            value, then, else_, ..
        } => {
            insert_default_arguments_expr(value, defaults);
            insert_default_arguments_block(then, defaults);
            if let Some(block) = else_ {
                insert_default_arguments_block(block, defaults);
            }
        }
        HirStmt::WhileSome { value, body, .. } => {
            insert_default_arguments_expr(value, defaults);
            insert_default_arguments_block(body, defaults);
        }
        HirStmt::Check { condition, .. } => {
            insert_default_arguments_expr(condition, defaults);
        }
    }
}

fn insert_default_arguments_lvalue(
    lvalue: &mut HirLValue,
    defaults: &HashMap<SmolStr, Vec<DefaultParam>>,
) {
    match lvalue {
        HirLValue::Var(_) => {}
        HirLValue::Field { base, .. } => insert_default_arguments_lvalue(base, defaults),
        HirLValue::Index { base, index } => {
            insert_default_arguments_lvalue(base, defaults);
            insert_default_arguments_expr(index, defaults);
        }
    }
}

fn normalize_call_arguments(args: &mut Vec<HirArg>, params: &[DefaultParam]) {
    if let Some(variadic_index) = params.iter().position(|param| param.variadic) {
        normalize_variadic_call_arguments(args, params, variadic_index);
        return;
    }

    let original = std::mem::take(args);
    let mut slots: Vec<Option<HirExpr>> = vec![None; params.len()];
    let mut extras = Vec::new();
    let mut next_positional = 0usize;

    for arg in original {
        if let Some(label) = &arg.label {
            if let Some(index) = params.iter().position(|param| &param.name == label) {
                if slots[index].is_none() {
                    slots[index] = Some(arg.value);
                } else {
                    extras.push(arg.value);
                }
            } else {
                extras.push(arg.value);
            }
        } else {
            while next_positional < slots.len() && slots[next_positional].is_some() {
                next_positional += 1;
            }
            if next_positional < slots.len() {
                slots[next_positional] = Some(arg.value);
                next_positional += 1;
            } else {
                extras.push(arg.value);
            }
        }
    }

    let mut stopped_at_required = false;
    for (index, param) in params.iter().enumerate() {
        if let Some(value) = slots[index].take() {
            args.push(HirArg {
                label: None,
                spread: false,
                value,
            });
        } else if let Some(default) = &param.default {
            args.push(HirArg {
                label: None,
                spread: false,
                value: default.clone(),
            });
        } else {
            stopped_at_required = true;
            break;
        }
    }

    if stopped_at_required {
        for value in slots.into_iter().flatten() {
            args.push(HirArg {
                label: None,
                spread: false,
                value,
            });
        }
    }
    args.extend(extras.into_iter().map(|value| HirArg {
        label: None,
        spread: false,
        value,
    }));
}

fn normalize_variadic_call_arguments(
    args: &mut Vec<HirArg>,
    params: &[DefaultParam],
    variadic_index: usize,
) {
    let original = std::mem::take(args);
    let mut slots: Vec<Option<HirArg>> = vec![None; variadic_index];
    let mut varargs = Vec::new();
    let mut next_positional = 0usize;

    for arg in original {
        if let Some(label) = &arg.label {
            if let Some(index) = params.iter().position(|param| &param.name == label) {
                if index < variadic_index {
                    if slots[index].is_none() {
                        slots[index] = Some(arg);
                    } else {
                        varargs.push(HirListElement {
                            spread: arg.spread,
                            value: arg.value,
                        });
                    }
                } else if index == variadic_index {
                    varargs.push(HirListElement {
                        spread: arg.spread,
                        value: arg.value,
                    });
                }
            } else {
                varargs.push(HirListElement {
                    spread: arg.spread,
                    value: arg.value,
                });
            }
        } else {
            while next_positional < slots.len() && slots[next_positional].is_some() {
                next_positional += 1;
            }
            if next_positional < slots.len() {
                slots[next_positional] = Some(arg);
                next_positional += 1;
            } else {
                varargs.push(HirListElement {
                    spread: arg.spread,
                    value: arg.value,
                });
            }
        }
    }

    let mut stopped_at_required = false;
    for (index, param) in params.iter().take(variadic_index).enumerate() {
        if let Some(arg) = slots[index].take() {
            args.push(HirArg {
                label: None,
                spread: false,
                value: arg.value,
            });
        } else if let Some(default) = &param.default {
            args.push(HirArg {
                label: None,
                spread: false,
                value: default.clone(),
            });
        } else {
            stopped_at_required = true;
            break;
        }
    }

    if stopped_at_required {
        for arg in slots.into_iter().flatten() {
            args.push(HirArg {
                label: None,
                spread: false,
                value: arg.value,
            });
        }
        args.extend(varargs.into_iter().map(|element| HirArg {
            label: None,
            spread: element.spread,
            value: element.value,
        }));
        return;
    }

    let list_ty = params
        .get(variadic_index)
        .map(|param| param.ty.clone())
        .unwrap_or_else(|| Ty::List(Box::new(Ty::Infer(0))));
    let elem_ty = match &list_ty {
        Ty::List(elem) => *elem.clone(),
        other => other.clone(),
    };
    let span = varargs
        .iter()
        .map(|element| element.value.span)
        .reduce(|acc, span| acc.cover(span))
        .unwrap_or(Span::DUMMY);
    let has_spread = varargs.iter().any(|element| element.spread);
    let kind = if has_spread {
        HirExprKind::ListSpreadLit {
            elem_ty: elem_ty.clone(),
            elements: varargs,
        }
    } else {
        HirExprKind::ListLit {
            elem_ty: elem_ty.clone(),
            elements: varargs.into_iter().map(|element| element.value).collect(),
        }
    };
    args.push(HirArg {
        label: None,
        spread: false,
        value: HirExpr {
            kind,
            ty: list_ty,
            span,
        },
    });
}

fn insert_default_arguments_expr(
    expr: &mut HirExpr,
    defaults: &HashMap<SmolStr, Vec<DefaultParam>>,
) {
    match &mut expr.kind {
        HirExprKind::Binary { lhs, rhs, .. } => {
            insert_default_arguments_expr(lhs, defaults);
            insert_default_arguments_expr(rhs, defaults);
        }
        HirExprKind::Unary { operand, .. } => insert_default_arguments_expr(operand, defaults),
        HirExprKind::Field { object, .. } | HirExprKind::TupleIndex { object, .. } => {
            insert_default_arguments_expr(object, defaults);
        }
        HirExprKind::Index { object, index } => {
            insert_default_arguments_expr(object, defaults);
            insert_default_arguments_expr(index, defaults);
        }
        HirExprKind::Call { callee, args } => {
            insert_default_arguments_expr(callee, defaults);
            for arg in args.iter_mut() {
                insert_default_arguments_expr(&mut arg.value, defaults);
            }
            if let HirExprKind::Var(name) = &callee.kind {
                if let Some(param_defaults) = defaults.get(name) {
                    normalize_call_arguments(args, param_defaults);
                }
            }
        }
        HirExprKind::MethodCall { receiver, args, .. } => {
            insert_default_arguments_expr(receiver, defaults);
            for arg in args {
                insert_default_arguments_expr(arg, defaults);
            }
        }
        HirExprKind::StructLit { fields, .. } | HirExprKind::EnumVariant { fields, .. } => {
            for (_, value) in fields {
                insert_default_arguments_expr(value, defaults);
            }
        }
        HirExprKind::ListLit { elements, .. }
        | HirExprKind::SetLit { elements, .. }
        | HirExprKind::TupleLit(elements) => {
            for elem in elements {
                insert_default_arguments_expr(elem, defaults);
            }
        }
        HirExprKind::ListSpreadLit { elements, .. } => {
            for elem in elements {
                insert_default_arguments_expr(&mut elem.value, defaults);
            }
        }
        HirExprKind::Some_(inner)
        | HirExprKind::Ok_(inner)
        | HirExprKind::Err_(inner)
        | HirExprKind::Propagate(inner) => insert_default_arguments_expr(inner, defaults),
        HirExprKind::IfExpr { cond, then, else_ } => {
            insert_default_arguments_expr(cond, defaults);
            insert_default_arguments_expr(then, defaults);
            insert_default_arguments_expr(else_, defaults);
        }
        HirExprKind::Range { start, end } => {
            insert_default_arguments_expr(start, defaults);
            insert_default_arguments_expr(end, defaults);
        }
        HirExprKind::MapLit { entries, .. } => {
            for (key, value) in entries {
                insert_default_arguments_expr(key, defaults);
                insert_default_arguments_expr(value, defaults);
            }
        }
        HirExprKind::StructUpdate { base, updates, .. } => {
            insert_default_arguments_expr(base, defaults);
            for (_, value) in updates {
                insert_default_arguments_expr(value, defaults);
            }
        }
        HirExprKind::InterpolatedStr(parts) => {
            for part in parts {
                if let HirStrPart::Expr(value) = part {
                    insert_default_arguments_expr(value, defaults);
                }
            }
        }
        HirExprKind::BoolLit(_)
        | HirExprKind::IntLit(_)
        | HirExprKind::FloatLit(_)
        | HirExprKind::StrLit(_)
        | HirExprKind::BytesLit(_)
        | HirExprKind::Unit
        | HirExprKind::Var(_)
        | HirExprKind::GlobalConst(_)
        | HirExprKind::None_
        | HirExprKind::Closure { .. } => {}
        HirExprKind::IsCheck { value, .. } => insert_default_arguments_expr(value, defaults),
    }
}

impl<'a> Lowerer<'a> {
    fn lower_block(&mut self, block: &Block, tp: &[SmolStr]) -> HirBlock {
        self.push();
        let stmts = block
            .stmts
            .iter()
            .filter_map(|s| self.lower_stmt(s, tp))
            .collect();
        self.pop();
        HirBlock {
            stmts,
            span: block.span,
        }
    }

    fn lower_stmt(&mut self, stmt: &Stmt, tp: &[SmolStr]) -> Option<HirStmt> {
        match stmt {
            Stmt::Const(c) => {
                let ty = self.lower_ast_ty(&c.ty, tp);
                let mut val = self.lower_expr(&c.value, tp);
                apply_expected_expr_ty(&mut val, &ty);
                self.bind(c.name.text.clone(), ty.clone());
                Some(HirStmt::Let {
                    name: c.name.text.clone(),
                    ty,
                    mutable: false,
                    value: val,
                    span: c.span,
                })
            }
            Stmt::Var(v) => {
                let ty = self.lower_ast_ty(&v.ty, tp);
                let mut val = self.lower_expr(&v.value, tp);
                apply_expected_expr_ty(&mut val, &ty);
                self.bind(v.name.text.clone(), ty.clone());
                Some(HirStmt::Let {
                    name: v.name.text.clone(),
                    ty,
                    mutable: true,
                    value: val,
                    span: v.span,
                })
            }
            Stmt::Return(r) => {
                let val = r.value.as_ref().map(|e| {
                    let mut value = self.lower_expr(e, tp);
                    apply_expected_expr_ty(&mut value, &self.ret_ty);
                    value
                });
                Some(HirStmt::Return(val, r.span))
            }
            Stmt::Break(sp) => Some(HirStmt::Break(*sp)),
            Stmt::Continue(sp) => Some(HirStmt::Continue(*sp)),
            Stmt::Expr(e) => Some(HirStmt::Expr(self.lower_expr(e, tp))),
            Stmt::If(i) => {
                let cond = self.lower_expr(&i.condition, tp);
                let then = self.lower_block(&i.then_block, tp);
                let else_ifs = i
                    .else_ifs
                    .iter()
                    .map(|(c, b)| (self.lower_expr(c, tp), self.lower_block(b, tp)))
                    .collect();
                let else_ = i.else_block.as_ref().map(|b| self.lower_block(b, tp));
                Some(HirStmt::If {
                    cond,
                    then,
                    else_ifs,
                    else_,
                    span: i.span,
                })
            }
            Stmt::While(w) => {
                let cond = self.lower_expr(&w.condition, tp);
                let body = self.lower_block(&w.body, tp);
                Some(HirStmt::While {
                    cond,
                    body,
                    span: w.span,
                })
            }
            Stmt::For(f) => {
                let iterable = self.lower_expr(&f.iterable, tp);
                let elem_ty = elem_type(&iterable.ty);
                let second_ty = for_second_binding_ty(&iterable.ty);
                self.push();
                self.bind(f.binding.text.clone(), elem_ty.clone());
                if let Some(ref sb) = f.second_binding {
                    self.bind(sb.text.clone(), second_ty);
                }
                let body = self.lower_block(&f.body, tp);
                self.pop();
                Some(HirStmt::For {
                    binding: f.binding.text.clone(),
                    index_binding: f.second_binding.as_ref().map(|n| n.text.clone()),
                    elem_ty,
                    iterable,
                    body,
                    span: f.span,
                })
            }
            Stmt::Loop(l) => {
                let body = self.lower_block(&l.body, tp);
                Some(HirStmt::Loop { body, span: l.span })
            }
            Stmt::Repeat(r) => {
                let count = self.lower_expr(&r.count, tp);
                let body = self.lower_block(&r.body, tp);
                Some(HirStmt::Repeat {
                    count,
                    body,
                    span: r.span,
                })
            }
            Stmt::Match(m) => {
                let scrutinee = self.lower_expr(&m.scrutinee, tp);
                let arms = m
                    .cases
                    .iter()
                    .map(|c| self.lower_match_case(c, tp, &scrutinee.ty))
                    .collect();
                Some(HirStmt::Match {
                    scrutinee,
                    arms,
                    span: m.span,
                })
            }
            Stmt::Assign(a) => {
                let lvalue = lower_lvalue(&a.lvalue, self, tp);
                let value = self.lower_expr(&a.value, tp);
                Some(HirStmt::Assign {
                    lvalue,
                    value,
                    span: a.span,
                })
            }
            Stmt::CompoundAssign(c) => {
                // Lower `x += v` to `x = x + v` for v1
                let lvalue = lower_lvalue(&c.lvalue, self, tp);
                let cur = lvalue_to_expr(&lvalue, c.span);
                let rhs = self.lower_expr(&c.value, tp);
                let op = compound_op_to_binary(c.op);
                let ty = binary_result_ty(op, &cur.ty, &rhs.ty);
                let value = HirExpr {
                    kind: HirExprKind::Binary {
                        op,
                        lhs: Box::new(cur),
                        rhs: Box::new(rhs),
                    },
                    ty,
                    span: c.span,
                };
                Some(HirStmt::Assign {
                    lvalue,
                    value,
                    span: c.span,
                })
            }
            Stmt::IfSome(s) => {
                let value = self.lower_expr(&s.value, tp);
                let inner_ty = unwrap_ty(&value.ty);
                self.push();
                self.bind(s.binding.text.clone(), inner_ty.clone());
                let then = self.lower_block(&s.then_block, tp);
                self.pop();
                let else_ = s.else_block.as_ref().map(|b| self.lower_block(b, tp));
                Some(HirStmt::IfSome {
                    binding: s.binding.text.clone(),
                    inner_ty,
                    value,
                    then,
                    else_,
                    span: s.span,
                })
            }
            Stmt::WhileSome(s) => {
                let value = self.lower_expr(&s.value, tp);
                let inner_ty = unwrap_ty(&value.ty);
                self.push();
                self.bind(s.binding.text.clone(), inner_ty.clone());
                let body = self.lower_block(&s.body, tp);
                self.pop();
                Some(HirStmt::WhileSome {
                    binding: s.binding.text.clone(),
                    inner_ty,
                    value,
                    body,
                    span: s.span,
                })
            }
            Stmt::Using(u) => {
                let ty = self.lower_ast_ty(&u.ty, tp);
                let val = self.lower_expr(&u.value, tp);
                self.bind(u.name.text.clone(), ty.clone());
                Some(HirStmt::Using {
                    name: u.name.text.clone(),
                    ty,
                    value: val,
                    span: u.span,
                })
            }
            Stmt::Check(c) => {
                let cond = self.lower_expr(&c.condition, tp);
                Some(HirStmt::Check {
                    condition: cond,
                    message: c.message.clone(),
                    span: c.span,
                })
            }
        }
    }

    fn lower_match_case(&mut self, case: &MatchCase, tp: &[SmolStr], scr_ty: &Ty) -> HirArm {
        match case {
            MatchCase::Pattern {
                pattern,
                body,
                span,
                ..
            } => {
                let pat = lower_pattern(pattern, scr_ty);
                self.push();
                bind_hir_pattern_scope(self, &pat);
                let stmts = body.iter().filter_map(|s| self.lower_stmt(s, tp)).collect();
                self.pop();
                HirArm {
                    pattern: pat,
                    body: stmts,
                    span: *span,
                }
            }
            MatchCase::Else { body, span } => {
                self.push();
                let stmts = body.iter().filter_map(|s| self.lower_stmt(s, tp)).collect();
                self.pop();
                HirArm {
                    pattern: HirPattern::Wildcard,
                    body: stmts,
                    span: *span,
                }
            }
        }
    }
}

// ── Expression lowering ───────────────────────────────────────────────────────

impl<'a> Lowerer<'a> {
    pub fn lower_expr(&mut self, expr: &Expr, tp: &[SmolStr]) -> HirExpr {
        let span = expr.span();
        match expr {
            Expr::BoolLit(b, _) => HirExpr {
                kind: HirExprKind::BoolLit(*b),
                ty: Ty::Bool,
                span,
            },
            Expr::IntLit { raw, .. } => {
                let v: i64 = parse_int_lit(raw);
                HirExpr {
                    kind: HirExprKind::IntLit(v),
                    ty: Ty::Int,
                    span,
                }
            }
            Expr::FloatLit { raw, .. } => {
                let v: f64 = raw.parse().unwrap_or(0.0);
                HirExpr {
                    kind: HirExprKind::FloatLit(v),
                    ty: Ty::Float,
                    span,
                }
            }
            Expr::StrLit { value, .. } => HirExpr {
                kind: HirExprKind::StrLit(value.clone()),
                ty: Ty::String,
                span,
            },
            Expr::FStrLit { parts, .. } => {
                let hparts = parts
                    .iter()
                    .map(|p| match p {
                        FStrPart::Literal(s) => HirStrPart::Literal(s.clone()),
                        FStrPart::Interpolated(e) => HirStrPart::Expr(self.lower_expr(e, tp)),
                    })
                    .collect();
                HirExpr {
                    kind: HirExprKind::InterpolatedStr(hparts),
                    ty: Ty::String,
                    span,
                }
            }
            Expr::BytesLit { bytes, .. } => HirExpr {
                kind: HirExprKind::BytesLit(bytes.clone()),
                ty: Ty::Bytes,
                span,
            },
            Expr::None(_) => HirExpr {
                kind: HirExprKind::None_,
                ty: Ty::Optional(Box::new(Ty::Infer(0))),
                span,
            },
            Expr::SelfExpr(_) => HirExpr {
                kind: HirExprKind::Var(SmolStr::new("self")),
                ty: self.lookup("self"),
                span,
            },
            Expr::Ident(n) => {
                if let Some(ty) = self.lookup_var(&n.text) {
                    HirExpr {
                        kind: HirExprKind::Var(n.text.clone()),
                        ty,
                        span,
                    }
                } else if let Some(path) = self.resolve_def_path(&n.text) {
                    let ty = self.ty_for_def_path(&path);
                    HirExpr {
                        kind: HirExprKind::Var(path),
                        ty,
                        span,
                    }
                } else {
                    HirExpr {
                        kind: HirExprKind::Var(n.text.clone()),
                        ty: Ty::Error,
                        span,
                    }
                }
            }
            Expr::QualifiedIdent(q) => {
                let path = q.to_string();
                // Check stdlib / alias resolution first
                if let Some(c_name) = self.resolve_stdlib(&path) {
                    return HirExpr {
                        kind: HirExprKind::Var(SmolStr::new(c_name)),
                        ty: stdlib_c_func_ty(c_name),
                        span,
                    };
                }
                if let Some((def_id, variant)) = self.resolve_enum_variant(q) {
                    return HirExpr {
                        kind: HirExprKind::EnumVariant {
                            def_id,
                            variant,
                            fields: Vec::new(),
                        },
                        ty: Ty::Named(def_id, Vec::new()),
                        span,
                    };
                }
                if q.is_single() {
                    let name = q.last().text.clone();
                    if let Some(ty) = self.lookup_var(&name) {
                        HirExpr {
                            kind: HirExprKind::Var(name),
                            ty,
                            span,
                        }
                    } else if let Some(path) = self.resolve_def_path(&name) {
                        let ty = self.ty_for_def_path(&path);
                        HirExpr {
                            kind: HirExprKind::Var(path),
                            ty,
                            span,
                        }
                    } else {
                        HirExpr {
                            kind: HirExprKind::Var(name),
                            ty: Ty::Error,
                            span,
                        }
                    }
                } else {
                    if let Some(local_field) = self.lower_local_field_path(q, tp) {
                        return local_field;
                    }
                    let resolved = self
                        .resolve_def_path(&path)
                        .unwrap_or_else(|| SmolStr::new(&path));
                    let ty = self.ty_for_def_path(&resolved);
                    HirExpr {
                        kind: HirExprKind::Var(resolved),
                        ty,
                        span,
                    }
                }
            }
            Expr::Binary { op, lhs, rhs, .. } => {
                let l = self.lower_expr(lhs, tp);
                let r = self.lower_expr(rhs, tp);
                let ty = binary_result_ty(*op, &l.ty, &r.ty);
                HirExpr {
                    kind: HirExprKind::Binary {
                        op: *op,
                        lhs: Box::new(l),
                        rhs: Box::new(r),
                    },
                    ty,
                    span,
                }
            }
            Expr::Unary { op, operand, .. } => {
                let e = self.lower_expr(operand, tp);
                let ty = e.ty.clone();
                HirExpr {
                    kind: HirExprKind::Unary {
                        op: *op,
                        operand: Box::new(e),
                    },
                    ty,
                    span,
                }
            }
            Expr::Field { object, field, .. } => {
                let obj = self.lower_expr(object, tp);
                HirExpr {
                    kind: HirExprKind::Field {
                        object: Box::new(obj),
                        field: field.text.clone(),
                    },
                    ty: Ty::Infer(0),
                    span,
                }
            }
            Expr::TupleIndex { object, index, .. } => {
                let obj = self.lower_expr(object, tp);
                let ty = if let Ty::Tuple(elems) = &obj.ty {
                    elems.get(*index as usize).cloned().unwrap_or(Ty::Error)
                } else {
                    Ty::Infer(0)
                };
                HirExpr {
                    kind: HirExprKind::TupleIndex {
                        object: Box::new(obj),
                        index: *index,
                    },
                    ty,
                    span,
                }
            }
            Expr::Index { object, index, .. } => {
                let obj = self.lower_expr(object, tp);
                match index {
                    ori_ast::expr::IndexExpr::Single(index) => {
                        let idx = self.lower_expr(index, tp);
                        let ty = index_result_ty(&obj.ty);
                        HirExpr {
                            kind: HirExprKind::Index {
                                object: Box::new(obj),
                                index: Box::new(idx),
                            },
                            ty,
                            span,
                        }
                    }
                    ori_ast::expr::IndexExpr::Range { start, end } => {
                        // Desugar `obj[a..b]` to a method call on the underlying type.
                        // For strings: ori_string_slice(obj, a, b)
                        // For lists: runtime slice (future)
                        let start_h = start.as_ref().map(|e| self.lower_expr(e, tp));
                        let end_h = end.as_ref().map(|e| self.lower_expr(e, tp));
                        let s = start_h.unwrap_or(HirExpr {
                            kind: HirExprKind::IntLit(0),
                            ty: Ty::Int,
                            span,
                        });
                        // For end, use a large sentinel if not specified
                        let e = end_h.unwrap_or(HirExpr {
                            kind: HirExprKind::IntLit(i64::MAX),
                            ty: Ty::Int,
                            span,
                        });
                        let result_ty = obj.ty.clone();
                        HirExpr {
                            kind: HirExprKind::MethodCall {
                                receiver: Box::new(obj),
                                method: SmolStr::new("__slice"),
                                args: vec![s, e],
                            },
                            ty: result_ty,
                            span,
                        }
                    }
                }
            }
            Expr::Call { callee, args, .. } => {
                // Intercept builtin wrapper functions before generic lowering
                if let Expr::QualifiedIdent(q) = callee.as_ref() {
                    if q.parts.len() == 2 {
                        let receiver_name = &q.parts[0];
                        let method_name = &q.parts[1];
                        if let Some(receiver_ty) = self.lookup_var(&receiver_name.text) {
                            if let Ty::Any(trait_def_id) = &receiver_ty {
                                if let Some(return_ty) =
                                    self.trait_method_return_ty(*trait_def_id, method_name.as_str())
                                {
                                    let receiver = HirExpr {
                                        kind: HirExprKind::Var(receiver_name.text.clone()),
                                        ty: receiver_ty,
                                        span: receiver_name.span,
                                    };
                                    let args_h = args
                                        .iter()
                                        .map(|arg| match &arg.value {
                                            ori_ast::expr::ArgValue::Expr(e)
                                            | ori_ast::expr::ArgValue::Spread(e) => {
                                                self.lower_expr(e, tp)
                                            }
                                        })
                                        .collect();
                                    return HirExpr {
                                        kind: HirExprKind::MethodCall {
                                            receiver: Box::new(receiver),
                                            method: method_name.text.clone(),
                                            args: args_h,
                                        },
                                        ty: return_ty,
                                        span,
                                    };
                                }
                            }
                            if let Ty::Named(def_id, _) = &receiver_ty {
                                let def = self.def_map.get(*def_id);
                                let method_path =
                                    SmolStr::new(format!("{}.{}", def.path, method_name.text));
                                let resolved = if self.def_map.lookup(&method_path).is_some() {
                                    Some((method_path, Ty::Infer(0)))
                                } else {
                                    self.trait_method_func_for_type(*def_id, method_name.as_str())
                                };
                                if let Some((method_path, return_ty)) = resolved {
                                    let receiver = HirExpr {
                                        kind: HirExprKind::Var(receiver_name.text.clone()),
                                        ty: receiver_ty,
                                        span: receiver_name.span,
                                    };
                                    let callee_h = HirExpr {
                                        kind: HirExprKind::Var(method_path),
                                        ty: Ty::Infer(0),
                                        span: callee.span(),
                                    };
                                    let mut args_h = vec![HirArg {
                                        label: None,
                                        spread: false,
                                        value: receiver,
                                    }];
                                    args_h.extend(self.lower_call_args(args, tp));
                                    return HirExpr {
                                        kind: HirExprKind::Call {
                                            callee: Box::new(callee_h),
                                            args: args_h,
                                        },
                                        ty: return_ty,
                                        span,
                                    };
                                }
                            }
                        }
                    }
                    let name = q.to_string();
                    match name.as_str() {
                        "some" | "Some" => {
                            if let Some(a) = args.first() {
                                let e = match &a.value {
                                    ori_ast::expr::ArgValue::Expr(e)
                                    | ori_ast::expr::ArgValue::Spread(e) => e,
                                };
                                let inner = self.lower_expr(e, tp);
                                let ty = Ty::Optional(Box::new(inner.ty.clone()));
                                return HirExpr {
                                    kind: HirExprKind::Some_(Box::new(inner)),
                                    ty,
                                    span,
                                };
                            }
                        }
                        "success" | "Success" => {
                            if let Some(a) = args.first() {
                                let e = match &a.value {
                                    ori_ast::expr::ArgValue::Expr(e)
                                    | ori_ast::expr::ArgValue::Spread(e) => e,
                                };
                                let inner = self.lower_expr(e, tp);
                                let ty =
                                    Ty::Result(Box::new(inner.ty.clone()), Box::new(Ty::String));
                                return HirExpr {
                                    kind: HirExprKind::Ok_(Box::new(inner)),
                                    ty,
                                    span,
                                };
                            }
                        }
                        "error" | "Error" => {
                            if let Some(a) = args.first() {
                                let e = match &a.value {
                                    ori_ast::expr::ArgValue::Expr(e)
                                    | ori_ast::expr::ArgValue::Spread(e) => e,
                                };
                                let inner = self.lower_expr(e, tp);
                                let ty = Ty::Result(Box::new(Ty::Void), Box::new(inner.ty.clone()));
                                return HirExpr {
                                    kind: HirExprKind::Err_(Box::new(inner)),
                                    ty,
                                    span,
                                };
                            }
                        }
                        _ => {}
                    }
                    if let Some(def_id) = self.resolve_def_id_with_kind(&name, DefKind::Struct) {
                        let fields = self.lower_named_args(args, tp);
                        return HirExpr {
                            kind: HirExprKind::StructLit { def_id, fields },
                            ty: Ty::Named(def_id, Vec::new()),
                            span,
                        };
                    }
                    if let Some((def_id, variant)) = self.resolve_enum_variant(q) {
                        let fields = self.lower_named_args(args, tp);
                        return HirExpr {
                            kind: HirExprKind::EnumVariant {
                                def_id,
                                variant,
                                fields,
                            },
                            ty: Ty::Named(def_id, Vec::new()),
                            span,
                        };
                    }
                }
                if let Expr::Field { object, field, .. } = callee.as_ref() {
                    let obj_h = self.lower_expr(object, tp);
                    let mut resolved_method = None;
                    let mut resolved_return_ty = Ty::Infer(0);
                    if let Ty::Named(def_id, _) = &obj_h.ty {
                        let def = self.def_map.get(*def_id);
                        let m_path = format!("{}.{}", def.path, field.text);
                        if self.def_map.lookup(&m_path).is_some() {
                            resolved_method = Some(SmolStr::new(m_path));
                        } else if let Some((m_path, return_ty)) =
                            self.trait_method_func_for_type(*def_id, field.as_str())
                        {
                            resolved_method = Some(m_path);
                            resolved_return_ty = return_ty;
                        }
                    }
                    if let Some(m_path) = resolved_method {
                        let callee_h = HirExpr {
                            kind: HirExprKind::Var(m_path),
                            ty: Ty::Infer(0),
                            span: callee.span(),
                        };
                        let mut args_h = vec![HirArg {
                            label: None,
                            spread: false,
                            value: obj_h,
                        }];
                        args_h.extend(self.lower_call_args(args, tp));
                        return HirExpr {
                            kind: HirExprKind::Call {
                                callee: Box::new(callee_h),
                                args: args_h,
                            },
                            ty: resolved_return_ty,
                            span,
                        };
                    }
                    if let Ty::Any(trait_def_id) = &obj_h.ty {
                        if let Some(return_ty) =
                            self.trait_method_return_ty(*trait_def_id, field.as_str())
                        {
                            let args_h = args
                                .iter()
                                .map(|arg| match &arg.value {
                                    ori_ast::expr::ArgValue::Expr(e)
                                    | ori_ast::expr::ArgValue::Spread(e) => self.lower_expr(e, tp),
                                })
                                .collect();
                            return HirExpr {
                                kind: HirExprKind::MethodCall {
                                    receiver: Box::new(obj_h),
                                    method: field.text.clone(),
                                    args: args_h,
                                },
                                ty: return_ty,
                                span,
                            };
                        }
                    }
                    // If not resolved as method, fall through to normal lowering
                    let callee_h = HirExpr {
                        kind: HirExprKind::Field {
                            object: Box::new(obj_h),
                            field: field.text.clone(),
                        },
                        ty: Ty::Infer(0),
                        span: callee.span(),
                    };
                    let args_h = self.lower_call_args(args, tp);
                    return HirExpr {
                        kind: HirExprKind::Call {
                            callee: Box::new(callee_h),
                            args: args_h,
                        },
                        ty: Ty::Infer(0),
                        span,
                    };
                }

                let callee_h = self.lower_expr(callee, tp);
                let args_h = self.lower_call_args(args, tp);
                let ret_ty = match &callee_h.ty {
                    Ty::Func { ret, .. } => *ret.clone(),
                    _ => Ty::Infer(0),
                };
                HirExpr {
                    kind: HirExprKind::Call {
                        callee: Box::new(callee_h),
                        args: args_h,
                    },
                    ty: ret_ty,
                    span,
                }
            }
            Expr::Try { expr: inner, .. } => {
                let inner_h = self.lower_expr(inner, tp);
                let ty = unwrap_ty(&inner_h.ty);
                HirExpr {
                    kind: HirExprKind::Propagate(Box::new(inner_h)),
                    ty,
                    span,
                }
            }
            Expr::Range { start, end, .. } => {
                let s = self.lower_expr(start, tp);
                let e = self.lower_expr(end, tp);
                HirExpr {
                    kind: HirExprKind::Range {
                        start: Box::new(s),
                        end: Box::new(e),
                    },
                    ty: Ty::Range(Box::new(Ty::Int)),
                    span,
                }
            }
            Expr::List { elements, .. } => {
                let elems: Vec<HirExpr> = elements.iter().map(|e| self.lower_expr(e, tp)).collect();
                let elem_ty = elems.first().map(|e| e.ty.clone()).unwrap_or(Ty::Infer(0));
                let ty = Ty::List(Box::new(elem_ty.clone()));
                HirExpr {
                    kind: HirExprKind::ListLit {
                        elem_ty,
                        elements: elems,
                    },
                    ty,
                    span,
                }
            }
            Expr::Tuple { elements, .. } => {
                let elems: Vec<HirExpr> = elements.iter().map(|e| self.lower_expr(e, tp)).collect();
                let tys = elems.iter().map(|e| e.ty.clone()).collect();
                HirExpr {
                    kind: HirExprKind::TupleLit(elems),
                    ty: Ty::Tuple(tys),
                    span,
                }
            }
            Expr::IfExpr {
                condition,
                then_expr,
                else_expr,
                ..
            } => {
                let cond = self.lower_expr(condition, tp);
                let then = self.lower_expr(then_expr, tp);
                let else_ = self.lower_expr(else_expr, tp);
                let ty = then.ty.clone();
                HirExpr {
                    kind: HirExprKind::IfExpr {
                        cond: Box::new(cond),
                        then: Box::new(then),
                        else_: Box::new(else_),
                    },
                    ty,
                    span,
                }
            }
            Expr::StructLit {
                ty: type_name,
                fields,
                ..
            } => {
                let def_id = self
                    .resolve_def_id_with_kind(&type_name.to_string(), DefKind::Struct)
                    .unwrap_or(ori_types::DefId(u32::MAX));
                let ty = Ty::Named(def_id, Vec::new());
                let hfields: Vec<(SmolStr, HirExpr)> = fields
                    .iter()
                    .map(|f| (f.name.text.clone(), self.lower_expr(&f.value, tp)))
                    .collect();
                HirExpr {
                    kind: HirExprKind::StructLit {
                        def_id,
                        fields: hfields,
                    },
                    ty,
                    span,
                }
            }
            Expr::AnonStructLit { fields, .. } => {
                let hfields: Vec<(SmolStr, HirExpr)> = fields
                    .iter()
                    .map(|f| (f.name.text.clone(), self.lower_expr(&f.value, tp)))
                    .collect();
                HirExpr {
                    kind: HirExprKind::StructLit {
                        def_id: ori_types::DefId(u32::MAX),
                        fields: hfields,
                    },
                    ty: Ty::Infer(0),
                    span,
                }
            }
            Expr::EnumVariantUnit {
                ty: type_name,
                variant,
                ..
            } => {
                let def_path = type_name
                    .as_ref()
                    .map(|t| t.to_string())
                    .unwrap_or_default();
                let def_id = self
                    .resolve_def_id_with_kind(&def_path, DefKind::Enum)
                    .unwrap_or(ori_types::DefId(u32::MAX));
                HirExpr {
                    kind: HirExprKind::EnumVariant {
                        def_id,
                        variant: variant.text.clone(),
                        fields: Vec::new(),
                    },
                    ty: Ty::Named(def_id, Vec::new()),
                    span,
                }
            }
            Expr::EnumVariantNamed {
                ty: type_name,
                variant,
                fields,
                ..
            } => {
                let def_path = type_name
                    .as_ref()
                    .map(|t| t.to_string())
                    .unwrap_or_default();
                let def_id = self
                    .resolve_def_id_with_kind(&def_path, DefKind::Enum)
                    .unwrap_or(ori_types::DefId(u32::MAX));
                let hfields: Vec<(SmolStr, HirExpr)> = fields
                    .iter()
                    .map(|f| (f.name.text.clone(), self.lower_expr(&f.value, tp)))
                    .collect();
                HirExpr {
                    kind: HirExprKind::EnumVariant {
                        def_id,
                        variant: variant.text.clone(),
                        fields: hfields,
                    },
                    ty: Ty::Named(def_id, Vec::new()),
                    span,
                }
            }
            // ── Pipe: `value |> func` desugars to `func(value)` ─────────────
            Expr::Pipe { value, func, .. } => {
                let val = self.lower_expr(value, tp);
                let callee = self.lower_expr(func, tp);
                let ret_ty = match &callee.ty {
                    Ty::Func { ret, .. } => *ret.clone(),
                    _ => Ty::Infer(0),
                };
                HirExpr {
                    kind: HirExprKind::Call {
                        callee: Box::new(callee),
                        args: vec![HirArg {
                            label: None,
                            spread: false,
                            value: val,
                        }],
                    },
                    ty: ret_ty,
                    span,
                }
            }

            // ── Map literal: `{k: v, ...}` ──────────────────────────────────
            Expr::Map { entries, .. } => {
                let hentries: Vec<(HirExpr, HirExpr)> = entries
                    .iter()
                    .map(|(k, v)| (self.lower_expr(k, tp), self.lower_expr(v, tp)))
                    .collect();
                let key_ty = hentries
                    .first()
                    .map(|(k, _)| k.ty.clone())
                    .unwrap_or(Ty::Infer(0));
                let value_ty = hentries
                    .first()
                    .map(|(_, v)| v.ty.clone())
                    .unwrap_or(Ty::Infer(0));
                let ty = Ty::Map(Box::new(key_ty.clone()), Box::new(value_ty.clone()));
                HirExpr {
                    kind: HirExprKind::MapLit {
                        key_ty,
                        value_ty,
                        entries: hentries,
                    },
                    ty,
                    span,
                }
            }

            // ── Set literal: `#{a, b, ...}` ─────────────────────────────────
            Expr::Set { elements, .. } => {
                let elems: Vec<HirExpr> = elements.iter().map(|e| self.lower_expr(e, tp)).collect();
                let elem_ty = elems.first().map(|e| e.ty.clone()).unwrap_or(Ty::Infer(0));
                let ty = Ty::Set(Box::new(elem_ty.clone()));
                HirExpr {
                    kind: HirExprKind::SetLit {
                        elem_ty,
                        elements: elems,
                    },
                    ty,
                    span,
                }
            }

            // ── Struct update: `base with { field: value } end` ──────────────
            Expr::StructUpdate { base, updates, .. } => {
                let base_h = self.lower_expr(base, tp);
                let def_id = if let Ty::Named(id, _) = &base_h.ty {
                    *id
                } else {
                    ori_types::DefId(u32::MAX)
                };
                let hupdates: Vec<(SmolStr, HirExpr)> = updates
                    .iter()
                    .map(|f| (f.name.text.clone(), self.lower_expr(&f.value, tp)))
                    .collect();
                HirExpr {
                    kind: HirExprKind::StructUpdate {
                        def_id,
                        base: Box::new(base_h.clone()),
                        updates: hupdates,
                    },
                    ty: base_h.ty.clone(),
                    span,
                }
            }

            // ── `expr is TypeName` — runtime type checking ─
            Expr::IsCheck {
                value,
                ty: check_ty_ast,
                ..
            } => {
                let val = self.lower_expr(value, tp);
                // Resolve the check type. Primitive builtin names (int, string,
                // bool, etc.) are not in the DefMap, so we handle them directly
                // before calling lower_ast_ty, which would otherwise emit a
                // spurious "undefined type" diagnostic for primitives.
                let check_ty = lower_is_target_ty(check_ty_ast, self, tp);
                HirExpr {
                    kind: HirExprKind::IsCheck {
                        value: Box::new(val),
                        check_ty,
                    },
                    ty: Ty::Bool,
                    span,
                }
            }

            Expr::Closure(closure) => self.lower_closure_expr(closure, tp, span),
        }
    }

    fn lower_closure_expr(&mut self, closure: &ClosureExpr, tp: &[SmolStr], span: Span) -> HirExpr {
        let func_name = self.next_closure_name();
        let user_params: Vec<HirParam> = closure
            .params
            .iter()
            .map(|param| HirParam {
                name: param.name.text.clone(),
                ty: self.lower_ast_ty(&param.ty, tp),
                default: None,
                contract: None,
                variadic: false,
                span: param.span,
            })
            .collect();
        let param_names: Vec<SmolStr> =
            user_params.iter().map(|param| param.name.clone()).collect();
        let free_names = collect_closure_free_names(closure, &param_names);
        let captures: Vec<HirClosureCapture> = free_names
            .into_iter()
            .filter_map(|name| {
                self.lookup_var(&name)
                    .map(|ty| HirClosureCapture { name, ty })
            })
            .collect();

        let declared_ret = closure
            .return_ty
            .as_ref()
            .map(|ty| self.lower_ast_ty(ty, tp));
        let previous_ret = self.ret_ty.clone();

        self.push();
        for capture in &captures {
            self.bind(capture.name.clone(), capture.ty.clone());
        }
        for param in &user_params {
            self.bind(param.name.clone(), param.ty.clone());
        }
        self.ret_ty = declared_ret.clone().unwrap_or(Ty::Infer(0));

        let (body, return_ty) = match &closure.body {
            ClosureBody::Expr(expr) => {
                let mut value = self.lower_expr(expr, tp);
                if let Some(expected) = &declared_ret {
                    apply_expected_expr_ty(&mut value, expected);
                }
                let return_ty = declared_ret.clone().unwrap_or_else(|| value.ty.clone());
                let body = HirBlock {
                    stmts: vec![HirStmt::Return(Some(value), span)],
                    span,
                };
                (body, return_ty)
            }
            ClosureBody::Block(block) => {
                let return_ty = declared_ret.clone().unwrap_or(Ty::Void);
                self.ret_ty = return_ty.clone();
                (self.lower_block(block, tp), return_ty)
            }
        };
        self.ret_ty = previous_ret;
        self.pop();

        let mut synthetic_params = Vec::with_capacity(user_params.len() + 1);
        synthetic_params.push(HirParam {
            name: SmolStr::new("__env"),
            ty: Ty::Bytes,
            default: None,
            contract: None,
            variadic: false,
            span,
        });
        synthetic_params.extend(user_params.clone());

        let def_seed = u32::MAX.saturating_sub(self.closure_counter as u32);
        self.generated_funcs.push(HirFunc {
            def_id: ori_types::DefId(def_seed),
            name: func_name.clone(),
            params: synthetic_params,
            return_ty: return_ty.clone(),
            body,
            closure_captures: captures.clone(),
            is_public: false,
            is_mut: false,
            span,
        });

        HirExpr {
            kind: HirExprKind::Closure {
                func_name,
                captures,
            },
            ty: Ty::Func {
                params: user_params.into_iter().map(|param| param.ty).collect(),
                ret: Box::new(return_ty),
            },
            span,
        }
    }
}

// ── Pattern lowering ──────────────────────────────────────────────────────────

fn collect_closure_free_names(closure: &ClosureExpr, params: &[SmolStr]) -> Vec<SmolStr> {
    let mut bound: HashSet<SmolStr> = params.iter().cloned().collect();
    let mut names = Vec::new();
    match &closure.body {
        ClosureBody::Expr(expr) => collect_free_names_expr(expr, &mut bound, &mut names),
        ClosureBody::Block(block) => collect_free_names_block(block, &mut bound, &mut names),
    }
    names
}

fn push_free_name(name: &SmolStr, bound: &HashSet<SmolStr>, names: &mut Vec<SmolStr>) {
    if !bound.contains(name) && !names.contains(name) {
        names.push(name.clone());
    }
}

fn collect_free_names_block(block: &Block, bound: &mut HashSet<SmolStr>, names: &mut Vec<SmolStr>) {
    for stmt in &block.stmts {
        collect_free_names_stmt(stmt, bound, names);
    }
}

fn collect_free_names_stmt(stmt: &Stmt, bound: &mut HashSet<SmolStr>, names: &mut Vec<SmolStr>) {
    match stmt {
        Stmt::Const(local) => {
            collect_free_names_expr(&local.value, bound, names);
            bound.insert(local.name.text.clone());
        }
        Stmt::Var(local) => {
            collect_free_names_expr(&local.value, bound, names);
            bound.insert(local.name.text.clone());
        }
        Stmt::Assign(assign) => {
            collect_free_names_lvalue(&assign.lvalue, bound, names);
            collect_free_names_expr(&assign.value, bound, names);
        }
        Stmt::CompoundAssign(assign) => {
            collect_free_names_lvalue(&assign.lvalue, bound, names);
            collect_free_names_expr(&assign.value, bound, names);
        }
        Stmt::Return(ret) => {
            if let Some(value) = &ret.value {
                collect_free_names_expr(value, bound, names);
            }
        }
        Stmt::Expr(expr) => collect_free_names_expr(expr, bound, names),
        Stmt::If(if_stmt) => {
            collect_free_names_expr(&if_stmt.condition, bound, names);
            collect_free_names_nested_block(&if_stmt.then_block, bound, names);
            for (cond, block) in &if_stmt.else_ifs {
                collect_free_names_expr(cond, bound, names);
                collect_free_names_nested_block(block, bound, names);
            }
            if let Some(block) = &if_stmt.else_block {
                collect_free_names_nested_block(block, bound, names);
            }
        }
        Stmt::IfSome(if_some) => {
            collect_free_names_expr(&if_some.value, bound, names);
            let mut nested = bound.clone();
            nested.insert(if_some.binding.text.clone());
            collect_free_names_block(&if_some.then_block, &mut nested, names);
            if let Some(block) = &if_some.else_block {
                collect_free_names_nested_block(block, bound, names);
            }
        }
        Stmt::While(while_stmt) => {
            collect_free_names_expr(&while_stmt.condition, bound, names);
            collect_free_names_nested_block(&while_stmt.body, bound, names);
        }
        Stmt::WhileSome(while_some) => {
            collect_free_names_expr(&while_some.value, bound, names);
            let mut nested = bound.clone();
            nested.insert(while_some.binding.text.clone());
            collect_free_names_block(&while_some.body, &mut nested, names);
        }
        Stmt::For(for_stmt) => {
            collect_free_names_expr(&for_stmt.iterable, bound, names);
            let mut nested = bound.clone();
            nested.insert(for_stmt.binding.text.clone());
            if let Some(second) = &for_stmt.second_binding {
                nested.insert(second.text.clone());
            }
            collect_free_names_block(&for_stmt.body, &mut nested, names);
        }
        Stmt::Repeat(repeat) => {
            collect_free_names_expr(&repeat.count, bound, names);
            collect_free_names_nested_block(&repeat.body, bound, names);
        }
        Stmt::Loop(loop_stmt) => collect_free_names_nested_block(&loop_stmt.body, bound, names),
        Stmt::Match(match_stmt) => {
            collect_free_names_expr(&match_stmt.scrutinee, bound, names);
            for case in &match_stmt.cases {
                let mut nested = bound.clone();
                match case {
                    MatchCase::Pattern { pattern, body, .. } => {
                        bind_pattern_names(pattern, &mut nested);
                        for stmt in body {
                            collect_free_names_stmt(stmt, &mut nested, names);
                        }
                    }
                    MatchCase::Else { body, .. } => {
                        for stmt in body {
                            collect_free_names_stmt(stmt, &mut nested, names);
                        }
                    }
                }
            }
        }
        Stmt::Using(using) => {
            collect_free_names_expr(&using.value, bound, names);
            bound.insert(using.name.text.clone());
        }
        Stmt::Check(check) => collect_free_names_expr(&check.condition, bound, names),
        Stmt::Break(_) | Stmt::Continue(_) => {}
    }
}

fn collect_free_names_nested_block(
    block: &Block,
    bound: &HashSet<SmolStr>,
    names: &mut Vec<SmolStr>,
) {
    let mut nested = bound.clone();
    collect_free_names_block(block, &mut nested, names);
}

fn collect_free_names_lvalue(
    lvalue: &ori_ast::stmt::LValue,
    bound: &HashSet<SmolStr>,
    names: &mut Vec<SmolStr>,
) {
    match lvalue {
        ori_ast::stmt::LValue::Ident(name) => push_free_name(&name.text, bound, names),
        ori_ast::stmt::LValue::Field { base, .. } => collect_free_names_lvalue(base, bound, names),
        ori_ast::stmt::LValue::Index { base, index, .. } => {
            collect_free_names_lvalue(base, bound, names);
            let mut nested = bound.clone();
            collect_free_names_expr(index, &mut nested, names);
        }
    }
}

fn collect_free_names_expr(expr: &Expr, bound: &mut HashSet<SmolStr>, names: &mut Vec<SmolStr>) {
    match expr {
        Expr::Ident(name) => push_free_name(&name.text, bound, names),
        Expr::QualifiedIdent(q) if q.is_single() => push_free_name(&q.last().text, bound, names),
        Expr::SelfExpr(_) => push_free_name(&SmolStr::new("self"), bound, names),
        Expr::Range { start, end, .. } => {
            collect_free_names_expr(start, bound, names);
            collect_free_names_expr(end, bound, names);
        }
        Expr::List { elements, .. } | Expr::Set { elements, .. } | Expr::Tuple { elements, .. } => {
            for item in elements {
                collect_free_names_expr(item, bound, names);
            }
        }
        Expr::Map { entries, .. } => {
            for (key, value) in entries {
                collect_free_names_expr(key, bound, names);
                collect_free_names_expr(value, bound, names);
            }
        }
        Expr::StructLit { fields, .. } | Expr::AnonStructLit { fields, .. } => {
            for field in fields {
                collect_free_names_expr(&field.value, bound, names);
            }
        }
        Expr::EnumVariantNamed { fields, .. } => {
            for field in fields {
                collect_free_names_expr(&field.value, bound, names);
            }
        }
        Expr::Unary { operand, .. } | Expr::Try { expr: operand, .. } => {
            collect_free_names_expr(operand, bound, names);
        }
        Expr::Binary { lhs, rhs, .. } => {
            collect_free_names_expr(lhs, bound, names);
            collect_free_names_expr(rhs, bound, names);
        }
        Expr::Field { object, .. } | Expr::TupleIndex { object, .. } => {
            collect_free_names_expr(object, bound, names);
        }
        Expr::Call { callee, args, .. } => {
            collect_free_names_expr(callee, bound, names);
            for arg in args {
                match &arg.value {
                    ori_ast::expr::ArgValue::Expr(expr) | ori_ast::expr::ArgValue::Spread(expr) => {
                        collect_free_names_expr(expr, bound, names);
                    }
                }
            }
        }
        Expr::Index { object, index, .. } => {
            collect_free_names_expr(object, bound, names);
            match index {
                ori_ast::expr::IndexExpr::Single(index) => {
                    collect_free_names_expr(index, bound, names);
                }
                ori_ast::expr::IndexExpr::Range { start, end } => {
                    if let Some(start) = start {
                        collect_free_names_expr(start, bound, names);
                    }
                    if let Some(end) = end {
                        collect_free_names_expr(end, bound, names);
                    }
                }
            }
        }
        Expr::Pipe { value, func, .. } => {
            collect_free_names_expr(value, bound, names);
            collect_free_names_expr(func, bound, names);
        }
        Expr::IfExpr {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            collect_free_names_expr(condition, bound, names);
            collect_free_names_expr(then_expr, bound, names);
            collect_free_names_expr(else_expr, bound, names);
        }
        Expr::FStrLit { parts, .. } => {
            for part in parts {
                if let FStrPart::Interpolated(expr) = part {
                    collect_free_names_expr(expr, bound, names);
                }
            }
        }
        Expr::StructUpdate { base, updates, .. } => {
            collect_free_names_expr(base, bound, names);
            for update in updates {
                collect_free_names_expr(&update.value, bound, names);
            }
        }
        Expr::IsCheck { value, .. } => collect_free_names_expr(value, bound, names),
        Expr::Closure(_) => {}
        Expr::BoolLit(..)
        | Expr::IntLit { .. }
        | Expr::FloatLit { .. }
        | Expr::StrLit { .. }
        | Expr::BytesLit { .. }
        | Expr::None(_)
        | Expr::EnumVariantUnit { .. }
        | Expr::QualifiedIdent(_) => {}
    }
}

fn bind_pattern_names(pattern: &ori_ast::pattern::Pattern, bound: &mut HashSet<SmolStr>) {
    use ori_ast::pattern::Pattern;
    match pattern {
        Pattern::Binding(name) => {
            bound.insert(name.text.clone());
        }
        Pattern::Some(inner, _) | Pattern::Success(inner, _) | Pattern::Error(inner, _) => {
            bind_pattern_names(inner, bound);
        }
        Pattern::VariantNamed { fields, .. } => {
            for field in fields {
                bind_pattern_names(&field.pattern, bound);
            }
        }
        Pattern::Tuple(items, _) => {
            for item in items {
                bind_pattern_names(item, bound);
            }
        }
        Pattern::Wildcard(_)
        | Pattern::Literal(_)
        | Pattern::None(_)
        | Pattern::VariantUnit { .. } => {}
    }
}

fn lower_pattern(pat: &ori_ast::pattern::Pattern, scr_ty: &Ty) -> HirPattern {
    use ori_ast::pattern::Pattern;
    match pat {
        Pattern::Wildcard(_) => HirPattern::Wildcard,
        Pattern::Binding(n) => HirPattern::Binding(n.text.clone(), scr_ty.clone()),
        Pattern::None(_) => HirPattern::None_,
        Pattern::Some(p, _) => {
            let inner_ty = if let Ty::Optional(inner) = scr_ty {
                &**inner
            } else {
                &Ty::Infer(0)
            };
            HirPattern::Some_(Box::new(lower_pattern(p, inner_ty)))
        }
        Pattern::Success(p, _) => {
            let ok_ty = if let Ty::Result(ok, _) = scr_ty {
                &**ok
            } else {
                &Ty::Infer(0)
            };
            HirPattern::Ok_(Box::new(lower_pattern(p, ok_ty)))
        }
        Pattern::Error(p, _) => {
            let err_ty = if let Ty::Result(_, err) = scr_ty {
                &**err
            } else {
                &Ty::Infer(0)
            };
            HirPattern::Err_(Box::new(lower_pattern(p, err_ty)))
        }
        Pattern::Literal(e) => match e.as_ref() {
            Expr::BoolLit(b, _) => HirPattern::BoolLit(*b),
            Expr::IntLit { raw, .. } => HirPattern::IntLit(parse_int_lit(raw)),
            Expr::StrLit { value, .. } => HirPattern::StrLit(value.clone()),
            _ => HirPattern::Wildcard,
        },
        Pattern::Tuple(pats, _) => {
            let elem_tys = if let Ty::Tuple(elems) = scr_ty {
                elems.clone()
            } else {
                vec![Ty::Infer(0); pats.len()]
            };
            HirPattern::Tuple(
                pats.iter()
                    .zip(elem_tys.iter())
                    .map(|(pat, ty)| lower_pattern(pat, ty))
                    .collect(),
            )
        }
        Pattern::VariantUnit { name, .. } => {
            if let Ty::Named(def_id, _) = scr_ty {
                HirPattern::Variant {
                    def_id: *def_id,
                    variant: name.text.clone(),
                    fields: Vec::new(),
                }
            } else {
                HirPattern::Binding(name.text.clone(), Ty::Infer(0))
            }
        }
        Pattern::VariantNamed { name, fields, .. } => {
            if let Ty::Named(def_id, _) = scr_ty {
                HirPattern::Variant {
                    def_id: *def_id,
                    variant: name.text.clone(),
                    fields: fields
                        .iter()
                        .map(|field| {
                            (
                                field.name.text.clone(),
                                lower_pattern(&field.pattern, &Ty::Infer(0)),
                            )
                        })
                        .collect(),
                }
            } else {
                HirPattern::Wildcard
            }
        }
    }
}

fn bind_hir_pattern_scope(lowerer: &mut Lowerer<'_>, pat: &HirPattern) {
    match pat {
        HirPattern::Binding(name, ty) => lowerer.bind(name.clone(), ty.clone()),
        HirPattern::Some_(inner) | HirPattern::Ok_(inner) | HirPattern::Err_(inner) => {
            bind_hir_pattern_scope(lowerer, inner);
        }
        HirPattern::Variant { fields, .. } => {
            for (_, pat) in fields {
                bind_hir_pattern_scope(lowerer, pat);
            }
        }
        HirPattern::Tuple(patterns) => {
            for pat in patterns {
                bind_hir_pattern_scope(lowerer, pat);
            }
        }
        _ => {}
    }
}

// ── Utilities ─────────────────────────────────────────────────────────────────

fn qualified_prefix(q: &ori_ast::common::QualifiedName) -> Option<String> {
    if q.parts.len() < 2 {
        return None;
    }
    Some(
        q.parts[..q.parts.len() - 1]
            .iter()
            .map(|part| part.text.as_str())
            .collect::<Vec<_>>()
            .join("."),
    )
}

fn parse_int_lit(raw: &str) -> i64 {
    let s = raw.replace('_', "");
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        i64::from_str_radix(hex, 16).unwrap_or(0)
    } else if let Some(bin) = s.strip_prefix("0b").or_else(|| s.strip_prefix("0B")) {
        i64::from_str_radix(bin, 2).unwrap_or(0)
    } else if let Some(oct) = s.strip_prefix("0o").or_else(|| s.strip_prefix("0O")) {
        i64::from_str_radix(oct, 8).unwrap_or(0)
    } else {
        // strip any suffix like u8, i32
        let num: String = s.chars().take_while(|c| c.is_ascii_digit()).collect();
        num.parse().unwrap_or(0)
    }
}

fn elem_type(ty: &Ty) -> Ty {
    match ty {
        Ty::List(t) | Ty::Set(t) | Ty::Range(t) => *t.clone(),
        Ty::Map(key, _) => *key.clone(),
        _ => Ty::Infer(0),
    }
}

fn for_second_binding_ty(ty: &Ty) -> Ty {
    match ty {
        Ty::Map(_, value) => *value.clone(),
        _ => Ty::Int,
    }
}

fn index_result_ty(ty: &Ty) -> Ty {
    match ty {
        Ty::List(t) | Ty::Set(t) => *t.clone(),
        Ty::Map(_, v) => *v.clone(),
        Ty::String => Ty::String,
        Ty::Tuple(elems) => elems.first().cloned().unwrap_or(Ty::Infer(0)),
        _ => Ty::Infer(0),
    }
}

fn apply_expected_expr_ty(expr: &mut HirExpr, expected: &Ty) {
    match (&mut expr.kind, expected) {
        (HirExprKind::None_, Ty::Optional(_))
        | (HirExprKind::Some_(_), Ty::Optional(_))
        | (HirExprKind::Ok_(_), Ty::Result(_, _))
        | (HirExprKind::Err_(_), Ty::Result(_, _)) => {
            expr.ty = expected.clone();
        }
        _ => {}
    }
}

fn unwrap_ty(ty: &Ty) -> Ty {
    match ty {
        Ty::Optional(t) => *t.clone(),
        Ty::Result(ok, _) => *ok.clone(),
        _ => ty.clone(),
    }
}

fn binary_result_ty(op: ori_ast::expr::BinaryOp, lty: &Ty, _rty: &Ty) -> Ty {
    use ori_ast::expr::BinaryOp::*;
    match op {
        Add | Sub | Mul | Div | Rem => lty.clone(),
        Eq | Ne | Lt | Le | Gt | Ge | And | Or => Ty::Bool,
    }
}

fn lower_lvalue(lv: &ori_ast::stmt::LValue, lowerer: &mut Lowerer, tp: &[SmolStr]) -> HirLValue {
    use ori_ast::stmt::LValue;
    match lv {
        LValue::Ident(n) => {
            if lowerer.lookup_var(&n.text).is_some() {
                HirLValue::Var(n.text.clone())
            } else if let Some(path) = lowerer.resolve_def_path(&n.text) {
                HirLValue::Var(path)
            } else {
                HirLValue::Var(n.text.clone())
            }
        }
        LValue::Field { base, field, .. } => HirLValue::Field {
            base: Box::new(lower_lvalue(base, lowerer, tp)),
            field: field.text.clone(),
        },
        LValue::Index { base, index, .. } => HirLValue::Index {
            base: Box::new(lower_lvalue(base, lowerer, tp)),
            index: Box::new(lowerer.lower_expr(index, tp)),
        },
    }
}

fn lvalue_to_expr(lv: &HirLValue, span: Span) -> HirExpr {
    match lv {
        HirLValue::Var(name) => HirExpr {
            kind: HirExprKind::Var(name.clone()),
            ty: Ty::Infer(0),
            span,
        },
        HirLValue::Field { base, field } => {
            let obj = lvalue_to_expr(base, span);
            HirExpr {
                kind: HirExprKind::Field {
                    object: Box::new(obj),
                    field: field.clone(),
                },
                ty: Ty::Infer(0),
                span,
            }
        }
        HirLValue::Index { base, index } => {
            let obj = lvalue_to_expr(base, span);
            HirExpr {
                kind: HirExprKind::Index {
                    object: Box::new(obj),
                    index: index.clone(),
                },
                ty: Ty::Infer(0),
                span,
            }
        }
    }
}

fn compound_op_to_binary(op: ori_ast::stmt::CompoundOp) -> ori_ast::expr::BinaryOp {
    use ori_ast::expr::BinaryOp;
    use ori_ast::stmt::CompoundOp;
    match op {
        CompoundOp::Add => BinaryOp::Add,
        CompoundOp::Sub => BinaryOp::Sub,
        CompoundOp::Mul => BinaryOp::Mul,
        CompoundOp::Div => BinaryOp::Div,
    }
}


/// Resolve the type after `is` in an `IsCheck` expression.
///
/// Primitive builtin type names (e.g. `int`, `string`, `bool`) are not
/// registered in the `DefMap`, so we map them directly to their `Ty` variants
/// instead of calling `lower_ast_ty` which would emit a spurious error.
fn lower_is_target_ty(name: &ori_ast::common::QualifiedName, l: &mut Lowerer, tp: &[SmolStr]) -> Ty {
    if name.is_single() {
        let s = name.last().text.as_str();
        match s {
            "bool" => return Ty::Bool,
            "int" => return Ty::Int,
            "int8" => return Ty::Int8,
            "int16" => return Ty::Int16,
            "int32" => return Ty::Int32,
            "int64" => return Ty::Int64,
            "u8" => return Ty::U8,
            "u16" => return Ty::U16,
            "u32" => return Ty::U32,
            "u64" => return Ty::U64,
            "float" => return Ty::Float,
            "float32" => return Ty::Float32,
            "float64" => return Ty::Float64,
            "string" => return Ty::String,
            "bytes" => return Ty::Bytes,
            _ => {}
        }
    }
    let ty_node = ori_ast::ty::Type::Named(name.clone());
    l.lower_ast_ty(&ty_node, tp)
}