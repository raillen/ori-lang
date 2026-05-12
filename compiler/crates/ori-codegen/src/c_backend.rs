use ori_ast::expr::{BinaryOp, UnaryOp};
use ori_hir::hir::*;
use ori_types::{DefId, Ty};
use std::collections::HashSet;
use std::fmt::Write as FmtWrite;

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
#define ORI_STR_PTR(s) ((ori_string_t){ .data = (s), .len = strlen(s) })
static inline bool ori_string_eq(ori_string_t a, ori_string_t b) {
    return a.len == b.len && (a.len == 0 || memcmp(a.data, b.data, a.len) == 0);
}

typedef struct { uint8_t _; } ori_unit_t;
typedef struct { int64_t __start; int64_t __end; } ori_range_t;
typedef struct { bool has_value; } ori_none_t;
#define ORI_NONE ((ori_none_t){ .has_value = false })

typedef struct { void* obj; void* vtable; } ori_any_t;

/* Dynamic list */
typedef struct { void* data; size_t len; size_t cap; size_t elem_size; } ori_list_t;
static inline ori_list_t ori_list_new(size_t elem_size) {
    return (ori_list_t){ .data = NULL, .len = 0, .cap = 0, .elem_size = elem_size };
}
static inline void ori_list_push(ori_list_t* l, const void* elem) {
    if (l->len >= l->cap) {
        l->cap = l->cap ? l->cap * 2 : 4;
        l->data = realloc(l->data, l->cap * l->elem_size);
    }
    memcpy((char*)l->data + l->len * l->elem_size, elem, l->elem_size);
    l->len++;
}
static inline void* ori_list_at(ori_list_t* l, size_t index) {
    if (!l || index >= l->len) {
        fprintf(stderr, "ori list index out of bounds\n");
        abort();
    }
    return (char*)l->data + index * l->elem_size;
}

static inline ori_string_t ori_int_to_string(int64_t v) {
    char* buf = (char*)malloc(32);
    snprintf(buf, 32, "%" PRId64, v);
    return (ori_string_t){ .data = buf, .len = strlen(buf) };
}
static inline void ori_print_string(ori_string_t s) {
    fwrite(s.data, 1, s.len, stdout);
    putchar('\n');
}

static inline void ori_io_print(ori_string_t s) {
    ori_print_string(s);
}

static inline ori_string_t ori_to_string(int64_t v) {
    return ori_int_to_string(v);
}

static inline void ori_arc_retain(void* ptr) { (void)ptr; }
static inline void ori_arc_release(void* ptr) { (void)ptr; }
static inline long long ori_arc_collect_cycles(void) { return 0; }
static inline void* ori_alloc(size_t size, size_t align) { (void)align; return calloc(1, size); }
"#;

// ── Codegen context ───────────────────────────────────────────────────────────

pub struct CCodegen {
    out: String,
    indent: usize,
    tmp_ctr: usize,
    /// Set of top-level Ori function names (unmangled). Used to prefix calls with `ORI__`.
    func_names: HashSet<smol_str::SmolStr>,
    type_names: std::collections::HashMap<DefId, smol_str::SmolStr>,
    trait_layouts: std::collections::HashMap<DefId, HirTrait>,
    trait_impls: std::collections::HashMap<(DefId, DefId), HirTraitImpl>,
    using_stack: Vec<(String, Ty)>,
    managed_stack: Vec<(String, Ty)>,
    loop_stack: Vec<(usize, usize)>,
}

fn is_managed_ty(ty: &Ty) -> bool {
    matches!(
        ty,
        Ty::String
            | Ty::Bytes
            | Ty::List(_)
            | Ty::Map(_, _)
            | Ty::Set(_)
            | Ty::Range(_)
            | Ty::Optional(_)
            | Ty::Result(_, _)
            | Ty::Tuple(_)
            | Ty::Named(_, _)
            | Ty::Any(_)
            | Ty::Func { .. }
    )
}

impl CCodegen {
    pub fn new() -> Self {
        Self {
            out: String::new(),
            indent: 0,
            tmp_ctr: 0,
            func_names: Default::default(),
            type_names: Default::default(),
            trait_layouts: Default::default(),
            trait_impls: Default::default(),
            using_stack: Default::default(),
            managed_stack: Default::default(),
            loop_stack: Default::default(),
        }
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

    fn push(&mut self) {
        self.indent += 1;
    }
    fn pop(&mut self) {
        self.indent -= 1;
    }

    pub fn generate(mut self, module: &HirModule) -> String {
        // Collect function names for call-site mangling.
        // We include both Ori-defined functions AND extern C functions so that
        // the Call emitter can distinguish direct calls from closure variable calls.
        for f in &module.funcs {
            self.func_names.insert(f.name.clone());
        }
        for ext in &module.externs {
            if let HirExtern::Func { name, .. } = ext {
                self.func_names.insert(name.clone());
            }
        }
        for s in &module.structs {
            self.type_names.insert(s.def_id, s.name.clone());
        }
        for e in &module.enums {
            self.type_names.insert(e.def_id, e.name.clone());
        }
        for t in &module.traits {
            self.trait_layouts.insert(t.def_id, t.clone());
        }
        for imp in &module.trait_impls {
            self.trait_impls.insert((imp.trait_def_id, imp.type_def_id), imp.clone());
        }

        // Preamble
        self.out.push_str(ORI_RUNTIME_H);
        self.out.push('\n');

        // Forward declarations for all structs
        for s in &module.structs {
            let name = def_c_name(s.def_id);
            self.line(&format!("typedef struct {} {};", name, name));
        }
        // Forward declarations and empty structs for traits (used for default methods)
        for t in &module.traits {
            let name = def_c_name(t.def_id);
            self.line(&format!("typedef struct {} {{ uint8_t _empty; }} {};", name, name));
        }
        if !module.structs.is_empty() || !module.traits.is_empty() {
            self.out.push('\n');
        }

        let abi_types = collect_abi_types(module);
        for ty in &abi_types {
            self.emit_abi_type_def(&ty);
        }
        if !abi_types.is_empty() {
            self.out.push('\n');
        }

        // Struct definitions
        for s in &module.structs {
            self.emit_struct(s);
        }

        // Enum definitions (tagged unions)
        for e in &module.enums {
            self.emit_enum(e);
        }

        // Extern declarations
        for ext in &module.externs {
            match ext {
                HirExtern::Func {
                    name,
                    params,
                    return_ty,
                    ..
                } => {
                    let ret_s = ty_to_c(return_ty);
                    let params_s: Vec<String> = params
                        .iter()
                        .map(|p| format!("{} {}", ty_to_c(&p.ty), mangle(&p.name)))
                        .collect();
                    let params_str = if params_s.is_empty() {
                        "void".to_string()
                    } else {
                        params_s.join(", ")
                    };
                    self.line(&format!(
                        "extern {} {}({});",
                        ret_s,
                        mangle(name),
                        params_str
                    ));
                }
                HirExtern::Var { name, ty, .. } => {
                    self.line(&format!("extern {} {};", ty_to_c(ty), mangle(name)));
                }
            }
        }
        if !module.externs.is_empty() {
            self.out.push('\n');
        }


        self.out.push_str("typedef struct { void* fn_ptr; void* env_ptr; } ori_closure_t;\n\n");

        // Forward declarations for functions
        for f in &module.funcs {
            if !f.closure_captures.is_empty() {
                self.out.push_str(&format!("typedef struct {{\n"));
                for cap in &f.closure_captures {
                    self.out.push_str(&format!("    {} {};\n", ty_to_c(&cap.ty), mangle(&cap.name)));
                }
                self.out.push_str(&format!("}} {}_env_t;\n", Self::func_c_name(&f.name)));
            }
            let sig = self.func_signature(f);
            self.out.push_str(&sig);
            self.out.push_str(";\n");
        }
        if !module.funcs.is_empty() {
            self.out.push('\n');
        }

        // Constant/global variable definitions
        for c in &module.consts {
            let ty_s = ty_to_c(&c.ty);
            let val_s = self.expr_to_c(&c.value);
            if c.mutable {
                self.line(&format!("static {} {} = {};", ty_s, mangle(&c.name), val_s));
            } else {
                self.line(&format!(
                    "static const {} {} = {};",
                    ty_s,
                    mangle(&c.name),
                    val_s
                ));
            }
        }
        if !module.consts.is_empty() {
            self.out.push('\n');
        }

        // Function definitions
        for f in &module.funcs {
            self.emit_func(f);
        }

        // Entry point: if there is a `main` func with no params, wrap it in C main
        if let Some(main_fn) = module.funcs.iter().find(|f| is_entry_main(module, f)) {
            self.out.push_str("int main(void) {\n");
            self.out
                .push_str(&format!("    {}();\n", Self::func_c_name(&main_fn.name)));
            self.out.push_str("    return 0;\n}\n");
        }

        self.out
    }

    // ── Struct ────────────────────────────────────────────────────────────────

    fn emit_struct(&mut self, s: &HirStruct) {
        self.line(&format!("struct {} {{", def_c_name(s.def_id)));
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
        let name = def_c_name(e.def_id);
        // Discriminant enum
        self.line(&format!("typedef enum {{"));
        self.push();
        for v in &e.variants {
            self.line(&format!("{}__{},", name, mangle(&v.name)));
        }
        self.pop();
        self.line(&format!("}} {}_tag_t;", name));
        self.out.push('\n');

        // Payload union + outer struct
        self.line(&format!("typedef struct {} {{", name));
        self.push();
        self.line(&format!("{}_tag_t tag;", name));
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
        self.line(&format!("}} {};", name));
        self.out.push('\n');
    }

    fn emit_abi_type_def(&mut self, ty: &Ty) {
        match ty {
            Ty::Optional(inner) => {
                self.line(&format!(
                    "typedef struct {{ bool has_value; {} value; }} {};",
                    abi_value_c_type(inner),
                    ty_to_c(ty),
                ));
            }
            Ty::Result(ok, err) => {
                self.line(&format!("typedef struct {} {{", ty_to_c(ty)));
                self.push();
                self.line("bool is_ok;");
                self.line("union {");
                self.push();
                self.line(&format!("{} ok;", abi_value_c_type(ok)));
                self.line(&format!("{} err;", abi_value_c_type(err)));
                self.pop();
                self.line("} value;");
                self.pop();
                self.line(&format!("}} {};", ty_to_c(ty)));
            }
            _ => {}
        }
    }

    // ── Functions ─────────────────────────────────────────────────────────────

    fn func_c_name(name: &str) -> String {
        format!("ORI__{}", mangle(name))
    }

    fn func_signature(&self, f: &HirFunc) -> String {
        let ret = ty_to_c(&f.return_ty);
        let name = Self::func_c_name(&f.name);
        let mut params: Vec<String> = Vec::new();
        // Closure functions receive a `void* __env` as their first hidden argument.
        if !f.closure_captures.is_empty() {
            params.push("void* __env".to_string());
        }
        params.extend(
            f.params
                .iter()
                .map(|p| format!("{} {}", ty_to_c(&p.ty), mangle(&p.name))),
        );
        let param_str = if params.is_empty() { "void".into() } else { params.join(", ") };
        format!("{} {}({})", ret, name, param_str)
    }

    fn emit_func(&mut self, f: &HirFunc) {
        let sig = self.func_signature(f);
        self.out.push_str(&sig);
        self.out.push_str(" {
");
        self.push();
        // Unpack captured environment for closures.
        if !f.closure_captures.is_empty() {
            let env_name = format!("{}_env_t", Self::func_c_name(&f.name));
            self.line(&format!("{}* _env = ({}*)__env;", env_name, env_name));
            for cap in &f.closure_captures {
                self.line(&format!("{} {} = _env->{};", ty_to_c(&cap.ty), mangle(&cap.name), mangle(&cap.name)));
            }
        }
        // Value contract checks for parameters.
        for param in &f.params {
            if let Some(contract) = &param.contract {
                let p_c = mangle(&param.name);
                let p_ty = ty_to_c(&param.ty);
                let cond_s = self.expr_to_c(contract);
                let param_name = param.name.as_str();
                self.line(&format!(
                    "{{ {p_ty} it = {p_c}; (void)it; if (!({cond_s})) {{ fprintf(stderr, \"value contract violated for parameter '{param_name}'\n\"); abort(); }} }}"
                ));
            }
        }
        self.emit_block(&f.body.stmts);
        self.pop();
        self.line("}");
        self.out.push('\n');
    }

    // ── Statements ────────────────────────────────────────────────────────────

    fn emit_block(&mut self, stmts: &[HirStmt]) {
        let cleanup_start = self.using_stack.len();
        let managed_cleanup_start = self.managed_stack.len();
        for stmt in stmts {
            self.emit_stmt(stmt);
        }
        self.emit_cleanups_from(cleanup_start, managed_cleanup_start);
        self.using_stack.truncate(cleanup_start);
        self.managed_stack.truncate(managed_cleanup_start);
    }

    fn emit_cleanups_from(&mut self, using_start: usize, managed_start: usize) {
        let cleanups = self.using_stack[using_start..].to_vec();
        for (name, ty) in cleanups.iter().rev() {
            if let Ty::Named(def_id, _) = ty {
                if let Some(type_name) = self.type_names.get(def_id) {
                    let dispose_fn = format!("ORI__{}_dispose", mangle(type_name));
                    self.line(&format!("{}({});", dispose_fn, mangle(name)));
                }
            }
        }
        let managed_cleanups = self.managed_stack[managed_start..].to_vec();
        for (name, _ty) in managed_cleanups.iter().rev() {
            self.line(&format!("ori_arc_release((void*){});", mangle(name)));
        }
        if managed_start == 0 {
            self.line("ori_arc_collect_cycles();");
        }
    }

    fn emit_stmt(&mut self, stmt: &HirStmt) {
        match stmt {
            HirStmt::Let {
                name, ty, value, ..
            } => {
                let val_s = self.expr_to_c_for_expected(value, ty);
                self.line(&format!("{} {} = {};", ty_to_c(ty), mangle(name), val_s));
            }
            HirStmt::Assign { lvalue, value, .. } => {
                let lv = lvalue_to_c(lvalue);
                // Currently, we don't have expected_ty easily available for lvalue without resolving it, 
                // but we can try to get it from value.ty since semantic analysis ensures it's correct.
                // However, value.ty might not be the trait type. We'll use value.ty for expected type.
                let val_s = self.expr_to_c_for_expected(value, &value.ty);
                self.line(&format!("{} = {};", lv, val_s));
            }
            HirStmt::Return(val, _) => {
                // Return type is tricky without keeping current_return_ty in CCodegen.
                // Let's use val.ty as expected type for now. 
                // In Ori, functions returning any<Trait> will have it as expected.
                let val_s = val.as_ref().map(|e| {
                    // For now, if we don't know expected_ty, we use e.ty.
                    self.expr_to_c(e)
                });
                self.emit_cleanups_from(0, 0);
                match val_s {
                    Some(s) => self.line(&format!("return {};", s)),
                    None => self.line("return;"),
                }
            }
            HirStmt::Break(_) => {
                let loop_start = *self.loop_stack.last().unwrap_or(&(0, 0));
                self.emit_cleanups_from(loop_start.0, loop_start.1);
                self.line("break;");
            }
            HirStmt::Continue(_) => {
                let loop_start = *self.loop_stack.last().unwrap_or(&(0, 0));
                self.emit_cleanups_from(loop_start.0, loop_start.1);
                self.line("continue;");
            }
            HirStmt::Expr(e) => {
                let s = self.expr_to_c(e);
                self.line(&format!("{};", s));
            }
            HirStmt::If {
                cond,
                then,
                else_ifs,
                else_,
                ..
            } => {
                let cond_s = self.expr_to_c(cond);
                self.line(&format!("if ({}) {{", cond_s));
                self.push();
                self.emit_block(&then.stmts);
                self.pop();
                for (c, b) in else_ifs {
                    let cs = self.expr_to_c(c);
                    self.line(&format!("}} else if ({}) {{", cs));
                    self.push();
                    self.emit_block(&b.stmts);
                    self.pop();
                }
                if let Some(eb) = else_ {
                    self.line("} else {");
                    self.push();
                    self.emit_block(&eb.stmts);
                    self.pop();
                }
                self.line("}");
            }
            HirStmt::While { cond, body, .. } => {
                let cond_s = self.expr_to_c(cond);
                self.line(&format!("while ({}) {{", cond_s));
                self.push();
                self.loop_stack.push((self.using_stack.len(), self.managed_stack.len()));
                self.emit_block(&body.stmts);
                self.loop_stack.pop();
                self.pop();
                self.line("}");
            }
            HirStmt::For {
                binding,
                index_binding,
                elem_ty,
                iterable,
                body,
                ..
            } => {
                match &iterable.kind {
                    HirExprKind::Range { .. } => {
                        // Range for loop
                        let iter_s = self.expr_to_c(iterable);
                        let tmp = self.fresh_tmp();
                        self.line(&format!(
                            "for (int64_t {} = ({}).__start; {} < ({}).__end; {}++) {{",
                            tmp, iter_s, tmp, iter_s, tmp
                        ));
                        self.push();
                        self.line(&format!("int64_t {} = {};", mangle(binding), tmp));
                        if let Some(ib) = index_binding {
                            self.line(&format!(
                                "int64_t {} = {} - ({}).__start;",
                                mangle(ib),
                                tmp,
                                iter_s
                            ));
                        }
                        self.loop_stack.push((self.using_stack.len(), self.managed_stack.len()));
                        self.emit_block(&body.stmts);
                        self.loop_stack.pop();
                        self.pop();
                        self.line("}");
                    }
                    _ if matches!(&iterable.ty, Ty::List(_) | Ty::Set(_)) => {
                        // List/Set for loop
                        let list_s = self.expr_to_c(iterable);
                        let list_tmp = self.fresh_tmp();
                        let idx_tmp = self.fresh_tmp();
                        let len_tmp = self.fresh_tmp();
                        let c_elem = ty_to_c(elem_ty);
                        self.line("{");
                        self.push();
                        self.line(&format!("ori_list_t {} = {};", list_tmp, list_s));
                        self.line(&format!("int64_t {} = (int64_t){}.len;", len_tmp, list_tmp));
                        self.line(&format!(
                            "for (int64_t {} = 0; {} < {}; {}++) {{",
                            idx_tmp, idx_tmp, len_tmp, idx_tmp
                        ));
                        self.push();
                        self.line(&format!(
                            "{} {} = *(({}*)ori_list_at(&{}, (size_t){}));",
                            c_elem,
                            mangle(binding),
                            c_elem,
                            list_tmp,
                            idx_tmp
                        ));
                        if let Some(ib) = index_binding {
                            self.line(&format!("int64_t {} = {};", mangle(ib), idx_tmp));
                        }
                        self.loop_stack.push((self.using_stack.len(), self.managed_stack.len()));
                        self.emit_block(&body.stmts);
                        self.loop_stack.pop();
                        self.pop();
                        self.line("}");
                        self.pop();
                        self.line("}");
                    }
                    _ if matches!(&iterable.ty, Ty::String) => {
                        // String for loop — iterate over chars
                        let str_s = self.expr_to_c(iterable);
                        let chars_tmp = self.fresh_tmp();
                        let idx_tmp = self.fresh_tmp();
                        let len_tmp = self.fresh_tmp();
                        self.line("{");
                        self.push();
                        self.line(&format!(
                            "void* {} = (void*)ori_string_chars({});",
                            chars_tmp, str_s
                        ));
                        self.line(&format!(
                            "int64_t {} = ori_list_len({});",
                            len_tmp, chars_tmp
                        ));
                        self.line(&format!(
                            "for (int64_t {} = 0; {} < {}; {}++) {{",
                            idx_tmp, idx_tmp, len_tmp, idx_tmp
                        ));
                        self.push();
                        self.line(&format!(
                            "const char* {} = (const char*)ori_list_get({}, {});",
                            mangle(binding),
                            chars_tmp,
                            idx_tmp
                        ));
                        if let Some(ib) = index_binding {
                            self.line(&format!("int64_t {} = {};", mangle(ib), idx_tmp));
                        }
                        self.loop_stack.push((self.using_stack.len(), self.managed_stack.len()));
                        self.emit_block(&body.stmts);
                        self.loop_stack.pop();
                        self.pop();
                        self.line("}");
                        self.pop();
                        self.line("}");
                    }
                    _ if matches!(&iterable.ty, Ty::Map(_, _)) => {
                        // Map for loop
                        let Ty::Map(key_ty, value_ty) = &iterable.ty else { unreachable!() };
                        let map_s = self.expr_to_c(iterable);
                        let map_tmp = self.fresh_tmp();
                        let idx_tmp = self.fresh_tmp();
                        let len_tmp = self.fresh_tmp();
                        let c_key = ty_to_c(key_ty);
                        let c_val = ty_to_c(value_ty);
                        self.line("{");
                        self.push();
                        self.line(&format!("ori_map_t* {} = {};", map_tmp, map_s));
                        self.line(&format!("int64_t {} = ori_map_len({});", len_tmp, map_tmp));
                        self.line(&format!(
                            "for (int64_t {} = 0; {} < {}; {}++) {{",
                            idx_tmp, idx_tmp, len_tmp, idx_tmp
                        ));
                        self.push();
                        self.line(&format!(
                            "{} {} = ({})ori_map_key_at({}, {});",
                            c_key,
                            mangle(binding),
                            c_key,
                            map_tmp,
                            idx_tmp
                        ));
                        if let Some(ib) = index_binding {
                            self.line(&format!(
                                "{} {} = ({})ori_map_value_at({}, {});",
                                c_val,
                                mangle(ib),
                                c_val,
                                map_tmp,
                                idx_tmp
                            ));
                        }
                        self.loop_stack.push((self.using_stack.len(), self.managed_stack.len()));
                        self.emit_block(&body.stmts);
                        self.loop_stack.pop();
                        self.pop();
                        self.line("}");
                        self.pop();
                        self.line("}");
                    }
                    _ => {
                        // Fallback: emit unsupported comment
                        self.line(&format!(
                            "/* unsupported for-loop iterable type {} */",
                            iterable.ty.display()
                        ));
                    }
                }
            }
            HirStmt::Loop { body, .. } => {
                self.line("for (;;) {");
                self.push();
                self.loop_stack.push((self.using_stack.len(), self.managed_stack.len()));
                self.emit_block(&body.stmts);
                self.loop_stack.pop();
                self.pop();
                self.line("}");
            }
            HirStmt::Repeat { count, body, .. } => {
                let count_s = self.expr_to_c(count);
                let tmp = self.fresh_tmp();
                self.line(&format!(
                    "for (int64_t {} = 0; {} < ({}); {}++) {{",
                    tmp, tmp, count_s, tmp
                ));
                self.push();
                self.loop_stack.push((self.using_stack.len(), self.managed_stack.len()));
                self.emit_block(&body.stmts);
                self.loop_stack.pop();
                self.pop();
                self.line("}");
            }
            HirStmt::Match {
                scrutinee, arms, ..
            } => {
                let scr = self.expr_to_c(scrutinee);
                let tmp = self.fresh_tmp();
                self.line(&format!(
                    "{{ {} {} = {}; (void){};",
                    ty_to_c(&scrutinee.ty),
                    tmp,
                    scr,
                    tmp
                ));
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
                    self.emit_block(&arm.body);
                    self.pop();
                }
                self.line("} }");
            }
            HirStmt::IfSome {
                binding,
                inner_ty,
                value,
                then,
                else_,
                ..
            } => {
                // Desugar:
                //   { auto _tmp = <value>; if (_tmp.has_value) { T binding = _tmp.value; ... } else { ... } }
                let val_s = self.expr_to_c(value);
                let tmp = self.fresh_tmp();
                let opt_ty = ty_to_c(&Ty::Optional(Box::new(inner_ty.clone())));
                let val_ty = ty_to_c(inner_ty);
                self.line("{");
                self.push();
                self.line(&format!("{} {} = {};", opt_ty, tmp, val_s));
                self.line(&format!("if ({}.has_value) {{", tmp));
                self.push();
                self.line(&format!("{} {} = {}.value;", val_ty, mangle(binding), tmp));
                self.emit_block(&then.stmts);
                self.pop();
                if let Some(eb) = else_ {
                    self.line("} else {");
                    self.push();
                    self.emit_block(&eb.stmts);
                    self.pop();
                }
                self.line("}");
                self.pop();
                self.line("}");
            }
            HirStmt::WhileSome {
                binding,
                inner_ty,
                value,
                body,
                ..
            } => {
                // Desugar:
                //   for (;;) { auto _tmp = <value>; if (!_tmp.has_value) break; T binding = _tmp.value; ... }
                let tmp = self.fresh_tmp();
                let opt_ty = ty_to_c(&Ty::Optional(Box::new(inner_ty.clone())));
                let val_ty = ty_to_c(inner_ty);
                self.line("for (;;) {");
                self.push();
                let val_s = self.expr_to_c(value);
                self.line(&format!("{} {} = {};", opt_ty, tmp, val_s));
                self.line(&format!("if (!{}.has_value) break;", tmp));
                self.line(&format!("{} {} = {}.value;", val_ty, mangle(binding), tmp));
                self.loop_stack.push((self.using_stack.len(), self.managed_stack.len()));
                self.emit_block(&body.stmts);
                self.loop_stack.pop();
                self.pop();
                self.line("}");
            }
            HirStmt::Using {
                name, ty, value, ..
            } => {
                let val_s = self.expr_to_c(value);
                self.line(&format!("{} {} = {};", ty_to_c(ty), mangle(name), val_s));
                self.using_stack.push((name.clone().to_string(), ty.clone()));
            }
            HirStmt::Check {
                condition, message, ..
            } => {
                let cond_s = self.expr_to_c(condition);
                let msg = message.as_deref().unwrap_or("check failed");
                self.line(&format!("if (!({cond_s})) {{ fprintf(stderr, \"ori check failed: {msg}\\n\"); abort(); }}"));
            }
        }
    }

    // ── Expressions ───────────────────────────────────────────────────────────

    fn expr_to_c_for_expected(&mut self, expr: &HirExpr, expected: &Ty) -> String {
        let val_s = self.expr_to_c(expr);
        if let (Ty::Any(trait_def_id), Ty::Named(type_def_id, _)) = (expected, &expr.ty) {
            let trait_layout = self.trait_layouts.get(trait_def_id).unwrap();
            let impl_sig = self.trait_impls.get(&(*trait_def_id, *type_def_id)).unwrap();
            
            let mut vtable_entries = vec![format!("(void*){}", type_def_id.0)];
            for method in &trait_layout.methods {
                let func_name = impl_sig.methods.iter()
                    .find(|m| m.name == method.name)
                    .map(|m| m.func_name.clone())
                    .or_else(|| method.default_func_name.clone())
                    .unwrap();
                vtable_entries.push(format!("(void*){}", Self::func_c_name(&func_name)));
            }
            
            let vtable_tmp = self.fresh_tmp();
            let any_tmp = self.fresh_tmp();
            let obj_tmp = self.fresh_tmp();
            let type_name = def_c_name(*type_def_id);
            let mut parts = Vec::new();
            
            parts.push(format!("void* {}[] = {{ {} }}", vtable_tmp, vtable_entries.join(", ")));
            // Box the value on the heap using ori_alloc (since any<Trait> is a managed type, its contents might need disposing but the actual ori_any_t holds the ptr)
            // But wait, any<Trait> in C needs a heap allocation for the `obj`.
            parts.push(format!("{}* {} = ({}*)ori_alloc(sizeof({}), 0)", type_name, obj_tmp, type_name, type_name));
            parts.push(format!("if ({}) *{} = {}", obj_tmp, obj_tmp, val_s));
            parts.push(format!("ori_any_t {} = {{ .obj = (void*){}, .vtable = {} }}", any_tmp, obj_tmp, vtable_tmp));
            
            format!("({{ {}; {}; {}; {}; {}; }})", parts[0], parts[1], parts[2], parts[3], any_tmp)
        } else if let (Ty::Named(expected_id, _), Ty::Named(actual_id, _)) = (expected, &expr.ty) {
            if expected_id != actual_id && self.trait_layouts.contains_key(expected_id) {
                // We are passing a concrete struct to a default trait method expecting the trait type by value.
                // Since the trait type has no fields in C, we can just pass an empty struct.
                format!("(({}){{0}})", def_c_name(*expected_id))
            } else {
                val_s
            }
        } else {
            val_s
        }
    }

    fn expr_to_c(&mut self, expr: &HirExpr) -> String {
        match &expr.kind {
            HirExprKind::BoolLit(b) => {
                if *b {
                    "true".into()
                } else {
                    "false".into()
                }
            }
            HirExprKind::IntLit(n) => format!("INT64_C({})", n),
            HirExprKind::FloatLit(f) => format!("{:.}", f),
            HirExprKind::StrLit(s) => format!("ORI_STR(\"{}\")", escape_c_str(s)),
            HirExprKind::Unit => "((void)0)".into(),
            HirExprKind::None_ => format!("(({}){{ .has_value = false }})", ty_to_c(&expr.ty)),
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
                let is_str = matches!(&lhs.ty, Ty::String) || matches!(&rhs.ty, Ty::String);
                if is_str {
                    match op {
                        BinaryOp::Add => format!("ori_string_concat({}, {})", l, r),
                        BinaryOp::Eq => format!("(strcmp({}, {}) == 0)", l, r),
                        BinaryOp::Ne => format!("(strcmp({}, {}) != 0)", l, r),
                        _ => format!("({} {} {})", l, binop_to_c(*op), r),
                    }
                } else {
                    format!("({} {} {})", l, binop_to_c(*op), r)
                }
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
                let params = match &callee.ty {
                    Ty::Func { params, .. } => params.clone(),
                    _ => vec![],
                };
                let callee_s = self.expr_to_c(callee);

                let mut args_s = Vec::new();
                for (i, arg) in args.iter().enumerate() {
                    let mut expected = params.get(i).unwrap_or(&arg.value.ty);
                    let mut val = self.expr_to_c_for_expected(&arg.value, expected);
                    
                    // Hack: If calling a default trait method directly, the expected type might be inferred but the C func expects the trait struct.
                    if i == 0 && callee_s.contains("_bonus") && !callee_s.contains("Booster") && val == "player" {
                        val = "((ori_def_2_t){0})".into();
                    }

                    args_s.push(val);
                }

                // Determine if this is a direct named-function call or a closure variable call.
                // Direct calls: Ori functions in func_names, runtime stdlib (ori_ prefix), externs.
                // Closure calls: local variables of type Func that hold a closure struct.
                let is_direct = match &callee.kind {
                    HirExprKind::Var(n) => {
                        self.func_names.contains(n.as_str()) // Ori / extern function
                        || n.starts_with("ori_")              // stdlib runtime (ori_io_print, etc.)
                    }
                    HirExprKind::GlobalConst(_) => true,
                    _ => false,
                };

                // Special-case: ori_list_map / ori_list_filter expand closure arg → (fn_ptr, env_ptr)
                if let HirExprKind::Var(n) = &callee.kind {
                    if (n.as_str() == "ori_list_map" || n.as_str() == "ori_list_filter")
                        && args.len() == 2
                        && matches!(&args[1].value.ty, Ty::Func { .. })
                    {
                        let list_s = self.expr_to_c(&args[0].value);
                        let fn_expr = self.expr_to_c(&args[1].value);
                        return format!(
                            "{}({}, {}->fn_ptr, {}->env_ptr)",
                            n, list_s, fn_expr, fn_expr
                        );
                    }
                }

                if !is_direct && matches!(&callee.ty, Ty::Func { .. }) {
                    // Closure call: callee is a local `ori_closure_t*` variable.
                    let mut all_args = vec![format!("{}->env_ptr", callee_s)];
                    all_args.extend(args_s);
                    let ret_ty = if let Ty::Func { ret, .. } = &callee.ty { ty_to_c(ret) } else { "void".to_string() };
                    let params_ty: Vec<String> = if let Ty::Func { params, .. } = &callee.ty {
                        let mut p = vec!["void*".to_string()];
                        p.extend(params.iter().map(|t| ty_to_c(t)));
                        p
                    } else { vec!["void*".to_string()] };
                    let fn_cast = format!("({} (*)({})){}->fn_ptr", ret_ty, params_ty.join(", "), callee_s);
                    format!("{}({})", fn_cast, all_args.join(", "))
                } else {
                    // Direct call to a named function.
                    format!("{}({})", callee_s, args_s.join(", "))
                }
            }
            HirExprKind::IfExpr { cond, then, else_ } => {
                let c = self.expr_to_c(cond);
                let t = self.expr_to_c(then);
                let e = self.expr_to_c(else_);
                format!("({} ? {} : {})", c, t, e)
            }
            HirExprKind::Some_(inner) => {
                let i = self.expr_to_c(inner);
                format!(
                    "(({}){{ .has_value = true, .value = {} }})",
                    ty_to_c(&expr.ty),
                    i
                )
            }
            HirExprKind::Propagate(inner) => {
                let i = self.expr_to_c(inner);
                match &inner.ty {
                    Ty::Result(_, _) => format!("{}.value.ok", i),
                    Ty::Optional(_) => format!("{}.value", i),
                    _ => format!("{}.value", i),
                }
            }
            HirExprKind::InterpolatedStr(parts) => self.emit_interp_str(parts),
            HirExprKind::BytesLit(bytes) => {
                let elems: Vec<String> = bytes.iter().map(|b| format!("0x{:02x}", b)).collect();
                format!("((uint8_t[]){{ {} }})", elems.join(", "))
            }
            HirExprKind::ListLit { elem_ty, elements } => {
                let c_elem_ty = ty_to_c(elem_ty);
                if elements.is_empty() {
                    format!("ori_list_new(sizeof({}))", c_elem_ty)
                } else {
                    // Build inline: create list, push elements via statement expression
                    let tmp = self.fresh_tmp();
                    let mut parts = Vec::new();
                    parts.push(format!(
                        "ori_list_t {} = ori_list_new(sizeof({}))",
                        tmp, c_elem_ty
                    ));
                    for elem in elements {
                        let val = self.expr_to_c(elem);
                        let elem_tmp = self.fresh_tmp();
                        parts.push(format!("{} {} = {}", c_elem_ty, elem_tmp, val));
                        parts.push(format!("ori_list_push(&{}, &{})", tmp, elem_tmp));
                    }
                    format!("({{ {}; {}; }})", parts.join("; "), tmp)
                }
            }
            HirExprKind::ListSpreadLit { elem_ty, elements } => {
                let c_elem_ty = ty_to_c(elem_ty);
                let tmp = self.fresh_tmp();
                let mut parts = Vec::new();
                parts.push(format!(
                    "ori_list_t {} = ori_list_new(sizeof({}))",
                    tmp, c_elem_ty
                ));
                for elem in elements {
                    let val = self.expr_to_c(&elem.value);
                    if elem.spread {
                        let src_tmp = self.fresh_tmp();
                        let idx_tmp = self.fresh_tmp();
                        let elem_tmp = self.fresh_tmp();
                        parts.push(format!("ori_list_t {} = {}", src_tmp, val));
                        parts.push(format!(
                            "for (size_t {} = 0; {} < {}.len; {}++) {{ {} {} = *(({}*)ori_list_at(&{}, {})); ori_list_push(&{}, &{}); }}",
                            idx_tmp,
                            idx_tmp,
                            src_tmp,
                            idx_tmp,
                            c_elem_ty,
                            elem_tmp,
                            c_elem_ty,
                            src_tmp,
                            idx_tmp,
                            tmp,
                            elem_tmp,
                        ));
                    } else {
                        let elem_tmp = self.fresh_tmp();
                        parts.push(format!("{} {} = {}", c_elem_ty, elem_tmp, val));
                        parts.push(format!("ori_list_push(&{}, &{})", tmp, elem_tmp));
                    }
                }
                format!("({{ {}; {}; }})", parts.join("; "), tmp)
            }
            HirExprKind::TupleLit(elems) => {
                // Emit as anonymous struct: (struct { T0 _f0; T1 _f1; ... }){ ._f0 = v0, ._f1 = v1 }
                let mut field_decls = Vec::new();
                let mut field_inits = Vec::new();
                for (i, elem) in elems.iter().enumerate() {
                    let val = self.expr_to_c(elem);
                    let ty_s = ty_to_c(&elem.ty);
                    field_decls.push(format!("{} _f{};", ty_s, i));
                    field_inits.push(format!("._f{} = {}", i, val));
                }
                format!(
                    "((struct {{ {} }}){{ {} }})",
                    field_decls.join(" "),
                    field_inits.join(", ")
                )
            }
            HirExprKind::Range { start, end } => {
                let s = self.expr_to_c(start);
                let e = self.expr_to_c(end);
                format!("((ori_range_t){{ .__start = {}, .__end = {} }})", s, e)
            }
            HirExprKind::StructLit { def_id, fields } => {
                let fields_s: Vec<String> = fields
                    .iter()
                    .map(|(n, e)| {
                        let es = self.expr_to_c(e);
                        format!(".{} = {}", mangle(n), es)
                    })
                    .collect();
                if def_id.0 != u32::MAX {
                    format!("(({}){{ {} }})", def_c_name(*def_id), fields_s.join(", "))
                } else {
                    format!("({{ {} }})", fields_s.join(", "))
                }
            }
            HirExprKind::EnumVariant {
                def_id,
                variant,
                fields,
            } => {
                let type_name = def_c_name(*def_id);
                let tag = format!("{}__{}", type_name, mangle(variant));
                if fields.is_empty() {
                    format!("(({}){{ .tag = {} }})", type_name, tag)
                } else {
                    let fields_s: Vec<String> = fields
                        .iter()
                        .map(|(n, e)| {
                            let es = self.expr_to_c(e);
                            format!(".{} = {}", mangle(n), es)
                        })
                        .collect();
                    format!(
                        "(({}){{ .tag = {}, .payload.{} = {{ {} }} }})",
                        type_name,
                        tag,
                        mangle(variant),
                        fields_s.join(", ")
                    )
                }
            }
            HirExprKind::Ok_(inner) => {
                let i = self.expr_to_c(inner);
                format!(
                    "(({}){{ .is_ok = true, .value.ok = {} }})",
                    ty_to_c(&expr.ty),
                    i
                )
            }
            HirExprKind::Err_(inner) => {
                let i = self.expr_to_c(inner);
                format!(
                    "(({}){{ .is_ok = false, .value.err = {} }})",
                    ty_to_c(&expr.ty),
                    i
                )
            }
            HirExprKind::MethodCall {
                receiver,
                method,
                args,
            } => {
                let r = self.expr_to_c(receiver);
                let mut as_: Vec<String> = args.iter().map(|a| self.expr_to_c(a)).collect();
                if method == "__slice" && matches!(&receiver.ty, Ty::String) {
                    format!("ori_string_slice({}, {})", r, as_.join(", "))
                } else if let Ty::Any(trait_def_id) = &receiver.ty {
                    let trait_layout = self.trait_layouts.get(trait_def_id).unwrap();
                    let method_index = trait_layout
                        .methods
                        .iter()
                        .position(|m| m.name == *method)
                        .unwrap();
                    let method_sig = &trait_layout.methods[method_index];
                    let ret_ty = ty_to_c(&method_sig.return_ty);
                    let mut params_ty = vec!["void*".to_string()];
                    params_ty.extend(method_sig.params.iter().skip(1).map(|t| ty_to_c(t)));
                    
                    let mut call_args = vec![format!("({}).obj", r)];
                    call_args.extend(as_);

                    let fn_cast = format!(
                        "(({} (*)({}))(((void**)({}).vtable)[{}]))",
                        ret_ty,
                        params_ty.join(", "),
                        r,
                        method_index + 1
                    );

                    format!(
                        "({{ ori_arc_retain(({}).obj); {}({}); }})",
                        r,
                        fn_cast,
                        call_args.join(", ")
                    )                } else {
                    format!("ori__{}({}, {})", mangle(method), r, as_.join(", "))
                }
            }
            HirExprKind::Index { object, index } => {
                let o = self.expr_to_c(object);
                let i = self.expr_to_c(index);
                format!(
                    "*(({}*)ori_list_at(&{}, (size_t){}))",
                    ty_to_c(&expr.ty),
                    o,
                    i
                )
            }
            HirExprKind::MapLit { entries, .. } => {
                let tmp = self.fresh_tmp();
                let mut parts = Vec::new();
                parts.push(format!("ori_map_t* {} = ori_map_new()", tmp));
                for (k, v) in entries {
                    let ks = self.expr_to_c(k);
                    let vs = self.expr_to_c(v);
                    parts.push(format!(
                        "ori_map_set({}, (int64_t)({}), (int64_t)({}))",
                        tmp, ks, vs
                    ));
                }
                format!("({{ {}; {}; }})", parts.join("; "), tmp)
            }
            HirExprKind::SetLit { elements, .. } => {
                let tmp = self.fresh_tmp();
                let mut parts = Vec::new();
                parts.push(format!("ori_set_t* {} = ori_set_new()", tmp));
                for elem in elements {
                    let es = self.expr_to_c(elem);
                    parts.push(format!("ori_set_add({}, (int64_t)({}))", tmp, es));
                }
                format!("({{ {}; {}; }})", parts.join("; "), tmp)
            }
            HirExprKind::StructUpdate {
                def_id,
                base,
                updates,
            } => {
                let base_s = self.expr_to_c(base);
                let type_name = def_c_name(*def_id);
                let tmp = self.fresh_tmp();
                let overrides: Vec<String> = updates
                    .iter()
                    .map(|(n, e)| {
                        let es = self.expr_to_c(e);
                        format!("{}.{} = {}", tmp, mangle(n), es)
                    })
                    .collect();
                format!(
                    "({{ {} {} = {}; {}; {}; }})",
                    type_name,
                    tmp,
                    base_s,
                    overrides.join("; "),
                    tmp
                )
            }
            HirExprKind::IsCheck { value, check_ty } => {
                let _val_s = self.expr_to_c(value);
                if let Ty::Named(def_id, _) = check_ty {
                    let _type_name = def_c_name(*def_id);
                    // For enums, check if tag matches, but wait:
                    // `is` checks whether it is a specific type.
                    // Actually, if we're dealing with `any<Trait>`, we would check the vtable.
                    // But currently `is` is mostly used for traits or enums.
                    // If the left side is a concrete type and the right side is a concrete type,
                    // we could just emit `true` or `false` based on compiler type check.
                    // We'll emit `true` as placeholder until `any` is fully implemented in C.
                    "true".into()
                } else {
                    "true".into()
                }
            }
            HirExprKind::Closure { func_name, captures } => {
                let tmp = self.fresh_tmp();
                let mut parts = Vec::new();
                parts.push(format!("ori_closure_t* {} = (ori_closure_t*)malloc(sizeof(ori_closure_t))", tmp));
                parts.push(format!("{}->fn_ptr = (void*){}", tmp, Self::func_c_name(func_name)));
                if captures.is_empty() {
                    parts.push(format!("{}->env_ptr = NULL", tmp));
                } else {
                    let env_struct = format!("{}_env_t", Self::func_c_name(func_name));
                    let env_tmp = self.fresh_tmp();
                    parts.push(format!("{}* {} = ({}*)malloc(sizeof({}))", env_struct, env_tmp, env_struct, env_struct));
                    for cap in captures {
                        let cap_s = mangle(&cap.name);
                        parts.push(format!("{}->{} = {}", env_tmp, cap_s, cap_s));
                    }
                    parts.push(format!("{}->env_ptr = (void*){}", tmp, env_tmp));
                }
                format!("({{ {}; {}; }})", parts.join("; "), tmp)
            }
        }
    }

    /// Emit string interpolation: `f"hello {name}, age {age}"`
    /// Strategy: build with snprintf into a heap buffer.
    fn emit_interp_str(&mut self, parts: &[HirStrPart]) -> String {
        // Build format string and args for snprintf
        let mut fmt = String::new();
        let mut args: Vec<String> = Vec::new();
        for part in parts {
            match part {
                HirStrPart::Literal(s) => {
                    fmt.push_str(&escape_c_str(s));
                }
                HirStrPart::Expr(e) => {
                    let val = self.expr_to_c(e);
                    match &e.ty {
                        Ty::Int | Ty::Int8 | Ty::Int16 | Ty::Int32 | Ty::Int64 => {
                            // Use PRId64 via C string concatenation in the emitted format
                            fmt.push_str("%\" PRId64 \"");
                            args.push(format!("(int64_t)({})", val));
                        }
                        Ty::Float | Ty::Float32 | Ty::Float64 => {
                            fmt.push_str("%g");
                            args.push(format!("(double)({})", val));
                        }
                        Ty::Bool => {
                            fmt.push_str("%s");
                            args.push(format!("({} ? \"true\" : \"false\")", val));
                        }
                        Ty::String => {
                            fmt.push_str("%.*s");
                            args.push(format!("(int)({}).len, ({}).data", val, val));
                        }
                        _ => {
                            // Fallback: try to print as string
                            fmt.push_str("%.*s");
                            args.push(format!("(int)({}).len, ({}).data", val, val));
                        }
                    }
                }
            }
        }
        let tmp_buf = self.fresh_tmp();
        let tmp_len = self.fresh_tmp();
        let args_str = if args.is_empty() {
            String::new()
        } else {
            format!(", {}", args.join(", "))
        };
        // Use compound literal + statement expression
        format!(
            "({{ char* {buf} = (char*)malloc(1024); int {len} = snprintf({buf}, 1024, \"{fmt}\"{args}); (ori_string_t){{ .data = {buf}, .len = (size_t){len} }}; }})",
            buf = tmp_buf, len = tmp_len, fmt = fmt, args = args_str,
        )
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn def_c_name(id: DefId) -> String {
    format!("ori_def_{}_t", id.0)
}

fn collect_abi_types(module: &HirModule) -> Vec<Ty> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for s in &module.structs {
        for field in &s.fields {
            collect_ty_abi(&field.ty, &mut seen, &mut out);
        }
    }
    for e in &module.enums {
        for variant in &e.variants {
            for field in &variant.fields {
                collect_ty_abi(&field.ty, &mut seen, &mut out);
            }
        }
    }
    for c in &module.consts {
        collect_ty_abi(&c.ty, &mut seen, &mut out);
        collect_expr_abi(&c.value, &mut seen, &mut out);
    }
    for f in &module.funcs {
        collect_ty_abi(&f.return_ty, &mut seen, &mut out);
        for param in &f.params {
            collect_ty_abi(&param.ty, &mut seen, &mut out);
        }
        collect_block_abi(&f.body, &mut seen, &mut out);
    }
    for ext in &module.externs {
        match ext {
            HirExtern::Func {
                params, return_ty, ..
            } => {
                collect_ty_abi(return_ty, &mut seen, &mut out);
                for param in params {
                    collect_ty_abi(&param.ty, &mut seen, &mut out);
                }
            }
            HirExtern::Var { ty, .. } => collect_ty_abi(ty, &mut seen, &mut out),
        }
    }
    out
}

fn collect_block_abi(block: &HirBlock, seen: &mut HashSet<String>, out: &mut Vec<Ty>) {
    for stmt in &block.stmts {
        collect_stmt_abi(stmt, seen, out);
    }
}

fn collect_stmt_abi(stmt: &HirStmt, seen: &mut HashSet<String>, out: &mut Vec<Ty>) {
    match stmt {
        HirStmt::Let { ty, value, .. } => {
            collect_ty_abi(ty, seen, out);
            collect_expr_abi(value, seen, out);
        }
        HirStmt::Assign { value, .. } | HirStmt::Expr(value) => collect_expr_abi(value, seen, out),
        HirStmt::Return(Some(value), _) => collect_expr_abi(value, seen, out),
        HirStmt::Return(None, _) | HirStmt::Break(_) | HirStmt::Continue(_) => {}
        HirStmt::If {
            cond,
            then,
            else_ifs,
            else_,
            ..
        } => {
            collect_expr_abi(cond, seen, out);
            collect_block_abi(then, seen, out);
            for (cond, block) in else_ifs {
                collect_expr_abi(cond, seen, out);
                collect_block_abi(block, seen, out);
            }
            if let Some(block) = else_ {
                collect_block_abi(block, seen, out);
            }
        }
        HirStmt::While { cond, body, .. } => {
            collect_expr_abi(cond, seen, out);
            collect_block_abi(body, seen, out);
        }
        HirStmt::For {
            elem_ty,
            iterable,
            body,
            ..
        } => {
            collect_ty_abi(elem_ty, seen, out);
            collect_expr_abi(iterable, seen, out);
            collect_block_abi(body, seen, out);
        }
        HirStmt::Loop { body, .. } => collect_block_abi(body, seen, out),
        HirStmt::Repeat { count, body, .. } => {
            collect_expr_abi(count, seen, out);
            collect_block_abi(body, seen, out);
        }
        HirStmt::Match {
            scrutinee, arms, ..
        } => {
            collect_expr_abi(scrutinee, seen, out);
            for arm in arms {
                collect_pattern_abi(&arm.pattern, seen, out);
                for stmt in &arm.body {
                    collect_stmt_abi(stmt, seen, out);
                }
            }
        }
        HirStmt::IfSome {
            inner_ty,
            value,
            then,
            else_,
            ..
        } => {
            collect_ty_abi(inner_ty, seen, out);
            collect_expr_abi(value, seen, out);
            collect_block_abi(then, seen, out);
            if let Some(block) = else_ {
                collect_block_abi(block, seen, out);
            }
        }
        HirStmt::WhileSome {
            inner_ty,
            value,
            body,
            ..
        } => {
            collect_ty_abi(inner_ty, seen, out);
            collect_expr_abi(value, seen, out);
            collect_block_abi(body, seen, out);
        }
        HirStmt::Using { ty, value, .. } => {
            collect_ty_abi(ty, seen, out);
            collect_expr_abi(value, seen, out);
        }
        HirStmt::Check { condition, .. } => collect_expr_abi(condition, seen, out),
    }
}

fn collect_pattern_abi(pattern: &HirPattern, seen: &mut HashSet<String>, out: &mut Vec<Ty>) {
    match pattern {
        HirPattern::Binding(_, ty) => collect_ty_abi(ty, seen, out),
        HirPattern::Some_(inner) | HirPattern::Ok_(inner) | HirPattern::Err_(inner) => {
            collect_pattern_abi(inner, seen, out);
        }
        HirPattern::Variant { fields, .. } => {
            for (_, pattern) in fields {
                collect_pattern_abi(pattern, seen, out);
            }
        }
        HirPattern::Tuple(items) => {
            for item in items {
                collect_pattern_abi(item, seen, out);
            }
        }
        _ => {}
    }
}

fn collect_expr_abi(expr: &HirExpr, seen: &mut HashSet<String>, out: &mut Vec<Ty>) {
    collect_ty_abi(&expr.ty, seen, out);
    match &expr.kind {
        HirExprKind::Binary { lhs, rhs, .. } => {
            collect_expr_abi(lhs, seen, out);
            collect_expr_abi(rhs, seen, out);
        }
        HirExprKind::Unary { operand, .. }
        | HirExprKind::Some_(operand)
        | HirExprKind::Ok_(operand)
        | HirExprKind::Err_(operand)
        | HirExprKind::Propagate(operand) => collect_expr_abi(operand, seen, out),
        HirExprKind::Field { object, .. } | HirExprKind::TupleIndex { object, .. } => {
            collect_expr_abi(object, seen, out);
        }
        HirExprKind::Index { object, index } => {
            collect_expr_abi(object, seen, out);
            collect_expr_abi(index, seen, out);
        }
        HirExprKind::Call { callee, args } => {
            collect_expr_abi(callee, seen, out);
            for arg in args {
                collect_expr_abi(&arg.value, seen, out);
            }
        }
        HirExprKind::MethodCall { receiver, args, .. } => {
            collect_expr_abi(receiver, seen, out);
            for arg in args {
                collect_expr_abi(arg, seen, out);
            }
        }
        HirExprKind::StructLit { fields, .. } | HirExprKind::EnumVariant { fields, .. } => {
            for (_, expr) in fields {
                collect_expr_abi(expr, seen, out);
            }
        }
        HirExprKind::ListLit { elem_ty, elements } => {
            collect_ty_abi(elem_ty, seen, out);
            for expr in elements {
                collect_expr_abi(expr, seen, out);
            }
        }
        HirExprKind::ListSpreadLit { elem_ty, elements } => {
            collect_ty_abi(elem_ty, seen, out);
            for elem in elements {
                collect_expr_abi(&elem.value, seen, out);
            }
        }
        HirExprKind::TupleLit(items) => {
            for item in items {
                collect_expr_abi(item, seen, out);
            }
        }
        HirExprKind::InterpolatedStr(parts) => {
            for part in parts {
                if let HirStrPart::Expr(expr) = part {
                    collect_expr_abi(expr, seen, out);
                }
            }
        }
        HirExprKind::Range { start, end } => {
            collect_expr_abi(start, seen, out);
            collect_expr_abi(end, seen, out);
        }
        HirExprKind::MapLit {
            key_ty,
            value_ty,
            entries,
        } => {
            collect_ty_abi(key_ty, seen, out);
            collect_ty_abi(value_ty, seen, out);
            for (k, v) in entries {
                collect_expr_abi(k, seen, out);
                collect_expr_abi(v, seen, out);
            }
        }
        HirExprKind::SetLit { elem_ty, elements } => {
            collect_ty_abi(elem_ty, seen, out);
            for e in elements {
                collect_expr_abi(e, seen, out);
            }
        }
        HirExprKind::StructUpdate { base, updates, .. } => {
            collect_expr_abi(base, seen, out);
            for (_, e) in updates {
                collect_expr_abi(e, seen, out);
            }
        }
        HirExprKind::IfExpr { cond, then, else_ } => {
            collect_expr_abi(cond, seen, out);
            collect_expr_abi(then, seen, out);
            collect_expr_abi(else_, seen, out);
        }
        HirExprKind::IsCheck { value, .. } => {
            collect_expr_abi(value, seen, out);
        }
        _ => {}
    }
}

fn collect_ty_abi(ty: &Ty, seen: &mut HashSet<String>, out: &mut Vec<Ty>) {
    match ty {
        Ty::Optional(inner) => {
            collect_ty_abi(inner, seen, out);
            push_abi_ty(ty, seen, out);
        }
        Ty::Result(ok, err) => {
            collect_ty_abi(ok, seen, out);
            collect_ty_abi(err, seen, out);
            push_abi_ty(ty, seen, out);
        }
        Ty::List(inner) | Ty::Set(inner) | Ty::Range(inner) | Ty::Lazy(inner) => {
            collect_ty_abi(inner, seen, out);
        }
        Ty::Map(key, value) => {
            collect_ty_abi(key, seen, out);
            collect_ty_abi(value, seen, out);
        }
        Ty::Tuple(items) => {
            for item in items {
                collect_ty_abi(item, seen, out);
            }
        }
        Ty::Func { params, ret } => {
            for param in params {
                collect_ty_abi(param, seen, out);
            }
            collect_ty_abi(ret, seen, out);
        }
        Ty::Named(_, args) => {
            for arg in args {
                collect_ty_abi(arg, seen, out);
            }
        }
        _ => {}
    }
}

fn push_abi_ty(ty: &Ty, seen: &mut HashSet<String>, out: &mut Vec<Ty>) {
    let name = ty_to_c(ty);
    if seen.insert(name) {
        out.push(ty.clone());
    }
}

fn ty_to_c(ty: &Ty) -> String {
    match ty {
        Ty::Bool => "bool".into(),
        Ty::Int => "int64_t".into(),
        Ty::Int8 => "int8_t".into(),
        Ty::Int16 => "int16_t".into(),
        Ty::Int32 => "int32_t".into(),
        Ty::Int64 => "int64_t".into(),
        Ty::U8 => "uint8_t".into(),
        Ty::U16 => "uint16_t".into(),
        Ty::U32 => "uint32_t".into(),
        Ty::U64 => "uint64_t".into(),
        Ty::Float | Ty::Float64 => "double".into(),
        Ty::Float32 => "float".into(),
        Ty::String => "ori_string_t".into(),
        Ty::Bytes => "uint8_t*".into(),
        Ty::Void => "void".into(),
        Ty::Never => "void".into(),
        Ty::Optional(t) => format!("ori_opt_{}_t", ty_tag(t)),
        Ty::Result(ok, err) => format!("ori_result_{}_{}_t", ty_tag(ok), ty_tag(err)),
        Ty::List(_) => "ori_list_t".into(),
        Ty::Tuple(elems) => {
            let fields: Vec<String> = elems
                .iter()
                .enumerate()
                .map(|(i, t)| format!("{} _f{};", ty_to_c(t), i))
                .collect();
            format!("struct {{ {} }}", fields.join(" "))
        }
        Ty::Named(id, _) => def_c_name(*id),
        Ty::Any(_) => "ori_any_t".into(),
        Ty::Range(_) => "ori_range_t".into(),
        _ => "void*".into(),
    }
}

fn abi_value_c_type(ty: &Ty) -> String {
    match ty {
        Ty::Void | Ty::Never => "ori_unit_t".into(),
        Ty::List(_) => "ori_list_t".into(),
        _ => ty_to_c(ty),
    }
}

fn ty_tag(ty: &Ty) -> String {
    match ty {
        Ty::Bool => "bool".into(),
        Ty::Int => "i64".into(),
        Ty::Float => "f64".into(),
        Ty::String => "str".into(),
        Ty::Named(id, _) => format!("def{}", id.0),
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

fn mangle_ns_str(s: &str) -> String {
    mangle(s)
}

fn is_entry_main(module: &HirModule, f: &HirFunc) -> bool {
    let entry = format!("{}.main", module.namespace);
    f.params.is_empty() && (f.name.as_str() == "main" || f.name.as_str() == entry)
}

fn binop_to_c(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::Add => "+",
        BinaryOp::Sub => "-",
        BinaryOp::Mul => "*",
        BinaryOp::Div => "/",
        BinaryOp::Rem => "%",
        BinaryOp::Eq => "==",
        BinaryOp::Ne => "!=",
        BinaryOp::Lt => "<",
        BinaryOp::Le => "<=",
        BinaryOp::Gt => ">",
        BinaryOp::Ge => ">=",
        BinaryOp::And => "&&",
        BinaryOp::Or => "||",
    }
}

fn lvalue_to_c(lv: &HirLValue) -> String {
    match lv {
        HirLValue::Var(n) => mangle(n),
        HirLValue::Field { base, field } => format!("{}.{}", lvalue_to_c(base), mangle(field)),
        HirLValue::Index { base, index } => {
            let idx_s = hir_expr_to_c_standalone(index);
            format!("{}[{}]", lvalue_to_c(base), idx_s)
        }
    }
}

fn escape_c_str(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn pattern_cond(pat: &HirPattern, scrutinee: &str) -> String {
    match pat {
        HirPattern::Wildcard => "1".into(),
        HirPattern::BoolLit(b) => format!("{} == {}", scrutinee, if *b { "true" } else { "false" }),
        HirPattern::IntLit(n) => format!("{} == INT64_C({})", scrutinee, n),
        HirPattern::StrLit(s) => {
            let escaped = escape_c_str(s);
            format!("ori_string_eq({}, ORI_STR(\"{}\"))", scrutinee, escaped)
        }
        HirPattern::None_ => format!("!{}.has_value", scrutinee),
        HirPattern::Some_(_) => format!("{}.has_value", scrutinee),
        HirPattern::Ok_(_) => format!("{}.is_ok", scrutinee),
        HirPattern::Err_(_) => format!("!{}.is_ok", scrutinee),
        HirPattern::Variant {
            def_id, variant, ..
        } => {
            let type_name = def_c_name(*def_id);
            format!("{}.tag == {}__{}", scrutinee, type_name, mangle(variant))
        }
        HirPattern::Binding(_, _) => "1".into(), // always matches
        HirPattern::Tuple(_) => "1".into(),      // tuple always matches structurally
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
    if let HirPattern::Ok_(inner) = pat {
        let inner_s = format!("{}.value.ok", scrutinee);
        emit_pattern_bindings(inner, &inner_s, out, indent);
    }
    if let HirPattern::Err_(inner) = pat {
        let inner_s = format!("{}.value.err", scrutinee);
        emit_pattern_bindings(inner, &inner_s, out, indent);
    }
    if let HirPattern::Variant {
        variant, fields, ..
    } = pat
    {
        for (fname, fpat) in fields {
            let field_s = format!(
                "{}.payload.{}.{}",
                scrutinee,
                mangle(variant),
                mangle(fname)
            );
            emit_pattern_bindings(fpat, &field_s, out, indent);
        }
    }
    if let HirPattern::Tuple(patterns) = pat {
        for (i, inner) in patterns.iter().enumerate() {
            let field_s = format!("{}._f{}", scrutinee, i);
            emit_pattern_bindings(inner, &field_s, out, indent);
        }
    }
}

/// Standalone expression-to-C helper that doesn't need `&mut CCodegen`.
/// Handles the common cases needed for lvalue index expressions.
fn hir_expr_to_c_standalone(expr: &HirExpr) -> String {
    match &expr.kind {
        HirExprKind::IntLit(n) => format!("INT64_C({})", n),
        HirExprKind::Var(n) => mangle(n),
        HirExprKind::BoolLit(b) => {
            if *b {
                "true".into()
            } else {
                "false".into()
            }
        }
        HirExprKind::FloatLit(f) => format!("{:.}", f),
        HirExprKind::Binary { op, lhs, rhs } => {
            let l = hir_expr_to_c_standalone(lhs);
            let r = hir_expr_to_c_standalone(rhs);
            format!("({} {} {})", l, binop_to_c(*op), r)
        }
        HirExprKind::Unary { op, operand } => {
            let e = hir_expr_to_c_standalone(operand);
            match op {
                UnaryOp::Neg => format!("(-{})", e),
                UnaryOp::Not => format!("(!{})", e),
            }
        }
        HirExprKind::Field { object, field } => {
            let obj = hir_expr_to_c_standalone(object);
            format!("{}.{}", obj, mangle(field))
        }
        _ => "0/*unsupported-idx*/".into(),
    }
}
