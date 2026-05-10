use smol_str::SmolStr;
use std::collections::HashMap;
use ori_diagnostics::{Diagnostic, DiagnosticSink, FileId, Label};
use ori_ast::expr::{BinaryOp, Expr, UnaryOp};
use ori_ast::item::{FuncDecl, Item, SourceFile};
use ori_ast::stmt::{Block, Stmt};
use crate::def::{DefMap};
use crate::lower::lower_type;
use crate::ty::Ty;

// ── Environment ───────────────────────────────────────────────────────────────

/// A lexical scope: maps variable names to their types.
#[derive(Debug, Default, Clone)]
struct Scope {
    vars: HashMap<SmolStr, Ty>,
}

impl Scope {
    fn bind(&mut self, name: SmolStr, ty: Ty) { self.vars.insert(name, ty); }
    fn get(&self, name: &str) -> Option<&Ty> { self.vars.get(name) }
}

// ── Checker ───────────────────────────────────────────────────────────────────

pub struct Checker<'a> {
    def_map:   &'a DefMap,
    namespace: &'a str,
    file_id:   FileId,
    sink:      &'a mut DiagnosticSink,
    scopes:    Vec<Scope>,
}

impl<'a> Checker<'a> {
    pub fn new(
        def_map:   &'a DefMap,
        namespace: &'a str,
        file_id:   FileId,
        sink:      &'a mut DiagnosticSink,
    ) -> Self {
        Self { def_map, namespace, file_id, sink, scopes: vec![Scope::default()] }
    }

    pub fn check_file(&mut self, file: &SourceFile) {
        for item in &file.items {
            if let Item::Func(f) = &item.item {
                self.check_func(f);
            }
        }
    }

    fn check_func(&mut self, func: &FuncDecl) {
        let tp: Vec<SmolStr> = func.type_params.iter().map(|p| p.name.text.clone()).collect();
        self.push_scope();
        // Bind parameters into scope
        for param in &func.params {
            let ty = self.lower(&param.ty, &tp);
            self.bind(param.name.text.clone(), ty);
        }
        let expected_ret = func.return_ty.as_ref()
            .map(|t| self.lower(t, &tp))
            .unwrap_or(Ty::Void);
        self.check_block(&func.body, &expected_ret, &tp);
        self.pop_scope();
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
                if !ret_ty.is_assignable_to(expected_ret) {
                    let span = r.value.as_ref().map(|e| e.span()).unwrap_or(r.span);
                    self.sink.emit(
                        Diagnostic::error("type.return_mismatch", format!(
                            "return type `{}` does not match declared `{}`",
                            ret_ty.display(), expected_ret.display()
                        ))
                        .with_label(Label::primary(self.file_id, span, "returned here"))
                        .with_why(format!("function declares return type `{}`", expected_ret.display()))
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
                self.push_scope();
                self.bind(f.binding.text.clone(), elem_ty);
                if let Some(idx) = &f.second_binding {
                    self.bind(idx.text.clone(), Ty::Int);
                }
                self.check_block(&f.body, expected_ret, tp);
                self.pop_scope();
            }
            Stmt::Loop(l)   => self.check_block(&l.body, expected_ret, tp),
            Stmt::Repeat(r) => {
                self.infer_expr(&r.count); // must be int — not enforced yet
                self.check_block(&r.body, expected_ret, tp);
            }
            Stmt::Match(m) => {
                self.infer_expr(&m.scrutinee);
                for case in &m.cases {
                    match case {
                        ori_ast::stmt::MatchCase::Pattern { body, .. } => {
                            self.push_scope();
                            for s in body { self.check_stmt(s, expected_ret, tp); }
                            self.pop_scope();
                        }
                        ori_ast::stmt::MatchCase::Else { body, .. } => {
                            self.push_scope();
                            for s in body { self.check_stmt(s, expected_ret, tp); }
                            self.pop_scope();
                        }
                    }
                }
            }
            Stmt::Check(c) => {
                let cond_ty = self.infer_expr(&c.condition);
                self.expect_bool(&cond_ty, c.condition.span());
            }
            Stmt::Expr(e) => { self.infer_expr(e); }
            // Remaining statement kinds — not checked in v1
            _ => {}
        }
    }

    // ── Expression type inference ─────────────────────────────────────────────

    pub fn infer_expr(&mut self, expr: &Expr) -> Ty {
        match expr {
            Expr::BoolLit(..)  => Ty::Bool,
            Expr::IntLit { .. }   => Ty::Int,
            Expr::FloatLit { .. } => Ty::Float,
            Expr::StrLit { .. }   => Ty::String,
            Expr::FStrLit { .. }  => Ty::String,
            Expr::BytesLit { .. } => Ty::Bytes,
            Expr::None(_)         => Ty::Optional(Box::new(Ty::Infer(0))),
            Expr::SelfExpr(_)     => Ty::Infer(0), // resolved by method checker
            Expr::Ident(n) => {
                self.lookup_var(&n.text, n.span)
            }
            Expr::QualifiedIdent(q) => {
                // Single-segment names may be local variables — check scope first
                if q.is_single() {
                    let name = q.last().as_str();
                    // Try local scope
                    for scope in self.scopes.iter().rev() {
                        if let Some(ty) = scope.get(name) { return ty.clone(); }
                    }
                }
                // Fall back to global def_map
                let path = q.to_string();
                if let Some(id) = self.def_map.lookup(&path) {
                    let def = self.def_map.get(id);
                    match def.kind {
                        crate::def::DefKind::Const | crate::def::DefKind::Var => Ty::Infer(id.0),
                        _ => Ty::Infer(0),
                    }
                } else {
                    Ty::Infer(0)
                }
            }
            Expr::Range { .. } => Ty::Range(Box::new(Ty::Int)),
            Expr::List { elements, .. } => {
                let elem = elements.first().map(|e| self.infer_expr(e)).unwrap_or(Ty::Infer(0));
                Ty::List(Box::new(elem))
            }
            Expr::Tuple { elements, .. } => {
                Ty::Tuple(elements.iter().map(|e| self.infer_expr(e)).collect())
            }
            Expr::Unary { op, operand, span } => {
                let t = self.infer_expr(operand);
                match op {
                    UnaryOp::Neg => {
                        if !t.is_numeric() && !t.is_error() {
                            self.sink.emit(Diagnostic::error("type.unary_neg_non_numeric",
                                format!("unary `-` applied to non-numeric type `{}`", t.display()))
                                .with_label(Label::primary(self.file_id, *span, "here")));
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
            Expr::Field { object, field, span } => {
                // Field access — type unknown until struct layout is resolved
                let _ = self.infer_expr(object);
                let _ = (field, span);
                Ty::Infer(0)
            }
            Expr::Call { callee, .. } => {
                let _ = self.infer_expr(callee);
                Ty::Infer(0) // return type unknown without full signature lookup
            }
            Expr::Try { expr, .. } => {
                let inner = self.infer_expr(expr);
                // `expr?` unwraps result/optional
                match &inner {
                    Ty::Result(ok, _)  => *ok.clone(),
                    Ty::Optional(t)    => *t.clone(),
                    _ => inner,
                }
            }
            Expr::IfExpr { condition, then_expr, else_expr, span } => {
                let cond_ty = self.infer_expr(condition);
                self.expect_bool(&cond_ty, condition.span());
                let then_ty = self.infer_expr(then_expr);
                let else_ty = self.infer_expr(else_expr);
                if then_ty != else_ty && !then_ty.is_error() && !else_ty.is_error() {
                    self.sink.emit(Diagnostic::error("type.if_branch_mismatch", format!(
                        "`if` branches have different types: `{}` vs `{}`",
                        then_ty.display(), else_ty.display()
                    ))
                    .with_label(Label::primary(self.file_id, *span, "branches diverge")));
                    return Ty::Error;
                }
                then_ty
            }
            _ => Ty::Infer(0),
        }
    }

    fn infer_binary(&mut self, op: BinaryOp, lt: &Ty, rt: &Ty, span: ori_diagnostics::Span) -> Ty {
        use BinaryOp::*;
        match op {
            Add | Sub | Mul | Div | Rem => {
                if lt.is_numeric() && lt == rt { lt.clone() }
                else if lt.is_error() || rt.is_error() { Ty::Error }
                else {
                    self.sink.emit(Diagnostic::error("type.arithmetic_type_mismatch", format!(
                        "arithmetic operator requires matching numeric types, got `{}` and `{}`",
                        lt.display(), rt.display()
                    )).with_label(Label::primary(self.file_id, span, "here")));
                    Ty::Error
                }
            }
            Eq | Ne | Lt | Le | Gt | Ge => {
                if lt == rt || lt.is_error() || rt.is_error() { Ty::Bool }
                else {
                    self.sink.emit(Diagnostic::error("type.comparison_type_mismatch", format!(
                        "comparison between `{}` and `{}`",
                        lt.display(), rt.display()
                    )).with_label(Label::primary(self.file_id, span, "here")));
                    Ty::Bool
                }
            }
            And | Or => Ty::Bool,
        }
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn lower(&mut self, ty: &ori_ast::ty::Type, type_params: &[SmolStr]) -> Ty {
        lower_type(ty, self.namespace, type_params, self.def_map, self.file_id, self.sink)
    }

    fn push_scope(&mut self) { self.scopes.push(Scope::default()); }
    fn pop_scope(&mut self)  { self.scopes.pop(); }

    fn bind(&mut self, name: SmolStr, ty: Ty) {
        if let Some(s) = self.scopes.last_mut() { s.bind(name, ty); }
    }

    fn lookup_var(&mut self, name: &str, span: ori_diagnostics::Span) -> Ty {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) { return ty.clone(); }
        }
        self.sink.emit(
            Diagnostic::error("name.undefined", format!("undefined variable `{}`", name))
                .with_label(Label::primary(self.file_id, span, "not in scope"))
                .with_action("declare the variable with `const` or `var` before using it"),
        );
        Ty::Error
    }

    fn expect_bool(&mut self, ty: &Ty, span: ori_diagnostics::Span) {
        if ty != &Ty::Bool && !ty.is_error() {
            self.sink.emit(
                Diagnostic::error("type.expected_bool",
                    format!("expected `bool`, found `{}`", ty.display()))
                    .with_label(Label::primary(self.file_id, span, "this expression"))
                    .with_action("use a boolean expression here"),
            );
        }
    }

    fn expect_assignable(&mut self, from: &Ty, to: &Ty, span: ori_diagnostics::Span) {
        if !from.is_assignable_to(to) {
            self.sink.emit(
                Diagnostic::error("type.type_mismatch",
                    format!("type mismatch: expected `{}`, found `{}`", to.display(), from.display()))
                    .with_label(Label::primary(self.file_id, span, "this expression"))
                    .with_action(format!("change the expression to produce `{}`", to.display())),
            );
        }
    }
}

// ── Utilities ─────────────────────────────────────────────────────────────────

/// Extract the element type from a `list<T>`, `set<T>`, `range<T>`, or string.
fn elem_of(ty: &Ty) -> Option<Ty> {
    match ty {
        Ty::List(t) | Ty::Set(t) | Ty::Range(t) => Some(*t.clone()),
        Ty::String => Some(Ty::String), // string iteration yields strings (grapheme clusters)
        _ => None,
    }
}
