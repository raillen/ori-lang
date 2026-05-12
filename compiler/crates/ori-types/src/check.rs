use crate::def::{DefId, DefKind, DefMap};
use crate::ty::expand_ty_aliases;
use crate::lower::lower_type_with_aliases;
use crate::resolve::{
    import_aliases, EnumSig, FuncSig, ImplSig, ReExport, StructSig, TraitSig, ValueSig,
    WhereConstraintSig,
};
use crate::ty::Ty;
use ori_ast::common::{Name, QualifiedName};
use ori_ast::expr::{Arg, ArgValue, BinaryOp, ClosureBody, Expr, UnaryOp};
use ori_ast::item::{FuncDecl, ImplementDecl, Item, ParamKind, SourceFile};
use ori_ast::pattern::Pattern;
use ori_ast::stmt::{Block, LValue, Stmt};
use ori_diagnostics::{Diagnostic, DiagnosticSink, FileId, Label};
use smol_str::SmolStr;
use std::collections::{HashMap, HashSet};

// ── Environment ───────────────────────────────────────────────────────────────

/// A lexical scope: maps variable names to their types.
#[derive(Debug, Default, Clone)]
struct Scope {
    vars: HashMap<SmolStr, Ty>,
}

impl Scope {
    fn bind(&mut self, name: SmolStr, ty: Ty) {
        self.vars.insert(name, ty);
    }
    fn get(&self, name: &str) -> Option<&Ty> {
        self.vars.get(name)
    }
}

// ── Checker ───────────────────────────────────────────────────────────────────

pub struct Checker<'a> {
    def_map: &'a DefMap,
    func_sigs: &'a [FuncSig], // return types for all declared functions
    value_sigs: &'a [ValueSig],
    struct_sigs: &'a [StructSig],
    enum_sigs: &'a [EnumSig],
    trait_sigs: &'a [TraitSig],
    impl_sigs: &'a [ImplSig],
    reexports: &'a [ReExport],
    namespace: &'a str,
    file_id: FileId,
    sink: &'a mut DiagnosticSink,
    scopes: Vec<Scope>,
    aliases: HashMap<SmolStr, SmolStr>,
    used_aliases: HashSet<SmolStr>,
    current_return_ty: Option<Ty>,
    infer: HashMap<u32, Ty>,
    /// `DefId` -> `(arity, underlying_ty)` for each `type alias` declaration.
    type_alias_map: HashMap<DefId, (usize, Ty)>,
}

impl<'a> Checker<'a> {
    pub fn new(
        def_map: &'a DefMap,
        func_sigs: &'a [FuncSig],
        value_sigs: &'a [ValueSig],
        struct_sigs: &'a [StructSig],
        enum_sigs: &'a [EnumSig],
        trait_sigs: &'a [TraitSig],
        impl_sigs: &'a [ImplSig],
        type_alias_sigs: &[crate::resolve::TypeAliasSig],
        reexports: &'a [ReExport],
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
            value_sigs,
            struct_sigs,
            enum_sigs,
            trait_sigs,
            impl_sigs,
            reexports,
            namespace,
            file_id,
            sink,
            scopes: vec![Scope::default()],
            aliases: HashMap::new(),
            used_aliases: HashSet::new(),
            current_return_ty: None,
            infer: HashMap::new(),
            type_alias_map,
        }
    }

    /// Look up the return type of a function by its DefId.
    fn func_return_ty(&self, def_id: DefId) -> Option<Ty> {
        self.func_sigs
            .iter()
            .find(|s| s.def_id == def_id)
            .map(|s| s.return_ty.clone())
    }

    fn func_sig(&self, def_id: DefId) -> Option<FuncSig> {
        self.func_sigs.iter().find(|s| s.def_id == def_id).cloned()
    }

    fn value_ty(&self, def_id: DefId) -> Option<Ty> {
        self.value_sigs
            .iter()
            .find(|s| s.def_id == def_id)
            .map(|s| s.ty.clone())
    }

    fn struct_field_ty(&self, def_id: DefId, field: &str) -> Option<Ty> {
        self.struct_sigs
            .iter()
            .find(|s| s.def_id == def_id)
            .and_then(|s| s.fields.iter().find(|(name, _)| name == field))
            .map(|(_, ty)| ty.clone())
    }

    fn enum_sig(&self, def_id: DefId) -> Option<&EnumSig> {
        self.enum_sigs.iter().find(|s| s.def_id == def_id)
    }

    fn trait_sig(&self, def_id: DefId) -> Option<&TraitSig> {
        self.trait_sigs.iter().find(|s| s.def_id == def_id)
    }

    fn named_type_implements_trait(&self, type_def_id: DefId, trait_def_id: DefId) -> bool {
        self.impl_sigs
            .iter()
            .any(|sig| sig.type_def_id == type_def_id && sig.trait_def_id == trait_def_id)
    }

    fn trait_method_for_type(
        &self,
        type_def_id: DefId,
        method: &str,
    ) -> Option<crate::resolve::TraitMethodSig> {
        let mut matches = Vec::new();
        for impl_sig in self
            .impl_sigs
            .iter()
            .filter(|sig| sig.type_def_id == type_def_id)
        {
            if let Some(method_sig) = self
                .trait_sig(impl_sig.trait_def_id)
                .and_then(|trait_sig| trait_sig.methods.iter().find(|sig| sig.name == method))
            {
                matches.push(method_sig.clone());
            }
        }
        (matches.len() == 1).then(|| matches.remove(0))
    }

    pub fn check_file(&mut self, file: &SourceFile) {
        self.aliases.clear();
        self.used_aliases.clear();
        // Collect local top-level definition names for conflict detection
        let local_names: Vec<SmolStr> = file
            .items
            .iter()
            .filter_map(|i| match &i.item {
                Item::Func(f) => Some(f.name.text.clone()),
                Item::Struct(s) => Some(s.name.text.clone()),
                Item::Enum(e) => Some(e.name.text.clone()),
                Item::Const(c) => Some(c.name.text.clone()),
                Item::Var(v) => Some(v.name.text.clone()),
                _ => None,
            })
            .collect();
        for import in &file.imports {
            let alias = import
                .alias
                .as_ref()
                .map(|a| a.text.clone())
                .unwrap_or_else(|| import.path.last().text.clone());
            let alias_span = import.alias.as_ref().map(|a| a.span).unwrap_or(import.span);
            if self.aliases.contains_key(&alias) {
                self.sink.emit(
                    Diagnostic::error(
                        "bind.duplicate_alias",
                        format!("alias `{}` is already used by another import", alias),
                    )
                    .with_label(Label::primary(
                        self.file_id,
                        alias_span,
                        "duplicate alias here",
                    ))
                    .with_action("rename one of the import aliases"),
                );
            } else if local_names.contains(&alias) {
                self.sink.emit(
                    Diagnostic::error(
                        "bind.alias_shadows_local",
                        format!(
                            "import alias `{}` conflicts with a local definition of the same name",
                            alias
                        ),
                    )
                    .with_label(Label::primary(
                        self.file_id,
                        alias_span,
                        "alias defined here",
                    ))
                    .with_action("rename the import alias or the local definition"),
                );
            }
            self.aliases
                .insert(alias, SmolStr::new(import.path.to_string()));
        }
        self.aliases = import_aliases(file, self.reexports);
        for item in &file.items {
            match &item.item {
                Item::Func(f) => self.check_func(f, &[]),
                Item::Const(c) => {
                    let expected = self.lower(&c.ty, &[]);
                    let actual = self.infer_expr(&c.value);
                    self.expect_assignable(&actual, &expected, c.value.span());
                }
                Item::Var(v) => {
                    let expected = self.lower(&v.ty, &[]);
                    let actual = self.infer_expr(&v.value);
                    self.expect_assignable(&actual, &expected, v.value.span());
                }
                Item::Struct(s) => {
                    let tp: Vec<SmolStr> =
                        s.type_params.iter().map(|p| p.name.text.clone()).collect();
                    for field in &s.fields {
                        if let Some(contract) = field.contract.as_deref() {
                            let expected = self.lower(&field.ty, &tp);
                            self.push_scope();
                            self.bind(SmolStr::new("it"), expected);
                            let actual = self.infer_expr(contract);
                            self.expect_bool(&actual, contract.span());
                            self.pop_scope();
                        }
                    }
                    let previous_self = self
                        .aliases
                        .insert(SmolStr::new("Self"), s.name.text.clone());
                    for m in &s.methods {
                        self.check_func(m, &tp);
                    }
                    restore_alias(&mut self.aliases, "Self", previous_self);
                }
                Item::Implement(i) => {
                    self.check_implement_decl(i);
                    let tp: Vec<SmolStr> =
                        i.type_params.iter().map(|p| p.name.text.clone()).collect();
                    let previous_self = self
                        .aliases
                        .insert(SmolStr::new("Self"), SmolStr::new(i.for_type.to_string()));
                    for m in &i.methods {
                        self.check_func(m, &tp);
                    }
                    restore_alias(&mut self.aliases, "Self", previous_self);
                }
                Item::Trait(t) => {
                    let tp: Vec<SmolStr> =
                        t.type_params.iter().map(|p| p.name.text.clone()).collect();
                    let previous_self = self
                        .aliases
                        .insert(SmolStr::new("Self"), t.name.text.clone());
                    for m in &t.members {
                        if let ori_ast::item::TraitMember::Default(func) = m {
                            self.check_func(func, &tp);
                        }
                    }
                    restore_alias(&mut self.aliases, "Self", previous_self);
                }
                _ => {}
            }
        }
        // Emit warnings for unused imports
        for import in &file.imports {
            if import.visibility.is_public() {
                continue;
            }
            let alias = import
                .alias
                .as_ref()
                .map(|a| a.text.clone())
                .unwrap_or_else(|| import.path.last().text.clone());
            if !self.used_aliases.contains(&alias) {
                let alias_span = import.alias.as_ref().map(|a| a.span).unwrap_or(import.span);
                self.sink.emit(
                    Diagnostic::warning(
                        "bind.unused_import",
                        format!("import `{}` is never used", import.path),
                    )
                    .with_label(Label::primary(self.file_id, alias_span, "unused import"))
                    .with_action("remove this import or use it"),
                );
            }
        }
    }

    fn check_func(&mut self, func: &FuncDecl, outer_tp: &[SmolStr]) {
        let mut tp = outer_tp.to_vec();
        tp.extend(func.type_params.iter().map(|p| p.name.text.clone()));
        self.push_scope();
        let param_tys: Vec<Ty> = func
            .params
            .iter()
            .map(|param| param_binding_ty(param, self.lower(&param.ty, &tp)))
            .collect();
        // Bind parameters into scope
        for (param, ty) in func.params.iter().zip(param_tys.iter()) {
            self.bind(param.name.text.clone(), ty.clone());
        }
        for (param, expected) in func.params.iter().zip(param_tys.iter()) {
            if let Some(default) = param_default_expr(&param.kind) {
                let actual = self.infer_expr(default);
                self.expect_assignable(&actual, expected, default.span());
            }
        }
        for (param, expected) in func.params.iter().zip(param_tys.iter()) {
            if let Some(contract) = param_contract_expr(&param.kind) {
                self.push_scope();
                self.bind(SmolStr::new("it"), expected.clone());
                self.bind(param.name.text.clone(), expected.clone());
                let actual = self.infer_expr(contract);
                self.expect_bool(&actual, contract.span());
                self.pop_scope();
            }
        }
        let expected_ret = func
            .return_ty
            .as_ref()
            .map(|t| self.lower(t, &tp))
            .unwrap_or(Ty::Void);
        let prev_ret_ty = self.current_return_ty.take();
        self.current_return_ty = Some(expected_ret.clone());
        self.check_block(&func.body, &expected_ret, &tp);
        self.current_return_ty = prev_ret_ty;
        self.pop_scope();
    }

    fn check_implement_decl(&mut self, implement: &ImplementDecl) {
        let trait_name = implement.trait_name.to_string();
        let Some(trait_def_id) = self.resolve_def_id(&trait_name) else {
            self.sink.emit(
                Diagnostic::error(
                    "impl.trait_not_found",
                    format!("trait `{}` was not found", trait_name),
                )
                .with_label(Label::primary(
                    self.file_id,
                    implement.trait_name.span,
                    "trait used here",
                ))
                .with_action("define or import this trait before implementing it"),
            );
            return;
        };
        if self.def_map.get(trait_def_id).kind != DefKind::Trait {
            self.sink.emit(
                Diagnostic::error(
                    "impl.trait_not_found",
                    format!("`{}` is not a trait", trait_name),
                )
                .with_label(Label::primary(
                    self.file_id,
                    implement.trait_name.span,
                    "not a trait",
                ))
                .with_action("use the name of a trait in `implement Trait for Type`"),
            );
            return;
        }

        let type_name = implement.for_type.to_string();
        let Some(type_def_id) = self.resolve_def_id(&type_name) else {
            self.sink.emit(
                Diagnostic::error(
                    "impl.type_not_found",
                    format!("type `{}` was not found", type_name),
                )
                .with_label(Label::primary(
                    self.file_id,
                    implement.for_type.span,
                    "type used here",
                ))
                .with_action("define or import this type before implementing a trait for it"),
            );
            return;
        };
        if !matches!(
            self.def_map.get(type_def_id).kind,
            DefKind::Struct | DefKind::Enum | DefKind::TypeAlias
        ) {
            self.sink.emit(
                Diagnostic::error(
                    "impl.type_not_found",
                    format!("`{}` is not a concrete type", type_name),
                )
                .with_label(Label::primary(
                    self.file_id,
                    implement.for_type.span,
                    "not a type",
                ))
                .with_action("implement traits for a struct, enum, or type alias"),
            );
            return;
        }

        let Some(trait_sig) = self.trait_sig(trait_def_id).cloned() else {
            return;
        };
        let self_ty = Ty::Named(type_def_id, Vec::new());
        for expected in trait_sig.methods {
            let implemented = implement
                .methods
                .iter()
                .find(|method| method.name.text == expected.name);
            let Some(method) = implemented else {
                if !expected.has_default {
                    self.sink.emit(
                        Diagnostic::error(
                            "impl.missing_method",
                            format!(
                                "implement `{}` for `{}` is missing method `{}`",
                                trait_name, type_name, expected.name
                            ),
                        )
                        .with_label(Label::primary(
                            self.file_id,
                            implement.span,
                            "implement block here",
                        ))
                        .with_action(format!(
                            "add `func {}` with the signature required by the trait",
                            expected.name
                        )),
                    );
                }
                continue;
            };

            if method.is_mut != expected.is_mut {
                self.sink.emit(
                    Diagnostic::error(
                        "impl.mut_mismatch",
                        format!(
                            "method `{}` has a different mutability than the trait requires",
                            expected.name
                        ),
                    )
                    .with_label(Label::primary(
                        self.file_id,
                        method.span,
                        "implemented here",
                    ))
                    .with_label(Label::primary(
                        self.file_id,
                        expected.span,
                        "trait method declared here",
                    ))
                    .with_action("make both declarations use the same `mut func` form"),
                );
            }

            let (actual_params, actual_return) =
                self.lower_implement_method_signature(method, implement);
            let expected_params: Vec<Ty> = expected
                .params
                .iter()
                .map(|ty| substitute_trait_self(ty, trait_def_id, &self_ty))
                .collect();
            let expected_return =
                substitute_trait_self(&expected.return_ty, trait_def_id, &self_ty);
            if actual_params != expected_params || actual_return != expected_return {
                self.sink.emit(
                    Diagnostic::error(
                        "impl.wrong_signature",
                        format!(
                            "method `{}` does not match the trait signature",
                            expected.name
                        ),
                    )
                    .with_label(Label::primary(
                        self.file_id,
                        method.span,
                        "implemented signature here",
                    ))
                    .with_label(Label::primary(
                        self.file_id,
                        expected.span,
                        "trait signature here",
                    ))
                    .with_why(format!(
                        "expected `({}) -> {}`, found `({}) -> {}`",
                        display_tys(&expected_params),
                        expected_return.display(),
                        display_tys(&actual_params),
                        actual_return.display(),
                    ))
                    .with_action("change the implementation method signature to match the trait"),
                );
            }
        }
    }

    fn lower_implement_method_signature(
        &mut self,
        method: &FuncDecl,
        implement: &ImplementDecl,
    ) -> (Vec<Ty>, Ty) {
        let mut tp: Vec<SmolStr> = implement
            .type_params
            .iter()
            .map(|p| p.name.text.clone())
            .collect();
        tp.extend(method.type_params.iter().map(|p| p.name.text.clone()));
        let previous_self = self.aliases.insert(
            SmolStr::new("Self"),
            SmolStr::new(implement.for_type.to_string()),
        );
        let params = method
            .params
            .iter()
            .map(|p| self.lower(&p.ty, &tp))
            .collect();
        let return_ty = method
            .return_ty
            .as_ref()
            .map(|ty| self.lower(ty, &tp))
            .unwrap_or(Ty::Void);
        restore_alias(&mut self.aliases, "Self", previous_self);
        (params, return_ty)
    }

    fn check_block(&mut self, block: &Block, expected_ret: &Ty, tp: &[SmolStr]) {
        self.push_scope();
        for stmt in &block.stmts {
            self.check_stmt(stmt, expected_ret, tp);
        }
        self.pop_scope();
    }

    fn check_stmt(&mut self, stmt: &Stmt, expected_ret: &Ty, tp: &[SmolStr]) {
        match stmt {
            Stmt::Const(c) => {
                let ann_ty = self.lower(&c.ty, tp);
                let val_ty = self.infer_expr(&c.value);
                self.expect_assignable(&val_ty, &ann_ty, c.value.span());
                self.bind(c.name.text.clone(), ann_ty);
            }
            Stmt::Var(v) => {
                let ann_ty = self.lower(&v.ty, tp);
                let val_ty = self.infer_expr(&v.value);
                self.expect_assignable(&val_ty, &ann_ty, v.value.span());
                self.bind(v.name.text.clone(), ann_ty);
            }
            Stmt::Return(r) => {
                let ret_ty = r.value.as_ref().map_or(Ty::Void, |e| self.infer_expr(e));
                if !self.unify(&ret_ty, expected_ret) {
                    let span = r.value.as_ref().map(|e| e.span()).unwrap_or(r.span);
                    self.sink.emit(
                        Diagnostic::error(
                            "type.return_mismatch",
                            format!(
                                "return type `{}` does not match declared `{}`",
                                ret_ty.display(),
                                expected_ret.display()
                            ),
                        )
                        .with_label(Label::primary(self.file_id, span, "returned here"))
                        .with_why(format!(
                            "function declares return type `{}`",
                            expected_ret.display()
                        ))
                        .with_action("adjust the returned expression or the function return type"),
                    );
                }
            }
            Stmt::If(i) => {
                let cond_ty = self.infer_expr(&i.condition);
                self.expect_bool(&cond_ty, i.condition.span());
                self.check_block(&i.then_block, expected_ret, tp);
                for (cond, block) in &i.else_ifs {
                    let c = self.infer_expr(cond);
                    self.expect_bool(&c, cond.span());
                    self.check_block(block, expected_ret, tp);
                }
                if let Some(eb) = &i.else_block {
                    self.check_block(eb, expected_ret, tp);
                }
            }
            Stmt::While(w) => {
                let cond_ty = self.infer_expr(&w.condition);
                self.expect_bool(&cond_ty, w.condition.span());
                self.check_block(&w.body, expected_ret, tp);
            }
            Stmt::For(f) => {
                // Infer element type from iterable
                let iter_ty = self.infer_expr(&f.iterable);
                let elem_ty = elem_of(&iter_ty).unwrap_or(Ty::Error);
                let second_ty = for_second_binding_ty(&iter_ty);
                self.push_scope();
                self.bind(f.binding.text.clone(), elem_ty);
                if let Some(idx) = &f.second_binding {
                    self.bind(idx.text.clone(), second_ty);
                }
                self.check_block(&f.body, expected_ret, tp);
                self.pop_scope();
            }
            Stmt::Loop(l) => self.check_block(&l.body, expected_ret, tp),
            Stmt::Repeat(r) => {
                let count_ty = self.infer_expr(&r.count);
                if !count_ty.is_error() && !count_ty.contains_infer() && count_ty != Ty::Int {
                    self.sink.emit(
                        Diagnostic::error(
                            "type.repeat_count_not_int",
                            format!("repeat count must be `int`, found `{}`", count_ty.display()),
                        )
                        .with_label(Label::primary(
                            self.file_id,
                            r.count.span(),
                            "expected int here",
                        ))
                        .with_action("use an integer expression for the repeat count"),
                    );
                }
                self.check_block(&r.body, expected_ret, tp);
            }
            Stmt::Match(m) => {
                let scr_ty = self.infer_expr(&m.scrutinee);
                for case in &m.cases {
                    match case {
                        ori_ast::stmt::MatchCase::Pattern {
                            pattern,
                            guard,
                            body,
                            ..
                        } => {
                            self.push_scope();
                            self.check_pattern_type(pattern, &scr_ty);
                            if let Some(guard) = guard {
                                let guard_ty = self.infer_expr(guard);
                                self.expect_bool(&guard_ty, guard.span());
                            }
                            for s in body {
                                self.check_stmt(s, expected_ret, tp);
                            }
                            self.pop_scope();
                        }
                        ori_ast::stmt::MatchCase::Else { body, .. } => {
                            self.push_scope();
                            for s in body {
                                self.check_stmt(s, expected_ret, tp);
                            }
                            self.pop_scope();
                        }
                    }
                }
                self.check_match_exhaustiveness(&scr_ty, &m.cases, m.span);
            }
            Stmt::Check(c) => {
                let cond_ty = self.infer_expr(&c.condition);
                self.expect_bool(&cond_ty, c.condition.span());
            }
            Stmt::Expr(e) => {
                self.infer_expr(e);
            }
            Stmt::IfSome(s) => {
                let val_ty = self.infer_expr(&s.value);
                let inner_ty = match &val_ty {
                    Ty::Optional(t) => *t.clone(),
                    _ if val_ty.is_error() || val_ty.contains_infer() => Ty::Infer(0),
                    _ => {
                        self.sink.emit(
                            Diagnostic::error(
                                "type.ifsome_not_optional",
                                format!(
                                    "`if some` requires an `optional<T>`, found `{}`",
                                    val_ty.display()
                                ),
                            )
                            .with_label(Label::primary(
                                self.file_id,
                                s.value.span(),
                                "expected optional here",
                            ))
                            .with_action("change the expression to return `optional<T>`"),
                        );
                        Ty::Error
                    }
                };
                self.push_scope();
                self.bind(s.binding.text.clone(), inner_ty);
                self.check_block(&s.then_block, expected_ret, tp);
                self.pop_scope();
                if let Some(eb) = &s.else_block {
                    self.check_block(eb, expected_ret, tp);
                }
            }
            Stmt::WhileSome(s) => {
                let val_ty = self.infer_expr(&s.value);
                let inner_ty = match &val_ty {
                    Ty::Optional(t) => *t.clone(),
                    _ if val_ty.is_error() || val_ty.contains_infer() => Ty::Infer(0),
                    _ => {
                        self.sink.emit(
                            Diagnostic::error(
                                "type.whilesome_not_optional",
                                format!(
                                    "`while some` requires an `optional<T>`, found `{}`",
                                    val_ty.display()
                                ),
                            )
                            .with_label(Label::primary(
                                self.file_id,
                                s.value.span(),
                                "expected optional here",
                            ))
                            .with_action("change the expression to return `optional<T>`"),
                        );
                        Ty::Error
                    }
                };
                self.push_scope();
                self.bind(s.binding.text.clone(), inner_ty);
                self.check_block(&s.body, expected_ret, tp);
                self.pop_scope();
            }
            Stmt::Using(u) => {
                let ann_ty = self.lower(&u.ty, &[]);
                let val_ty = self.infer_expr(&u.value);
                self.expect_assignable(&val_ty, &ann_ty, u.value.span());
                self.bind(u.name.text.clone(), ann_ty);
            }
            Stmt::Assign(a) => {
                let rhs_ty = self.infer_expr(&a.value);
                let lhs_ty = self.infer_lvalue_ty(&a.lvalue);
                self.expect_assignable(&rhs_ty, &lhs_ty, a.value.span());
            }
            Stmt::CompoundAssign(c) => {
                let rhs_ty = self.infer_expr(&c.value);
                let lhs_ty = self.infer_lvalue_ty(&c.lvalue);
                // Both sides should be the same numeric type
                self.expect_assignable(&rhs_ty, &lhs_ty, c.value.span());
            }
            // Remaining statement kinds — not checked in v1
            _ => {}
        }
    }

    // ── Expression type inference ─────────────────────────────────────────────

    pub fn infer_expr(&mut self, expr: &Expr) -> Ty {
        match expr {
            Expr::BoolLit(..) => Ty::Bool,
            Expr::IntLit { .. } => Ty::Int,
            Expr::FloatLit { .. } => Ty::Float,
            Expr::StrLit { .. } => Ty::String,
            Expr::FStrLit { .. } => Ty::String,
            Expr::BytesLit { .. } => Ty::Bytes,
            Expr::None(_) => Ty::Optional(Box::new(Ty::Infer(0))),
            Expr::SelfExpr(span) => self.lookup_var("self", *span),
            Expr::Ident(n) => self.lookup_var(&n.text, n.span),
            Expr::QualifiedIdent(q) => {
                // Single-segment names may be local variables — check scope first
                if q.is_single() {
                    let name = q.last().as_str();
                    // Try local scope
                    for scope in self.scopes.iter().rev() {
                        if let Some(ty) = scope.get(name) {
                            return ty.clone();
                        }
                    }
                }
                if let Some((def_id, _variant)) = self.resolve_enum_variant(q) {
                    return Ty::Named(def_id, Vec::new());
                }
                // Fall back to global def_map
                let path = q.to_string();
                if let Some(id) = self.resolve_def_id(&path) {
                    self.check_visibility(id, q.span);
                    let def = self.def_map.get(id);
                    match def.kind {
                        crate::def::DefKind::Const | crate::def::DefKind::Var => {
                            self.value_ty(id).unwrap_or(Ty::Infer(id.0))
                        }
                        _ => Ty::Infer(0),
                    }
                } else if let Some(first) = q.parts.first() {
                    if let Some(mut ty) = self.lookup_local_var(&first.text) {
                        for field in q.parts.iter().skip(1) {
                            ty = self.infer_field_access(ty, field);
                            if ty.is_error() {
                                break;
                            }
                        }
                        ty
                    } else {
                        Ty::Infer(0)
                    }
                } else {
                    Ty::Infer(0)
                }
            }
            Expr::Range { .. } => Ty::Range(Box::new(Ty::Int)),
            Expr::List { elements, .. } => {
                if elements.is_empty() {
                    Ty::List(Box::new(Ty::Infer(0)))
                } else {
                    let first_ty = self.infer_expr(&elements[0]);
                    for elem in elements.iter().skip(1) {
                        let elem_ty = self.infer_expr(elem);
                        if !elem_ty.is_error()
                            && !first_ty.is_error()
                            && !elem_ty.contains_infer()
                            && !first_ty.contains_infer()
                            && !elem_ty.is_assignable_to(&first_ty)
                        {
                            self.sink.emit(
                                Diagnostic::error(
                                    "type.list_element_mismatch",
                                    format!(
                                        "list element type `{}` does not match first element `{}`",
                                        elem_ty.display(),
                                        first_ty.display()
                                    ),
                                )
                                .with_label(Label::primary(
                                    self.file_id,
                                    elem.span(),
                                    "mismatched element",
                                ))
                                .with_action(format!(
                                    "all list elements should be `{}`",
                                    first_ty.display()
                                )),
                            );
                        }
                    }
                    Ty::List(Box::new(first_ty))
                }
            }
            Expr::Tuple { elements, .. } => {
                Ty::Tuple(elements.iter().map(|e| self.infer_expr(e)).collect())
            }
            Expr::Unary { op, operand, span } => {
                let t = self.infer_expr(operand);
                match op {
                    UnaryOp::Neg => {
                        if !t.is_numeric() && !t.is_error() {
                            self.sink.emit(
                                Diagnostic::error(
                                    "type.unary_neg_non_numeric",
                                    format!(
                                        "unary `-` applied to non-numeric type `{}`",
                                        t.display()
                                    ),
                                )
                                .with_label(Label::primary(
                                    self.file_id,
                                    *span,
                                    "here",
                                )),
                            );
                            return Ty::Error;
                        }
                        t
                    }
                    UnaryOp::Not => Ty::Bool,
                }
            }
            Expr::Binary { op, lhs, rhs, span } => {
                let lt = self.infer_expr(lhs);
                let rt = self.infer_expr(rhs);
                self.infer_binary(*op, &lt, &rt, *span)
            }
            Expr::Field { object, field, .. } => {
                let obj_ty = self.infer_expr(object);
                self.infer_field_access(obj_ty, field)
            }
            Expr::Call { callee, args, .. } => {
                // If callee is a named function, look up its return type
                if let Expr::QualifiedIdent(q) = callee.as_ref() {
                    let path = q.to_string();
                    if let Some(def_id) = self.resolve_def_id(&path) {
                        self.check_visibility(def_id, q.span);
                        let def = self.def_map.get(def_id);
                        if def.kind == DefKind::Func {
                            if let Some(sig) = self.func_sig(def_id) {
                                let mut params = sig.params.clone();
                                let mut ret = self
                                    .func_return_ty(def_id)
                                    .unwrap_or_else(|| sig.return_ty.clone());
                                let mut subst = HashMap::new();
                                if params.iter().any(contains_generic_param)
                                    || contains_generic_param(&ret)
                                {
                                    subst = self.infer_generic_call_substitutions(args, &sig);
                                    if !subst.is_empty() {
                                        params = params
                                            .iter()
                                            .map(|ty| substitute_generic_params(ty, &subst))
                                            .collect();
                                        ret = substitute_generic_params(&ret, &subst);
                                    }
                                }
                                self.check_where_constraints(
                                    &sig.where_constraints,
                                    &subst,
                                    expr.span(),
                                );
                                self.check_call_args_with_defaults(
                                    args,
                                    &params,
                                    &sig.param_names,
                                    &sig.param_defaults,
                                    &sig.param_variadic,
                                    expr.span(),
                                );
                                return ret;
                            }
                        } else if def.kind == DefKind::Struct {
                            self.check_struct_constructor_args(def_id, args);
                            return Ty::Named(def_id, Vec::new());
                        }
                    }
                    if let Some((def_id, _variant)) = self.resolve_enum_variant(q) {
                        self.check_enum_variant_args(args);
                        return Ty::Named(def_id, Vec::new());
                    }
                    let expanded_path = self.expand_alias(&path);
                    if let Some(ret) = self.infer_stdlib_call(&expanded_path, args, expr.span()) {
                        return ret;
                    }
                }
                let callee_ty = self.infer_expr(callee);
                if let Ty::Func { params, ret } = callee_ty {
                    self.check_call_args(args, &params, expr.span());
                    return *ret;
                }
                for a in args {
                    let expr = match &a.value {
                        ArgValue::Expr(e) | ArgValue::Spread(e) => e.as_ref(),
                    };
                    self.infer_expr(expr);
                }
                Ty::Infer(0)
            }
            Expr::Try { expr, span, .. } => {
                let inner = self.infer_expr(expr);
                if inner.is_error() {
                    return Ty::Error;
                }
                match &inner {
                    Ty::Result(ok, err) => {
                        let cur_ret = self.current_return_ty.clone().unwrap_or(Ty::Infer(0));
                        if cur_ret.contains_infer() {
                            return *ok.clone();
                        }
                        if let Ty::Result(_, cur_err) = &cur_ret {
                            if !err.is_assignable_to(cur_err) && !err.is_error() {
                                self.sink.emit(Diagnostic::error(
                                    "type.propagate_err_mismatch",
                                    format!("cannot propagate error type `{}` in a function that returns error type `{}`", err.display(), cur_err.display())
                                ).with_label(Label::primary(self.file_id, *span, "propagated here")));
                            }
                        } else {
                            self.sink.emit(Diagnostic::error(
                                "type.propagate_return_mismatch",
                                format!("cannot use `?` operator on a `result` in a function that returns `{}`", cur_ret.display())
                            ).with_label(Label::primary(self.file_id, *span, "propagated here")));
                        }
                        *ok.clone()
                    }
                    Ty::Optional(ok) => {
                        let cur_ret = self.current_return_ty.clone().unwrap_or(Ty::Infer(0));
                        if cur_ret.contains_infer() {
                            return *ok.clone();
                        }
                        if !matches!(&cur_ret, Ty::Optional(_)) {
                            self.sink.emit(Diagnostic::error(
                                "type.propagate_return_mismatch",
                                format!("cannot use `?` operator on an `optional` in a function that returns `{}`", cur_ret.display())
                            ).with_label(Label::primary(self.file_id, *span, "propagated here")));
                        }
                        *ok.clone()
                    }
                    _ => {
                        self.sink.emit(Diagnostic::error(
                            "type.propagate_not_result_or_optional",
                            format!("the `?` operator can only be applied to `result` or `optional`, found `{}`", inner.display())
                        ).with_label(Label::primary(self.file_id, *span, "cannot apply `?` here")));
                        Ty::Error
                    }
                }
            }
            Expr::IfExpr {
                condition,
                then_expr,
                else_expr,
                span,
            } => {
                let cond_ty = self.infer_expr(condition);
                self.expect_bool(&cond_ty, condition.span());
                let then_ty = self.infer_expr(then_expr);
                let else_ty = self.infer_expr(else_expr);
                if then_ty != else_ty && !then_ty.is_error() && !else_ty.is_error() {
                    self.sink.emit(
                        Diagnostic::error(
                            "type.if_branch_mismatch",
                            format!(
                                "`if` branches have different types: `{}` vs `{}`",
                                then_ty.display(),
                                else_ty.display()
                            ),
                        )
                        .with_label(Label::primary(
                            self.file_id,
                            *span,
                            "branches diverge",
                        )),
                    );
                    return Ty::Error;
                }
                then_ty
            }
            Expr::Index {
                object,
                index,
                span,
            } => {
                let obj_ty = self.infer_expr(object);
                match index {
                    ori_ast::expr::IndexExpr::Single(idx_expr) => {
                        let idx_ty = self.infer_expr(idx_expr);
                        match &obj_ty {
                            Ty::List(elem) => {
                                if !idx_ty.is_assignable_to(&Ty::Int) && !idx_ty.is_error() {
                                    self.sink.emit(
                                        Diagnostic::error(
                                            "type.index_not_int",
                                            format!(
                                                "list index must be `int`, found `{}`",
                                                idx_ty.display()
                                            ),
                                        )
                                        .with_label(Label::primary(
                                            self.file_id,
                                            *span,
                                            "index here",
                                        ))
                                        .with_action("use an integer index"),
                                    );
                                }
                                *elem.clone()
                            }
                            Ty::Map(key, val) => {
                                if !idx_ty.is_assignable_to(key) && !idx_ty.is_error() {
                                    self.sink.emit(
                                        Diagnostic::error(
                                            "type.map_key_mismatch",
                                            format!(
                                                "map key type is `{}`, found `{}`",
                                                key.display(),
                                                idx_ty.display()
                                            ),
                                        )
                                        .with_label(Label::primary(self.file_id, *span, "key here"))
                                        .with_action(format!("use a `{}` key", key.display())),
                                    );
                                }
                                *val.clone()
                            }
                            Ty::String => {
                                if !idx_ty.is_assignable_to(&Ty::Int) && !idx_ty.is_error() {
                                    self.sink.emit(
                                        Diagnostic::error(
                                            "type.index_not_int",
                                            format!(
                                                "string index must be `int`, found `{}`",
                                                idx_ty.display()
                                            ),
                                        )
                                        .with_label(Label::primary(
                                            self.file_id,
                                            *span,
                                            "index here",
                                        ))
                                        .with_action("use an integer index"),
                                    );
                                }
                                Ty::String
                            }
                            Ty::Tuple(elems) => {
                                // Tuple index: try to resolve a constant int index
                                if let Expr::IntLit { raw, .. } = idx_expr.as_ref() {
                                    let i = raw.parse::<usize>().unwrap_or(usize::MAX);
                                    if i < elems.len() {
                                        elems[i].clone()
                                    } else {
                                        self.sink.emit(
                                            Diagnostic::error("type.tuple_index_out_of_bounds",
                                                format!("tuple has {} elements, index {} is out of bounds", elems.len(), i))
                                                .with_label(Label::primary(self.file_id, *span, "out of bounds")));
                                        Ty::Error
                                    }
                                } else {
                                    Ty::Infer(0)
                                }
                            }
                            _ if obj_ty.is_error() || obj_ty.contains_infer() => Ty::Infer(0),
                            _ => {
                                self.sink.emit(
                                    Diagnostic::error(
                                        "type.not_indexable",
                                        format!(
                                            "type `{}` does not support indexing",
                                            obj_ty.display()
                                        ),
                                    )
                                    .with_label(Label::primary(self.file_id, *span, "indexed here"))
                                    .with_action(
                                        "only list, map, string, and tuple values can be indexed",
                                    ),
                                );
                                Ty::Error
                            }
                        }
                    }
                    ori_ast::expr::IndexExpr::Range { start, end } => {
                        if let Some(s) = start {
                            self.infer_expr(s);
                        }
                        if let Some(e) = end {
                            self.infer_expr(e);
                        }
                        // Slicing returns same collection type
                        match &obj_ty {
                            Ty::List(_) | Ty::String => obj_ty,
                            _ if obj_ty.is_error() || obj_ty.contains_infer() => Ty::Infer(0),
                            _ => {
                                self.sink.emit(
                                    Diagnostic::error(
                                        "type.not_sliceable",
                                        format!(
                                            "type `{}` does not support slicing",
                                            obj_ty.display()
                                        ),
                                    )
                                    .with_label(Label::primary(self.file_id, *span, "sliced here")),
                                );
                                Ty::Error
                            }
                        }
                    }
                }
            }
            Expr::TupleIndex {
                object,
                index,
                span,
            } => {
                let obj_ty = self.infer_expr(object);
                if let Ty::Tuple(elems) = &obj_ty {
                    let i = *index as usize;
                    if i < elems.len() {
                        elems[i].clone()
                    } else {
                        self.sink.emit(
                            Diagnostic::error(
                                "type.tuple_index_out_of_bounds",
                                format!(
                                    "tuple has {} elements, index {} is out of bounds",
                                    elems.len(),
                                    i
                                ),
                            )
                            .with_label(Label::primary(
                                self.file_id,
                                *span,
                                "out of bounds",
                            )),
                        );
                        Ty::Error
                    }
                } else if obj_ty.is_error() || obj_ty.contains_infer() {
                    Ty::Infer(0)
                } else {
                    self.sink.emit(
                        Diagnostic::error(
                            "type.tuple_index_on_non_tuple",
                            format!("cannot use tuple index on `{}`", obj_ty.display()),
                        )
                        .with_label(Label::primary(
                            self.file_id,
                            *span,
                            "not a tuple",
                        )),
                    );
                    Ty::Error
                }
            }
            Expr::IsCheck { value, ty, span } => {
                self.infer_expr(value);
                self.check_is_target(ty, *span);
                Ty::Bool
            }
            Expr::Closure(closure) => {
                let param_tys: Vec<Ty> = closure
                    .params
                    .iter()
                    .map(|param| self.lower(&param.ty, &[]))
                    .collect();
                let declared_ret = closure.return_ty.as_ref().map(|ty| self.lower(ty, &[]));

                self.push_scope();
                for (param, ty) in closure.params.iter().zip(param_tys.iter()) {
                    self.bind(param.name.text.clone(), ty.clone());
                }

                let ret_ty = match &closure.body {
                    ClosureBody::Expr(expr) => {
                        let actual = self.infer_expr(expr);
                        if let Some(expected) = &declared_ret {
                            self.expect_assignable(&actual, expected, expr.span());
                            expected.clone()
                        } else {
                            actual
                        }
                    }
                    ClosureBody::Block(block) => {
                        let expected = declared_ret.clone().unwrap_or(Ty::Void);
                        let prev_ret = self.current_return_ty.take();
                        self.current_return_ty = Some(expected.clone());
                        self.check_block(block, &expected, &[]);
                        self.current_return_ty = prev_ret;
                        expected
                    }
                };
                self.pop_scope();

                Ty::Func {
                    params: param_tys,
                    ret: Box::new(ret_ty),
                }
            }
            _ => Ty::Infer(0),
        }
    }

    fn infer_binary(&mut self, op: BinaryOp, lt: &Ty, rt: &Ty, span: ori_diagnostics::Span) -> Ty {
        use BinaryOp::*;
        match op {
            Add | Sub | Mul | Div | Rem => {
                if lt.is_numeric() && lt == rt {
                    lt.clone()
                } else if lt.is_error() || rt.is_error() {
                    Ty::Error
                } else {
                    self.sink.emit(
                        Diagnostic::error(
                            "type.arithmetic_type_mismatch",
                            format!(
                        "arithmetic operator requires matching numeric types, got `{}` and `{}`",
                        lt.display(), rt.display()
                    ),
                        )
                        .with_label(Label::primary(
                            self.file_id,
                            span,
                            "here",
                        )),
                    );
                    Ty::Error
                }
            }
            Eq | Ne | Lt | Le | Gt | Ge => {
                if lt == rt || lt.is_error() || rt.is_error() {
                    Ty::Bool
                } else {
                    self.sink.emit(
                        Diagnostic::error(
                            "type.comparison_type_mismatch",
                            format!(
                                "comparison between `{}` and `{}`",
                                lt.display(),
                                rt.display()
                            ),
                        )
                        .with_label(Label::primary(
                            self.file_id,
                            span,
                            "here",
                        )),
                    );
                    Ty::Bool
                }
            }
            And | Or => Ty::Bool,
        }
    }

    fn check_call_args(&mut self, args: &[Arg], params: &[Ty], span: ori_diagnostics::Span) {
        let param_variadic = vec![false; params.len()];
        self.check_call_args_with_defaults(args, params, &[], &[], &param_variadic, span);
    }

    fn check_call_args_with_defaults(
        &mut self,
        args: &[Arg],
        params: &[Ty],
        param_names: &[SmolStr],
        param_defaults: &[bool],
        param_variadic: &[bool],
        span: ori_diagnostics::Span,
    ) {
        let has_spread = args
            .iter()
            .any(|arg| matches!(arg.value, ArgValue::Spread(_)));
        let variadic_index = param_variadic.iter().position(|variadic| *variadic);
        let fixed_count = variadic_index.unwrap_or(params.len());
        let min_args = min_required_arg_count(fixed_count, param_defaults);
        let spread_into_variadic = has_spread && variadic_index.is_some();
        if !spread_into_variadic
            && (args.len() < min_args || (variadic_index.is_none() && args.len() > params.len()))
        {
            let expected = if variadic_index.is_some() {
                format!("at least {min_args}")
            } else if min_args == params.len() {
                params.len().to_string()
            } else {
                format!("{min_args} to {}", params.len())
            };
            self.sink.emit(
                Diagnostic::error(
                    "type.arg_count_mismatch",
                    format!(
                        "function expects {} argument(s), got {}",
                        expected,
                        args.len()
                    ),
                )
                .with_label(Label::primary(self.file_id, span, "called here"))
                .with_action("pass the expected number of arguments"),
            );
        }
        if param_names.is_empty() || args.iter().all(|arg| arg.label.is_none()) {
            for (index, (arg, expected)) in args
                .iter()
                .take(fixed_count)
                .zip(params.iter().take(fixed_count))
                .enumerate()
            {
                self.check_single_call_arg(arg, expected, index);
            }
            if let Some(index) = variadic_index {
                if let Some(expected) = params.get(index) {
                    for arg in args.iter().skip(index) {
                        self.check_variadic_call_arg(arg, expected, index);
                    }
                }
            } else {
                for arg in args.iter().skip(params.len()) {
                    self.infer_call_arg(arg);
                }
            }
        } else {
            let (slots, extras) = self.order_call_arg_slots(args, param_names);
            for (index, (arg, expected)) in slots
                .iter()
                .take(fixed_count)
                .zip(params.iter().take(fixed_count))
                .enumerate()
            {
                if let Some(arg) = arg {
                    self.check_single_call_arg(arg, expected, index);
                }
            }
            if let Some(index) = variadic_index {
                if let Some(expected) = params.get(index) {
                    if let Some(Some(arg)) = slots.get(index) {
                        self.check_variadic_call_arg(arg, expected, index);
                    }
                    for arg in extras {
                        self.check_variadic_call_arg(arg, expected, index);
                    }
                }
            } else {
                for arg in extras {
                    self.infer_call_arg(arg);
                }
            }
        }
    }

    fn order_call_arg_slots<'b>(
        &mut self,
        args: &'b [Arg],
        param_names: &[SmolStr],
    ) -> (Vec<Option<&'b Arg>>, Vec<&'b Arg>) {
        let mut slots: Vec<Option<&Arg>> = vec![None; param_names.len()];
        let mut extras = Vec::new();
        let mut next_positional = 0usize;
        let mut seen_named = false;

        for arg in args {
            if let Some(label) = &arg.label {
                seen_named = true;
                let Some(index) = param_names.iter().position(|name| name == &label.text) else {
                    self.sink.emit(
                        Diagnostic::error(
                            "type.unknown_arg_label",
                            format!("function has no parameter named `{}`", label.text),
                        )
                        .with_label(Label::primary(self.file_id, label.span, "unknown label"))
                        .with_action("use one of the function parameter names"),
                    );
                    extras.push(arg);
                    continue;
                };
                if slots[index].is_some() {
                    self.sink.emit(
                        Diagnostic::error(
                            "type.duplicate_arg_label",
                            format!("argument `{}` was passed more than once", label.text),
                        )
                        .with_label(Label::primary(self.file_id, label.span, "duplicate label"))
                        .with_action("remove one of the duplicated arguments"),
                    );
                    extras.push(arg);
                } else {
                    slots[index] = Some(arg);
                }
            } else {
                if seen_named {
                    self.sink.emit(
                        Diagnostic::error(
                            "type.positional_after_named_arg",
                            "positional argument cannot appear after a named argument",
                        )
                        .with_label(Label::primary(
                            self.file_id,
                            arg.span,
                            "positional argument here",
                        ))
                        .with_action("use a named argument here too"),
                    );
                }
                while next_positional < slots.len() && slots[next_positional].is_some() {
                    next_positional += 1;
                }
                if next_positional < slots.len() {
                    slots[next_positional] = Some(arg);
                    next_positional += 1;
                } else {
                    extras.push(arg);
                }
            }
        }

        (slots, extras)
    }

    fn infer_generic_call_substitutions(
        &mut self,
        args: &[Arg],
        sig: &FuncSig,
    ) -> HashMap<u32, Ty> {
        let mut subst = HashMap::new();
        let variadic_index = sig.param_variadic.iter().position(|variadic| *variadic);
        let fixed_count = variadic_index.unwrap_or(sig.params.len());

        if sig.param_names.is_empty() || args.iter().all(|arg| arg.label.is_none()) {
            for (arg, expected) in args
                .iter()
                .take(fixed_count)
                .zip(sig.params.iter().take(fixed_count))
            {
                self.infer_generic_single_arg_substitution(arg, expected, &mut subst);
            }
            if let Some(index) = variadic_index {
                if let Some(expected) = sig.params.get(index) {
                    for arg in args.iter().skip(index) {
                        self.infer_generic_variadic_arg_substitution(arg, expected, &mut subst);
                    }
                }
            }
        } else {
            let mut slots: Vec<Option<&Arg>> = vec![None; sig.param_names.len()];
            let mut extras = Vec::new();
            let mut next_positional = 0usize;

            for arg in args {
                if let Some(label) = &arg.label {
                    if let Some(index) = sig.param_names.iter().position(|name| name == &label.text)
                    {
                        if slots[index].is_none() {
                            slots[index] = Some(arg);
                        } else {
                            extras.push(arg);
                        }
                    } else {
                        extras.push(arg);
                    }
                } else {
                    while next_positional < slots.len() && slots[next_positional].is_some() {
                        next_positional += 1;
                    }
                    if next_positional < slots.len() {
                        slots[next_positional] = Some(arg);
                        next_positional += 1;
                    } else {
                        extras.push(arg);
                    }
                }
            }

            for (arg, expected) in slots
                .iter()
                .take(fixed_count)
                .zip(sig.params.iter().take(fixed_count))
            {
                if let Some(arg) = arg {
                    self.infer_generic_single_arg_substitution(arg, expected, &mut subst);
                }
            }
            if let Some(index) = variadic_index {
                if let Some(expected) = sig.params.get(index) {
                    if let Some(Some(arg)) = slots.get(index) {
                        self.infer_generic_variadic_arg_substitution(arg, expected, &mut subst);
                    }
                    for arg in extras {
                        self.infer_generic_variadic_arg_substitution(arg, expected, &mut subst);
                    }
                }
            }
        }

        subst
    }

    fn infer_generic_single_arg_substitution(
        &mut self,
        arg: &Arg,
        expected: &Ty,
        subst: &mut HashMap<u32, Ty>,
    ) {
        let ArgValue::Expr(expr) = &arg.value else {
            return;
        };
        let actual = self.infer_expr(expr);
        infer_generic_substitution(expected, &actual, subst);
    }

    fn infer_generic_variadic_arg_substitution(
        &mut self,
        arg: &Arg,
        expected: &Ty,
        subst: &mut HashMap<u32, Ty>,
    ) {
        let actual = match &arg.value {
            ArgValue::Expr(expr) | ArgValue::Spread(expr) => self.infer_expr(expr),
        };
        let expected = match (&arg.value, expected) {
            (ArgValue::Expr(_), Ty::List(elem_ty)) => elem_ty.as_ref(),
            _ => expected,
        };
        infer_generic_substitution(expected, &actual, subst);
    }

    fn check_single_call_arg(&mut self, arg: &Arg, expected: &Ty, index: usize) {
        let expr = match &arg.value {
            ArgValue::Expr(e) => e.as_ref(),
            ArgValue::Spread(e) => {
                let actual = self.infer_expr(e);
                let code = if matches!(actual, Ty::List(_)) {
                    "type.spread_non_variadic"
                } else {
                    "type.spread_non_list"
                };
                self.sink.emit(
                    Diagnostic::error(code, "spread arguments require a variadic parameter")
                        .with_label(Label::primary(self.file_id, arg.span, "spread here"))
                        .with_action(
                            "use `..expr` only when passing values to a variadic parameter",
                        ),
                );
                return;
            }
        };
        let actual = self.infer_expr(expr);
        if !self.unify(&actual, expected) {
            self.sink.emit(
                Diagnostic::error(
                    "type.arg_type_mismatch",
                    format!(
                        "argument {} expects `{}`, found `{}`",
                        index + 1,
                        expected.display(),
                        actual.display(),
                    ),
                )
                .with_label(Label::primary(self.file_id, arg.span, "this argument"))
                .with_action(format!(
                    "change the argument to produce `{}`",
                    expected.display()
                )),
            );
        }
    }

    fn check_variadic_call_arg(&mut self, arg: &Arg, elem_ty: &Ty, index: usize) {
        match &arg.value {
            ArgValue::Expr(expr) => {
                let actual = self.infer_expr(expr);
                if !self.unify(&actual, elem_ty) {
                    self.sink.emit(
                        Diagnostic::error(
                            "type.arg_type_mismatch",
                            format!(
                                "variadic argument {} expects `{}`, found `{}`",
                                index + 1,
                                elem_ty.display(),
                                actual.display(),
                            ),
                        )
                        .with_label(Label::primary(self.file_id, arg.span, "this argument"))
                        .with_action(format!(
                            "change the argument to produce `{}`",
                            elem_ty.display()
                        )),
                    );
                }
            }
            ArgValue::Spread(expr) => {
                let actual = self.infer_expr(expr);
                let expected = Ty::List(Box::new(elem_ty.clone()));
                if !self.unify(&actual, &expected) {
                    self.sink.emit(
                        Diagnostic::error(
                            "type.arg_type_mismatch",
                            format!(
                                "spread argument expects `{}`, found `{}`",
                                expected.display(),
                                actual.display(),
                            ),
                        )
                        .with_label(Label::primary(self.file_id, arg.span, "this argument"))
                        .with_action(format!("spread a `{}` value", expected.display())),
                    );
                }
            }
        }
    }

    fn infer_call_arg(&mut self, arg: &Arg) {
        let expr = match &arg.value {
            ArgValue::Expr(e) | ArgValue::Spread(e) => e.as_ref(),
        };
        self.infer_expr(expr);
    }

    fn infer_stdlib_call(
        &mut self,
        path: &str,
        args: &[Arg],
        span: ori_diagnostics::Span,
    ) -> Option<Ty> {
        let (params, mut ret) = stdlib_func_sig(path)?;
        self.check_call_args(args, &params, span);
        let first_arg_ty = args.first().and_then(|arg| match &arg.value {
            ArgValue::Expr(expr) | ArgValue::Spread(expr) => Some(self.infer_expr(expr)),
        });
        match (path, first_arg_ty.as_ref()) {
            ("ori.list.get", Some(Ty::List(elem))) => ret = *elem.clone(),
            ("ori.map.get", Some(Ty::Map(_, value))) => ret = *value.clone(),
            _ => {}
        }
        Some(ret)
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn check_is_target(&mut self, ty: &QualifiedName, span: ori_diagnostics::Span) {
        let name = ty.to_string();
        if is_builtin_type_name(ty) {
            return;
        }

        let Some(def_id) = self.resolve_def_id(&name) else {
            self.sink.emit(
                Diagnostic::error("type.undefined_name", format!("undefined type `{}`", name))
                    .with_label(Label::primary(self.file_id, ty.span, "type used here"))
                    .with_action("use a type that is defined or imported in this namespace"),
            );
            return;
        };

        self.check_visibility(def_id, ty.span);
        if !matches!(
            self.def_map.get(def_id).kind,
            DefKind::Struct | DefKind::Enum | DefKind::Trait | DefKind::TypeAlias
        ) {
            self.sink.emit(
                Diagnostic::error(
                    "type.is_target_not_type",
                    format!("`{}` cannot be used as a type check target", name),
                )
                .with_label(Label::primary(self.file_id, span, "`is` expression here"))
                .with_action("use a struct, enum, trait, alias, or primitive type after `is`"),
            );
        }
    }

    fn check_where_constraints(
        &mut self,
        constraints: &[WhereConstraintSig],
        subst: &HashMap<u32, Ty>,
        span: ori_diagnostics::Span,
    ) {
        for constraint in constraints {
            let Some(actual) = subst.get(&constraint.param_index) else {
                continue;
            };
            if actual.is_error() || actual.contains_infer() || contains_generic_param(actual) {
                continue;
            }

            let satisfied = self.type_satisfies_trait(actual, constraint.trait_def_id);
            let failed = if constraint.negative {
                satisfied
            } else {
                !satisfied
            };
            if !failed {
                continue;
            }

            let trait_name = self.def_map.get(constraint.trait_def_id).name.clone();
            let relation = if constraint.negative {
                "must not implement"
            } else {
                "must implement"
            };
            self.sink.emit(
                Diagnostic::error(
                    "generic.constraint_not_satisfied",
                    format!(
                        "`{}` {} `{}`, but call uses `{}`",
                        constraint.param_name,
                        relation,
                        trait_name,
                        actual.display()
                    ),
                )
                .with_label(Label::primary(self.file_id, span, "generic call here"))
                .with_action("pass a value whose type satisfies the function `where` clause"),
            );
        }
    }

    fn type_satisfies_trait(&self, ty: &Ty, trait_def_id: DefId) -> bool {
        match ty {
            Ty::Named(type_def_id, _) => {
                self.named_type_implements_trait(*type_def_id, trait_def_id)
            }
            Ty::Any(id) => *id == trait_def_id,
            _ => false,
        }
    }

    fn check_struct_constructor_args(&mut self, def_id: DefId, args: &[Arg]) {
        let fields = self
            .struct_sigs
            .iter()
            .find(|s| s.def_id == def_id)
            .map(|s| s.fields.clone())
            .unwrap_or_default();
        let mut provided = HashSet::new();

        for arg in args {
            let expr = match &arg.value {
                ArgValue::Expr(e) | ArgValue::Spread(e) => e.as_ref(),
            };
            let actual = self.infer_expr(expr);
            let Some(label) = &arg.label else {
                self.sink.emit(
                    Diagnostic::error(
                        "type.struct_literal_named_fields_required",
                        "struct construction requires named fields",
                    )
                    .with_label(Label::primary(self.file_id, arg.span, "field name missing"))
                    .with_action("write the argument as `field: value`"),
                );
                continue;
            };

            if let Some((_, expected)) = fields.iter().find(|(name, _)| name == &label.text) {
                provided.insert(label.text.clone());
                self.expect_assignable(&actual, expected, arg.span);
            } else {
                self.sink.emit(
                    Diagnostic::error(
                        "type.no_such_field",
                        format!("struct has no field `{}`", label.text),
                    )
                    .with_label(Label::primary(self.file_id, label.span, "unknown field"))
                    .with_action("use a field declared on the struct"),
                );
            }
        }

        for (name, _) in fields {
            if !provided.contains(&name) {
                self.sink.emit(
                    Diagnostic::error(
                        "type.missing_struct_field",
                        format!("struct construction is missing field `{}`", name),
                    )
                    .with_action(format!("provide `{}` in the struct literal", name)),
                );
            }
        }
    }

    fn check_enum_variant_args(&mut self, args: &[Arg]) {
        for arg in args {
            let expr = match &arg.value {
                ArgValue::Expr(e) | ArgValue::Spread(e) => e.as_ref(),
            };
            self.infer_expr(expr);
            if arg.label.is_none() {
                self.sink.emit(
                    Diagnostic::error(
                        "type.enum_variant_named_fields_required",
                        "enum variant construction requires named fields",
                    )
                    .with_label(Label::primary(self.file_id, arg.span, "field name missing"))
                    .with_action("write the argument as `field: value`"),
                );
            }
        }
    }

    fn lower(&mut self, ty: &ori_ast::ty::Type, type_params: &[SmolStr]) -> Ty {
        self.mark_type_alias_usage(ty);
        let raw = lower_type_with_aliases(
            ty,
            self.namespace,
            type_params,
            self.def_map,
            self.file_id,
            self.sink,
            &self.aliases,
        );
        expand_ty_aliases(raw, self.def_map, &self.type_alias_map)
    }

    /// Walk an AST type and mark any alias prefix as used.
    fn mark_type_alias_usage(&mut self, ty: &ori_ast::ty::Type) {
        use ori_ast::ty::Type as T;
        match ty {
            T::Named(q) | T::Any(q, _) => {
                if let Some(first) = q.parts.first() {
                    if self.aliases.contains_key(&first.text) {
                        self.used_aliases.insert(first.text.clone());
                    }
                }
            }
            T::Generic { name, args, .. } => {
                if let Some(first) = name.parts.first() {
                    if self.aliases.contains_key(&first.text) {
                        self.used_aliases.insert(first.text.clone());
                    }
                }
                for arg in args {
                    self.mark_type_alias_usage(arg);
                }
            }
            T::Optional(inner, _)
            | T::List(inner, _)
            | T::Set(inner, _)
            | T::Range(inner, _)
            | T::Lazy(inner, _) => self.mark_type_alias_usage(inner),
            T::Result(a, b, _) | T::Map(a, b, _) => {
                self.mark_type_alias_usage(a);
                self.mark_type_alias_usage(b);
            }
            T::Tuple(elems, _) => {
                for e in elems {
                    self.mark_type_alias_usage(e);
                }
            }
            T::Func {
                params, return_ty, ..
            } => {
                for p in params {
                    self.mark_type_alias_usage(p);
                }
                if let Some(r) = return_ty {
                    self.mark_type_alias_usage(r);
                }
            }
            _ => {} // primitives
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(Scope::default());
    }
    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn bind(&mut self, name: SmolStr, ty: Ty) {
        if let Some(s) = self.scopes.last_mut() {
            s.bind(name, ty);
        }
    }

    fn lookup_local_var(&self, name: &str) -> Option<Ty> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        None
    }

    fn lookup_var(&mut self, name: &str, span: ori_diagnostics::Span) -> Ty {
        if let Some(ty) = self.lookup_local_var(name) {
            return ty;
        }
        self.sink.emit(
            Diagnostic::error("name.undefined", format!("undefined variable `{}`", name))
                .with_label(Label::primary(self.file_id, span, "not in scope"))
                .with_action("declare the variable with `const` or `var` before using it"),
        );
        Ty::Error
    }

    fn infer_field_access(&mut self, obj_ty: Ty, field: &Name) -> Ty {
        if let Ty::Named(def_id, _) = &obj_ty {
            if let Some(ty) = self.struct_field_ty(*def_id, field.as_str()) {
                return ty;
            }

            // Method fallback
            let def = self.def_map.get(*def_id);
            let method_path = format!("{}.{}", def.path, field.text);
            if let Some(m_def_id) = self.def_map.lookup(&method_path) {
                if let Some(sig) = self.func_sig(m_def_id) {
                    let mut params = sig.params.clone();
                    if !params.is_empty() {
                        params.remove(0); // Remove `self`
                    }
                    return Ty::Func {
                        params,
                        ret: Box::new(sig.return_ty.clone()),
                    };
                }
            }

            if let Some(method) = self.trait_method_for_type(*def_id, field.as_str()) {
                let mut params = method.params;
                if !params.is_empty() {
                    params.remove(0);
                }
                return Ty::Func {
                    params,
                    ret: Box::new(method.return_ty),
                };
            }

            self.sink.emit(
                Diagnostic::error(
                    "type.no_such_field",
                    format!(
                        "type `{}` has no field or method `{}`",
                        obj_ty.display(),
                        field.text
                    ),
                )
                .with_label(Label::primary(
                    self.file_id,
                    field.span,
                    "unknown field/method",
                ))
                .with_action("use a field or method declared on the struct"),
            );
            return Ty::Error;
        }
        if let Ty::Any(trait_def_id) = &obj_ty {
            if let Some(trait_sig) = self.trait_sig(*trait_def_id) {
                if let Some(method) = trait_sig.methods.iter().find(|m| m.name == field.text) {
                    let mut params = method.params.clone();
                    if !params.is_empty() {
                        params.remove(0);
                    }
                    return Ty::Func {
                        params,
                        ret: Box::new(method.return_ty.clone()),
                    };
                }
            }

            self.sink.emit(
                Diagnostic::error(
                    "type.no_such_method",
                    format!(
                        "`{}` is not a method declared by `{}`",
                        field.text,
                        obj_ty.display()
                    ),
                )
                .with_label(Label::primary(
                    self.file_id,
                    field.span,
                    "unknown trait method",
                ))
                .with_action("call only methods declared by the trait in `any<Trait>`"),
            );
            return Ty::Error;
        }
        if let Ty::Tuple(elems) = &obj_ty {
            if let Ok(idx) = field.text.parse::<usize>() {
                if idx < elems.len() {
                    return elems[idx].clone();
                } else {
                    self.sink.emit(
                        Diagnostic::error(
                            "type.tuple_index_out_of_bounds",
                            format!(
                                "tuple has {} elements, index {} is out of bounds",
                                elems.len(),
                                idx
                            ),
                        )
                        .with_label(Label::primary(
                            self.file_id,
                            field.span,
                            "out of bounds",
                        )),
                    );
                    return Ty::Error;
                }
            } else {
                self.sink.emit(
                    Diagnostic::error(
                        "type.field_on_tuple_not_int",
                        format!("tuple indices must be integers, found `{}`", field.text),
                    )
                    .with_label(Label::primary(
                        self.file_id,
                        field.span,
                        "invalid tuple index",
                    )),
                );
                return Ty::Error;
            }
        }
        if obj_ty.is_error() || obj_ty.contains_infer() {
            Ty::Infer(0)
        } else {
            self.sink.emit(
                Diagnostic::error(
                    "type.field_on_non_struct",
                    format!(
                        "cannot access field `{}` on `{}`",
                        field.text,
                        obj_ty.display()
                    ),
                )
                .with_label(Label::primary(
                    self.file_id,
                    field.span,
                    "field access here",
                ))
                .with_action("access fields only on struct values"),
            );
            Ty::Error
        }
    }

    fn expand_alias(&mut self, name: &str) -> String {
        let mut prefix_end = name.len();
        loop {
            let prefix = &name[..prefix_end];
            if let Some(full_ns) = self.aliases.get(prefix) {
                let root = prefix.split('.').next().unwrap_or(prefix);
                self.used_aliases.insert(SmolStr::new(root));
                let suffix = &name[prefix_end..];
                if suffix.is_empty() {
                    return full_ns.to_string();
                }
                return format!("{}{}", full_ns, suffix);
            }
            if let Some(dot) = name[..prefix_end].rfind('.') {
                prefix_end = dot;
            } else {
                break;
            }
        }
        name.to_string()
    }

    fn resolve_def_id(&mut self, name: &str) -> Option<DefId> {
        let expanded = self.expand_alias(name);
        self.def_map.lookup(&expanded).or_else(|| {
            self.def_map
                .lookup(&format!("{}.{}", self.namespace, expanded))
        })
    }

    fn resolve_enum_variant(&mut self, q: &QualifiedName) -> Option<(DefId, SmolStr)> {
        let enum_path = qualified_prefix(q)?;
        let def_id = self.resolve_def_id(&enum_path)?;
        if self.def_map.get(def_id).kind != DefKind::Enum {
            return None;
        }
        self.check_visibility(def_id, q.span);
        Some((def_id, q.last().text.clone()))
    }

    /// Returns the namespace prefix of a fully-qualified path (everything before the last dot).
    fn def_namespace(path: &str) -> &str {
        path.rfind('.').map(|i| &path[..i]).unwrap_or(path)
    }

    /// Emit a diagnostic if `def_id` refers to a private item in a different namespace.
    fn check_visibility(&mut self, def_id: DefId, span: ori_diagnostics::Span) {
        let def = self.def_map.get(def_id);
        if !def.is_public {
            let def_ns = Self::def_namespace(&def.path);
            if def_ns != self.namespace {
                self.sink.emit(
                    Diagnostic::error(
                        "name.private",
                        format!("`{}` is private to namespace `{}`", def.name, def_ns),
                    )
                    .with_label(Label::primary(self.file_id, span, "used here"))
                    .with_action("mark the definition as `pub` or use a public alternative"),
                );
            }
        }
    }

    fn expect_bool(&mut self, ty: &Ty, span: ori_diagnostics::Span) {
        if ty != &Ty::Bool && !ty.is_error() {
            self.sink.emit(
                Diagnostic::error(
                    "type.expected_bool",
                    format!("expected `bool`, found `{}`", ty.display()),
                )
                .with_label(Label::primary(self.file_id, span, "this expression"))
                .with_action("use a boolean expression here"),
            );
        }
    }

    fn expect_assignable(&mut self, from: &Ty, to: &Ty, span: ori_diagnostics::Span) {
        if !self.unify(from, to) {
            self.sink.emit(
                Diagnostic::error(
                    "type.type_mismatch",
                    format!(
                        "type mismatch: expected `{}`, found `{}`",
                        to.display(),
                        from.display()
                    ),
                )
                .with_label(Label::primary(self.file_id, span, "this expression"))
                .with_action(format!(
                    "change the expression to produce `{}`",
                    to.display()
                )),
            );
        }
    }

    // ── Unification helpers ─────────────────────────────────────────────
    /// Attempt to make `a` assignable to `b` by solving inference variables.
    /// Returns `true` if unification succeeds.
    fn unify(&mut self, a: &Ty, b: &Ty) -> bool {
        use Ty::*;
        if a == b {
            return true;
        }
        match (a, b) {
            (Infer(id), _) => return self.unify_infer(*id, b),
            (_, Infer(id)) => return self.unify_infer(*id, a),
            (Optional(x), Optional(y)) => self.unify(x, y),
            (Result(ok1, err1), Result(ok2, err2)) => {
                self.unify(ok1, ok2) && self.unify(err1, err2)
            }
            (List(x), List(y)) | (Set(x), Set(y)) | (Range(x), Range(y)) | (Lazy(x), Lazy(y)) => {
                self.unify(x, y)
            }
            (Map(ka, va), Map(kb, vb)) => self.unify(ka, kb) && self.unify(va, vb),
            (Tuple(xs), Tuple(ys)) if xs.len() == ys.len() => {
                xs.iter().zip(ys).all(|(x, y)| self.unify(x, y))
            }
            (
                Func {
                    params: pa,
                    ret: ra,
                },
                Func {
                    params: pb,
                    ret: rb,
                },
            ) if pa.len() == pb.len() => {
                pa.iter().zip(pb).all(|(x, y)| self.unify(x, y)) && self.unify(ra, rb)
            }
            (Named(id1, args1), Named(id2, args2)) if id1 == id2 && args1.len() == args2.len() => {
                args1.iter().zip(args2).all(|(x, y)| self.unify(x, y))
            }
            (Named(type_id, _), Any(trait_id)) => {
                self.named_type_implements_trait(*type_id, *trait_id)
            }
            (Any(id1), Any(id2)) => id1 == id2,
            _ => false,
        }
    }

    fn unify_infer(&mut self, id: u32, ty: &Ty) -> bool {
        if id == 0 {
            return !Self::contains_infer_id(ty, id) || matches!(ty, Ty::Infer(0));
        }
        if let Ty::Infer(other) = ty {
            if *other == id {
                return true;
            }
        }
        if let Some(bound) = self.infer.get(&id).cloned() {
            return self.unify(&bound, ty);
        }
        if Self::contains_infer_id(ty, id) {
            return false;
        } // occurs check
        self.infer.insert(id, ty.clone());
        true
    }

    /// Detect whether `ty` contains the given inference variable id (occurs check).
    fn contains_infer_id(ty: &Ty, id: u32) -> bool {
        match ty {
            Ty::Infer(i) => *i == id,
            Ty::Optional(t) | Ty::List(t) | Ty::Set(t) | Ty::Range(t) | Ty::Lazy(t) => {
                Self::contains_infer_id(t, id)
            }
            Ty::Result(ok, err) | Ty::Map(ok, err) => {
                Self::contains_infer_id(ok, id) || Self::contains_infer_id(err, id)
            }
            Ty::Tuple(ts) => ts.iter().any(|t| Self::contains_infer_id(t, id)),
            Ty::Func { params, ret } => {
                params.iter().any(|p| Self::contains_infer_id(p, id))
                    || Self::contains_infer_id(ret, id)
            }
            Ty::Named(_, args) => args.iter().any(|a| Self::contains_infer_id(a, id)),
            _ => false,
        }
    }

    fn check_match_exhaustiveness(
        &mut self,
        scr_ty: &Ty,
        cases: &[ori_ast::stmt::MatchCase],
        span: ori_diagnostics::Span,
    ) {
        if scr_ty.is_error() || scr_ty.contains_infer() {
            return;
        }

        if cases
            .iter()
            .any(|case| self.case_is_unguarded_catch_all(case, scr_ty))
        {
            return;
        }

        match scr_ty {
            Ty::Bool => {
                let mut seen_true = false;
                let mut seen_false = false;
                for case in cases {
                    let ori_ast::stmt::MatchCase::Pattern {
                        pattern,
                        guard: None,
                        ..
                    } = case
                    else {
                        continue;
                    };
                    if let Pattern::Literal(expr) = pattern {
                        match expr.as_ref() {
                            Expr::BoolLit(true, _) => seen_true = true,
                            Expr::BoolLit(false, _) => seen_false = true,
                            _ => {}
                        }
                    }
                }
                let mut missing = Vec::new();
                if !seen_true {
                    missing.push("true".to_string());
                }
                if !seen_false {
                    missing.push("false".to_string());
                }
                self.emit_match_non_exhaustive(span, missing);
            }
            Ty::Optional(_) => {
                let mut seen_some = false;
                let mut seen_none = false;
                for case in cases {
                    let ori_ast::stmt::MatchCase::Pattern {
                        pattern,
                        guard: None,
                        ..
                    } = case
                    else {
                        continue;
                    };
                    match pattern {
                        Pattern::Some(_, _) => seen_some = true,
                        Pattern::None(_) => seen_none = true,
                        _ => {}
                    }
                }
                let mut missing = Vec::new();
                if !seen_some {
                    missing.push("some(...)".to_string());
                }
                if !seen_none {
                    missing.push("none".to_string());
                }
                self.emit_match_non_exhaustive(span, missing);
            }
            Ty::Result(_, _) => {
                let mut seen_success = false;
                let mut seen_error = false;
                for case in cases {
                    let ori_ast::stmt::MatchCase::Pattern {
                        pattern,
                        guard: None,
                        ..
                    } = case
                    else {
                        continue;
                    };
                    match pattern {
                        Pattern::Success(_, _) => seen_success = true,
                        Pattern::Error(_, _) => seen_error = true,
                        _ => {}
                    }
                }
                let mut missing = Vec::new();
                if !seen_success {
                    missing.push("success(...)".to_string());
                }
                if !seen_error {
                    missing.push("error(...)".to_string());
                }
                self.emit_match_non_exhaustive(span, missing);
            }
            Ty::Named(def_id, _) if self.def_map.get(*def_id).kind == DefKind::Enum => {
                let Some(enum_sig) = self.enum_sig(*def_id) else {
                    return;
                };
                let mut covered = HashSet::new();
                for case in cases {
                    let ori_ast::stmt::MatchCase::Pattern {
                        pattern,
                        guard: None,
                        ..
                    } = case
                    else {
                        continue;
                    };
                    if let Some(name) = self.covered_enum_variant(pattern, enum_sig) {
                        covered.insert(name);
                    }
                }
                let missing: Vec<String> = enum_sig
                    .variants
                    .iter()
                    .filter(|variant| !covered.contains(*variant))
                    .map(|variant| variant.to_string())
                    .collect();
                self.emit_match_non_exhaustive(span, missing);
            }
            _ => {}
        }
    }

    fn case_is_unguarded_catch_all(&self, case: &ori_ast::stmt::MatchCase, scr_ty: &Ty) -> bool {
        match case {
            ori_ast::stmt::MatchCase::Else { .. } => true,
            ori_ast::stmt::MatchCase::Pattern {
                pattern,
                guard: None,
                ..
            } => self.pattern_is_catch_all(pattern, scr_ty),
            ori_ast::stmt::MatchCase::Pattern { .. } => false,
        }
    }

    fn pattern_is_catch_all(&self, pattern: &Pattern, scr_ty: &Ty) -> bool {
        match pattern {
            Pattern::Wildcard(_) => true,
            Pattern::Binding(name) => {
                if let Ty::Named(def_id, _) = scr_ty {
                    if let Some(enum_sig) = self.enum_sig(*def_id) {
                        return !enum_sig
                            .variants
                            .iter()
                            .any(|variant| variant == &name.text);
                    }
                }
                true
            }
            _ => false,
        }
    }

    fn covered_enum_variant(&self, pattern: &Pattern, enum_sig: &EnumSig) -> Option<SmolStr> {
        let name = match pattern {
            Pattern::VariantUnit { name, .. }
            | Pattern::VariantNamed { name, .. }
            | Pattern::Binding(name)
                if enum_sig
                    .variants
                    .iter()
                    .any(|variant| variant == &name.text) =>
            {
                name.text.clone()
            }
            _ => return None,
        };
        Some(name)
    }

    fn emit_match_non_exhaustive(&mut self, span: ori_diagnostics::Span, missing: Vec<String>) {
        if missing.is_empty() {
            return;
        }
        self.sink.emit(
            Diagnostic::error(
                "match.non_exhaustive",
                format!("match is not exhaustive; missing {}", missing.join(", ")),
            )
            .with_label(Label::primary(self.file_id, span, "match checked here"))
            .with_action("add the missing cases or a `case else` arm"),
        );
    }

    /// Check that a pattern is consistent with the scrutinee type.

    /// Check that a pattern is consistent with the scrutinee type.
    /// Also binds variables introduced by the pattern into the current scope.
    fn check_pattern_type(&mut self, pat: &Pattern, scr_ty: &Ty) {
        match pat {
            Pattern::Wildcard(_) => {}
            Pattern::Binding(n) => {
                // Bind the variable with the scrutinee's type
                self.bind(n.text.clone(), scr_ty.clone());
            }
            Pattern::Literal(expr) => {
                let lit_ty = self.infer_expr(expr);
                if !lit_ty.is_error()
                    && !scr_ty.is_error()
                    && !lit_ty.contains_infer()
                    && !scr_ty.contains_infer()
                    && !lit_ty.is_assignable_to(scr_ty)
                {
                    self.sink.emit(
                        Diagnostic::error(
                            "type.pattern_mismatch",
                            format!(
                                "pattern type `{}` does not match scrutinee `{}`",
                                lit_ty.display(),
                                scr_ty.display()
                            ),
                        )
                        .with_label(Label::primary(self.file_id, expr.span(), "pattern here"))
                        .with_action(format!("expected pattern of type `{}`", scr_ty.display())),
                    );
                }
            }
            Pattern::Some(inner, _) => {
                let inner_ty = match scr_ty {
                    Ty::Optional(t) => *t.clone(),
                    _ => Ty::Infer(0),
                };
                self.check_pattern_type(inner, &inner_ty);
            }
            Pattern::None(_) => {}
            Pattern::Success(inner, _) => {
                let ok_ty = match scr_ty {
                    Ty::Result(ok, _) => *ok.clone(),
                    _ => Ty::Infer(0),
                };
                self.check_pattern_type(inner, &ok_ty);
            }
            Pattern::Error(inner, _) => {
                let err_ty = match scr_ty {
                    Ty::Result(_, err) => *err.clone(),
                    _ => Ty::Infer(0),
                };
                self.check_pattern_type(inner, &err_ty);
            }
            Pattern::VariantUnit { name, .. } => {
                self.check_enum_variant_pattern(scr_ty, name);
            }
            Pattern::VariantNamed { name, fields, .. } => {
                self.check_enum_variant_pattern(scr_ty, name);
                for f in fields {
                    // Each field sub-pattern gets Infer for now (needs struct layout)
                    self.check_pattern_type(&f.pattern, &Ty::Infer(0));
                }
            }
            Pattern::Tuple(pats, _) => {
                let elem_tys = match scr_ty {
                    Ty::Tuple(ts) => ts.clone(),
                    _ => vec![Ty::Infer(0); pats.len()],
                };
                for (p, t) in pats.iter().zip(elem_tys.iter()) {
                    self.check_pattern_type(p, t);
                }
            }
        }
    }

    fn check_enum_variant_pattern(&mut self, scr_ty: &Ty, name: &Name) {
        if let Ty::Named(def_id, _) = scr_ty {
            if self.def_map.get(*def_id).kind != DefKind::Enum {
                return;
            }
            let Some(enum_sig) = self.enum_sig(*def_id) else {
                return;
            };
            if !enum_sig
                .variants
                .iter()
                .any(|variant| variant == &name.text)
            {
                self.sink.emit(
                    Diagnostic::error(
                        "type.unknown_enum_variant",
                        format!("enum has no variant `{}`", name.text),
                    )
                    .with_label(Label::primary(self.file_id, name.span, "unknown variant"))
                    .with_action("use a variant declared on this enum"),
                );
            }
        } else if !scr_ty.is_error() && !scr_ty.contains_infer() {
            self.sink.emit(
                Diagnostic::error(
                    "type.pattern_mismatch",
                    format!("enum variant pattern cannot match `{}`", scr_ty.display()),
                )
                .with_label(Label::primary(
                    self.file_id,
                    name.span,
                    "variant pattern here",
                ))
                .with_action("use enum variant patterns only when matching enum values"),
            );
        }
    }

    /// Infer the type of an lvalue target (variable, field, or index).
    fn infer_lvalue_ty(&mut self, lv: &LValue) -> Ty {
        match lv {
            LValue::Ident(n) => self.lookup_local_var(&n.text).unwrap_or(Ty::Infer(0)),
            LValue::Field { base, field, .. } => {
                let base_ty = self.infer_lvalue_ty(base);
                if let Ty::Named(def_id, _) = &base_ty {
                    self.struct_field_ty(*def_id, field.as_str())
                        .unwrap_or(Ty::Infer(0))
                } else {
                    Ty::Infer(0)
                }
            }
            LValue::Index { base, index, .. } => {
                let base_ty = self.infer_lvalue_ty(base);
                self.infer_expr(index);
                match &base_ty {
                    Ty::List(elem) => *elem.clone(),
                    Ty::Map(_, val) => *val.clone(),
                    _ => Ty::Infer(0),
                }
            }
        }
    }
}

// ── Utilities ─────────────────────────────────────────────────────────────────

fn qualified_prefix(q: &QualifiedName) -> Option<String> {
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

fn is_builtin_type_name(q: &QualifiedName) -> bool {
    q.is_single()
        && matches!(
            q.last().text.as_str(),
            "bool"
                | "int"
                | "int8"
                | "int16"
                | "int32"
                | "int64"
                | "u8"
                | "u16"
                | "u32"
                | "u64"
                | "float"
                | "float32"
                | "float64"
                | "string"
                | "bytes"
                | "void"
        )
}

fn stdlib_func_sig(path: &str) -> Option<(Vec<Ty>, Ty)> {
    let sig = match path {
        "ori.io.print" | "ori.io.println" | "ori.io.eprint" | "ori.io.eprintln" => {
            (vec![Ty::String], Ty::Void)
        }
        "ori.io.read_line" => (vec![], Ty::String),
        "ori.string.len" => (vec![Ty::String], Ty::Int),
        "ori.string.concat" => (vec![Ty::String, Ty::String], Ty::String),
        "ori.string.split" => (vec![Ty::String, Ty::String], Ty::List(Box::new(Ty::String))),
        "ori.string.slice" => (vec![Ty::String, Ty::Int, Ty::Int], Ty::String),
        "ori.string.contains" | "ori.string.starts_with" | "ori.string.ends_with" => {
            (vec![Ty::String, Ty::String], Ty::Bool)
        }
        "ori.string.trim" | "ori.string.to_upper" | "ori.string.to_lower" => {
            (vec![Ty::String], Ty::String)
        }
        "ori.string.replace" => (vec![Ty::String, Ty::String, Ty::String], Ty::String),
        "ori.string.chars" => (vec![Ty::String], Ty::List(Box::new(Ty::String))),
        "ori.list.new" => (vec![], Ty::List(Box::new(Ty::Infer(0)))),
        "ori.list.push" => (
            vec![Ty::List(Box::new(Ty::Infer(0))), Ty::Infer(0)],
            Ty::Void,
        ),
        "ori.list.get" => (
            vec![Ty::List(Box::new(Ty::Infer(0))), Ty::Int],
            Ty::Infer(0),
        ),
        "ori.list.set" => (
            vec![Ty::List(Box::new(Ty::Infer(0))), Ty::Int, Ty::Infer(0)],
            Ty::Void,
        ),
        "ori.list.len" => (vec![Ty::List(Box::new(Ty::Infer(0)))], Ty::Int),
        "ori.list.free" => (vec![Ty::List(Box::new(Ty::Infer(0)))], Ty::Void),
        "ori.set.new" => (vec![], Ty::Set(Box::new(Ty::Infer(0)))),
        "ori.set.add" => (
            vec![Ty::Set(Box::new(Ty::Infer(0))), Ty::Infer(0)],
            Ty::Void,
        ),
        "ori.set.contains" => (
            vec![Ty::Set(Box::new(Ty::Infer(0))), Ty::Infer(0)],
            Ty::Bool,
        ),
        "ori.set.len" => (vec![Ty::Set(Box::new(Ty::Infer(0)))], Ty::Int),
        "ori.set.free" => (vec![Ty::Set(Box::new(Ty::Infer(0)))], Ty::Void),
        "ori.map.new" => (
            vec![],
            Ty::Map(Box::new(Ty::Infer(0)), Box::new(Ty::Infer(0))),
        ),
        "ori.map.set" => (
            vec![
                Ty::Map(Box::new(Ty::Infer(0)), Box::new(Ty::Infer(0))),
                Ty::Infer(0),
                Ty::Infer(0),
            ],
            Ty::Void,
        ),
        "ori.map.get" => (
            vec![
                Ty::Map(Box::new(Ty::Infer(0)), Box::new(Ty::Infer(0))),
                Ty::Infer(0),
            ],
            Ty::Infer(0),
        ),
        "ori.map.contains" => (
            vec![
                Ty::Map(Box::new(Ty::Infer(0)), Box::new(Ty::Infer(0))),
                Ty::Infer(0),
            ],
            Ty::Bool,
        ),
        "ori.map.len" => (
            vec![Ty::Map(Box::new(Ty::Infer(0)), Box::new(Ty::Infer(0)))],
            Ty::Int,
        ),
        "ori.map.free" => (
            vec![Ty::Map(Box::new(Ty::Infer(0)), Box::new(Ty::Infer(0)))],
            Ty::Void,
        ),
        "ori.math.sqrt" => (vec![Ty::Float], Ty::Float),
        "ori.math.abs" => (vec![Ty::Int], Ty::Int),
        "ori.math.min" | "ori.math.max" => (vec![Ty::Int, Ty::Int], Ty::Int),
        "string" => (vec![Ty::Int], Ty::String),
        "len" => (vec![Ty::String], Ty::Int),
        _ => return None,
    };
    Some(sig)
}

fn restore_alias(aliases: &mut HashMap<SmolStr, SmolStr>, name: &str, previous: Option<SmolStr>) {
    if let Some(value) = previous {
        aliases.insert(SmolStr::new(name), value);
    } else {
        aliases.remove(name);
    }
}

fn substitute_trait_self(ty: &Ty, trait_def_id: DefId, self_ty: &Ty) -> Ty {
    match ty {
        Ty::Named(id, args) if *id == trait_def_id && args.is_empty() => self_ty.clone(),
        Ty::Named(id, args) => Ty::Named(
            *id,
            args.iter()
                .map(|arg| substitute_trait_self(arg, trait_def_id, self_ty))
                .collect(),
        ),
        Ty::Optional(inner) => Ty::Optional(Box::new(substitute_trait_self(
            inner,
            trait_def_id,
            self_ty,
        ))),
        Ty::Result(ok, err) => Ty::Result(
            Box::new(substitute_trait_self(ok, trait_def_id, self_ty)),
            Box::new(substitute_trait_self(err, trait_def_id, self_ty)),
        ),
        Ty::List(inner) => Ty::List(Box::new(substitute_trait_self(
            inner,
            trait_def_id,
            self_ty,
        ))),
        Ty::Map(key, value) => Ty::Map(
            Box::new(substitute_trait_self(key, trait_def_id, self_ty)),
            Box::new(substitute_trait_self(value, trait_def_id, self_ty)),
        ),
        Ty::Set(inner) => Ty::Set(Box::new(substitute_trait_self(
            inner,
            trait_def_id,
            self_ty,
        ))),
        Ty::Range(inner) => Ty::Range(Box::new(substitute_trait_self(
            inner,
            trait_def_id,
            self_ty,
        ))),
        Ty::Lazy(inner) => Ty::Lazy(Box::new(substitute_trait_self(
            inner,
            trait_def_id,
            self_ty,
        ))),
        Ty::Tuple(items) => Ty::Tuple(
            items
                .iter()
                .map(|item| substitute_trait_self(item, trait_def_id, self_ty))
                .collect(),
        ),
        Ty::Func { params, ret } => Ty::Func {
            params: params
                .iter()
                .map(|param| substitute_trait_self(param, trait_def_id, self_ty))
                .collect(),
            ret: Box::new(substitute_trait_self(ret, trait_def_id, self_ty)),
        },
        _ => ty.clone(),
    }
}

fn contains_generic_param(ty: &Ty) -> bool {
    match ty {
        Ty::Param { .. } => true,
        Ty::Optional(inner)
        | Ty::List(inner)
        | Ty::Set(inner)
        | Ty::Range(inner)
        | Ty::Lazy(inner) => contains_generic_param(inner),
        Ty::Result(ok, err) | Ty::Map(ok, err) => {
            contains_generic_param(ok) || contains_generic_param(err)
        }
        Ty::Tuple(items) => items.iter().any(contains_generic_param),
        Ty::Func { params, ret } => {
            params.iter().any(contains_generic_param) || contains_generic_param(ret)
        }
        Ty::Named(_, args) => args.iter().any(contains_generic_param),
        _ => false,
    }
}

fn infer_generic_substitution(template: &Ty, actual: &Ty, subst: &mut HashMap<u32, Ty>) {
    match (template, actual) {
        (Ty::Param { index, .. }, actual) => {
            if actual.is_error() || actual.contains_infer() || contains_generic_param(actual) {
                return;
            }
            subst.entry(*index).or_insert_with(|| actual.clone());
        }
        (Ty::Optional(t), Ty::Optional(a))
        | (Ty::List(t), Ty::List(a))
        | (Ty::Set(t), Ty::Set(a))
        | (Ty::Range(t), Ty::Range(a))
        | (Ty::Lazy(t), Ty::Lazy(a)) => infer_generic_substitution(t, a, subst),
        (Ty::Result(ok_t, err_t), Ty::Result(ok_a, err_a))
        | (Ty::Map(ok_t, err_t), Ty::Map(ok_a, err_a)) => {
            infer_generic_substitution(ok_t, ok_a, subst);
            infer_generic_substitution(err_t, err_a, subst);
        }
        (Ty::Tuple(items_t), Ty::Tuple(items_a)) if items_t.len() == items_a.len() => {
            for (item_t, item_a) in items_t.iter().zip(items_a) {
                infer_generic_substitution(item_t, item_a, subst);
            }
        }
        (
            Ty::Func {
                params: params_t,
                ret: ret_t,
            },
            Ty::Func {
                params: params_a,
                ret: ret_a,
            },
        ) if params_t.len() == params_a.len() => {
            for (param_t, param_a) in params_t.iter().zip(params_a) {
                infer_generic_substitution(param_t, param_a, subst);
            }
            infer_generic_substitution(ret_t, ret_a, subst);
        }
        (Ty::Named(id_t, args_t), Ty::Named(id_a, args_a))
            if id_t == id_a && args_t.len() == args_a.len() =>
        {
            for (arg_t, arg_a) in args_t.iter().zip(args_a) {
                infer_generic_substitution(arg_t, arg_a, subst);
            }
        }
        _ => {}
    }
}

fn substitute_generic_params(ty: &Ty, subst: &HashMap<u32, Ty>) -> Ty {
    match ty {
        Ty::Param { index, .. } => subst.get(index).cloned().unwrap_or_else(|| ty.clone()),
        Ty::Optional(inner) => Ty::Optional(Box::new(substitute_generic_params(inner, subst))),
        Ty::Result(ok, err) => Ty::Result(
            Box::new(substitute_generic_params(ok, subst)),
            Box::new(substitute_generic_params(err, subst)),
        ),
        Ty::List(inner) => Ty::List(Box::new(substitute_generic_params(inner, subst))),
        Ty::Map(key, value) => Ty::Map(
            Box::new(substitute_generic_params(key, subst)),
            Box::new(substitute_generic_params(value, subst)),
        ),
        Ty::Set(inner) => Ty::Set(Box::new(substitute_generic_params(inner, subst))),
        Ty::Range(inner) => Ty::Range(Box::new(substitute_generic_params(inner, subst))),
        Ty::Lazy(inner) => Ty::Lazy(Box::new(substitute_generic_params(inner, subst))),
        Ty::Tuple(items) => Ty::Tuple(
            items
                .iter()
                .map(|item| substitute_generic_params(item, subst))
                .collect(),
        ),
        Ty::Func { params, ret } => Ty::Func {
            params: params
                .iter()
                .map(|param| substitute_generic_params(param, subst))
                .collect(),
            ret: Box::new(substitute_generic_params(ret, subst)),
        },
        Ty::Named(def_id, args) => Ty::Named(
            *def_id,
            args.iter()
                .map(|arg| substitute_generic_params(arg, subst))
                .collect(),
        ),
        _ => ty.clone(),
    }
}

fn param_default_expr(kind: &ParamKind) -> Option<&Expr> {
    match kind {
        ParamKind::Default(expr) | ParamKind::DefaultAndContract(expr, _) => Some(expr),
        _ => None,
    }
}

fn param_binding_ty(param: &ori_ast::item::Param, ty: Ty) -> Ty {
    if matches!(param.kind, ParamKind::Variadic) {
        Ty::List(Box::new(ty))
    } else {
        ty
    }
}

fn param_contract_expr(kind: &ParamKind) -> Option<&Expr> {
    match kind {
        ParamKind::Contract(expr) | ParamKind::DefaultAndContract(_, expr) => Some(expr),
        _ => None,
    }
}

fn min_required_arg_count(param_count: usize, param_defaults: &[bool]) -> usize {
    (0..param_count)
        .rfind(|index| !param_defaults.get(*index).copied().unwrap_or(false))
        .map(|index| index + 1)
        .unwrap_or(0)
}

fn display_tys(types: &[Ty]) -> String {
    types
        .iter()
        .map(|ty| ty.display())
        .collect::<Vec<_>>()
        .join(", ")
}

/// Extract the element type from a `list<T>`, `set<T>`, `range<T>`, or string.
fn elem_of(ty: &Ty) -> Option<Ty> {
    match ty {
        Ty::List(t) | Ty::Set(t) | Ty::Range(t) => Some(*t.clone()),
        Ty::Map(key, _) => Some(*key.clone()),
        Ty::String => Some(Ty::String), // string iteration yields strings (grapheme clusters)
        _ => None,
    }
}

fn for_second_binding_ty(ty: &Ty) -> Ty {
    match ty {
        Ty::Map(_, value) => *value.clone(),
        _ => Ty::Int,
    }
}
