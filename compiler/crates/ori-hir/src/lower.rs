use smol_str::SmolStr;
use std::collections::HashMap;
use ori_diagnostics::{DiagnosticSink, FileId, Span};
use ori_ast::common::Visibility;
use ori_ast::expr::{Expr, FStrPart};
use ori_ast::item::{Item, SourceFile};
use ori_ast::stmt::{Block, MatchCase, Stmt};
use ori_types::{DefMap, Ty, lower_type};
use crate::hir::*;

/// Maps an Ori stdlib qualified path to the C function name used at link time.
fn stdlib_c_name(ori_path: &str) -> Option<&'static str> {
    match ori_path {
        // ori.io
        "ori.io.print" | "ori.io.println"  => Some("ori_io_print"),
        "ori.io.eprint" | "ori.io.eprintln" => Some("ori_io_eprint"),
        "ori.io.read_line"                 => Some("ori_io_read_line"),
        // ori.string
        "ori.string.len"    => Some("ori_string_len"),
        "ori.string.concat" => Some("ori_string_concat"),
        "ori.string.slice"  => Some("ori_string_slice"),
        // builtin conversion functions
        "string" => Some("ori_to_string"),
        "int"    => Some("ori_to_int"),
        "float"  => Some("ori_to_float"),
        "len"    => Some("ori_len"),
        // list operations (used as method calls: list.push, list.get, etc.)
        "ori.list.new"  | "list.new"  => Some("ori_list_new"),
        "ori.list.push" | "list.push" => Some("ori_list_push"),
        "ori.list.get"  | "list.get"  => Some("ori_list_get"),
        "ori.list.set"  | "list.set"  => Some("ori_list_set"),
        "ori.list.len"  | "list.len"  => Some("ori_list_len"),
        "ori.list.free" | "list.free" => Some("ori_list_free"),
        // ori.math
        "ori.math.sqrt" => Some("sqrt"),
        "ori.math.abs"  => Some("ori_math_abs"),
        "ori.math.min"  => Some("ori_math_min"),
        "ori.math.max"  => Some("ori_math_max"),
        _ => None,
    }
}

// ── Scope stack ───────────────────────────────────────────────────────────────

#[derive(Default)]
struct Scope {
    vars: HashMap<SmolStr, Ty>,
}

struct Lowerer<'a> {
    def_map:   &'a DefMap,
    namespace: &'a str,
    file_id:   FileId,
    sink:      &'a mut DiagnosticSink,
    scopes:    Vec<Scope>,
    /// `import ori.io as io` → `io` maps to `ori.io`.
    aliases:   HashMap<SmolStr, SmolStr>,
    /// Current function's return type (for `?` desugaring).
    ret_ty:    Ty,
}

impl<'a> Lowerer<'a> {
    fn new(def_map: &'a DefMap, namespace: &'a str, file_id: FileId, sink: &'a mut DiagnosticSink) -> Self {
        Self { def_map, namespace, file_id, sink, scopes: vec![Scope::default()],
               aliases: HashMap::new(), ret_ty: Ty::Void }
    }

    /// Resolve `io.print` → `ori.io.print` using the import alias map,
    /// then look up in the stdlib table.
    fn resolve_stdlib(&self, name: &str) -> Option<&'static str> {
        // Direct hit (e.g., builtin `string`, `len`, `int`)
        if let Some(c) = stdlib_c_name(name) { return Some(c); }
        // Qualified via alias: `io.print` where `io → ori.io`
        if let Some(dot) = name.find('.') {
            let prefix = &name[..dot];
            let suffix = &name[dot + 1..];
            if let Some(full_ns) = self.aliases.get(prefix) {
                let full = format!("{}.{}", full_ns, suffix);
                return stdlib_c_name(&full);
            }
        }
        None
    }
    fn push(&mut self) { self.scopes.push(Scope::default()); }
    fn pop(&mut self)  { self.scopes.pop(); }
    fn bind(&mut self, name: SmolStr, ty: Ty) {
        if let Some(s) = self.scopes.last_mut() { s.vars.insert(name, ty); }
    }
    fn lookup(&self, name: &str) -> Ty {
        for s in self.scopes.iter().rev() {
            if let Some(t) = s.vars.get(name) { return t.clone(); }
        }
        Ty::Error
    }
    fn lower_ast_ty(&mut self, t: &ori_ast::ty::Type, tp: &[SmolStr]) -> Ty {
        lower_type(t, self.namespace, tp, self.def_map, self.file_id, self.sink)
    }
    fn dummy_expr(ty: Ty, span: Span) -> HirExpr {
        HirExpr { kind: HirExprKind::Unit, ty, span }
    }
    fn err_expr(span: Span) -> HirExpr {
        HirExpr { kind: HirExprKind::Unit, ty: Ty::Error, span }
    }
}

// ── Public entry ──────────────────────────────────────────────────────────────

pub fn lower(
    file:      &SourceFile,
    def_map:   &DefMap,
    namespace: &str,
    file_id:   FileId,
    sink:      &mut DiagnosticSink,
) -> HirModule {
    let mut l = Lowerer::new(def_map, namespace, file_id, sink);

    // Build alias map from imports: `import ori.io as io` → `io → ori.io`
    for import in &file.imports {
        if let Some(alias) = &import.alias {
            l.aliases.insert(alias.text.clone(), SmolStr::new(import.path.to_string()));
        }
    }
    let mut structs = Vec::new();
    let mut enums   = Vec::new();
    let mut funcs   = Vec::new();
    let mut consts  = Vec::new();

    for item in &file.items {
        match &item.item {
            Item::Struct(s) => {
                let tp: Vec<SmolStr> = s.type_params.iter().map(|p| p.name.text.clone()).collect();
                let fields = s.fields.iter().map(|f| HirField {
                    name: f.name.text.clone(),
                    ty:   l.lower_ast_ty(&f.ty, &tp),
                    span: f.span,
                }).collect();
                let path = format!("{}.{}", namespace, s.name.text);
                let def_id = def_map.lookup(&path).unwrap_or(ori_types::DefId(u32::MAX));
                structs.push(HirStruct {
                    def_id, name: s.name.text.clone(), fields,
                    is_public: s.visibility == Visibility::Public, span: s.span,
                });
            }
            Item::Enum(e) => {
                let path = format!("{}.{}", namespace, e.name.text);
                let def_id = def_map.lookup(&path).unwrap_or(ori_types::DefId(u32::MAX));
                let tp: Vec<SmolStr> = e.type_params.iter().map(|p| p.name.text.clone()).collect();
                let variants = e.variants.iter().map(|v| HirVariant {
                    name: v.name.text.clone(),
                    fields: v.fields.iter().map(|f| HirField {
                        name: f.name.text.clone(),
                        ty:   l.lower_ast_ty(&f.ty, &tp),
                        span: f.span,
                    }).collect(),
                    span: v.span,
                }).collect();
                enums.push(HirEnum {
                    def_id, name: e.name.text.clone(), variants,
                    is_public: e.visibility == Visibility::Public, span: e.span,
                });
            }
            Item::Func(f) => {
                let tp: Vec<SmolStr> = f.type_params.iter().map(|p| p.name.text.clone()).collect();
                let params: Vec<HirParam> = f.params.iter().map(|p| HirParam {
                    name: p.name.text.clone(),
                    ty:   l.lower_ast_ty(&p.ty, &tp),
                    span: p.span,
                }).collect();
                let return_ty = f.return_ty.as_ref()
                    .map(|t| l.lower_ast_ty(t, &tp))
                    .unwrap_or(Ty::Void);
                l.push();
                for p in &params { l.bind(p.name.clone(), p.ty.clone()); }
                l.ret_ty = return_ty.clone();
                let body = l.lower_block(&f.body, &tp);
                l.pop();
                let path = format!("{}.{}", namespace, f.name.text);
                let def_id = def_map.lookup(&path).unwrap_or(ori_types::DefId(u32::MAX));
                funcs.push(HirFunc {
                    def_id, name: f.name.text.clone(), params, return_ty, body,
                    is_public: f.visibility == Visibility::Public,
                    is_mut: f.is_mut, span: f.span,
                });
            }
            Item::Const(c) => {
                let value = l.lower_expr(&c.value, &[]);
                let ty    = l.lower_ast_ty(&c.ty, &[]);
                let path  = format!("{}.{}", namespace, c.name.text);
                let def_id = def_map.lookup(&path).unwrap_or(ori_types::DefId(u32::MAX));
                consts.push(HirConst {
                    def_id, name: c.name.text.clone(), ty, value,
                    is_public: c.visibility == Visibility::Public, span: c.span,
                });
            }
            _ => {} // alias, trait, implement, extern, var — not yet lowered
        }
    }

    HirModule { namespace: SmolStr::new(namespace), structs, enums, funcs, consts }
}

// ── Statement lowering ────────────────────────────────────────────────────────

impl<'a> Lowerer<'a> {
    fn lower_block(&mut self, block: &Block, tp: &[SmolStr]) -> HirBlock {
        self.push();
        let stmts = block.stmts.iter().filter_map(|s| self.lower_stmt(s, tp)).collect();
        self.pop();
        HirBlock { stmts, span: block.span }
    }

    fn lower_stmt(&mut self, stmt: &Stmt, tp: &[SmolStr]) -> Option<HirStmt> {
        match stmt {
            Stmt::Const(c) => {
                let ty  = self.lower_ast_ty(&c.ty, tp);
                let val = self.lower_expr(&c.value, tp);
                self.bind(c.name.text.clone(), ty.clone());
                Some(HirStmt::Let { name: c.name.text.clone(), ty, mutable: false, value: val, span: c.span })
            }
            Stmt::Var(v) => {
                let ty  = self.lower_ast_ty(&v.ty, tp);
                let val = self.lower_expr(&v.value, tp);
                self.bind(v.name.text.clone(), ty.clone());
                Some(HirStmt::Let { name: v.name.text.clone(), ty, mutable: true, value: val, span: v.span })
            }
            Stmt::Return(r) => {
                let val = r.value.as_ref().map(|e| self.lower_expr(e, tp));
                Some(HirStmt::Return(val, r.span))
            }
            Stmt::Break(sp)    => Some(HirStmt::Break(*sp)),
            Stmt::Continue(sp) => Some(HirStmt::Continue(*sp)),
            Stmt::Expr(e)      => Some(HirStmt::Expr(self.lower_expr(e, tp))),
            Stmt::If(i) => {
                let cond    = self.lower_expr(&i.condition, tp);
                let then    = self.lower_block(&i.then_block, tp);
                let else_ifs = i.else_ifs.iter()
                    .map(|(c, b)| (self.lower_expr(c, tp), self.lower_block(b, tp)))
                    .collect();
                let else_ = i.else_block.as_ref().map(|b| self.lower_block(b, tp));
                Some(HirStmt::If { cond, then, else_ifs, else_, span: i.span })
            }
            Stmt::While(w) => {
                let cond = self.lower_expr(&w.condition, tp);
                let body = self.lower_block(&w.body, tp);
                Some(HirStmt::While { cond, body, span: w.span })
            }
            Stmt::For(f) => {
                let iterable = self.lower_expr(&f.iterable, tp);
                let elem_ty  = elem_type(&iterable.ty);
                self.push();
                self.bind(f.binding.text.clone(), elem_ty.clone());
                let body = self.lower_block(&f.body, tp);
                self.pop();
                Some(HirStmt::For {
                    binding: f.binding.text.clone(), elem_ty, iterable, body, span: f.span,
                })
            }
            Stmt::Loop(l) => {
                let body = self.lower_block(&l.body, tp);
                Some(HirStmt::Loop { body, span: l.span })
            }
            Stmt::Match(m) => {
                let scrutinee = self.lower_expr(&m.scrutinee, tp);
                let arms = m.cases.iter().map(|c| self.lower_match_case(c, tp)).collect();
                Some(HirStmt::Match { scrutinee, arms, span: m.span })
            }
            Stmt::Assign(a) => {
                let lvalue = lower_lvalue(&a.lvalue, self, tp);
                let value  = self.lower_expr(&a.value, tp);
                Some(HirStmt::Assign { lvalue, value, span: a.span })
            }
            Stmt::CompoundAssign(c) => {
                // Lower `x += v` to `x = x + v` for v1
                let lvalue = lower_lvalue(&c.lvalue, self, tp);
                let cur    = lvalue_to_expr(&lvalue, c.span);
                let rhs    = self.lower_expr(&c.value, tp);
                let op     = compound_op_to_binary(c.op);
                let ty     = binary_result_ty(op, &cur.ty, &rhs.ty);
                let value  = HirExpr {
                    kind: HirExprKind::Binary { op, lhs: Box::new(cur), rhs: Box::new(rhs) },
                    ty, span: c.span,
                };
                Some(HirStmt::Assign { lvalue, value, span: c.span })
            }
            _ => None, // using, check, ifsome, whilesome — TODO
        }
    }

    fn lower_match_case(&mut self, case: &MatchCase, tp: &[SmolStr]) -> HirArm {
        match case {
            MatchCase::Pattern { pattern, body, span, .. } => {
                let pat   = lower_pattern(pattern);
                let stmts = body.iter().filter_map(|s| self.lower_stmt(s, tp)).collect();
                HirArm { pattern: pat, body: stmts, span: *span }
            }
            MatchCase::Else { body, span } => {
                let stmts = body.iter().filter_map(|s| self.lower_stmt(s, tp)).collect();
                HirArm { pattern: HirPattern::Wildcard, body: stmts, span: *span }
            }
        }
    }
}

// ── Expression lowering ───────────────────────────────────────────────────────

impl<'a> Lowerer<'a> {
    pub fn lower_expr(&mut self, expr: &Expr, tp: &[SmolStr]) -> HirExpr {
        let span = expr.span();
        match expr {
            Expr::BoolLit(b, _) => HirExpr { kind: HirExprKind::BoolLit(*b), ty: Ty::Bool, span },
            Expr::IntLit { raw, .. } => {
                let v: i64 = parse_int_lit(raw);
                HirExpr { kind: HirExprKind::IntLit(v), ty: Ty::Int, span }
            }
            Expr::FloatLit { raw, .. } => {
                let v: f64 = raw.parse().unwrap_or(0.0);
                HirExpr { kind: HirExprKind::FloatLit(v), ty: Ty::Float, span }
            }
            Expr::StrLit { value, .. } =>
                HirExpr { kind: HirExprKind::StrLit(value.clone()), ty: Ty::String, span },
            Expr::FStrLit { parts, .. } => {
                let hparts = parts.iter().map(|p| match p {
                    FStrPart::Literal(s) => HirStrPart::Literal(s.clone()),
                    FStrPart::Interpolated(e) => HirStrPart::Expr(self.lower_expr(e, tp)),
                }).collect();
                HirExpr { kind: HirExprKind::InterpolatedStr(hparts), ty: Ty::String, span }
            }
            Expr::BytesLit { bytes, .. } =>
                HirExpr { kind: HirExprKind::BytesLit(bytes.clone()), ty: Ty::Bytes, span },
            Expr::None(_) =>
                HirExpr { kind: HirExprKind::None_, ty: Ty::Optional(Box::new(Ty::Infer(0))), span },
            Expr::SelfExpr(_) =>
                HirExpr { kind: HirExprKind::Var(SmolStr::new("self")), ty: self.lookup("self"), span },
            Expr::Ident(n) => {
                let ty = self.lookup(&n.text);
                HirExpr { kind: HirExprKind::Var(n.text.clone()), ty, span }
            }
            Expr::QualifiedIdent(q) => {
                let path = q.to_string();
                // Check stdlib / alias resolution first
                if let Some(c_name) = self.resolve_stdlib(&path) {
                    return HirExpr {
                        kind: HirExprKind::Var(SmolStr::new(c_name)),
                        ty:   Ty::Infer(0),
                        span,
                    };
                }
                if q.is_single() {
                    let name = q.last().text.clone();
                    let ty = self.lookup(&name);
                    HirExpr { kind: HirExprKind::Var(name), ty, span }
                } else {
                    let ty = if let Some(id) = self.def_map.lookup(&path) {
                        Ty::Named(id, Vec::new())
                    } else { Ty::Error };
                    HirExpr { kind: HirExprKind::Var(SmolStr::new(&path)), ty, span }
                }
            }
            Expr::Binary { op, lhs, rhs, .. } => {
                let l = self.lower_expr(lhs, tp);
                let r = self.lower_expr(rhs, tp);
                let ty = binary_result_ty(*op, &l.ty, &r.ty);
                HirExpr { kind: HirExprKind::Binary { op: *op, lhs: Box::new(l), rhs: Box::new(r) }, ty, span }
            }
            Expr::Unary { op, operand, .. } => {
                let e = self.lower_expr(operand, tp);
                let ty = e.ty.clone();
                HirExpr { kind: HirExprKind::Unary { op: *op, operand: Box::new(e) }, ty, span }
            }
            Expr::Field { object, field, .. } => {
                let obj = self.lower_expr(object, tp);
                HirExpr { kind: HirExprKind::Field { object: Box::new(obj), field: field.text.clone() }, ty: Ty::Infer(0), span }
            }
            Expr::TupleIndex { object, index, .. } => {
                let obj = self.lower_expr(object, tp);
                HirExpr { kind: HirExprKind::TupleIndex { object: Box::new(obj), index: *index }, ty: Ty::Infer(0), span }
            }
            Expr::Call { callee, args, .. } => {
                // Intercept builtin wrapper functions before generic lowering
                if let Expr::QualifiedIdent(q) = callee.as_ref() {
                    let name = q.to_string();
                    match name.as_str() {
                        "some" | "Some" => {
                            if let Some(a) = args.first() {
                                let e = match &a.value {
                                    ori_ast::expr::ArgValue::Expr(e) | ori_ast::expr::ArgValue::Spread(e) => e,
                                };
                                let inner = self.lower_expr(e, tp);
                                let ty = Ty::Optional(Box::new(inner.ty.clone()));
                                return HirExpr { kind: HirExprKind::Some_(Box::new(inner)), ty, span };
                            }
                        }
                        "success" | "Success" => {
                            if let Some(a) = args.first() {
                                let e = match &a.value {
                                    ori_ast::expr::ArgValue::Expr(e) | ori_ast::expr::ArgValue::Spread(e) => e,
                                };
                                let inner = self.lower_expr(e, tp);
                                let ty = Ty::Result(Box::new(inner.ty.clone()), Box::new(Ty::String));
                                return HirExpr { kind: HirExprKind::Ok_(Box::new(inner)), ty, span };
                            }
                        }
                        "error" | "Error" => {
                            if let Some(a) = args.first() {
                                let e = match &a.value {
                                    ori_ast::expr::ArgValue::Expr(e) | ori_ast::expr::ArgValue::Spread(e) => e,
                                };
                                let inner = self.lower_expr(e, tp);
                                let ty = Ty::Result(Box::new(Ty::Void), Box::new(inner.ty.clone()));
                                return HirExpr { kind: HirExprKind::Err_(Box::new(inner)), ty, span };
                            }
                        }
                        _ => {}
                    }
                }
                let callee_h = self.lower_expr(callee, tp);
                let args_h: Vec<HirExpr> = args.iter().map(|a| match &a.value {
                    ori_ast::expr::ArgValue::Expr(e) => self.lower_expr(e, tp),
                    ori_ast::expr::ArgValue::Spread(e) => self.lower_expr(e, tp),
                }).collect();
                HirExpr { kind: HirExprKind::Call { callee: Box::new(callee_h), args: args_h }, ty: Ty::Infer(0), span }
            }
            Expr::Try { expr: inner, .. } => {
                let inner_h = self.lower_expr(inner, tp);
                let ty = unwrap_ty(&inner_h.ty);
                HirExpr { kind: HirExprKind::Propagate(Box::new(inner_h)), ty, span }
            }
            Expr::Range { start, end, .. } => {
                let s = self.lower_expr(start, tp);
                let e = self.lower_expr(end, tp);
                HirExpr { kind: HirExprKind::Range { start: Box::new(s), end: Box::new(e) }, ty: Ty::Range(Box::new(Ty::Int)), span }
            }
            Expr::List { elements, .. } => {
                let elems: Vec<HirExpr> = elements.iter().map(|e| self.lower_expr(e, tp)).collect();
                let elem_ty = elems.first().map(|e| e.ty.clone()).unwrap_or(Ty::Infer(0));
                let ty = Ty::List(Box::new(elem_ty.clone()));
                HirExpr { kind: HirExprKind::ListLit { elem_ty, elements: elems }, ty, span }
            }
            Expr::Tuple { elements, .. } => {
                let elems: Vec<HirExpr> = elements.iter().map(|e| self.lower_expr(e, tp)).collect();
                let tys = elems.iter().map(|e| e.ty.clone()).collect();
                HirExpr { kind: HirExprKind::TupleLit(elems), ty: Ty::Tuple(tys), span }
            }
            Expr::IfExpr { condition, then_expr, else_expr, .. } => {
                let cond = self.lower_expr(condition, tp);
                let then = self.lower_expr(then_expr, tp);
                let else_ = self.lower_expr(else_expr, tp);
                let ty = then.ty.clone();
                HirExpr { kind: HirExprKind::IfExpr { cond: Box::new(cond), then: Box::new(then), else_: Box::new(else_) }, ty, span }
            }
            Expr::StructLit { ty: type_name, fields, .. } => {
                let path = format!("{}.{}", self.namespace, type_name.last().as_str());
                let def_id = self.def_map.lookup(&path)
                    .or_else(|| self.def_map.lookup(&type_name.to_string()))
                    .unwrap_or(ori_types::DefId(u32::MAX));
                let ty = Ty::Named(def_id, Vec::new());
                let hfields: Vec<(SmolStr, HirExpr)> = fields.iter()
                    .map(|f| (f.name.text.clone(), self.lower_expr(&f.value, tp)))
                    .collect();
                HirExpr { kind: HirExprKind::StructLit { def_id, fields: hfields }, ty, span }
            }
            Expr::AnonStructLit { fields, .. } => {
                let hfields: Vec<(SmolStr, HirExpr)> = fields.iter()
                    .map(|f| (f.name.text.clone(), self.lower_expr(&f.value, tp)))
                    .collect();
                HirExpr { kind: HirExprKind::StructLit { def_id: ori_types::DefId(u32::MAX), fields: hfields }, ty: Ty::Infer(0), span }
            }
            Expr::EnumVariantUnit { ty: type_name, variant, .. } => {
                let name = type_name.as_ref().map(|t| format!("{}.{}", t, variant.text))
                    .unwrap_or_else(|| variant.text.to_string());
                HirExpr { kind: HirExprKind::Var(SmolStr::new(&name)), ty: Ty::Infer(0), span }
            }
            Expr::EnumVariantNamed { ty: type_name, variant, fields, .. } => {
                let def_path = type_name.as_ref()
                    .map(|t| format!("{}.{}", self.namespace, t.last().as_str()))
                    .unwrap_or_default();
                let def_id = self.def_map.lookup(&def_path)
                    .unwrap_or(ori_types::DefId(u32::MAX));
                let hfields: Vec<(SmolStr, HirExpr)> = fields.iter()
                    .map(|f| (f.name.text.clone(), self.lower_expr(&f.value, tp)))
                    .collect();
                // Enum variants with named fields are treated as struct-like
                HirExpr { kind: HirExprKind::StructLit { def_id, fields: hfields }, ty: Ty::Infer(0), span }
            }
            _ => Lowerer::err_expr(span),
        }
    }
}

// ── Pattern lowering ──────────────────────────────────────────────────────────

fn lower_pattern(pat: &ori_ast::pattern::Pattern) -> HirPattern {
    use ori_ast::pattern::Pattern;
    match pat {
        Pattern::Wildcard(_)  => HirPattern::Wildcard,
        Pattern::Binding(n)   => HirPattern::Binding(n.text.clone(), Ty::Infer(0)),
        Pattern::None(_)      => HirPattern::None_,
        Pattern::Some(p, _)   => HirPattern::Some_(Box::new(lower_pattern(p))),
        Pattern::Success(p,_) => HirPattern::Ok_(Box::new(lower_pattern(p))),
        Pattern::Error(p, _)  => HirPattern::Err_(Box::new(lower_pattern(p))),
        Pattern::Literal(e)   => match e.as_ref() {
            Expr::BoolLit(b, _)   => HirPattern::BoolLit(*b),
            Expr::IntLit { raw, ..}  => HirPattern::IntLit(parse_int_lit(raw)),
            Expr::StrLit { value, ..} => HirPattern::StrLit(value.clone()),
            _ => HirPattern::Wildcard,
        },
        Pattern::Tuple(pats, _) => HirPattern::Tuple(pats.iter().map(lower_pattern).collect()),
        _ => HirPattern::Wildcard,
    }
}

// ── Utilities ─────────────────────────────────────────────────────────────────

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
        _ => Ty::Infer(0),
    }
}

fn unwrap_ty(ty: &Ty) -> Ty {
    match ty {
        Ty::Optional(t)   => *t.clone(),
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
        LValue::Ident(n) => HirLValue::Var(n.text.clone()),
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
            kind: HirExprKind::Var(name.clone()), ty: Ty::Infer(0), span,
        },
        HirLValue::Field { base, field } => {
            let obj = lvalue_to_expr(base, span);
            HirExpr {
                kind: HirExprKind::Field { object: Box::new(obj), field: field.clone() },
                ty: Ty::Infer(0), span,
            }
        }
        HirLValue::Index { base, index } => {
            let obj = lvalue_to_expr(base, span);
            HirExpr {
                kind: HirExprKind::Index { object: Box::new(obj), index: index.clone() },
                ty: Ty::Infer(0), span,
            }
        }
    }
}

fn compound_op_to_binary(op: ori_ast::stmt::CompoundOp) -> ori_ast::expr::BinaryOp {
    use ori_ast::stmt::CompoundOp;
    use ori_ast::expr::BinaryOp;
    match op {
        CompoundOp::Add => BinaryOp::Add,
        CompoundOp::Sub => BinaryOp::Sub,
        CompoundOp::Mul => BinaryOp::Mul,
        CompoundOp::Div => BinaryOp::Div,
    }
}
