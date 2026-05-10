use std::fmt::Write as FmtWrite;
use ori_ast::expr::{BinaryOp, UnaryOp};
use ori_hir::hir::*;
use ori_types::Ty;

// ── Runtime header ────────────────────────────────────────────────────────────

const ORI_RUNTIME_H: &str = r#"/* Ori runtime — generated, do not edit */
#include <stdint.h>
#include <inttypes.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct { const char* data; size_t len; } ori_string_t;
#define ORI_STR(s) ((ori_string_t){ .data = (s), .len = sizeof(s) - 1 })

typedef struct { bool has_value; } ori_none_t;
#define ORI_NONE ((ori_none_t){ .has_value = false })

static inline ori_string_t ori_int_to_string(int64_t v) {
    char* buf = (char*)malloc(32);
    snprintf(buf, 32, "%" PRId64, v);
    return (ori_string_t){ .data = buf, .len = strlen(buf) };
}
static inline void ori_print_string(ori_string_t s) {
    fwrite(s.data, 1, s.len, stdout);
    putchar('\n');
}
"#;

// ── Codegen context ───────────────────────────────────────────────────────────

pub struct CCodegen {
    out:        String,
    indent:     usize,
    tmp_ctr:    usize,
    /// Set of top-level Ori function names (unmangled). Used to prefix calls with `ORI__`.
    func_names: std::collections::HashSet<smol_str::SmolStr>,
}

impl CCodegen {
    pub fn new() -> Self {
        Self { out: String::new(), indent: 0, tmp_ctr: 0, func_names: Default::default() }
    }

    fn fresh_tmp(&mut self) -> String {
        self.tmp_ctr += 1;
        format!("_ori_tmp{}", self.tmp_ctr)
    }

    fn emit_indent(&mut self) {
        for _ in 0..self.indent {
            self.out.push_str("    ");
        }
    }

    fn line(&mut self, s: &str) {
        self.emit_indent();
        self.out.push_str(s);
        self.out.push('\n');
    }

    fn push(&mut self) { self.indent += 1; }
    fn pop(&mut self)  { self.indent -= 1; }

    pub fn generate(mut self, module: &HirModule) -> String {
        // Collect function names for call-site mangling
        for f in &module.funcs {
            self.func_names.insert(f.name.clone());
        }

        // Preamble
        self.out.push_str(ORI_RUNTIME_H);
        self.out.push('\n');

        // Forward declarations for all structs
        for s in &module.structs {
            self.line(&format!("typedef struct {} {};", mangle(&s.name), mangle(&s.name)));
        }
        if !module.structs.is_empty() { self.out.push('\n'); }

        // Struct definitions
        for s in &module.structs {
            self.emit_struct(s);
        }

        // Enum definitions (tagged unions)
        for e in &module.enums {
            self.emit_enum(e);
        }

        // Forward declarations for functions
        for f in &module.funcs {
            let sig = self.func_signature(f);
            self.out.push_str(&sig);
            self.out.push_str(";\n");
        }
        if !module.funcs.is_empty() { self.out.push('\n'); }

        // Constant definitions
        for c in &module.consts {
            let ty_s = ty_to_c(&c.ty);
            let val_s = self.expr_to_c(&c.value);
            self.line(&format!("static const {} {} = {};", ty_s, mangle(&c.name), val_s));
        }
        if !module.consts.is_empty() { self.out.push('\n'); }

        // Function definitions
        for f in &module.funcs {
            self.emit_func(f);
        }

        // Entry point: if there is a `main` func with no params, wrap it in C main
        let has_main = module.funcs.iter().any(|f| f.name == "main" && f.params.is_empty());
        if has_main {
            self.out.push_str("int main(void) {\n");
            self.out.push_str("    ORI__main();\n");
            self.out.push_str("    return 0;\n}\n");
        }

        self.out
    }

    // ── Struct ────────────────────────────────────────────────────────────────

    fn emit_struct(&mut self, s: &HirStruct) {
        self.line(&format!("struct {} {{", mangle(&s.name)));
        self.push();
        for f in &s.fields {
            self.line(&format!("{} {};", ty_to_c(&f.ty), mangle(&f.name)));
        }
        self.pop();
        self.line("};");
        self.out.push('\n');
    }

    // ── Enum (tagged union) ───────────────────────────────────────────────────

    fn emit_enum(&mut self, e: &HirEnum) {
        // Discriminant enum
        self.line(&format!("typedef enum {{"));
        self.push();
        for v in &e.variants {
            self.line(&format!("{}__{},", mangle(&e.name), mangle(&v.name)));
        }
        self.pop();
        self.line(&format!("}} {}_tag_t;", mangle(&e.name)));
        self.out.push('\n');

        // Payload union + outer struct
        self.line(&format!("typedef struct {} {{", mangle(&e.name)));
        self.push();
        self.line(&format!("{}_tag_t tag;", mangle(&e.name)));
        self.line("union {");
        self.push();
        for v in &e.variants {
            if !v.fields.is_empty() {
                self.line("struct {");
                self.push();
                for f in &v.fields {
                    self.line(&format!("{} {};", ty_to_c(&f.ty), mangle(&f.name)));
                }
                self.pop();
                self.line(&format!("}} {};", mangle(&v.name)));
            }
        }
        self.pop();
        self.line("} payload;");
        self.pop();
        self.line(&format!("}} {};", mangle(&e.name)));
        self.out.push('\n');
    }

    // ── Functions ─────────────────────────────────────────────────────────────

    fn func_c_name(name: &str) -> String {
        format!("ORI__{}", mangle(name))
    }

    fn func_signature(&self, f: &HirFunc) -> String {
        let ret  = ty_to_c(&f.return_ty);
        let name = Self::func_c_name(&f.name);
        let params: Vec<String> = f.params.iter()
            .map(|p| format!("{} {}", ty_to_c(&p.ty), mangle(&p.name)))
            .collect();
        let param_str = if params.is_empty() { "void".into() } else { params.join(", ") };
        format!("{} {}({})", ret, name, param_str)
    }

    fn emit_func(&mut self, f: &HirFunc) {
        let sig = self.func_signature(f);
        self.out.push_str(&sig);
        self.out.push_str(" {\n");
        self.push();
        for stmt in &f.body.stmts {
            self.emit_stmt(stmt);
        }
        self.pop();
        self.line("}");
        self.out.push('\n');
    }

    // ── Statements ────────────────────────────────────────────────────────────

    fn emit_stmt(&mut self, stmt: &HirStmt) {
        match stmt {
            HirStmt::Let { name, ty, value, .. } => {
                let val_s = self.expr_to_c(value);
                self.line(&format!("{} {} = {};", ty_to_c(ty), mangle(name), val_s));
            }
            HirStmt::Assign { lvalue, value, .. } => {
                let lv = lvalue_to_c(lvalue);
                let val_s = self.expr_to_c(value);
                self.line(&format!("{} = {};", lv, val_s));
            }
            HirStmt::Return(val, _) => {
                match val {
                    Some(e) => {
                        let s = self.expr_to_c(e);
                        self.line(&format!("return {};", s));
                    }
                    None => self.line("return;"),
                }
            }
            HirStmt::Break(_)    => self.line("break;"),
            HirStmt::Continue(_) => self.line("continue;"),
            HirStmt::Expr(e) => {
                let s = self.expr_to_c(e);
                self.line(&format!("{};", s));
            }
            HirStmt::If { cond, then, else_ifs, else_, .. } => {
                let cond_s = self.expr_to_c(cond);
                self.line(&format!("if ({}) {{", cond_s));
                self.push();
                for s in &then.stmts { self.emit_stmt(s); }
                self.pop();
                for (c, b) in else_ifs {
                    let cs = self.expr_to_c(c);
                    self.line(&format!("}} else if ({}) {{", cs));
                    self.push();
                    for s in &b.stmts { self.emit_stmt(s); }
                    self.pop();
                }
                if let Some(eb) = else_ {
                    self.line("} else {");
                    self.push();
                    for s in &eb.stmts { self.emit_stmt(s); }
                    self.pop();
                }
                self.line("}");
            }
            HirStmt::While { cond, body, .. } => {
                let cond_s = self.expr_to_c(cond);
                self.line(&format!("while ({}) {{", cond_s));
                self.push();
                for s in &body.stmts { self.emit_stmt(s); }
                self.pop();
                self.line("}");
            }
            HirStmt::For { binding, iterable, body, .. } => {
                // Emit as: for (int64_t _i = 0; _i < range.end; _i++) { binding = _i; ... }
                // For v1, only int ranges are supported
                let iter_s = self.expr_to_c(iterable);
                let tmp = self.fresh_tmp();
                self.line(&format!("for (int64_t {} = ({}).__start; {} < ({}).__end; {}++) {{",
                    tmp, iter_s, tmp, iter_s, tmp));
                self.push();
                self.line(&format!("int64_t {} = {};", mangle(binding), tmp));
                for s in &body.stmts { self.emit_stmt(s); }
                self.pop();
                self.line("}");
            }
            HirStmt::Loop { body, .. } => {
                self.line("for (;;) {");
                self.push();
                for s in &body.stmts { self.emit_stmt(s); }
                self.pop();
                self.line("}");
            }
            HirStmt::Match { scrutinee, arms, .. } => {
                let scr = self.expr_to_c(scrutinee);
                let tmp = self.fresh_tmp();
                self.line(&format!("{{ {} {} = {}; (void){};", ty_to_c(&scrutinee.ty), tmp, scr, tmp));
                // Emit as if-else chain
                for (i, arm) in arms.iter().enumerate() {
                    let cond = pattern_cond(&arm.pattern, &tmp);
                    if i == 0 {
                        self.line(&format!("if ({}) {{", cond));
                    } else if cond == "1" {
                        self.line("} else {");
                    } else {
                        self.line(&format!("}} else if ({}) {{", cond));
                    }
                    self.push();
                    // Bind pattern variables
                    emit_pattern_bindings(&arm.pattern, &tmp, &mut self.out, self.indent);
                    for s in &arm.body { self.emit_stmt(s); }
                    self.pop();
                }
                self.line("} }");
            }
        }
    }

    // ── Expressions ───────────────────────────────────────────────────────────

    fn expr_to_c(&mut self, expr: &HirExpr) -> String {
        match &expr.kind {
            HirExprKind::BoolLit(b)  => if *b { "true".into() } else { "false".into() },
            HirExprKind::IntLit(n)   => format!("INT64_C({})", n),
            HirExprKind::FloatLit(f) => format!("{:.}", f),
            HirExprKind::StrLit(s)   => format!("ORI_STR(\"{}\")", escape_c_str(s)),
            HirExprKind::Unit        => "((void)0)".into(),
            HirExprKind::None_       => "ORI_NONE".into(),
            HirExprKind::Var(n) => {
                // Top-level functions get the ORI__ prefix; local vars don't
                if self.func_names.contains(n.as_str()) {
                    Self::func_c_name(n)
                } else {
                    mangle(n)
                }
            }
            HirExprKind::GlobalConst(id) => format!("ORI_CONST_{}", id.0),
            HirExprKind::Binary { op, lhs, rhs } => {
                let l = self.expr_to_c(lhs);
                let r = self.expr_to_c(rhs);
                format!("({} {} {})", l, binop_to_c(*op), r)
            }
            HirExprKind::Unary { op, operand } => {
                let e = self.expr_to_c(operand);
                match op {
                    UnaryOp::Neg => format!("(-{})", e),
                    UnaryOp::Not => format!("(!{})", e),
                }
            }
            HirExprKind::Field { object, field } => {
                let obj = self.expr_to_c(object);
                format!("{}.{}", obj, mangle(field))
            }
            HirExprKind::TupleIndex { object, index } => {
                let obj = self.expr_to_c(object);
                format!("{}._f{}", obj, index)
            }
            HirExprKind::Call { callee, args } => {
                let callee_s = self.expr_to_c(callee);
                let args_s: Vec<String> = args.iter().map(|a| self.expr_to_c(a)).collect();
                format!("{}({})", callee_s, args_s.join(", "))
            }
            HirExprKind::IfExpr { cond, then, else_ } => {
                let c = self.expr_to_c(cond);
                let t = self.expr_to_c(then);
                let e = self.expr_to_c(else_);
                format!("({} ? {} : {})", c, t, e)
            }
            HirExprKind::Some_(inner) => {
                let i = self.expr_to_c(inner);
                format!("({{ .has_value = true, .value = {} }})", i)
            }
            HirExprKind::Propagate(inner) => {
                // Simplified: just unwrap (panics not supported yet)
                let i = self.expr_to_c(inner);
                format!("{}.value", i)
            }
            HirExprKind::InterpolatedStr(_) => "ORI_STR(\"\")".into(), // TODO: full interpolation
            HirExprKind::BytesLit(_)        => "/* bytes */NULL".into(),
            HirExprKind::ListLit { .. }     => "/* list */NULL".into(),
            HirExprKind::TupleLit(_)        => "/* tuple */".into(),
            HirExprKind::Range { start, end } => {
                let s = self.expr_to_c(start);
                let e = self.expr_to_c(end);
                format!("((ori_range_t){{ .__start = {}, .__end = {} }})", s, e)
            }
            HirExprKind::StructLit { fields, .. } => {
                let fields_s: Vec<String> = fields.iter()
                    .map(|(n, e)| { let es = self.expr_to_c(e); format!(".{} = {}", mangle(n), es) })
                    .collect();
                format!("({{ {} }})", fields_s.join(", "))
            }
            HirExprKind::Ok_(inner) => {
                let i = self.expr_to_c(inner);
                format!("({{ .is_ok = true, .value.ok = {} }})", i)
            }
            HirExprKind::Err_(inner) => {
                let i = self.expr_to_c(inner);
                format!("({{ .is_ok = false, .value.err = {} }})", i)
            }
            HirExprKind::MethodCall { receiver, method, args } => {
                let r = self.expr_to_c(receiver);
                let as_: Vec<String> = args.iter().map(|a| self.expr_to_c(a)).collect();
                format!("ori__{}({}, {})", mangle(method), r, as_.join(", "))
            }
            HirExprKind::Index { object, index } => {
                let o = self.expr_to_c(object);
                let i = self.expr_to_c(index);
                format!("{}[{}]", o, i)
            }
            HirExprKind::Closure => "/* closure */NULL".into(),
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn ty_to_c(ty: &Ty) -> String {
    match ty {
        Ty::Bool    => "bool".into(),
        Ty::Int     => "int64_t".into(),
        Ty::Int8    => "int8_t".into(),
        Ty::Int16   => "int16_t".into(),
        Ty::Int32   => "int32_t".into(),
        Ty::Int64   => "int64_t".into(),
        Ty::U8      => "uint8_t".into(),
        Ty::U16     => "uint16_t".into(),
        Ty::U32     => "uint32_t".into(),
        Ty::U64     => "uint64_t".into(),
        Ty::Float | Ty::Float64 => "double".into(),
        Ty::Float32 => "float".into(),
        Ty::String  => "ori_string_t".into(),
        Ty::Bytes   => "uint8_t*".into(),
        Ty::Void    => "void".into(),
        Ty::Never   => "void".into(),
        Ty::Optional(t) => format!("ori_opt_{}_t", ty_tag(t)),
        Ty::Result(ok, err) => format!("ori_result_{}_{}_t", ty_tag(ok), ty_tag(err)),
        Ty::List(t)  => format!("ori_list_{}_t", ty_tag(t)),
        Ty::Tuple(_) => "void*".into(), // TODO: named tuple struct
        Ty::Named(id, _) => format!("ori_def_{}_t", id.0),
        Ty::Range(_) => "ori_range_t".into(),
        _ => "void*".into(),
    }
}

fn ty_tag(ty: &Ty) -> String {
    match ty {
        Ty::Bool    => "bool".into(),
        Ty::Int     => "i64".into(),
        Ty::Float   => "f64".into(),
        Ty::String  => "str".into(),
        Ty::Named(id,_) => format!("def{}", id.0),
        _ => "any".into(),
    }
}

fn mangle(name: &str) -> String {
    // Replace characters invalid in C identifiers
    name.replace(['.', '-', '<', '>', ' '], "_")
}

fn mangle_ns(ns: &str, name: &str) -> String {
    let ns_m = ns.replace('.', "_");
    format!("ORI__{}__{}", ns_m, mangle(name))
}

fn mangle_ns_str(s: &str) -> String { mangle(s) }

fn binop_to_c(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::Add => "+",  BinaryOp::Sub => "-",
        BinaryOp::Mul => "*",  BinaryOp::Div => "/",  BinaryOp::Rem => "%",
        BinaryOp::Eq  => "==", BinaryOp::Ne  => "!=",
        BinaryOp::Lt  => "<",  BinaryOp::Le  => "<=",
        BinaryOp::Gt  => ">",  BinaryOp::Ge  => ">=",
        BinaryOp::And => "&&", BinaryOp::Or  => "||",
    }
}

fn lvalue_to_c(lv: &HirLValue) -> String {
    match lv {
        HirLValue::Var(n)          => mangle(n),
        HirLValue::Field { base, field } => format!("{}.{}", lvalue_to_c(base), mangle(field)),
        HirLValue::Index { base, index } => {
            // index is an HirExpr but we can't easily call expr_to_c here without &mut self
            // For now emit a placeholder
            format!("{}[0/*TODO*/]", lvalue_to_c(base))
        }
    }
}

fn escape_c_str(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n").replace('\r', "\\r")
}

fn pattern_cond(pat: &HirPattern, scrutinee: &str) -> String {
    match pat {
        HirPattern::Wildcard      => "1".into(),
        HirPattern::BoolLit(b)    => format!("{} == {}", scrutinee, if *b { "true" } else { "false" }),
        HirPattern::IntLit(n)     => format!("{} == INT64_C({})", scrutinee, n),
        HirPattern::StrLit(_)     => "1".into(), // TODO: string comparison
        HirPattern::None_         => format!("!{}.has_value", scrutinee),
        HirPattern::Some_(_)      => format!("{}.has_value", scrutinee),
        HirPattern::Binding(_, _) => "1".into(), // always matches
        _                         => "1".into(),
    }
}

fn emit_pattern_bindings(pat: &HirPattern, scrutinee: &str, out: &mut String, indent: usize) {
    let pad = "    ".repeat(indent);
    if let HirPattern::Binding(name, _) = pat {
        let _ = writeln!(out, "{}__auto_type {} = {};", pad, mangle(name), scrutinee);
    }
    if let HirPattern::Some_(inner) = pat {
        let inner_s = format!("{}.value", scrutinee);
        emit_pattern_bindings(inner, &inner_s, out, indent);
    }
}
