use std::collections::HashMap;
use smol_str::SmolStr;

use cranelift_codegen::ir::{self, types, AbiParam, InstBuilder};
use cranelift_codegen::{settings, Context};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{DataDescription, DataId, FuncId, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};

use ori_ast::expr::{BinaryOp, UnaryOp};
use ori_hir::hir::*;
use ori_types::Ty;

// == String collection ==

fn collect_strings_expr(expr: &HirExpr, out: &mut Vec<SmolStr>) {
    match &expr.kind {
        HirExprKind::StrLit(s) => { if !out.contains(s) { out.push(s.clone()); } }
        HirExprKind::Call { callee, args } => {
            collect_strings_expr(callee, out);
            for a in args { collect_strings_expr(a, out); }
        }
        HirExprKind::Binary { lhs, rhs, .. } => {
            collect_strings_expr(lhs, out); collect_strings_expr(rhs, out);
        }
        HirExprKind::Unary { operand, .. } => collect_strings_expr(operand, out),
        HirExprKind::Field { object, .. } => collect_strings_expr(object, out),
        HirExprKind::IfExpr { cond, then, else_ } => {
            collect_strings_expr(cond, out);
            collect_strings_expr(then, out);
            collect_strings_expr(else_, out);
        }
        HirExprKind::Propagate(e) | HirExprKind::Some_(e) | HirExprKind::Ok_(e)
        | HirExprKind::Err_(e) => collect_strings_expr(e, out),
        HirExprKind::ListLit { elements, .. } => { for e in elements { collect_strings_expr(e, out); } }
        HirExprKind::TupleLit(elems) => { for e in elems { collect_strings_expr(e, out); } }
        HirExprKind::InterpolatedStr(parts) => {
            for p in parts { if let HirStrPart::Expr(e) = p { collect_strings_expr(e, out); } }
        }
        HirExprKind::Range { start, end } => {
            collect_strings_expr(start, out); collect_strings_expr(end, out);
        }
        _ => {}
    }
}

fn collect_strings_block(block: &HirBlock, out: &mut Vec<SmolStr>) {
    for s in &block.stmts { collect_strings_stmt(s, out); }
}

fn collect_strings_stmt(stmt: &HirStmt, out: &mut Vec<SmolStr>) {
    match stmt {
        HirStmt::Let { value, .. } => collect_strings_expr(value, out),
        HirStmt::Assign { value, .. } => collect_strings_expr(value, out),
        HirStmt::Return(Some(e), _) => collect_strings_expr(e, out),
        HirStmt::Expr(e) => collect_strings_expr(e, out),
        HirStmt::If { cond, then, else_ifs, else_, .. } => {
            collect_strings_expr(cond, out);
            collect_strings_block(then, out);
            for (c, b) in else_ifs { collect_strings_expr(c, out); collect_strings_block(b, out); }
            if let Some(eb) = else_ { collect_strings_block(eb, out); }
        }
        HirStmt::While { cond, body, .. } => { collect_strings_expr(cond, out); collect_strings_block(body, out); }
        HirStmt::For  { iterable, body, .. } => { collect_strings_expr(iterable, out); collect_strings_block(body, out); }
        HirStmt::Loop { body, .. } => collect_strings_block(body, out),
        HirStmt::Match { scrutinee, arms, .. } => {
            collect_strings_expr(scrutinee, out);
            for arm in arms { for s in &arm.body { collect_strings_stmt(s, out); } }
        }
        _ => {}
    }
}

fn collect_all_strings(hir: &HirModule) -> Vec<SmolStr> {
    let mut out = Vec::new();
    for f in &hir.funcs { collect_strings_block(&f.body, &mut out); }
    for c in &hir.consts { collect_strings_expr(&c.value, &mut out); }
    out
}

// == Type mapping ==

fn cl_type(ty: &Ty, ptr_ty: types::Type) -> Option<types::Type> {
    match ty {
        Ty::Bool                             => Some(types::I8),
        Ty::Int | Ty::Int64 | Ty::U64        => Some(types::I64),
        Ty::Int32 | Ty::U32                  => Some(types::I32),
        Ty::Int16 | Ty::U16                  => Some(types::I16),
        Ty::Int8  | Ty::U8                   => Some(types::I8),
        Ty::Float | Ty::Float64              => Some(types::F64),
        Ty::Float32                          => Some(types::F32),
        Ty::String | Ty::Bytes               => Some(ptr_ty),
        Ty::Void | Ty::Never                 => None,
        Ty::Named(_, _)                      => Some(ptr_ty),
        Ty::Infer(_)                         => Some(types::I64),
        _                                    => Some(types::I64),
    }
}

fn is_float_ty(ty: &Ty) -> bool {
    matches!(ty, Ty::Float | Ty::Float32 | Ty::Float64)
}

// == Module-level backend ==

pub struct NativeBackend {
    module:      ObjectModule,
    ptr_ty:      types::Type,
    func_ids:    HashMap<SmolStr, FuncId>,
    /// Extern C functions declared for stdlib/runtime use.
    stdlib_ids:  HashMap<SmolStr, FuncId>,
    /// Static string data: source text → DataId in the object.
    string_data: HashMap<SmolStr, DataId>,
}

impl NativeBackend {
    pub fn new() -> Result<Self, String> {
        let flags = settings::Flags::new(settings::builder());
        let isa = cranelift_native::builder()
            .map_err(|e| format!("native ISA unavailable: {e}"))?
            .finish(flags)
            .map_err(|e| format!("ISA build failed: {e}"))?;
        let ptr_ty = isa.pointer_type();
        let builder = ObjectBuilder::new(
            isa, "ori_module", cranelift_module::default_libcall_names(),
        ).map_err(|e| format!("ObjectBuilder failed: {e}"))?;
        Ok(Self {
            module: ObjectModule::new(builder),
            ptr_ty,
            func_ids:    HashMap::new(),
            stdlib_ids:  HashMap::new(),
            string_data: HashMap::new(),
        })
    }

    pub fn compile(mut self, hir: &HirModule) -> Result<Vec<u8>, String> {
        self.emit_module_strings(hir)?;
        self.declare_stdlib()?;
        self.declare_all(hir)?;
        self.define_all(hir)?;
        self.module.finish().emit().map_err(|e| format!("object emit failed: {e}"))
    }

    /// Emit all string literals as static null-terminated data in .rodata.
    fn emit_module_strings(&mut self, hir: &HirModule) -> Result<(), String> {
        for s in collect_all_strings(hir) {
            if self.string_data.contains_key(&s) { continue; }
            let mut bytes: Vec<u8> = s.as_bytes().to_vec();
            bytes.push(0); // null-terminate for `puts` compatibility
            let mut desc = DataDescription::new();
            desc.define(bytes.into_boxed_slice());
            let id = self.module
                .declare_anonymous_data(false, false)
                .map_err(|e| format!("declare string data: {e}"))?;
            self.module
                .define_data(id, &desc)
                .map_err(|e| format!("define string data: {e}"))?;
            self.string_data.insert(s, id);
        }
        Ok(())
    }

    /// Declare C library / runtime functions used by the stdlib mapping.
    fn declare_stdlib(&mut self) -> Result<(), String> {
        let pt = self.ptr_ty;
        let mut decl = |name: &'static str, params: &[types::Type], ret: Option<types::Type>| {
            let mut sig = self.module.make_signature();
            for &p in params { sig.params.push(AbiParam::new(p)); }
            if let Some(r) = ret { sig.returns.push(AbiParam::new(r)); }
            self.module
                .declare_function(name, Linkage::Import, &sig)
                .map_err(|e| format!("declare {name}: {e}"))
        };
        // ori_io_print(ptr: *u8, len: i64) -- prints len bytes from ptr
        let id = decl("ori_io_print", &[pt, types::I64], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_io_print"), id);
        // ori_io_eprint(ptr: *u8, len: i64) -- stderr print
        let id = decl("ori_io_eprint", &[pt, types::I64], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_io_eprint"), id);
        // ori_to_string(n: i64) -> (*u8, i64) -- TODO: multi-value; stub as ptr for now
        let id = decl("ori_int_to_cstr", &[types::I64], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_to_string"), id);
        // strlen(ptr: *u8) -> i64  (returns size_t, we treat as i64)
        let id = decl("strlen", &[pt], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("strlen"), id);
        // malloc / free for runtime allocation
        let id = decl("malloc", &[types::I64], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("malloc"), id);
        let id = decl("free", &[pt], None)?;
        self.stdlib_ids.insert(SmolStr::new("free"), id);
        Ok(())
    }

    fn make_sig(&self, f: &HirFunc) -> ir::Signature {
        let mut sig = self.module.make_signature();
        for p in &f.params {
            if let Some(t) = cl_type(&p.ty, self.ptr_ty) {
                sig.params.push(AbiParam::new(t));
            }
        }
        if let Some(t) = cl_type(&f.return_ty, self.ptr_ty) {
            sig.returns.push(AbiParam::new(t));
        }
        sig
    }

    fn declare_all(&mut self, hir: &HirModule) -> Result<(), String> {
        for f in &hir.funcs {
            let sig  = self.make_sig(f);
            let link = if f.is_public || f.name == "main" { Linkage::Export } else { Linkage::Local };
            let id   = self.module
                .declare_function(&format!("ORI__{}", f.name), link, &sig)
                .map_err(|e| format!("declare '{}': {e}", f.name))?;
            self.func_ids.insert(f.name.clone(), id);
        }
        if hir.funcs.iter().any(|f| f.name == "main" && f.params.is_empty()) {
            let mut sig = self.module.make_signature();
            sig.returns.push(AbiParam::new(types::I32));
            self.module.declare_function("main", Linkage::Export, &sig)
                .map_err(|e| format!("declare main: {e}"))?;
        }
        Ok(())
    }

    fn define_all(&mut self, hir: &HirModule) -> Result<(), String> {
        for f in &hir.funcs {
            let sig     = self.make_sig(f);
            let func_id = self.func_ids[&f.name];
            let mut ctx = self.module.make_context();
            ctx.func.signature = sig;

            // Pre-declare ALL function references (user + stdlib) before builder takes ownership
            let mut func_refs: HashMap<SmolStr, ir::FuncRef> = HashMap::new();
            for (name, &id) in self.func_ids.iter().chain(self.stdlib_ids.iter()) {
                let fref = self.module.declare_func_in_func(id, &mut ctx.func);
                func_refs.insert(name.clone(), fref);
            }

            // Pre-declare all string global values
            let mut string_gvs: HashMap<SmolStr, ir::GlobalValue> = HashMap::new();
            for (s, &data_id) in &self.string_data {
                let gv = self.module.declare_data_in_func(data_id, &mut ctx.func);
                string_gvs.insert(s.clone(), gv);
            }

            let mut bctx = FunctionBuilderContext::new();
            {
                let builder = FunctionBuilder::new(&mut ctx.func, &mut bctx);
                FuncCodegen {
                    builder, func_refs: &func_refs,
                    string_gvs: &string_gvs,
                    vars: HashMap::new(), ptr_ty: self.ptr_ty,
                    loop_stack: Vec::new(), terminated: false,
                }.emit(f)?;
            }
            self.module.define_function(func_id, &mut ctx)
                .map_err(|e| format!("define '{}': {e}", f.name))?;
        }

        // Define C main wrapper
        if let Some(&ori_main_id) = self.func_ids.get("main".into()) {
            let mut sig = self.module.make_signature();
            sig.returns.push(AbiParam::new(types::I32));
            let main_id = self.module.declare_function("main", Linkage::Export, &sig)
                .map_err(|e| format!("re-declare main: {e}"))?;
            let mut ctx  = self.module.make_context();
            ctx.func.signature = sig;
            let ori_ref = self.module.declare_func_in_func(ori_main_id, &mut ctx.func);
            let mut bctx = FunctionBuilderContext::new();
            {
                let mut b = FunctionBuilder::new(&mut ctx.func, &mut bctx);
                let blk = b.create_block();
                b.switch_to_block(blk);
                b.seal_block(blk);
                b.ins().call(ori_ref, &[]);
                let zero = b.ins().iconst(types::I32, 0);
                b.ins().return_(&[zero]);
                b.seal_all_blocks();
                b.finalize();
            }
            self.module.define_function(main_id, &mut ctx)
                .map_err(|e| format!("define main wrapper: {e}"))?;
        }
        Ok(())
    }
}

// == Per-function codegen ==

struct FuncCodegen<'a> {
    builder:    FunctionBuilder<'a>,
    func_refs:  &'a HashMap<SmolStr, ir::FuncRef>,
    string_gvs: &'a HashMap<SmolStr, ir::GlobalValue>,
    vars:       HashMap<SmolStr, (Variable, Ty)>,
    ptr_ty:     types::Type,
    loop_stack: Vec<(ir::Block, ir::Block)>,
    terminated: bool,
}

impl<'a> FuncCodegen<'a> {
    fn emit(mut self, f: &HirFunc) -> Result<(), String> {
        let entry = self.builder.create_block();
        self.builder.append_block_params_for_function_params(entry);
        self.builder.switch_to_block(entry);
        self.builder.seal_block(entry);

        // Bind parameters
        let params: Vec<(SmolStr, Ty, ir::Value)> = f.params.iter()
            .enumerate()
            .filter_map(|(i, p)| {
                cl_type(&p.ty, self.ptr_ty).map(|_| {
                    let v = self.builder.block_params(entry)[i];
                    (p.name.clone(), p.ty.clone(), v)
                })
            })
            .collect();
        for (name, ty, val) in params {
            if let Some(cl_ty) = cl_type(&ty, self.ptr_ty) {
                let var = self.builder.declare_var(cl_ty);
                self.builder.def_var(var, val);
                self.vars.insert(name, (var, ty));
            }
        }

        self.emit_block(&f.body)?;

        if !self.terminated {
            if cl_type(&f.return_ty, self.ptr_ty).is_none() {
                self.builder.ins().return_(&[]);
            } else {
                let zero = self.zero_val(&f.return_ty);
                self.builder.ins().return_(&[zero]);
            }
        }

        self.builder.seal_all_blocks();
        self.builder.finalize();
        Ok(())
    }

    fn zero_val(&mut self, ty: &Ty) -> ir::Value {
        match ty {
            Ty::Float | Ty::Float64 => self.builder.ins().f64const(0.0),
            Ty::Float32             => self.builder.ins().f32const(0.0),
            _ => {
                let cl = cl_type(ty, self.ptr_ty).unwrap_or(types::I64);
                self.builder.ins().iconst(cl, 0)
            }
        }
    }

    // == Statements ==

    fn emit_block(&mut self, block: &HirBlock) -> Result<(), String> {
        for s in &block.stmts {
            if self.terminated { break; }
            self.emit_stmt(s)?;
        }
        Ok(())
    }

    fn emit_stmt(&mut self, stmt: &HirStmt) -> Result<(), String> {
        match stmt {
            HirStmt::Let { name, ty, value, .. } => {
                let val = self.emit_expr(value)?;
                if let Some(cl_ty) = cl_type(ty, self.ptr_ty) {
                    let var = self.builder.declare_var(cl_ty);
                    self.builder.def_var(var, val);
                    self.vars.insert(name.clone(), (var, ty.clone()));
                }
            }
            HirStmt::Assign { lvalue, value, .. } => {
                let val = self.emit_expr(value)?;
                if let HirLValue::Var(name) = lvalue {
                    if let Some((var, _)) = self.vars.get(name) {
                        let var = *var;
                        self.builder.def_var(var, val);
                    }
                }
            }
            HirStmt::Return(val, _) => {
                match val {
                    Some(e) => { let v = self.emit_expr(e)?; self.builder.ins().return_(&[v]); }
                    None    => { self.builder.ins().return_(&[]); }
                }
                self.terminated = true;
            }
            HirStmt::Break(_) => {
                if let Some((_, exit)) = self.loop_stack.last().copied() {
                    self.builder.ins().jump(exit, &[]);
                    self.terminated = true;
                }
            }
            HirStmt::Continue(_) => {
                if let Some((header, _)) = self.loop_stack.last().copied() {
                    self.builder.ins().jump(header, &[]);
                    self.terminated = true;
                }
            }
            HirStmt::Expr(e) => { self.emit_expr(e)?; }

            HirStmt::If { cond, then, else_ifs, else_, .. } => {
                self.emit_if(cond, then, else_ifs, else_.as_ref())?;
            }
            HirStmt::While { cond, body, .. }    => self.emit_while(cond, body)?,
            HirStmt::Loop  { body, .. }          => self.emit_loop(body)?,
            HirStmt::For { binding, elem_ty, iterable, body, .. } => {
                self.emit_for(binding, elem_ty, iterable, body)?;
            }
            HirStmt::Match { scrutinee, arms, .. } => self.emit_match(scrutinee, arms)?,
        }
        Ok(())
    }

    // == Control flow ==

    fn emit_if(&mut self, cond: &HirExpr, then: &HirBlock,
               else_ifs: &[(HirExpr, HirBlock)], else_: Option<&HirBlock>) -> Result<(), String> {
        let then_block  = self.builder.create_block();
        let merge_block = self.builder.create_block();

        let else_target = if else_ifs.is_empty() && else_.is_none() {
            merge_block
        } else {
            self.builder.create_block()
        };

        let cv = self.emit_expr(cond)?;
        self.builder.ins().brif(cv, then_block, &[], else_target, &[]);

        // then branch
        self.builder.seal_block(then_block);
        self.builder.switch_to_block(then_block);
        self.terminated = false;
        self.emit_block(then)?;
        if !self.terminated { self.builder.ins().jump(merge_block, &[]); }

        // else / else-if branch
        if else_target != merge_block {
            self.builder.seal_block(else_target);
            self.builder.switch_to_block(else_target);
            self.terminated = false;
            if !else_ifs.is_empty() {
                self.emit_if(&else_ifs[0].0, &else_ifs[0].1, &else_ifs[1..], else_)?;
            } else if let Some(eb) = else_ {
                self.emit_block(eb)?;
            }
            if !self.terminated { self.builder.ins().jump(merge_block, &[]); }
        }

        self.builder.seal_block(merge_block);
        self.builder.switch_to_block(merge_block);
        self.terminated = false;
        Ok(())
    }

    fn emit_while(&mut self, cond: &HirExpr, body: &HirBlock) -> Result<(), String> {
        let header = self.builder.create_block();
        let body_b = self.builder.create_block();
        let exit   = self.builder.create_block();
        self.builder.ins().jump(header, &[]);
        self.builder.switch_to_block(header);
        let cv = self.emit_expr(cond)?;
        self.builder.ins().brif(cv, body_b, &[], exit, &[]);
        self.builder.seal_block(body_b);
        self.builder.switch_to_block(body_b);
        self.terminated = false;
        self.loop_stack.push((header, exit));
        self.emit_block(body)?;
        self.loop_stack.pop();
        if !self.terminated { self.builder.ins().jump(header, &[]); }
        self.builder.seal_block(header);
        self.builder.seal_block(exit);
        self.builder.switch_to_block(exit);
        self.terminated = false;
        Ok(())
    }

    fn emit_loop(&mut self, body: &HirBlock) -> Result<(), String> {
        let header = self.builder.create_block();
        let exit   = self.builder.create_block();
        self.builder.ins().jump(header, &[]);
        self.builder.seal_block(header);
        self.builder.switch_to_block(header);
        self.terminated = false;
        self.loop_stack.push((header, exit));
        self.emit_block(body)?;
        self.loop_stack.pop();
        if !self.terminated { self.builder.ins().jump(header, &[]); }
        self.builder.seal_block(exit);
        self.builder.switch_to_block(exit);
        self.terminated = false;
        Ok(())
    }

    fn emit_for(&mut self, binding: &SmolStr, elem_ty: &Ty,
                iterable: &HirExpr, body: &HirBlock) -> Result<(), String> {
        let (start_v, end_v) = if let HirExprKind::Range { start, end } = &iterable.kind {
            (self.emit_expr(start)?, self.emit_expr(end)?)
        } else {
            let v = self.emit_expr(iterable)?;
            (v, v)
        };
        let idx_var = self.builder.declare_var(types::I64);
        self.builder.def_var(idx_var, start_v);
        let end_var = self.builder.declare_var(types::I64);
        self.builder.def_var(end_var, end_v);
        if let Some(cl_ty) = cl_type(elem_ty, self.ptr_ty) {
            let bvar = self.builder.declare_var(cl_ty);
            self.vars.insert(binding.clone(), (bvar, elem_ty.clone()));
        }
        let header = self.builder.create_block();
        let body_b = self.builder.create_block();
        let exit   = self.builder.create_block();
        self.builder.ins().jump(header, &[]);
        self.builder.switch_to_block(header);
        let cur = self.builder.use_var(idx_var);
        let lim = self.builder.use_var(end_var);
        let cond = self.builder.ins().icmp(ir::condcodes::IntCC::SignedLessThanOrEqual, cur, lim);
        self.builder.ins().brif(cond, body_b, &[], exit, &[]);
        self.builder.seal_block(body_b);
        self.builder.switch_to_block(body_b);
        // Update binding variable
        if let Some((bvar, _)) = self.vars.get(binding) {
            let bvar = *bvar;
            let cur2 = self.builder.use_var(idx_var);
            self.builder.def_var(bvar, cur2);
        }
        self.terminated = false;
        self.loop_stack.push((header, exit));
        self.emit_block(body)?;
        self.loop_stack.pop();
        if !self.terminated {
            let cur2 = self.builder.use_var(idx_var);
            let one  = self.builder.ins().iconst(types::I64, 1);
            let next = self.builder.ins().iadd(cur2, one);
            self.builder.def_var(idx_var, next);
            self.builder.ins().jump(header, &[]);
        }
        self.builder.seal_block(header);
        self.builder.seal_block(exit);
        self.builder.switch_to_block(exit);
        self.terminated = false;
        Ok(())
    }

    fn emit_match(&mut self, scrutinee: &HirExpr, arms: &[HirArm]) -> Result<(), String> {
        let scr = self.emit_expr(scrutinee)?;
        let exit = self.builder.create_block();
        for arm in arms {
            let arm_blk  = self.builder.create_block();
            let next_blk = self.builder.create_block();
            let cond = self.pattern_cond(&arm.pattern, scr, &scrutinee.ty);
            self.builder.ins().brif(cond, arm_blk, &[], next_blk, &[]);
            self.builder.seal_block(arm_blk);
            self.builder.switch_to_block(arm_blk);
            self.terminated = false;
            self.bind_pattern(&arm.pattern, scr, &scrutinee.ty);
            for s in &arm.body { self.emit_stmt(s)?; }
            if !self.terminated { self.builder.ins().jump(exit, &[]); }
            self.builder.seal_block(next_blk);
            self.builder.switch_to_block(next_blk);
            self.terminated = false;
        }
        self.builder.ins().jump(exit, &[]);
        self.builder.seal_block(exit);
        self.builder.switch_to_block(exit);
        self.terminated = false;
        Ok(())
    }

    fn pattern_cond(&mut self, pat: &HirPattern, scr: ir::Value, scr_ty: &Ty) -> ir::Value {
        match pat {
            HirPattern::Wildcard | HirPattern::Binding(_, _) =>
                self.builder.ins().iconst(types::I8, 1),
            HirPattern::BoolLit(b) => {
                let lit = self.builder.ins().iconst(types::I8, if *b { 1 } else { 0 });
                self.builder.ins().icmp(ir::condcodes::IntCC::Equal, scr, lit)
            }
            HirPattern::IntLit(n) => {
                let cl = cl_type(scr_ty, self.ptr_ty).unwrap_or(types::I64);
                let lit = self.builder.ins().iconst(cl, *n);
                self.builder.ins().icmp(ir::condcodes::IntCC::Equal, scr, lit)
            }
            _ => self.builder.ins().iconst(types::I8, 1),
        }
    }

    fn bind_pattern(&mut self, pat: &HirPattern, val: ir::Value, ty: &Ty) {
        if let HirPattern::Binding(name, bind_ty) = pat {
            let bty = if *bind_ty == Ty::Infer(0) { ty } else { bind_ty };
            if let Some(cl_ty) = cl_type(bty, self.ptr_ty) {
                let var = self.builder.declare_var(cl_ty);
                self.builder.def_var(var, val);
                self.vars.insert(name.clone(), (var, bty.clone()));
            }
        }
    }

    // == Expressions ==

    fn emit_expr(&mut self, expr: &HirExpr) -> Result<ir::Value, String> {
        Ok(match &expr.kind {
            HirExprKind::BoolLit(b) =>
                self.builder.ins().iconst(types::I8, if *b { 1 } else { 0 }),
            HirExprKind::IntLit(n) => {
                let cl = cl_type(&expr.ty, self.ptr_ty).unwrap_or(types::I64);
                self.builder.ins().iconst(cl, *n)
            }
            HirExprKind::FloatLit(f) => match &expr.ty {
                Ty::Float32 => self.builder.ins().f32const(*f as f32),
                _           => self.builder.ins().f64const(*f),
            },
            HirExprKind::Unit | HirExprKind::None_ =>
                self.builder.ins().iconst(self.ptr_ty, 0),
            HirExprKind::StrLit(s) => {
                // Return a pointer to the static string data
                if let Some(&gv) = self.string_gvs.get(s.as_str()) {
                    self.builder.ins().global_value(self.ptr_ty, gv)
                } else {
                    self.builder.ins().iconst(self.ptr_ty, 0)
                }
            }
            HirExprKind::InterpolatedStr(_) =>
                self.builder.ins().iconst(self.ptr_ty, 0),
            HirExprKind::Var(name) => {
                if let Some((var, _)) = self.vars.get(name) {
                    let var = *var;
                    self.builder.use_var(var)
                } else {
                    self.builder.ins().iconst(types::I64, 0)
                }
            }
            HirExprKind::Binary { op, lhs, rhs } => {
                let lv = self.emit_expr(lhs)?;
                let rv = self.emit_expr(rhs)?;
                self.emit_binary(*op, lv, rv, &lhs.ty)?
            }
            HirExprKind::Unary { op, operand } => {
                let v = self.emit_expr(operand)?;
                match op {
                    UnaryOp::Neg if is_float_ty(&operand.ty) => self.builder.ins().fneg(v),
                    UnaryOp::Neg => self.builder.ins().ineg(v),
                    UnaryOp::Not => {
                        let zero = self.builder.ins().iconst(types::I8, 0);
                        self.builder.ins().icmp(ir::condcodes::IntCC::Equal, v, zero)
                    }
                }
            }
            HirExprKind::Call { callee, args } => {
                if let HirExprKind::Var(name) = &callee.kind {
                    // ori_io_print takes (ptr: *u8, len: i64) — build args accordingly
                    if name == "ori_io_print" || name == "ori_io_eprint" {
                        if let Some(&fref) = self.func_refs.get(name.as_str()) {
                            let mut cl_args = Vec::new();
                            for a in args {
                                let v = self.emit_expr(a)?;
                                // If arg is a string pointer, add (ptr, len)
                                if matches!(a.ty, Ty::String) {
                                    let len = self.str_len_from_ptr(v);
                                    cl_args.push(v);
                                    cl_args.push(len);
                                } else {
                                    cl_args.push(v);
                                }
                            }
                            self.builder.ins().call(fref, &cl_args);
                            self.builder.ins().iconst(types::I8, 0)
                        } else { self.builder.ins().iconst(types::I64, 0) }
                    } else {
                        let args_v: Vec<ir::Value> = args.iter()
                            .map(|a| self.emit_expr(a))
                            .collect::<Result<_, _>>()?;
                        if let Some(&fref) = self.func_refs.get(name.as_str()) {
                            let call = self.builder.ins().call(fref, &args_v);
                            let res  = self.builder.inst_results(call);
                            if res.is_empty() { self.builder.ins().iconst(types::I8, 0) }
                            else              { res[0] }
                        } else { self.builder.ins().iconst(types::I64, 0) }
                    }
                } else { self.builder.ins().iconst(types::I64, 0) }
            }
            HirExprKind::IfExpr { cond, then, else_ } => {
                let cv = self.emit_expr(cond)?;
                let tv = self.emit_expr(then)?;
                let ev = self.emit_expr(else_)?;
                self.builder.ins().select(cv, tv, ev)
            }
            HirExprKind::Propagate(inner) => self.emit_expr(inner)?,
            HirExprKind::Range { start, end } => {
                // Encode range as (start, end) pair packed into i128  simplified
                let _s = self.emit_expr(start)?;
                let _e = self.emit_expr(end)?;
                self.builder.ins().iconst(types::I64, 0) // range object not yet supported
            }
            _ => self.builder.ins().iconst(types::I64, 0),
        })
    }

    /// For a null-terminated string pointer, compute its length as an i64.
    /// Uses strlen-like logic: call strlen if available, else scan bytes.
    /// For now we use the `strlen` libc function declared on demand.
    fn str_len_from_ptr(&mut self, ptr: ir::Value) -> ir::Value {
        if let Some(&fref) = self.func_refs.get("strlen") {
            let call = self.builder.ins().call(fref, &[ptr]);
            // strlen declared as returning I64; result is already the right type
            return self.builder.inst_results(call)[0];
        }
        self.builder.ins().iconst(types::I64, 0)
    }

    fn emit_binary(&mut self, op: BinaryOp, lv: ir::Value, rv: ir::Value, ty: &Ty)
        -> Result<ir::Value, String>
    {
        use BinaryOp::*;
        use ir::condcodes::{FloatCC, IntCC};
        let float = is_float_ty(ty);
        Ok(match op {
            Add => if float { self.builder.ins().fadd(lv, rv) } else { self.builder.ins().iadd(lv, rv) },
            Sub => if float { self.builder.ins().fsub(lv, rv) } else { self.builder.ins().isub(lv, rv) },
            Mul => if float { self.builder.ins().fmul(lv, rv) } else { self.builder.ins().imul(lv, rv) },
            Div => if float { self.builder.ins().fdiv(lv, rv) } else { self.builder.ins().sdiv(lv, rv) },
            Rem => self.builder.ins().srem(lv, rv),
            Eq  => if float { self.builder.ins().fcmp(FloatCC::Equal, lv, rv) }
                   else     { self.builder.ins().icmp(IntCC::Equal, lv, rv) },
            Ne  => if float { self.builder.ins().fcmp(FloatCC::NotEqual, lv, rv) }
                   else     { self.builder.ins().icmp(IntCC::NotEqual, lv, rv) },
            Lt  => if float { self.builder.ins().fcmp(FloatCC::LessThan, lv, rv) }
                   else     { self.builder.ins().icmp(IntCC::SignedLessThan, lv, rv) },
            Le  => if float { self.builder.ins().fcmp(FloatCC::LessThanOrEqual, lv, rv) }
                   else     { self.builder.ins().icmp(IntCC::SignedLessThanOrEqual, lv, rv) },
            Gt  => if float { self.builder.ins().fcmp(FloatCC::GreaterThan, lv, rv) }
                   else     { self.builder.ins().icmp(IntCC::SignedGreaterThan, lv, rv) },
            Ge  => if float { self.builder.ins().fcmp(FloatCC::GreaterThanOrEqual, lv, rv) }
                   else     { self.builder.ins().icmp(IntCC::SignedGreaterThanOrEqual, lv, rv) },
            And => self.builder.ins().band(lv, rv),
            Or  => self.builder.ins().bor(lv, rv),
        })
    }
}

// == Public entry points ==

pub fn emit_native(hir: &HirModule, obj_path: &std::path::Path) -> Result<(), String> {
    let backend = NativeBackend::new()?;
    let bytes = backend.compile(hir)?;
    std::fs::write(obj_path, &bytes)
        .map_err(|e| format!("write {} failed: {e}", obj_path.display()))
}

/// Link `obj_path` into an executable at `exe_path`.
/// `extra_libs`: additional static libraries to link (e.g., libori_rt.a).
pub fn link(
    obj_path:   &std::path::Path,
    exe_path:   &std::path::Path,
    extra_libs: &[std::path::PathBuf],
) -> Result<(), String> {
    let mut cmd = std::process::Command::new("cc");
    cmd.arg("-o").arg(exe_path).arg(obj_path);
    for lib in extra_libs {
        cmd.arg(lib);
    }
    let status = cmd.status().map_err(|e| format!("could not invoke cc: {e}"))?;
    if status.success() { Ok(()) }
    else { Err(format!("linker exited with code {}", status.code().unwrap_or(-1))) }
}