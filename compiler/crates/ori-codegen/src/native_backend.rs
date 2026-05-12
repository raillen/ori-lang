use smol_str::SmolStr;
use std::collections::HashMap;

use cranelift_codegen::ir::{self, types, AbiParam, InstBuilder, MemFlags};
use cranelift_codegen::settings;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{DataDescription, DataId, FuncId, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};

use ori_ast::expr::{BinaryOp, UnaryOp};
use ori_hir::hir::*;
use ori_types::Ty;

// == String collection ==

fn collect_strings_expr(expr: &HirExpr, out: &mut Vec<SmolStr>) {
    match &expr.kind {
        HirExprKind::StrLit(s) => {
            if !out.contains(s) {
                out.push(s.clone());
            }
        }
        HirExprKind::Call { callee, args } => {
            collect_strings_expr(callee, out);
            for a in args {
                collect_strings_expr(&a.value, out);
            }
        }
        HirExprKind::Binary { lhs, rhs, .. } => {
            collect_strings_expr(lhs, out);
            collect_strings_expr(rhs, out);
        }
        HirExprKind::Unary { operand, .. } => collect_strings_expr(operand, out),
        HirExprKind::Field { object, .. } => collect_strings_expr(object, out),
        HirExprKind::IfExpr { cond, then, else_ } => {
            collect_strings_expr(cond, out);
            collect_strings_expr(then, out);
            collect_strings_expr(else_, out);
        }
        HirExprKind::Propagate(e)
        | HirExprKind::Some_(e)
        | HirExprKind::Ok_(e)
        | HirExprKind::Err_(e) => collect_strings_expr(e, out),
        HirExprKind::ListLit { elements, .. } => {
            for e in elements {
                collect_strings_expr(e, out);
            }
        }
        HirExprKind::ListSpreadLit { elements, .. } => {
            for e in elements {
                collect_strings_expr(&e.value, out);
            }
        }
        HirExprKind::TupleLit(elems) => {
            for e in elems {
                collect_strings_expr(e, out);
            }
        }
        HirExprKind::InterpolatedStr(parts) => {
            for p in parts {
                match p {
                    HirStrPart::Literal(s) => {
                        if !out.contains(s) {
                            out.push(s.clone());
                        }
                    }
                    HirStrPart::Expr(e) => collect_strings_expr(e, out),
                }
            }
        }
        HirExprKind::Range { start, end } => {
            collect_strings_expr(start, out);
            collect_strings_expr(end, out);
        }
        HirExprKind::StructLit { fields, .. } => {
            for (_, e) in fields {
                collect_strings_expr(e, out);
            }
        }
        HirExprKind::EnumVariant { fields, .. } => {
            for (_, e) in fields {
                collect_strings_expr(e, out);
            }
        }
        HirExprKind::MapLit { entries, .. } => {
            for (k, v) in entries {
                collect_strings_expr(k, out);
                collect_strings_expr(v, out);
            }
        }
        HirExprKind::SetLit { elements, .. } => {
            for e in elements {
                collect_strings_expr(e, out);
            }
        }
        HirExprKind::StructUpdate { base, updates, .. } => {
            collect_strings_expr(base, out);
            for (_, e) in updates {
                collect_strings_expr(e, out);
            }
        }
        HirExprKind::MethodCall { receiver, args, .. } => {
            collect_strings_expr(receiver, out);
            for a in args {
                collect_strings_expr(a, out);
            }
        }
        HirExprKind::Index { object, index } => {
            collect_strings_expr(object, out);
            collect_strings_expr(index, out);
        }
        HirExprKind::TupleIndex { object, .. } => {
            collect_strings_expr(object, out);
        }
        _ => {}
    }
}

fn collect_strings_block(block: &HirBlock, out: &mut Vec<SmolStr>) {
    for s in &block.stmts {
        collect_strings_stmt(s, out);
    }
}

fn collect_strings_stmt(stmt: &HirStmt, out: &mut Vec<SmolStr>) {
    match stmt {
        HirStmt::Let { value, .. } => collect_strings_expr(value, out),
        HirStmt::Assign { value, .. } => collect_strings_expr(value, out),
        HirStmt::Return(Some(e), _) => collect_strings_expr(e, out),
        HirStmt::Expr(e) => collect_strings_expr(e, out),
        HirStmt::If {
            cond,
            then,
            else_ifs,
            else_,
            ..
        } => {
            collect_strings_expr(cond, out);
            collect_strings_block(then, out);
            for (c, b) in else_ifs {
                collect_strings_expr(c, out);
                collect_strings_block(b, out);
            }
            if let Some(eb) = else_ {
                collect_strings_block(eb, out);
            }
        }
        HirStmt::While { cond, body, .. } => {
            collect_strings_expr(cond, out);
            collect_strings_block(body, out);
        }
        HirStmt::For { iterable, body, .. } => {
            collect_strings_expr(iterable, out);
            collect_strings_block(body, out);
        }
        HirStmt::Loop { body, .. } => collect_strings_block(body, out),
        HirStmt::Repeat { count, body, .. } => {
            collect_strings_expr(count, out);
            collect_strings_block(body, out);
        }
        HirStmt::Match {
            scrutinee, arms, ..
        } => {
            collect_strings_expr(scrutinee, out);
            for arm in arms {
                collect_strings_pattern(&arm.pattern, out);
                for s in &arm.body {
                    collect_strings_stmt(s, out);
                }
            }
        }
        HirStmt::IfSome {
            value, then, else_, ..
        } => {
            collect_strings_expr(value, out);
            collect_strings_block(then, out);
            if let Some(eb) = else_ {
                collect_strings_block(eb, out);
            }
        }
        HirStmt::WhileSome { value, body, .. } => {
            collect_strings_expr(value, out);
            collect_strings_block(body, out);
        }
        HirStmt::Using { value, .. } => collect_strings_expr(value, out),
        HirStmt::Check { condition, .. } => collect_strings_expr(condition, out),
        _ => {}
    }
}

fn collect_all_strings(hir: &HirModule) -> Vec<SmolStr> {
    let mut out = vec![SmolStr::new("")];
    for f in &hir.funcs {
        collect_strings_block(&f.body, &mut out);
    }
    for c in &hir.consts {
        collect_strings_expr(&c.value, &mut out);
    }
    out
}

fn collect_strings_pattern(pat: &HirPattern, out: &mut Vec<SmolStr>) {
    match pat {
        HirPattern::StrLit(s) => {
            if !out.contains(s) {
                out.push(s.clone());
            }
        }
        HirPattern::Some_(inner) | HirPattern::Ok_(inner) | HirPattern::Err_(inner) => {
            collect_strings_pattern(inner, out);
        }
        HirPattern::Variant { fields, .. } => {
            for (_, pat) in fields {
                collect_strings_pattern(pat, out);
            }
        }
        HirPattern::Tuple(patterns) => {
            for pat in patterns {
                collect_strings_pattern(pat, out);
            }
        }
        _ => {}
    }
}

// == Type mapping ==

fn cl_type(ty: &Ty, ptr_ty: types::Type) -> Option<types::Type> {
    match ty {
        Ty::Bool => Some(types::I8),
        Ty::Int | Ty::Int64 | Ty::U64 => Some(types::I64),
        Ty::Int32 | Ty::U32 => Some(types::I32),
        Ty::Int16 | Ty::U16 => Some(types::I16),
        Ty::Int8 | Ty::U8 => Some(types::I8),
        Ty::Float | Ty::Float64 => Some(types::F64),
        Ty::Float32 => Some(types::F32),
        Ty::String | Ty::Bytes | Ty::Func { .. } => Some(ptr_ty),
        Ty::Any(_) => Some(ptr_ty),
        Ty::Optional(_)
        | Ty::Result(_, _)
        | Ty::List(_)
        | Ty::Map(_, _)
        | Ty::Set(_)
        | Ty::Range(_)
        | Ty::Tuple(_) => Some(ptr_ty),
        Ty::Void | Ty::Never => None,
        Ty::Named(_, _) => Some(ptr_ty),
        Ty::Infer(_) => Some(types::I64),
        _ => Some(types::I64),
    }
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

#[derive(Clone, Copy)]
struct StringParts {
    ptr: ir::Value,
    len: ir::Value,
}

fn const_static_bytes(expr: &HirExpr, ty: &Ty) -> Option<Vec<u8>> {
    match (&expr.kind, ty) {
        (HirExprKind::BoolLit(value), Ty::Bool) => Some(vec![if *value { 1 } else { 0 }]),
        (HirExprKind::IntLit(value), Ty::Int8 | Ty::U8) => Some(vec![*value as u8]),
        (HirExprKind::IntLit(value), Ty::Int16 | Ty::U16) => {
            Some((*value as i16).to_le_bytes().to_vec())
        }
        (HirExprKind::IntLit(value), Ty::Int32 | Ty::U32) => {
            Some((*value as i32).to_le_bytes().to_vec())
        }
        (HirExprKind::IntLit(value), Ty::Int | Ty::Int64 | Ty::U64) => {
            Some(value.to_le_bytes().to_vec())
        }
        (HirExprKind::FloatLit(value), Ty::Float32) => Some((*value as f32).to_le_bytes().to_vec()),
        (HirExprKind::FloatLit(value), Ty::Float | Ty::Float64) => {
            Some(value.to_le_bytes().to_vec())
        }
        _ => None,
    }
}

fn is_float_ty(ty: &Ty) -> bool {
    matches!(ty, Ty::Float | Ty::Float32 | Ty::Float64)
}

fn mangle_symbol(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn native_func_symbol(name: &str) -> String {
    format!("ORI__{}", mangle_symbol(name))
}

fn native_func_wrapper_symbol(name: &str) -> String {
    format!("ORI__{}__fnptr_wrapper", mangle_symbol(name))
}

fn native_global_symbol(name: &str) -> String {
    format!("ORI_GLOBAL__{}", mangle_symbol(name))
}

fn is_entry_main(hir: &HirModule, f: &HirFunc) -> bool {
    let entry = format!("{}.main", hir.namespace);
    f.params.is_empty() && (f.name.as_str() == "main" || f.name.as_str() == entry)
}

fn is_synthetic_closure_func(f: &HirFunc) -> bool {
    f.params
        .first()
        .is_some_and(|param| param.name.as_str() == "__env")
        && f.name.contains(".__closure_")
}

/// Layout of an `optional<T>`: `{ has_value: i8, [padding], value: T }`.
fn optional_layout(inner: &Ty, ptr_ty: types::Type) -> (u32, u32) {
    // Returns (value_offset, total_size)
    let (val_size, val_align) = field_size_align(inner, ptr_ty);
    let val_offset = (1u32 + val_align as u32 - 1) & !(val_align as u32 - 1);
    let total = ((val_offset + val_size + val_align as u32 - 1) & !(val_align as u32 - 1)).max(2);
    (val_offset, total)
}

/// Layout of `result<T,E>`: `{ is_ok: i8, [padding], union { ok: T | err: E } }`.
fn result_layout(ok: &Ty, err: &Ty, ptr_ty: types::Type) -> (u32, u32, u32) {
    // Returns (payload_offset, ok_size, total_size)
    let (ok_size, ok_align) = field_size_align(ok, ptr_ty);
    let (err_size, err_align) = field_size_align(err, ptr_ty);
    let pay_align = ok_align.max(err_align);
    let pay_size = ok_size.max(err_size);
    let pay_offset = (1u32 + pay_align as u32 - 1) & !(pay_align as u32 - 1);
    let total = ((pay_offset + pay_size + pay_align as u32 - 1) & !(pay_align as u32 - 1)).max(2);
    (pay_offset, ok_size, total)
}

// == Struct layout ==

#[derive(Debug, Clone)]
pub struct FieldLayout {
    pub offset: u32,
    pub ty: Ty,
    pub contract: Option<HirExpr>,
}

#[derive(Debug, Clone, Default)]
pub struct StructLayout {
    pub size: u32,
    pub align: u8,
    pub fields: Vec<(SmolStr, FieldLayout)>,
}

impl StructLayout {
    pub fn field(&self, name: &str) -> Option<&FieldLayout> {
        self.fields.iter().find(|(n, _)| n == name).map(|(_, f)| f)
    }
}

fn field_size_align(ty: &Ty, ptr_ty: types::Type) -> (u32, u8) {
    let cl = cl_type(ty, ptr_ty).unwrap_or(ptr_ty);
    let bytes = cl.bytes() as u32;
    let align = bytes.min(8).max(1) as u8;
    (bytes, align)
}

fn tuple_layout(elems: &[Ty], ptr_ty: types::Type) -> (Vec<(u32, Ty)>, u32, u8) {
    let mut offset = 0u32;
    let mut max_align = 1u8;
    let mut fields = Vec::new();

    for ty in elems {
        let (size, align) = field_size_align(ty, ptr_ty);
        let aligned = (offset + align as u32 - 1) & !(align as u32 - 1);
        fields.push((aligned, ty.clone()));
        offset = aligned + size;
        if align > max_align {
            max_align = align;
        }
    }

    let total = ((offset + max_align as u32 - 1) & !(max_align as u32 - 1)).max(1);
    (fields, total, max_align)
}

fn closure_env_layout(captures: &[HirClosureCapture], ptr_ty: types::Type) -> (Vec<u32>, u32) {
    let mut offset = 0u32;
    let mut max_align = 1u8;
    let mut offsets = Vec::with_capacity(captures.len());

    for capture in captures {
        let (size, align) = field_size_align(&capture.ty, ptr_ty);
        let aligned = (offset + align as u32 - 1) & !(align as u32 - 1);
        offsets.push(aligned);
        offset = aligned + size;
        if align > max_align {
            max_align = align;
        }
    }

    let total = if captures.is_empty() {
        0
    } else {
        ((offset + max_align as u32 - 1) & !(max_align as u32 - 1)).max(1)
    };
    (offsets, total)
}

fn compute_struct_layout(fields: &[HirField], ptr_ty: types::Type) -> StructLayout {
    let mut offset = 0u32;
    let mut max_align = 1u8;
    let mut result = Vec::new();
    for f in fields {
        let (size, align) = field_size_align(&f.ty, ptr_ty);
        // Align to field requirement
        let aligned = (offset + align as u32 - 1) & !(align as u32 - 1);
        result.push((
            f.name.clone(),
            FieldLayout {
                offset: aligned,
                ty: f.ty.clone(),
                contract: f.contract.clone(),
            },
        ));
        offset = aligned + size;
        if align > max_align {
            max_align = align;
        }
    }
    // Pad total to struct alignment
    let total = ((offset + max_align as u32 - 1) & !(max_align as u32 - 1)).max(1);
    StructLayout {
        size: total,
        align: max_align,
        fields: result,
    }
}

#[derive(Debug, Clone)]
pub struct VariantLayout {
    pub tag: u32,
    pub fields: StructLayout, // Layout of the payload fields
}

#[derive(Debug, Clone, Default)]
pub struct EnumLayout {
    pub size: u32,
    pub align: u8,
    pub payload_offset: u32, // Offset where the union payload begins
    pub variants: HashMap<SmolStr, VariantLayout>,
}

impl EnumLayout {
    pub fn variant(&self, name: &str) -> Option<&VariantLayout> {
        self.variants.get(name)
    }
}

fn compute_enum_layout(variants: &[ori_hir::hir::HirVariant], ptr_ty: types::Type) -> EnumLayout {
    let mut variant_layouts = HashMap::new();
    let tag_size = 4u32; // int32_t tag
    let tag_align = 4u8;

    let mut max_payload_size = 0u32;
    let mut max_payload_align = 1u8;

    for (tag_idx, v) in variants.iter().enumerate() {
        let payload_layout = compute_struct_layout(&v.fields, ptr_ty);
        if payload_layout.size > max_payload_size {
            max_payload_size = payload_layout.size;
        }
        if payload_layout.align > max_payload_align {
            max_payload_align = payload_layout.align;
        }
        variant_layouts.insert(
            v.name.clone(),
            VariantLayout {
                tag: tag_idx as u32,
                fields: payload_layout,
            },
        );
    }

    let overall_align = tag_align.max(max_payload_align);
    let payload_offset =
        (tag_size + max_payload_align as u32 - 1) & !(max_payload_align as u32 - 1);

    // Total size padded to overall alignment
    let total = ((payload_offset + max_payload_size + overall_align as u32 - 1)
        & !(overall_align as u32 - 1))
        .max(tag_size);

    EnumLayout {
        size: total,
        align: overall_align,
        payload_offset,
        variants: variant_layouts,
    }
}

// == Module-level backend ==

#[derive(Debug, Clone)]
struct GlobalDataInfo {
    data_id: DataId,
    ty: Ty,
    mutable: bool,
}

#[derive(Debug, Clone)]
struct UsingCleanup {
    var: Variable,
    ty: Ty,
}

#[derive(Debug, Clone)]
struct ManagedCleanup {
    var: Variable,
    ty: Ty,
}

#[derive(Debug, Clone, Copy)]
struct LoopContext {
    continue_target: ir::Block,
    break_target: ir::Block,
    cleanup_start: usize,
    managed_cleanup_start: usize,
}

pub struct NativeBackend {
    module: ObjectModule,
    ptr_ty: types::Type,
    func_ids: HashMap<SmolStr, FuncId>,
    func_wrapper_ids: HashMap<SmolStr, FuncId>,
    stdlib_ids: HashMap<SmolStr, FuncId>,
    string_data: HashMap<SmolStr, DataId>,
    global_data: HashMap<SmolStr, GlobalDataInfo>,
    struct_layouts: HashMap<ori_types::DefId, StructLayout>,
    enum_layouts: HashMap<ori_types::DefId, EnumLayout>,
    type_names: HashMap<ori_types::DefId, SmolStr>,
    trait_layouts: HashMap<ori_types::DefId, HirTrait>,
    trait_impls: HashMap<(ori_types::DefId, ori_types::DefId), HirTraitImpl>,
    func_param_tys: HashMap<SmolStr, Vec<Ty>>,
}

impl NativeBackend {
    pub fn new() -> Result<Self, String> {
        let flags = settings::Flags::new(settings::builder());
        let isa = cranelift_native::builder()
            .map_err(|e| format!("native ISA unavailable: {e}"))?
            .finish(flags)
            .map_err(|e| format!("ISA build failed: {e}"))?;
        let ptr_ty = isa.pointer_type();
        let builder =
            ObjectBuilder::new(isa, "ori_module", cranelift_module::default_libcall_names())
                .map_err(|e| format!("ObjectBuilder failed: {e}"))?;
        Ok(Self {
            module: ObjectModule::new(builder),
            ptr_ty,
            func_ids: HashMap::new(),
            func_wrapper_ids: HashMap::new(),
            stdlib_ids: HashMap::new(),
            string_data: HashMap::new(),
            global_data: HashMap::new(),
            struct_layouts: HashMap::new(),
            enum_layouts: HashMap::new(),
            type_names: HashMap::new(),
            trait_layouts: HashMap::new(),
            trait_impls: HashMap::new(),
            func_param_tys: HashMap::new(),
        })
    }

    pub fn compile(mut self, hir: &HirModule) -> Result<Vec<u8>, String> {
        // Compute struct layouts before anything else
        for s in &hir.structs {
            let layout = compute_struct_layout(&s.fields, self.ptr_ty);
            self.struct_layouts.insert(s.def_id, layout);
            self.type_names.insert(s.def_id, s.name.clone());
        }
        for e in &hir.enums {
            let layout = compute_enum_layout(&e.variants, self.ptr_ty);
            self.enum_layouts.insert(e.def_id, layout);
            self.type_names.insert(e.def_id, e.name.clone());
        }
        for t in &hir.traits {
            self.trait_layouts.insert(t.def_id, t.clone());
        }
        for imp in &hir.trait_impls {
            self.trait_impls
                .insert((imp.trait_def_id, imp.type_def_id), imp.clone());
        }
        for f in &hir.funcs {
            self.func_param_tys.insert(
                f.name.clone(),
                f.params.iter().map(|param| param.ty.clone()).collect(),
            );
        }
        self.emit_module_strings(hir)?;
        self.emit_global_data(hir)?;
        self.declare_stdlib()?;
        self.declare_all(hir)?;
        self.define_all(hir)?;
        self.module
            .finish()
            .emit()
            .map_err(|e| format!("object emit failed: {e}"))
    }

    /// Emit all string literals as static null-terminated data in .data.
    fn emit_module_strings(&mut self, hir: &HirModule) -> Result<(), String> {
        for s in collect_all_strings(hir) {
            if self.string_data.contains_key(&s) {
                continue;
            }
            let mut bytes: Vec<u8> = Vec::new();
            // Prepend ori_heap_header_t (16 bytes on 64-bit: refcount i64, destructor ptr)
            bytes.extend_from_slice(&1_000_000_000i64.to_le_bytes()); // huge refcount so it never frees
            bytes.extend_from_slice(&0i64.to_le_bytes()); // null destructor
            bytes.extend_from_slice(s.as_bytes());
            bytes.push(0); // null-terminate for `puts` compatibility
            let mut desc = DataDescription::new();
            desc.define(bytes.into_boxed_slice());
            let id = self
                .module
                .declare_anonymous_data(true, false) // writable to allow refcount mutation
                .map_err(|e| format!("declare string data: {e}"))?;
            self.module
                .define_data(id, &desc)
                .map_err(|e| format!("define string data: {e}"))?;
            self.string_data.insert(s, id);
        }
        Ok(())
    }

    fn emit_global_data(&mut self, hir: &HirModule) -> Result<(), String> {
        for c in &hir.consts {
            let Some(bytes) = const_static_bytes(&c.value, &c.ty) else {
                continue;
            };
            let mut desc = DataDescription::new();
            desc.define(bytes.into_boxed_slice());
            let link = if c.is_public {
                Linkage::Export
            } else {
                Linkage::Local
            };
            let id = self
                .module
                .declare_data(&native_global_symbol(&c.name), link, c.mutable, false)
                .map_err(|e| format!("declare global '{}': {e}", c.name))?;
            self.module
                .define_data(id, &desc)
                .map_err(|e| format!("define global '{}': {e}", c.name))?;
            self.global_data.insert(
                c.name.clone(),
                GlobalDataInfo {
                    data_id: id,
                    ty: c.ty.clone(),
                    mutable: c.mutable,
                },
            );
        }
        Ok(())
    }

    /// Declare C library / runtime functions used by the stdlib mapping.
    fn declare_stdlib(&mut self) -> Result<(), String> {
        let pt = self.ptr_ty;
        let mut decl = |name: &'static str, params: &[types::Type], ret: Option<types::Type>| {
            let mut sig = self.module.make_signature();
            for &p in params {
                sig.params.push(AbiParam::new(p));
            }
            if let Some(r) = ret {
                sig.returns.push(AbiParam::new(r));
            }
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
        let id = decl("ori_io_read_line", &[], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_io_read_line"), id);
        // Compatibility pointer return for stored `string(n)` values.
        let id = decl("ori_int_to_cstr", &[types::I64], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_to_string"), id);
        // Length-aware numeric conversion used by direct print/interpolation paths.
        let id = decl("ori_to_string_parts", &[types::I64, pt, pt], None)?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_to_string_parts"), id);
        // strlen(ptr: *u8) -> i64
        let id = decl("strlen", &[pt], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("strlen"), id);
        let id = decl("strcmp", &[pt, pt], Some(types::I32))?;
        self.stdlib_ids.insert(SmolStr::new("strcmp"), id);
        let id = decl("ori_string_len", &[pt], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_string_len"), id);
        let id = decl("ori_string_concat", &[pt, pt], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_concat"), id);
        let id = decl(
            "ori_string_concat_parts",
            &[pt, types::I64, pt, types::I64],
            Some(pt),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_concat_parts"), id);
        let id = decl("ori_string_split", &[pt, pt], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_string_split"), id);
        let id = decl("ori_string_slice", &[pt, types::I64, types::I64], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_string_slice"), id);
        let id = decl("ori_string_contains", &[pt, pt], Some(types::I8))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_contains"), id);
        let id = decl("ori_string_starts_with", &[pt, pt], Some(types::I8))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_starts_with"), id);
        let id = decl("ori_string_ends_with", &[pt, pt], Some(types::I8))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_ends_with"), id);
        let id = decl("ori_string_trim", &[pt], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_string_trim"), id);
        let id = decl("ori_string_to_upper", &[pt], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_to_upper"), id);
        let id = decl("ori_string_to_lower", &[pt], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_to_lower"), id);
        let id = decl("ori_string_replace", &[pt, pt, pt], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_replace"), id);
        let id = decl("ori_string_chars", &[pt], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_string_chars"), id);
        let id = decl("ori_string_index_of", &[pt, pt], Some(types::I64))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_index_of"), id);
        let id = decl("ori_string_join", &[pt, pt], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_string_join"), id);
        let id = decl("ori_string_repeat", &[pt, types::I64], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_repeat"), id);
        let id = decl("ori_string_pad_left", &[pt, types::I64, pt], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_pad_left"), id);
        let id = decl("ori_string_pad_right", &[pt, types::I64, pt], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_pad_right"), id);
        // ori_len(ptr: *u8) -> i64
        let id = decl("ori_len", &[pt], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_len"), id);
        // ori_math_abs(n: i64) -> i64
        let id = decl("ori_math_sqrt", &[types::F64], Some(types::F64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_sqrt"), id);
        let id = decl("ori_math_abs", &[types::I64], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_abs"), id);
        // ori_math_min / ori_math_max
        let id = decl("ori_math_min", &[types::I64, types::I64], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_min"), id);
        let id = decl("ori_math_max", &[types::I64, types::I64], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_max"), id);
        let id = decl("ori_math_pow", &[types::F64, types::F64], Some(types::F64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_pow"), id);
        let id = decl("ori_math_floor", &[types::F64], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_floor"), id);
        let id = decl("ori_math_ceil", &[types::F64], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_ceil"), id);
        let id = decl("ori_math_round", &[types::F64], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_round"), id);
        let id = decl("ori_math_log", &[types::F64], Some(types::F64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_log"), id);
        let id = decl("ori_math_sin", &[types::F64], Some(types::F64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_sin"), id);
        let id = decl("ori_math_cos", &[types::F64], Some(types::F64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_cos"), id);
        let id = decl("ori_math_tan", &[types::F64], Some(types::F64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_tan"), id);
        let id = decl("ori_float_to_string", &[types::F64], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_float_to_string"), id);
        let id = decl("ori_bool_to_string", &[types::I8], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_bool_to_string"), id);
        let id = decl("ori_string_to_int", &[pt], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_to_int"), id);
        let id = decl("ori_string_to_float", &[pt], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_to_float"), id);
        // list<T> runtime
        let id = decl("ori_list_new", &[], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_list_new"), id);
        let id = decl("ori_list_push", &[pt, types::I64], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_list_push"), id);
        let id = decl("ori_list_get", &[pt, types::I64], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_list_get"), id);
        let id = decl("ori_list_set", &[pt, types::I64, types::I64], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_list_set"), id);
        let id = decl("ori_list_len", &[pt], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_list_len"), id);
        let id = decl("ori_list_free", &[pt], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_list_free"), id);
        let id = decl("ori_list_pop", &[pt], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_list_pop"), id);
        let id = decl("ori_list_remove", &[pt, types::I64], None)?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_list_remove"), id);
        let id = decl("ori_list_insert", &[pt, types::I64, types::I64], None)?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_list_insert"), id);
        let id = decl("ori_list_contains", &[pt, types::I64], Some(types::I8))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_list_contains"), id);
        let id = decl("ori_list_index_of", &[pt, types::I64], Some(types::I64))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_list_index_of"), id);
        let id = decl("ori_list_sort", &[pt], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_list_sort"), id);
        let id = decl("ori_list_reverse", &[pt], None)?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_list_reverse"), id);
        let id = decl("ori_list_slice", &[pt, types::I64, types::I64], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_list_slice"), id);
        let id = decl("ori_set_new", &[], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_set_new"), id);
        let id = decl("ori_set_add", &[pt, types::I64], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_set_add"), id);
        let id = decl("ori_set_contains", &[pt, types::I64], Some(types::I8))?;
        self.stdlib_ids.insert(SmolStr::new("ori_set_contains"), id);
        let id = decl("ori_set_len", &[pt], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_set_len"), id);
        let id = decl("ori_set_free", &[pt], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_set_free"), id);
        let id = decl("ori_set_remove", &[pt, types::I64], None)?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_set_remove"), id);
        let id = decl("ori_set_union", &[pt, pt], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_set_union"), id);
        let id = decl("ori_set_intersection", &[pt, pt], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_set_intersection"), id);
        let id = decl("ori_set_difference", &[pt, pt], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_set_difference"), id);
        let id = decl("ori_list_map", &[pt, pt, pt], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_list_map"), id);
        let id = decl("ori_list_filter", &[pt, pt, pt], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_list_filter"), id);
        let id = decl("ori_map_new", &[], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_new"), id);
        let id = decl("ori_map_set", &[pt, types::I64, types::I64], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_set"), id);
        let id = decl("ori_map_get", &[pt, types::I64], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_get"), id);
        let id = decl("ori_map_contains", &[pt, types::I64], Some(types::I8))?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_contains"), id);
        let id = decl("ori_map_len", &[pt], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_len"), id);
        let id = decl("ori_map_key_at", &[pt, types::I64], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_key_at"), id);
        let id = decl("ori_map_value_at", &[pt, types::I64], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_value_at"), id);
        let id = decl("ori_map_free", &[pt], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_free"), id);
        let id = decl("ori_map_remove", &[pt, types::I64], None)?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_map_remove"), id);
        let id = decl("ori_map_keys", &[pt], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_keys"), id);
        let id = decl("ori_map_values", &[pt], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_map_values"), id);
        let id = decl("ori_map_entries", &[pt], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_map_entries"), id);
        let id = decl("ori_arc_retain", &[pt], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_arc_retain"), id);
        let id = decl("ori_arc_release", &[pt], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_arc_release"), id);
        let id = decl("ori_arc_collect_cycles", &[], Some(types::I64))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_arc_collect_cycles"), id);
        // malloc / free for runtime allocation
        let id = decl("malloc", &[types::I64], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("malloc"), id);
        let id = decl("free", &[pt], None)?;
        self.stdlib_ids.insert(SmolStr::new("free"), id);
        let id = decl("ori_alloc", &[types::I64, pt], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_alloc"), id);
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

    fn make_closure_wrapper_sig(&self, f: &HirFunc) -> ir::Signature {
        let mut sig = self.module.make_signature();
        sig.params.push(AbiParam::new(self.ptr_ty));
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
            let sig = self.make_sig(f);
            let link = if f.is_public {
                Linkage::Export
            } else {
                Linkage::Local
            };
            let id = self
                .module
                .declare_function(&native_func_symbol(&f.name), link, &sig)
                .map_err(|e| format!("declare '{}': {e}", f.name))?;
            self.func_ids.insert(f.name.clone(), id);
        }
        for f in &hir.funcs {
            if is_synthetic_closure_func(f) {
                continue;
            }
            let sig = self.make_closure_wrapper_sig(f);
            let id = self
                .module
                .declare_function(&native_func_wrapper_symbol(&f.name), Linkage::Local, &sig)
                .map_err(|e| format!("declare closure wrapper '{}': {e}", f.name))?;
            self.func_wrapper_ids.insert(f.name.clone(), id);
        }
        if hir.funcs.iter().any(|f| is_entry_main(hir, f)) {
            let mut sig = self.module.make_signature();
            sig.returns.push(AbiParam::new(types::I32));
            self.module
                .declare_function("main", Linkage::Export, &sig)
                .map_err(|e| format!("declare main: {e}"))?;
        }
        Ok(())
    }

    fn define_all(&mut self, hir: &HirModule) -> Result<(), String> {
        let const_exprs: HashMap<SmolStr, HirExpr> = hir
            .consts
            .iter()
            .map(|c| (c.name.clone(), c.value.clone()))
            .collect();
        for f in &hir.funcs {
            let sig = self.make_sig(f);
            let func_id = self.func_ids[&f.name];
            let mut ctx = self.module.make_context();
            ctx.func.signature = sig;

            // Pre-declare ALL function references (user + stdlib) before builder takes ownership
            let mut func_refs: HashMap<SmolStr, ir::FuncRef> = HashMap::new();
            for (name, &id) in self.func_ids.iter().chain(self.stdlib_ids.iter()) {
                let fref = self.module.declare_func_in_func(id, &mut ctx.func);
                func_refs.insert(name.clone(), fref);
            }
            for (name, &id) in &self.func_wrapper_ids {
                let fref = self.module.declare_func_in_func(id, &mut ctx.func);
                func_refs.insert(SmolStr::new(format!("{name}.__fnptr_wrapper")), fref);
            }

            // Pre-declare all string global values
            let mut string_gvs: HashMap<SmolStr, ir::GlobalValue> = HashMap::new();
            for (s, &data_id) in &self.string_data {
                let gv = self.module.declare_data_in_func(data_id, &mut ctx.func);
                string_gvs.insert(s.clone(), gv);
            }

            let mut global_gvs: HashMap<SmolStr, ir::GlobalValue> = HashMap::new();
            for (name, info) in &self.global_data {
                let gv = self
                    .module
                    .declare_data_in_func(info.data_id, &mut ctx.func);
                global_gvs.insert(name.clone(), gv);
            }

            let mut bctx = FunctionBuilderContext::new();
            {
                let builder = FunctionBuilder::new(&mut ctx.func, &mut bctx);
                FuncCodegen {
                    builder,
                    func_refs: &func_refs,
                    string_gvs: &string_gvs,
                    global_gvs: &global_gvs,
                    global_data: &self.global_data,
                    const_exprs: &const_exprs,
                    struct_layouts: &self.struct_layouts,
                    enum_layouts: &self.enum_layouts,
                    type_names: &self.type_names,
                    trait_layouts: &self.trait_layouts,
                    trait_impls: &self.trait_impls,
                    func_param_tys: &self.func_param_tys,
                    vars: HashMap::new(),
                    ptr_ty: self.ptr_ty,
                    loop_stack: Vec::new(),
                    using_stack: Vec::new(),
                    managed_stack: Vec::new(),
                    current_return_ty: f.return_ty.clone(),
                    terminated: false,
                }
                .emit(f)?;
            }
            self.module
                .define_function(func_id, &mut ctx)
                .map_err(|e| format!("define '{}': {e}", f.name))?;
        }

        for f in &hir.funcs {
            if is_synthetic_closure_func(f) {
                continue;
            }
            let wrapper_id = self.func_wrapper_ids[&f.name];
            let original_id = self.func_ids[&f.name];
            let mut ctx = self.module.make_context();
            ctx.func.signature = self.make_closure_wrapper_sig(f);
            let original_ref = self.module.declare_func_in_func(original_id, &mut ctx.func);
            let mut bctx = FunctionBuilderContext::new();
            {
                let mut b = FunctionBuilder::new(&mut ctx.func, &mut bctx);
                let block = b.create_block();
                b.append_block_params_for_function_params(block);
                b.switch_to_block(block);
                b.seal_block(block);
                let params: Vec<ir::Value> =
                    b.block_params(block).iter().skip(1).copied().collect();
                let call = b.ins().call(original_ref, &params);
                let results = b.inst_results(call).to_vec();
                if results.is_empty() {
                    b.ins().return_(&[]);
                } else {
                    b.ins().return_(&results);
                }
                b.seal_all_blocks();
                b.finalize();
            }
            self.module
                .define_function(wrapper_id, &mut ctx)
                .map_err(|e| format!("define closure wrapper '{}': {e}", f.name))?;
        }

        // Define C main wrapper
        if let Some(entry_main) = hir.funcs.iter().find(|f| is_entry_main(hir, f)) {
            let ori_main_id = self.func_ids[&entry_main.name];
            let mut sig = self.module.make_signature();
            sig.returns.push(AbiParam::new(types::I32));
            let main_id = self
                .module
                .declare_function("main", Linkage::Export, &sig)
                .map_err(|e| format!("re-declare main: {e}"))?;
            let mut ctx = self.module.make_context();
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
            self.module
                .define_function(main_id, &mut ctx)
                .map_err(|e| format!("define main wrapper: {e}"))?;
        }
        Ok(())
    }
}

// == Per-function codegen ==

struct FuncCodegen<'a> {
    builder: FunctionBuilder<'a>,
    func_refs: &'a HashMap<SmolStr, ir::FuncRef>,
    string_gvs: &'a HashMap<SmolStr, ir::GlobalValue>,
    global_gvs: &'a HashMap<SmolStr, ir::GlobalValue>,
    global_data: &'a HashMap<SmolStr, GlobalDataInfo>,
    const_exprs: &'a HashMap<SmolStr, HirExpr>,
    struct_layouts: &'a HashMap<ori_types::DefId, StructLayout>,
    enum_layouts: &'a HashMap<ori_types::DefId, EnumLayout>,
    type_names: &'a HashMap<ori_types::DefId, SmolStr>,
    trait_layouts: &'a HashMap<ori_types::DefId, HirTrait>,
    trait_impls: &'a HashMap<(ori_types::DefId, ori_types::DefId), HirTraitImpl>,
    func_param_tys: &'a HashMap<SmolStr, Vec<Ty>>,
    vars: HashMap<SmolStr, (Variable, Ty)>,
    ptr_ty: types::Type,
    loop_stack: Vec<LoopContext>,
    using_stack: Vec<UsingCleanup>,
    managed_stack: Vec<ManagedCleanup>,
    current_return_ty: Ty,
    terminated: bool,
}

impl<'a> FuncCodegen<'a> {
    fn emit(mut self, f: &HirFunc) -> Result<(), String> {
        let entry = self.builder.create_block();
        self.builder.append_block_params_for_function_params(entry);
        self.builder.switch_to_block(entry);
        self.builder.seal_block(entry);

        // Bind parameters
        let params: Vec<(SmolStr, Ty, ir::Value)> = f
            .params
            .iter()
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
                self.vars.insert(name.clone(), (var, ty.clone()));
                if name.as_str() != "__env" && is_managed_ty(&ty) {
                    self.managed_stack.push(ManagedCleanup { var, ty });
                }
            }
        }

        self.emit_closure_capture_prologue(f)?;

        self.emit_param_contracts(&f.params)?;
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

    fn emit_param_contracts(&mut self, params: &[HirParam]) -> Result<(), String> {
        for param in params {
            let Some(contract) = &param.contract else {
                continue;
            };
            let Some((var, ty)) = self.vars.get(&param.name).cloned() else {
                continue;
            };
            let value = self.builder.use_var(var);
            self.emit_value_contract(&ty, value, contract, 2, false)?;
        }
        Ok(())
    }

    fn emit_closure_capture_prologue(&mut self, f: &HirFunc) -> Result<(), String> {
        if f.closure_captures.is_empty() {
            return Ok(());
        }
        let Some((env_var, _)) = self.vars.get("__env").cloned() else {
            return Err(format!(
                "closure `{}` has captures but no environment parameter",
                f.name
            ));
        };
        let env = self.builder.use_var(env_var);
        let (offsets, _) = closure_env_layout(&f.closure_captures, self.ptr_ty);
        for (capture, offset) in f.closure_captures.iter().zip(offsets) {
            let Some(cl_ty) = cl_type(&capture.ty, self.ptr_ty) else {
                continue;
            };
            let value = self
                .builder
                .ins()
                .load(cl_ty, MemFlags::new(), env, offset as i32);
            let var = self.builder.declare_var(cl_ty);
            self.builder.def_var(var, value);
            self.vars
                .insert(capture.name.clone(), (var, capture.ty.clone()));
            if is_managed_ty(&capture.ty) {
                self.emit_arc_retain_if_managed(&capture.ty, value)?;
                self.managed_stack.push(ManagedCleanup {
                    var,
                    ty: capture.ty.clone(),
                });
            }
        }
        Ok(())
    }

    fn emit_value_contract(
        &mut self,
        ty: &Ty,
        value: ir::Value,
        contract: &HirExpr,
        trap_code: u8,
        run_cleanup: bool,
    ) -> Result<(), String> {
        let Some(cl_ty) = cl_type(ty, self.ptr_ty) else {
            return Ok(());
        };
        let it_var = self.builder.declare_var(cl_ty);
        self.builder.def_var(it_var, value);
        let it_name = SmolStr::new("it");
        let previous = self.vars.insert(it_name.clone(), (it_var, ty.clone()));
        let condition = self.emit_expr(contract);
        if let Some(previous) = previous {
            self.vars.insert(it_name, previous);
        } else {
            self.vars.remove("it");
        }
        let condition = condition?;
        self.emit_trap_unless(
            condition,
            ir::TrapCode::user(trap_code).unwrap(),
            run_cleanup,
        )
    }

    fn emit_trap_unless(
        &mut self,
        condition: ir::Value,
        trap_code: ir::TrapCode,
        run_cleanup: bool,
    ) -> Result<(), String> {
        let ok_blk = self.builder.create_block();
        let fail_blk = self.builder.create_block();
        self.builder
            .ins()
            .brif(condition, ok_blk, &[], fail_blk, &[]);
        self.builder.seal_block(fail_blk);
        self.builder.switch_to_block(fail_blk);
        if run_cleanup {
            self.emit_scope_cleanup_calls_from(0, 0)?;
        }
        self.builder.ins().trap(trap_code);
        self.builder.seal_block(ok_blk);
        self.builder.switch_to_block(ok_blk);
        self.terminated = false;
        Ok(())
    }

    fn emit_expr_for_expected(
        &mut self,
        expr: &HirExpr,
        expected: &Ty,
    ) -> Result<ir::Value, String> {
        if matches!(expected, Ty::Func { .. }) {
            if let HirExprKind::Var(name) = &expr.kind {
                if self.vars.get(name).is_none()
                    && self.global_data.get(name).is_none()
                    && self.const_exprs.get(name).is_none()
                    && self
                        .func_refs
                        .contains_key(format!("{name}.__fnptr_wrapper").as_str())
                {
                    return self.emit_named_function_closure(name);
                }
            }
        }
        let value = self.emit_expr(expr)?;
        if let (Ty::Any(trait_def_id), Ty::Named(type_def_id, _)) = (expected, &expr.ty) {
            self.emit_any_box(*trait_def_id, *type_def_id, value)
        } else {
            Ok(value)
        }
    }

    fn emit_named_function_closure(&mut self, name: &SmolStr) -> Result<ir::Value, String> {
        let wrapper_name = SmolStr::new(format!("{name}.__fnptr_wrapper"));
        let fref = *self.func_refs.get(wrapper_name.as_str()).ok_or_else(|| {
            format!("missing closure wrapper reference `{wrapper_name}` in native codegen")
        })?;
        self.emit_closure_object(fref, None)
    }

    fn emit_closure_object(
        &mut self,
        fref: ir::FuncRef,
        env_ptr: Option<ir::Value>,
    ) -> Result<ir::Value, String> {
        let ptr_size = self.ptr_ty.bytes() as i64;
        let object = self.malloc_bytes((ptr_size * 2) as u32)?;
        let fn_ptr = self.builder.ins().func_addr(self.ptr_ty, fref);
        let env = env_ptr.unwrap_or_else(|| self.builder.ins().iconst(self.ptr_ty, 0));
        self.builder.ins().store(MemFlags::new(), fn_ptr, object, 0);
        self.builder
            .ins()
            .store(MemFlags::new(), env, object, ptr_size as i32);
        Ok(object)
    }

    fn emit_any_box(
        &mut self,
        trait_def_id: ori_types::DefId,
        type_def_id: ori_types::DefId,
        data_ptr: ir::Value,
    ) -> Result<ir::Value, String> {
        let trait_layout = self
            .trait_layouts
            .get(&trait_def_id)
            .ok_or_else(|| format!("missing trait layout for `{trait_def_id}`"))?
            .clone();
        let impl_sig = self
            .trait_impls
            .get(&(trait_def_id, type_def_id))
            .ok_or_else(|| {
                format!("missing implementation of `{trait_def_id}` for `{type_def_id}`")
            })?
            .clone();

        let ptr_size = self.ptr_ty.bytes() as i64;
        let vtable_size = (trait_layout.methods.len() as i64 + 1) * ptr_size;
        let vtable = self.malloc_bytes(vtable_size as u32)?;
        
        let type_id_val = self.builder.ins().iconst(self.ptr_ty, type_def_id.0 as i64);
        self.builder.ins().store(MemFlags::new(), type_id_val, vtable, 0);
        for (index, method) in trait_layout.methods.iter().enumerate() {
            let func_name = impl_sig
                .methods
                .iter()
                .find(|m| m.name == method.name)
                .map(|method| method.func_name.clone())
                .or_else(|| method.default_func_name.clone())
                .ok_or_else(|| format!("trait method `{}` has no implementation", method.name))?;
            let fref = *self.func_refs.get(func_name.as_str()).ok_or_else(|| {
                format!(
                    "missing function reference `{}` for trait dispatch",
                    func_name
                )
            })?;
            let fn_ptr = self.builder.ins().func_addr(self.ptr_ty, fref);
            self.builder.ins().store(
                MemFlags::new(),
                fn_ptr,
                vtable,
                ((index as i64 + 1) * ptr_size) as i32,
            );
        }

        let object = self.malloc_bytes((ptr_size * 2) as u32)?;
        self.builder
            .ins()
            .store(MemFlags::new(), data_ptr, object, 0);
        self.builder
            .ins()
            .store(MemFlags::new(), vtable, object, ptr_size as i32);
        Ok(object)
    }

    fn emit_dynamic_method_call(
        &mut self,
        receiver: &HirExpr,
        method_name: &SmolStr,
        args: &[HirExpr],
    ) -> Result<ir::Value, String> {
        let Ty::Any(trait_def_id) = &receiver.ty else {
            return Err(format!(
                "dynamic method call requires `any<Trait>`, got `{}`",
                receiver.ty.display()
            ));
        };
        let trait_def_id = *trait_def_id;
        let trait_layout = self
            .trait_layouts
            .get(&trait_def_id)
            .ok_or_else(|| format!("missing trait layout for `{trait_def_id}`"))?;
        let Some((method_index, method)) = trait_layout
            .methods
            .iter()
            .enumerate()
            .find(|(_, method)| method.name == *method_name)
        else {
            return Err(format!(
                "trait `{}` has no method `{}`",
                trait_layout.name, method_name
            ));
        };

        let any_ptr = self.emit_expr(receiver)?;
        let data_ptr = self
            .builder
            .ins()
            .load(self.ptr_ty, MemFlags::new(), any_ptr, 0);
        let ptr_size = self.ptr_ty.bytes() as i64;
        let vtable =
            self.builder
                .ins()
                .load(self.ptr_ty, MemFlags::new(), any_ptr, ptr_size as i32);
        let fn_ptr = self.builder.ins().load(
            self.ptr_ty,
            MemFlags::new(),
            vtable,
            ((method_index as i64 + 1) * ptr_size) as i32,
        );

        let mut sig = ir::Signature::new(self.builder.func.signature.call_conv);
        sig.params.push(AbiParam::new(self.ptr_ty));
        for param_ty in method.params.iter().skip(1) {
            if let Some(cl_ty) = cl_type(param_ty, self.ptr_ty) {
                sig.params.push(AbiParam::new(cl_ty));
            }
        }
        if let Some(ret_ty) = cl_type(&method.return_ty, self.ptr_ty) {
            sig.returns.push(AbiParam::new(ret_ty));
        }
        let sig_ref = self.builder.func.import_signature(sig);

        let mut call_args = vec![data_ptr];
        // The trait method expects `self` to be retained by the caller
        if let Some(&retain_ref) = self.func_refs.get("ori_arc_retain") {
            self.builder.ins().call(retain_ref, &[data_ptr]);
        }
        for (arg, expected) in args.iter().zip(method.params.iter().skip(1)) {
            let value = self.emit_expr_for_expected(arg, expected)?;
            self.emit_arc_retain_if_managed(expected, value)?;
            call_args.push(value);
        }
        let call = self
            .builder
            .ins()
            .call_indirect(sig_ref, fn_ptr, &call_args);
        let res = self.builder.inst_results(call);
        if res.is_empty() {
            Ok(self.builder.ins().iconst(types::I8, 0))
        } else {
            Ok(res[0])
        }
    }

    fn emit_closure_value(
        &mut self,
        func_name: &SmolStr,
        captures: &[HirClosureCapture],
    ) -> Result<ir::Value, String> {
        let fref = *self.func_refs.get(func_name.as_str()).ok_or_else(|| {
            format!("missing closure function reference `{func_name}` in native codegen")
        })?;
        let env_ptr = if captures.is_empty() {
            None
        } else {
            let (offsets, total) = closure_env_layout(captures, self.ptr_ty);
            let env = self.malloc_bytes(total)?;
            for (capture, offset) in captures.iter().zip(offsets) {
                let value = if let Some((var, _)) = self.vars.get(&capture.name).cloned() {
                    self.builder.use_var(var)
                } else if let Some(value) = self.load_global(&capture.name) {
                    value
                } else if let Some(expr) = self.const_exprs.get(&capture.name).cloned() {
                    self.emit_expr(&expr)?
                } else {
                    return Err(format!(
                        "closure capture `{}` is not available in native codegen",
                        capture.name
                    ));
                };
                self.emit_arc_retain_if_managed(&capture.ty, value)?;
                if let Some(cl_ty) = cl_type(&capture.ty, self.ptr_ty) {
                    let stored = if cl_ty == self.ptr_ty || cl_ty == types::I64 {
                        value
                    } else {
                        value
                    };
                    self.builder
                        .ins()
                        .store(MemFlags::new(), stored, env, offset as i32);
                }
            }
            Some(env)
        };
        self.emit_closure_object(fref, env_ptr)
    }

    fn emit_closure_call(
        &mut self,
        callee: &HirExpr,
        args: &[HirArg],
    ) -> Result<ir::Value, String> {
        let Ty::Func { params, ret } = &callee.ty else {
            return Err(format!(
                "closure call requires a function value, got `{}`",
                callee.ty.display()
            ));
        };
        let params = params.clone();
        let ret = *ret.clone();
        let closure = self.emit_expr(callee)?;
        let ptr_size = self.ptr_ty.bytes() as i64;
        let fn_ptr = self
            .builder
            .ins()
            .load(self.ptr_ty, MemFlags::new(), closure, 0);
        let env_ptr =
            self.builder
                .ins()
                .load(self.ptr_ty, MemFlags::new(), closure, ptr_size as i32);

        let mut sig = ir::Signature::new(self.builder.func.signature.call_conv);
        sig.params.push(AbiParam::new(self.ptr_ty));
        for param_ty in &params {
            if let Some(cl_ty) = cl_type(param_ty, self.ptr_ty) {
                sig.params.push(AbiParam::new(cl_ty));
            }
        }
        if let Some(ret_ty) = cl_type(&ret, self.ptr_ty) {
            sig.returns.push(AbiParam::new(ret_ty));
        }
        let sig_ref = self.builder.func.import_signature(sig);

        let mut call_args = vec![env_ptr];
        for (arg, expected) in args.iter().zip(params.iter()) {
            let value = self.emit_expr_for_expected(&arg.value, expected)?;
            self.emit_arc_retain_if_managed(expected, value)?;
            call_args.push(value);
        }
        let call = self
            .builder
            .ins()
            .call_indirect(sig_ref, fn_ptr, &call_args);
        let res = self.builder.inst_results(call);
        if res.is_empty() {
            Ok(self.builder.ins().iconst(types::I8, 0))
        } else {
            Ok(res[0])
        }
    }

    fn emit_new_list(&mut self) -> Result<ir::Value, String> {
        let new_ref = *self
            .func_refs
            .get("ori_list_new")
            .ok_or_else(|| "missing runtime function `ori_list_new`".to_string())?;
        let call = self.builder.ins().call(new_ref, &[]);
        Ok(self.builder.inst_results(call)[0])
    }

    fn emit_list_push_value(
        &mut self,
        list: ir::Value,
        value: ir::Value,
        elem_ty: &Ty,
    ) -> Result<(), String> {
        let push_ref = *self
            .func_refs
            .get("ori_list_push")
            .ok_or_else(|| "missing runtime function `ori_list_push`".to_string())?;
        let stored = self.to_list_storage_value(value, elem_ty);
        self.builder.ins().call(push_ref, &[list, stored]);
        Ok(())
    }

    fn emit_list_extend_from(
        &mut self,
        target: ir::Value,
        source: ir::Value,
        elem_ty: &Ty,
    ) -> Result<(), String> {
        let len_ref = *self
            .func_refs
            .get("ori_list_len")
            .ok_or_else(|| "missing runtime function `ori_list_len`".to_string())?;
        let get_ref = *self
            .func_refs
            .get("ori_list_get")
            .ok_or_else(|| "missing runtime function `ori_list_get`".to_string())?;
        let len_call = self.builder.ins().call(len_ref, &[source]);
        let len_v = self.builder.inst_results(len_call)[0];
        let idx_var = self.builder.declare_var(types::I64);
        let zero = self.builder.ins().iconst(types::I64, 0);
        self.builder.def_var(idx_var, zero);
        let len_var = self.builder.declare_var(types::I64);
        self.builder.def_var(len_var, len_v);

        let header = self.builder.create_block();
        let body = self.builder.create_block();
        let step = self.builder.create_block();
        let exit = self.builder.create_block();

        self.builder.ins().jump(header, &[]);
        self.builder.switch_to_block(header);
        let cur = self.builder.use_var(idx_var);
        let len = self.builder.use_var(len_var);
        let cond = self
            .builder
            .ins()
            .icmp(ir::condcodes::IntCC::SignedLessThan, cur, len);
        self.builder.ins().brif(cond, body, &[], exit, &[]);

        self.builder.seal_block(body);
        self.builder.switch_to_block(body);
        let cur = self.builder.use_var(idx_var);
        let call = self.builder.ins().call(get_ref, &[source, cur]);
        let stored = self.builder.inst_results(call)[0];
        let value = self.from_list_storage_value(stored, elem_ty);
        self.emit_list_push_value(target, value, elem_ty)?;
        self.builder.ins().jump(step, &[]);

        self.builder.seal_block(step);
        self.builder.switch_to_block(step);
        let cur = self.builder.use_var(idx_var);
        let one = self.builder.ins().iconst(types::I64, 1);
        let next = self.builder.ins().iadd(cur, one);
        self.builder.def_var(idx_var, next);
        self.builder.ins().jump(header, &[]);

        self.builder.seal_block(header);
        self.builder.seal_block(exit);
        self.builder.switch_to_block(exit);
        self.terminated = false;
        Ok(())
    }

    fn zero_val(&mut self, ty: &Ty) -> ir::Value {
        match ty {
            Ty::Float | Ty::Float64 => self.builder.ins().f64const(0.0),
            Ty::Float32 => self.builder.ins().f32const(0.0),
            _ => {
                let cl = cl_type(ty, self.ptr_ty).unwrap_or(types::I64);
                self.builder.ins().iconst(cl, 0)
            }
        }
    }

    fn load_global(&mut self, name: &str) -> Option<ir::Value> {
        let gv = *self.global_gvs.get(name)?;
        let info = self.global_data.get(name)?;
        let addr = self.builder.ins().global_value(self.ptr_ty, gv);
        let cl = cl_type(&info.ty, self.ptr_ty)?;
        Some(self.builder.ins().load(cl, MemFlags::new(), addr, 0))
    }

    fn store_global(&mut self, name: &str, value: ir::Value) -> bool {
        let Some(gv) = self.global_gvs.get(name).copied() else {
            return false;
        };
        let Some(info) = self.global_data.get(name) else {
            return false;
        };
        if !info.mutable {
            return false;
        }
        let addr = self.builder.ins().global_value(self.ptr_ty, gv);
        self.builder.ins().store(MemFlags::new(), value, addr, 0);
        true
    }

    fn emit_lvalue_value(&mut self, lvalue: &HirLValue) -> Result<(ir::Value, Ty), String> {
        match lvalue {
            HirLValue::Var(name) => {
                if let Some((var, ty)) = self.vars.get(name).cloned() {
                    Ok((self.builder.use_var(var), ty))
                } else if let Some(info) = self.global_data.get(name).cloned() {
                    let value = self.load_global(name).ok_or_else(|| {
                        format!("global `{name}` is not available in native codegen")
                    })?;
                    Ok((value, info.ty))
                } else {
                    Err(format!("undefined lvalue base `{name}` in native codegen"))
                }
            }
            _ => Err("unsupported indexed assignment base in native codegen".into()),
        }
    }

    fn malloc_bytes(&mut self, size: u32) -> Result<ir::Value, String> {
        let alloc_ref = *self
            .func_refs
            .get("ori_alloc")
            .ok_or_else(|| "missing runtime function `ori_alloc`".to_string())?;
        let size_v = self.builder.ins().iconst(types::I64, size as i64);
        let dtor_v = self.builder.ins().iconst(self.ptr_ty, 0);
        let call = self.builder.ins().call(alloc_ref, &[size_v, dtor_v]);
        Ok(self.builder.inst_results(call)[0])
    }

    fn string_ptr(&mut self, value: &str) -> Result<ir::Value, String> {
        let gv = *self.string_gvs
            .get(value)
            .ok_or_else(|| format!("string literal `{value}` was not emitted in native codegen"))?;
        let base = self.builder.ins().global_value(self.ptr_ty, gv);
        // Skip the 16-byte header to point to the string payload
        Ok(self.builder.ins().iadd_imm(base, 16))
    }

    fn int_to_string_parts(&mut self, value: ir::Value) -> Result<StringParts, String> {
        let fref = *self
            .func_refs
            .get("ori_to_string_parts")
            .ok_or_else(|| "missing runtime function `ori_to_string_parts`".to_string())?;
        let ptr_slot = self.builder.create_sized_stack_slot(ir::StackSlotData::new(
            ir::StackSlotKind::ExplicitSlot,
            8,
            3,
        ));
        let len_slot = self.builder.create_sized_stack_slot(ir::StackSlotData::new(
            ir::StackSlotKind::ExplicitSlot,
            8,
            3,
        ));
        let ptr_addr = self.builder.ins().stack_addr(self.ptr_ty, ptr_slot, 0);
        let len_addr = self.builder.ins().stack_addr(self.ptr_ty, len_slot, 0);
        self.builder.ins().call(fref, &[value, ptr_addr, len_addr]);
        let ptr = self
            .builder
            .ins()
            .load(self.ptr_ty, MemFlags::new(), ptr_addr, 0);
        let len = self
            .builder
            .ins()
            .load(types::I64, MemFlags::new(), len_addr, 0);
        Ok(StringParts { ptr, len })
    }

    fn emit_to_string_call_parts(&mut self, expr: &HirExpr) -> Result<Option<StringParts>, String> {
        let HirExprKind::Call { callee, args } = &expr.kind else {
            return Ok(None);
        };
        let HirExprKind::Var(name) = &callee.kind else {
            return Ok(None);
        };
        if name != "ori_to_string" {
            return Ok(None);
        }
        let Some(arg) = args.first() else {
            return Ok(None);
        };
        let value = self.emit_expr(&arg.value)?;
        let value = match &arg.value.ty {
            Ty::Int8 | Ty::Int16 | Ty::Int32 => self.builder.ins().sextend(types::I64, value),
            Ty::U8 | Ty::U16 | Ty::U32 => self.builder.ins().uextend(types::I64, value),
            _ => value,
        };
        self.int_to_string_parts(value).map(Some)
    }

    fn emit_as_string_parts(&mut self, expr: &HirExpr) -> Result<StringParts, String> {
        if let Some(parts) = self.emit_to_string_call_parts(expr)? {
            return Ok(parts);
        }
        if let HirExprKind::InterpolatedStr(parts) = &expr.kind {
            return self.emit_interpolated_string_parts(parts);
        }
        let value = self.emit_expr(expr)?;
        match &expr.ty {
            Ty::String => {
                let len = self.str_len_from_ptr(value)?;
                Ok(StringParts { ptr: value, len })
            }
            Ty::Int | Ty::Int64 | Ty::U64 => self.int_to_string_parts(value),
            Ty::Int8 | Ty::Int16 | Ty::Int32 => {
                let widened = self.builder.ins().sextend(types::I64, value);
                self.int_to_string_parts(widened)
            }
            Ty::U8 | Ty::U16 | Ty::U32 => {
                let widened = self.builder.ins().uextend(types::I64, value);
                self.int_to_string_parts(widened)
            }
            _ => Err(format!(
                "native interpolated strings do not support expression type `{}`",
                expr.ty.display()
            )),
        }
    }

    fn emit_interpolated_string_parts(
        &mut self,
        parts: &[HirStrPart],
    ) -> Result<StringParts, String> {
        let concat_ref = *self
            .func_refs
            .get("ori_string_concat_parts")
            .ok_or_else(|| "missing runtime function `ori_string_concat_parts`".to_string())?;
        let mut current = StringParts {
            ptr: self.string_ptr("")?,
            len: self.builder.ins().iconst(types::I64, 0),
        };
        for part in parts {
            let next = match part {
                HirStrPart::Literal(s) => StringParts {
                    ptr: self.string_ptr(s.as_str())?,
                    len: self.builder.ins().iconst(types::I64, s.len() as i64),
                },
                HirStrPart::Expr(expr) => self.emit_as_string_parts(expr)?,
            };
            let call = self
                .builder
                .ins()
                .call(concat_ref, &[current.ptr, current.len, next.ptr, next.len]);
            let len = self.builder.ins().iadd(current.len, next.len);
            current = StringParts {
                ptr: self.builder.inst_results(call)[0],
                len,
            };
        }
        Ok(current)
    }

    fn emit_interpolated_string(&mut self, parts: &[HirStrPart]) -> Result<ir::Value, String> {
        Ok(self.emit_interpolated_string_parts(parts)?.ptr)
    }

    // == Statements ==

    fn emit_block(&mut self, block: &HirBlock) -> Result<(), String> {
        self.emit_scoped_stmts(&block.stmts)
    }

    fn emit_scoped_stmts(&mut self, stmts: &[HirStmt]) -> Result<(), String> {
        let cleanup_start = self.using_stack.len();
        let managed_cleanup_start = self.managed_stack.len();
        for s in stmts {
            if self.terminated {
                break;
            }
            self.emit_stmt(s)?;
        }
        if !self.terminated {
            self.emit_scope_cleanup_calls_from(cleanup_start, managed_cleanup_start)?;
        }
        self.using_stack.truncate(cleanup_start);
        self.managed_stack.truncate(managed_cleanup_start);
        Ok(())
    }

    fn emit_scope_cleanup_calls_from(
        &mut self,
        using_start: usize,
        managed_start: usize,
    ) -> Result<(), String> {
        self.emit_using_cleanup_calls_from(using_start)?;
        self.emit_managed_cleanup_calls_from(managed_start)?;
        if managed_start == 0 {
            self.emit_arc_collect_cycles()?;
        }
        Ok(())
    }

    fn emit_using_cleanup_calls_from(&mut self, start: usize) -> Result<(), String> {
        let cleanups: Vec<UsingCleanup> = self.using_stack[start..].to_vec();
        for cleanup in cleanups.iter().rev() {
            self.emit_dispose_call(cleanup)?;
        }
        Ok(())
    }

    fn emit_managed_cleanup_calls_from(&mut self, start: usize) -> Result<(), String> {
        let cleanups: Vec<ManagedCleanup> = self.managed_stack[start..].to_vec();
        for cleanup in cleanups.iter().rev() {
            let value = self.builder.use_var(cleanup.var);
            self.emit_arc_release_if_managed(&cleanup.ty, value)?;
        }
        Ok(())
    }

    fn emit_dispose_call(&mut self, cleanup: &UsingCleanup) -> Result<(), String> {
        let Some(func_name) = self.dispose_func_name_for_ty(&cleanup.ty) else {
            return Ok(());
        };
        let Some(&dispose_ref) = self.func_refs.get(func_name.as_str()) else {
            return Ok(());
        };
        let value = self.builder.use_var(cleanup.var);
        self.builder.ins().call(dispose_ref, &[value]);
        Ok(())
    }

    fn dispose_func_name_for_ty(&self, ty: &Ty) -> Option<SmolStr> {
        match ty {
            Ty::Named(def_id, _) => self
                .type_names
                .get(def_id)
                .map(|name| SmolStr::new(format!("{name}.dispose"))),
            _ => None,
        }
    }

    fn emit_arc_retain_if_managed(&mut self, ty: &Ty, value: ir::Value) -> Result<(), String> {
        if !is_managed_ty(ty) {
            return Ok(());
        }
        let Some(&retain_ref) = self.func_refs.get("ori_arc_retain") else {
            return Ok(());
        };
        self.builder.ins().call(retain_ref, &[value]);
        Ok(())
    }

    fn emit_arc_release_if_managed(&mut self, ty: &Ty, value: ir::Value) -> Result<(), String> {
        if !is_managed_ty(ty) {
            return Ok(());
        }
        let Some(&release_ref) = self.func_refs.get("ori_arc_release") else {
            return Ok(());
        };
        self.builder.ins().call(release_ref, &[value]);
        Ok(())
    }

    fn emit_arc_collect_cycles(&mut self) -> Result<(), String> {
        if let Some(&collect_ref) = self.func_refs.get("ori_arc_collect_cycles") {
            self.builder.ins().call(collect_ref, &[]);
        }
        Ok(())
    }

    fn current_loop(&self) -> Option<LoopContext> {
        self.loop_stack.last().copied()
    }

    fn push_loop(&mut self, continue_target: ir::Block, break_target: ir::Block) {
        self.loop_stack.push(LoopContext {
            continue_target,
            break_target,
            cleanup_start: self.using_stack.len(),
            managed_cleanup_start: self.managed_stack.len(),
        });
    }

    fn pop_loop(&mut self) {
        self.loop_stack.pop();
    }

    fn emit_return(&mut self, val: Option<&HirExpr>) -> Result<(), String> {
        let return_ty = self.current_return_ty.clone();
        let return_value = val
            .map(|e| self.emit_expr_for_expected(e, &return_ty))
            .transpose()?;
        if let Some(value) = return_value {
            self.emit_arc_retain_if_managed(&return_ty, value)?;
        }
        self.emit_scope_cleanup_calls_from(0, 0)?;
        if let Some(v) = return_value {
            self.builder.ins().return_(&[v]);
        } else {
            self.builder.ins().return_(&[]);
        }
        self.terminated = true;
        Ok(())
    }

    fn emit_stmt(&mut self, stmt: &HirStmt) -> Result<(), String> {
        match stmt {
            HirStmt::Let {
                name, ty, value, ..
            } => {
                let val = self.emit_expr_for_expected(value, ty)?;
                if let Some(cl_ty) = cl_type(ty, self.ptr_ty) {
                    let var = self.builder.declare_var(cl_ty);
                    self.builder.def_var(var, val);
                    self.vars.insert(name.clone(), (var, ty.clone()));
                    if is_managed_ty(ty) {
                        self.emit_arc_retain_if_managed(ty, val)?;
                        self.managed_stack.push(ManagedCleanup {
                            var,
                            ty: ty.clone(),
                        });
                    }
                }
            }
            HirStmt::Assign { lvalue, value, .. } => {
                if let HirLValue::Var(name) = lvalue {
                    if let Some((var, ty)) = self.vars.get(name).cloned() {
                        let val = self.emit_expr_for_expected(value, &ty)?;
                        let old = self.builder.use_var(var);
                        self.emit_arc_retain_if_managed(&ty, val)?;
                        self.emit_arc_release_if_managed(&ty, old)?;
                        self.builder.def_var(var, val);
                    } else {
                        let val = if let Some(info) = self.global_data.get(name).cloned() {
                            let val = self.emit_expr_for_expected(value, &info.ty)?;
                            if info.mutable && is_managed_ty(&info.ty) {
                                if let Some(old) = self.load_global(name) {
                                    self.emit_arc_retain_if_managed(&info.ty, val)?;
                                    self.emit_arc_release_if_managed(&info.ty, old)?;
                                }
                            }
                            val
                        } else {
                            self.emit_expr(value)?
                        };
                        self.store_global(name, val);
                    }
                } else if let HirLValue::Index { base, index } = lvalue {
                    let val = self.emit_expr(value)?;
                    let (container, container_ty) = self.emit_lvalue_value(base)?;
                    if let Ty::List(elem_ty) = container_ty {
                        let idx = self.emit_expr(index)?;
                        let stored = self.to_list_storage_value(val, &elem_ty);
                        let set_ref = *self
                            .func_refs
                            .get("ori_list_set")
                            .ok_or_else(|| "missing runtime function `ori_list_set`".to_string())?;
                        self.builder.ins().call(set_ref, &[container, idx, stored]);
                    }
                }
            }
            HirStmt::Return(val, _) => self.emit_return(val.as_ref())?,
            HirStmt::Break(_) => {
                if let Some(ctx) = self.current_loop() {
                    self.emit_scope_cleanup_calls_from(
                        ctx.cleanup_start,
                        ctx.managed_cleanup_start,
                    )?;
                    self.builder.ins().jump(ctx.break_target, &[]);
                    self.terminated = true;
                }
            }
            HirStmt::Continue(_) => {
                if let Some(ctx) = self.current_loop() {
                    self.emit_scope_cleanup_calls_from(
                        ctx.cleanup_start,
                        ctx.managed_cleanup_start,
                    )?;
                    self.builder.ins().jump(ctx.continue_target, &[]);
                    self.terminated = true;
                }
            }
            HirStmt::Expr(e) => {
                self.emit_expr(e)?;
            }

            HirStmt::If {
                cond,
                then,
                else_ifs,
                else_,
                ..
            } => {
                self.emit_if(cond, then, else_ifs, else_.as_ref())?;
            }
            HirStmt::While { cond, body, .. } => self.emit_while(cond, body)?,
            HirStmt::Loop { body, .. } => self.emit_loop(body)?,
            HirStmt::For {
                binding,
                index_binding,
                elem_ty,
                iterable,
                body,
                ..
            } => {
                self.emit_for(binding, index_binding.as_ref(), elem_ty, iterable, body)?;
            }
            HirStmt::Match {
                scrutinee, arms, ..
            } => self.emit_match(scrutinee, arms)?,
            HirStmt::IfSome {
                binding,
                inner_ty,
                value,
                then,
                else_,
                ..
            } => {
                self.emit_if_some(binding, inner_ty, value, then, else_.as_ref())?;
            }
            HirStmt::WhileSome {
                binding,
                inner_ty,
                value,
                body,
                ..
            } => {
                self.emit_while_some(binding, inner_ty, value, body)?;
            }
            HirStmt::Using {
                name, ty, value, ..
            } => {
                let val = self.emit_expr_for_expected(value, ty)?;
                if let Some(cl_ty) = cl_type(ty, self.ptr_ty) {
                    let var = self.builder.declare_var(cl_ty);
                    self.builder.def_var(var, val);
                    self.vars.insert(name.clone(), (var, ty.clone()));
                    if is_managed_ty(ty) {
                        self.emit_arc_retain_if_managed(ty, val)?;
                        self.managed_stack.push(ManagedCleanup {
                            var,
                            ty: ty.clone(),
                        });
                    }
                    self.using_stack.push(UsingCleanup {
                        var,
                        ty: ty.clone(),
                    });
                }
            }
            HirStmt::Check { condition, .. } => {
                let cv = self.emit_expr(condition)?;
                self.emit_trap_unless(cv, ir::TrapCode::user(1).unwrap(), true)?;
            }
            HirStmt::Repeat { count, body, .. } => {
                // Desugar: for (int64_t _i = 0; _i < count; _i++) { body }
                let count_v = self.emit_expr(count)?;
                let idx_var = self.builder.declare_var(types::I64);
                let zero = self.builder.ins().iconst(types::I64, 0);
                self.builder.def_var(idx_var, zero);

                let header = self.builder.create_block();
                let body_b = self.builder.create_block();
                let exit = self.builder.create_block();

                self.builder.ins().jump(header, &[]);
                self.builder.switch_to_block(header);

                let cur = self.builder.use_var(idx_var);
                let cond =
                    self.builder
                        .ins()
                        .icmp(ir::condcodes::IntCC::SignedLessThan, cur, count_v);
                self.builder.ins().brif(cond, body_b, &[], exit, &[]);

                self.builder.seal_block(body_b);
                self.builder.switch_to_block(body_b);
                self.terminated = false;
                self.push_loop(header, exit);
                self.emit_block(body)?;
                self.pop_loop();
                if !self.terminated {
                    let cur2 = self.builder.use_var(idx_var);
                    let one = self.builder.ins().iconst(types::I64, 1);
                    let next = self.builder.ins().iadd(cur2, one);
                    self.builder.def_var(idx_var, next);
                    self.builder.ins().jump(header, &[]);
                }
                self.terminated = false;
                self.builder.seal_block(header);
                self.builder.seal_block(exit);
                self.builder.switch_to_block(exit);
                self.terminated = false;
            }
        }
        Ok(())
    }

    // == Control flow ==

    fn emit_if(
        &mut self,
        cond: &HirExpr,
        then: &HirBlock,
        else_ifs: &[(HirExpr, HirBlock)],
        else_: Option<&HirBlock>,
    ) -> Result<(), String> {
        let then_block = self.builder.create_block();
        let merge_block = self.builder.create_block();

        let else_target = if else_ifs.is_empty() && else_.is_none() {
            merge_block
        } else {
            self.builder.create_block()
        };

        let cv = self.emit_expr(cond)?;
        self.builder
            .ins()
            .brif(cv, then_block, &[], else_target, &[]);

        // then branch
        self.builder.seal_block(then_block);
        self.builder.switch_to_block(then_block);
        self.terminated = false;
        self.emit_block(then)?;
        if !self.terminated {
            self.builder.ins().jump(merge_block, &[]);
        }

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
            if !self.terminated {
                self.builder.ins().jump(merge_block, &[]);
            }
        }

        self.builder.seal_block(merge_block);
        self.builder.switch_to_block(merge_block);
        self.terminated = false;
        Ok(())
    }

    fn emit_while(&mut self, cond: &HirExpr, body: &HirBlock) -> Result<(), String> {
        let cond_blk = self.builder.create_block();
        let body_blk = self.builder.create_block();
        let exit_blk = self.builder.create_block();

        // Fall through to condition check
        if !self.terminated {
            self.builder.ins().jump(cond_blk, &[]);
        }
        self.terminated = false;

        // Condition block — do NOT seal yet (back-edge from body not known yet)
        self.builder.switch_to_block(cond_blk);
        let cv = self.emit_expr(cond)?;
        self.builder.ins().brif(cv, body_blk, &[], exit_blk, &[]);

        // Body block — only cond_blk jumps here, safe to seal immediately
        self.builder.seal_block(body_blk);
        self.builder.switch_to_block(body_blk);
        self.push_loop(cond_blk, exit_blk);
        self.emit_block(body)?;
        self.pop_loop();
        if !self.terminated {
            self.builder.ins().jump(cond_blk, &[]);
        }
        self.terminated = false;

        // Seal cond_blk NOW — both predecessors (entry and back-edge) are known
        self.builder.seal_block(cond_blk);

        // Exit block — only cond_blk branches here
        self.builder.seal_block(exit_blk);
        self.builder.switch_to_block(exit_blk);
        self.terminated = false;
        Ok(())
    }

    fn emit_loop(&mut self, body: &HirBlock) -> Result<(), String> {
        let header = self.builder.create_block();
        let exit = self.builder.create_block();
        self.builder.ins().jump(header, &[]);
        self.builder.switch_to_block(header);
        self.terminated = false;
        self.push_loop(header, exit);
        self.emit_block(body)?;
        self.pop_loop();
        if !self.terminated {
            self.builder.ins().jump(header, &[]);
        }
        self.builder.seal_block(header);
        self.builder.seal_block(exit);
        self.builder.switch_to_block(exit);
        self.terminated = false;
        Ok(())
    }

    fn emit_if_some(
        &mut self,
        binding: &SmolStr,
        inner_ty: &Ty,
        value: &HirExpr,
        then: &HirBlock,
        else_: Option<&HirBlock>,
    ) -> Result<(), String> {
        // 1. Emit the optional expression (returns a pointer to the stack struct)
        let opt_ptr = self.emit_expr(value)?;
        // 2. Read has_value (byte 0)
        let has_val = self
            .builder
            .ins()
            .load(types::I8, MemFlags::new(), opt_ptr, 0);
        let then_blk = self.builder.create_block();
        let merge_blk = self.builder.create_block();
        let else_blk = if else_.is_some() {
            self.builder.create_block()
        } else {
            merge_blk
        };
        self.builder
            .ins()
            .brif(has_val, then_blk, &[], else_blk, &[]);
        // then block: bind inner value
        self.builder.seal_block(then_blk);
        self.builder.switch_to_block(then_blk);
        self.terminated = false;
        if let Some(cl_ty) = cl_type(inner_ty, self.ptr_ty) {
            let (val_off, _) = optional_layout(inner_ty, self.ptr_ty);
            let inner_val =
                self.builder
                    .ins()
                    .load(cl_ty, MemFlags::new(), opt_ptr, val_off as i32);
            let var = self.builder.declare_var(cl_ty);
            self.builder.def_var(var, inner_val);
            self.vars.insert(binding.clone(), (var, inner_ty.clone()));
        }
        self.emit_block(then)?;
        if !self.terminated {
            self.builder.ins().jump(merge_blk, &[]);
        }
        // else block (if any)
        if let Some(eb) = else_ {
            self.builder.seal_block(else_blk);
            self.builder.switch_to_block(else_blk);
            self.terminated = false;
            self.emit_block(eb)?;
            if !self.terminated {
                self.builder.ins().jump(merge_blk, &[]);
            }
        }
        self.builder.seal_block(merge_blk);
        self.builder.switch_to_block(merge_blk);
        self.terminated = false;
        Ok(())
    }

    fn emit_while_some(
        &mut self,
        binding: &SmolStr,
        inner_ty: &Ty,
        value: &HirExpr,
        body: &HirBlock,
    ) -> Result<(), String> {
        let header_blk = self.builder.create_block();
        let body_blk = self.builder.create_block();
        let exit_blk = self.builder.create_block();
        if !self.terminated {
            self.builder.ins().jump(header_blk, &[]);
        }
        self.terminated = false;
        // Header: evaluate optional expression, check has_value
        self.builder.switch_to_block(header_blk);
        let opt_ptr = self.emit_expr(value)?;
        let has_val = self
            .builder
            .ins()
            .load(types::I8, MemFlags::new(), opt_ptr, 0);
        self.builder
            .ins()
            .brif(has_val, body_blk, &[], exit_blk, &[]);
        // Body: extract inner value, run body
        self.builder.seal_block(body_blk);
        self.builder.switch_to_block(body_blk);
        self.terminated = false;
        if let Some(cl_ty) = cl_type(inner_ty, self.ptr_ty) {
            let (val_off, _) = optional_layout(inner_ty, self.ptr_ty);
            let inner_val =
                self.builder
                    .ins()
                    .load(cl_ty, MemFlags::new(), opt_ptr, val_off as i32);
            let var = self.vars.get(binding).map(|(v, _)| *v).unwrap_or_else(|| {
                let v = self.builder.declare_var(cl_ty);
                self.vars.insert(binding.clone(), (v, inner_ty.clone()));
                v
            });
            self.builder.def_var(var, inner_val);
        }
        self.push_loop(header_blk, exit_blk);
        self.emit_block(body)?;
        self.pop_loop();
        if !self.terminated {
            self.builder.ins().jump(header_blk, &[]);
        }
        self.terminated = false;
        // Seal header after back-edge is known
        self.builder.seal_block(header_blk);
        self.builder.seal_block(exit_blk);
        self.builder.switch_to_block(exit_blk);
        self.terminated = false;
        Ok(())
    }

    fn emit_for(
        &mut self,
        binding: &SmolStr,
        index_binding: Option<&SmolStr>,
        elem_ty: &Ty,
        iterable: &HirExpr,
        body: &HirBlock,
    ) -> Result<(), String> {
        match &iterable.kind {
            HirExprKind::Range { start, end } => {
                self.emit_for_range(binding, index_binding, elem_ty, start, end, body)
            }
            _ if matches!(&iterable.ty, Ty::List(_)) => {
                self.emit_for_list(binding, index_binding, elem_ty, iterable, body)
            }
            _ if matches!(&iterable.ty, Ty::Set(_)) => {
                // Set is backed by OriList internally — same get/len interface
                self.emit_for_list(binding, index_binding, elem_ty, iterable, body)
            }
            _ if matches!(&iterable.ty, Ty::Map(_, _)) => {
                let Ty::Map(key_ty, value_ty) = &iterable.ty else {
                    unreachable!();
                };
                self.emit_for_map(binding, index_binding, key_ty, value_ty, iterable, body)
            }
            _ if matches!(&iterable.ty, Ty::String) => {
                self.emit_for_string(binding, index_binding, iterable, body)
            }
            _ => Err(format!(
                "unsupported `for` iterable type `{}`",
                iterable.ty.display()
            )),
        }
    }

    fn emit_for_range(
        &mut self,
        binding: &SmolStr,
        index_binding: Option<&SmolStr>,
        elem_ty: &Ty,
        start: &HirExpr,
        end: &HirExpr,
        body: &HirBlock,
    ) -> Result<(), String> {
        let start_v = self.emit_expr(start)?;
        let end_v = self.emit_expr(end)?;
        let idx_var = self.builder.declare_var(types::I64);
        self.builder.def_var(idx_var, start_v);
        let end_var = self.builder.declare_var(types::I64);
        self.builder.def_var(end_var, end_v);
        let asc_var = self.builder.declare_var(types::I8);
        let asc =
            self.builder
                .ins()
                .icmp(ir::condcodes::IntCC::SignedLessThanOrEqual, start_v, end_v);
        self.builder.def_var(asc_var, asc);
        if let Some(cl_ty) = cl_type(elem_ty, self.ptr_ty) {
            let bvar = self.builder.declare_var(cl_ty);
            self.vars.insert(binding.clone(), (bvar, elem_ty.clone()));
        }
        // Declare index counter for second binding (iteration count, not range value)
        let iter_count_var = if index_binding.is_some() {
            let v = self.builder.declare_var(types::I64);
            let zero = self.builder.ins().iconst(types::I64, 0);
            self.builder.def_var(v, zero);
            Some(v)
        } else {
            None
        };
        let header = self.builder.create_block();
        let body_b = self.builder.create_block();
        let step = self.builder.create_block();
        let exit = self.builder.create_block();
        self.builder.ins().jump(header, &[]);
        self.builder.switch_to_block(header);
        let cur = self.builder.use_var(idx_var);
        let lim = self.builder.use_var(end_var);
        let asc_flag = self.builder.use_var(asc_var);
        let cond_asc =
            self.builder
                .ins()
                .icmp(ir::condcodes::IntCC::SignedLessThanOrEqual, cur, lim);
        let cond_desc =
            self.builder
                .ins()
                .icmp(ir::condcodes::IntCC::SignedGreaterThanOrEqual, cur, lim);
        let cond = self.builder.ins().select(asc_flag, cond_asc, cond_desc);
        self.builder.ins().brif(cond, body_b, &[], exit, &[]);
        self.builder.seal_block(body_b);
        self.builder.switch_to_block(body_b);
        // Update binding variable
        if let Some((bvar, _)) = self.vars.get(binding) {
            let bvar = *bvar;
            let cur2 = self.builder.use_var(idx_var);
            self.builder.def_var(bvar, cur2);
        }
        // Update index binding
        if let (Some(ib_name), Some(ic_var)) = (index_binding, iter_count_var) {
            let ic = self.builder.use_var(ic_var);
            let ib_var = self.builder.declare_var(types::I64);
            self.builder.def_var(ib_var, ic);
            self.vars.insert(ib_name.clone(), (ib_var, Ty::Int));
        }
        self.terminated = false;
        self.push_loop(step, exit);
        self.emit_block(body)?;
        self.pop_loop();
        if !self.terminated {
            self.builder.ins().jump(step, &[]);
        }
        self.terminated = false;
        self.builder.seal_block(step);
        self.builder.switch_to_block(step);
        let cur2 = self.builder.use_var(idx_var);
        let asc_flag = self.builder.use_var(asc_var);
        let one = self.builder.ins().iconst(types::I64, 1);
        let neg_one = self.builder.ins().iconst(types::I64, -1);
        let inc = self.builder.ins().select(asc_flag, one, neg_one);
        let next = self.builder.ins().iadd(cur2, inc);
        self.builder.def_var(idx_var, next);
        // Increment iteration counter for index binding
        if let Some(ic_var) = iter_count_var {
            let ic = self.builder.use_var(ic_var);
            let one_ic = self.builder.ins().iconst(types::I64, 1);
            let next_ic = self.builder.ins().iadd(ic, one_ic);
            self.builder.def_var(ic_var, next_ic);
        }
        self.builder.ins().jump(header, &[]);
        self.builder.seal_block(header);
        self.builder.seal_block(exit);
        self.builder.switch_to_block(exit);
        self.terminated = false;
        Ok(())
    }

    fn emit_for_list(
        &mut self,
        binding: &SmolStr,
        index_binding: Option<&SmolStr>,
        elem_ty: &Ty,
        iterable: &HirExpr,
        body: &HirBlock,
    ) -> Result<(), String> {
        let list_v = self.emit_expr(iterable)?;
        let len_ref = *self
            .func_refs
            .get("ori_list_len")
            .ok_or_else(|| "missing runtime function `ori_list_len`".to_string())?;
        let get_ref = *self
            .func_refs
            .get("ori_list_get")
            .ok_or_else(|| "missing runtime function `ori_list_get`".to_string())?;
        let len_call = self.builder.ins().call(len_ref, &[list_v]);
        let len_v = self.builder.inst_results(len_call)[0];
        let idx_var = self.builder.declare_var(types::I64);
        let zero = self.builder.ins().iconst(types::I64, 0);
        self.builder.def_var(idx_var, zero);
        let len_var = self.builder.declare_var(types::I64);
        self.builder.def_var(len_var, len_v);
        if let Some(cl_ty) = cl_type(elem_ty, self.ptr_ty) {
            let bvar = self.builder.declare_var(cl_ty);
            self.vars.insert(binding.clone(), (bvar, elem_ty.clone()));
        }
        let header = self.builder.create_block();
        let body_b = self.builder.create_block();
        let step = self.builder.create_block();
        let exit = self.builder.create_block();
        self.builder.ins().jump(header, &[]);
        self.builder.switch_to_block(header);
        let cur = self.builder.use_var(idx_var);
        let len = self.builder.use_var(len_var);
        let cond = self
            .builder
            .ins()
            .icmp(ir::condcodes::IntCC::SignedLessThan, cur, len);
        self.builder.ins().brif(cond, body_b, &[], exit, &[]);
        self.builder.seal_block(body_b);
        self.builder.switch_to_block(body_b);
        if let Some((bvar, _)) = self.vars.get(binding) {
            let bvar = *bvar;
            let cur2 = self.builder.use_var(idx_var);
            let call = self.builder.ins().call(get_ref, &[list_v, cur2]);
            let elem = self.builder.inst_results(call)[0];
            let elem = self.from_list_storage_value(elem, elem_ty);
            self.builder.def_var(bvar, elem);
        }
        // Bind the index variable (0-based counter)
        if let Some(ib_name) = index_binding {
            let cur2 = self.builder.use_var(idx_var);
            let ib_var = self.builder.declare_var(types::I64);
            self.builder.def_var(ib_var, cur2);
            self.vars.insert(ib_name.clone(), (ib_var, Ty::Int));
        }
        self.terminated = false;
        self.push_loop(step, exit);
        self.emit_block(body)?;
        self.pop_loop();
        if !self.terminated {
            self.builder.ins().jump(step, &[]);
        }
        self.terminated = false;
        self.builder.seal_block(step);
        self.builder.switch_to_block(step);
        let cur2 = self.builder.use_var(idx_var);
        let one = self.builder.ins().iconst(types::I64, 1);
        let next = self.builder.ins().iadd(cur2, one);
        self.builder.def_var(idx_var, next);
        self.builder.ins().jump(header, &[]);
        self.builder.seal_block(header);
        self.builder.seal_block(exit);
        self.builder.switch_to_block(exit);
        self.terminated = false;
        Ok(())
    }

    fn emit_for_map(
        &mut self,
        binding: &SmolStr,
        value_binding: Option<&SmolStr>,
        key_ty: &Ty,
        value_ty: &Ty,
        iterable: &HirExpr,
        body: &HirBlock,
    ) -> Result<(), String> {
        let map_v = self.emit_expr(iterable)?;
        let len_ref = *self
            .func_refs
            .get("ori_map_len")
            .ok_or_else(|| "missing runtime function `ori_map_len`".to_string())?;
        let key_at_ref = *self
            .func_refs
            .get("ori_map_key_at")
            .ok_or_else(|| "missing runtime function `ori_map_key_at`".to_string())?;
        let value_at_ref = *self
            .func_refs
            .get("ori_map_value_at")
            .ok_or_else(|| "missing runtime function `ori_map_value_at`".to_string())?;
        let len_call = self.builder.ins().call(len_ref, &[map_v]);
        let len_v = self.builder.inst_results(len_call)[0];
        let idx_var = self.builder.declare_var(types::I64);
        let zero = self.builder.ins().iconst(types::I64, 0);
        self.builder.def_var(idx_var, zero);
        let len_var = self.builder.declare_var(types::I64);
        self.builder.def_var(len_var, len_v);

        if let Some(cl_ty) = cl_type(key_ty, self.ptr_ty) {
            let key_var = self.builder.declare_var(cl_ty);
            self.vars.insert(binding.clone(), (key_var, key_ty.clone()));
        }
        if let Some(value_name) = value_binding {
            if let Some(cl_ty) = cl_type(value_ty, self.ptr_ty) {
                let value_var = self.builder.declare_var(cl_ty);
                self.vars
                    .insert(value_name.clone(), (value_var, value_ty.clone()));
            }
        }

        let header = self.builder.create_block();
        let body_b = self.builder.create_block();
        let step = self.builder.create_block();
        let exit = self.builder.create_block();
        self.builder.ins().jump(header, &[]);
        self.builder.switch_to_block(header);
        let cur = self.builder.use_var(idx_var);
        let len = self.builder.use_var(len_var);
        let cond = self
            .builder
            .ins()
            .icmp(ir::condcodes::IntCC::SignedLessThan, cur, len);
        self.builder.ins().brif(cond, body_b, &[], exit, &[]);
        self.builder.seal_block(body_b);
        self.builder.switch_to_block(body_b);

        let cur2 = self.builder.use_var(idx_var);
        if let Some((key_var, _)) = self.vars.get(binding) {
            let key_var = *key_var;
            let key_call = self.builder.ins().call(key_at_ref, &[map_v, cur2]);
            let key = self.builder.inst_results(key_call)[0];
            let key = self.from_list_storage_value(key, key_ty);
            self.builder.def_var(key_var, key);
        }
        if let Some(value_name) = value_binding {
            if let Some((value_var, _)) = self.vars.get(value_name) {
                let value_var = *value_var;
                let value_call = self.builder.ins().call(value_at_ref, &[map_v, cur2]);
                let value = self.builder.inst_results(value_call)[0];
                let value = self.from_list_storage_value(value, value_ty);
                self.builder.def_var(value_var, value);
            }
        }

        self.terminated = false;
        self.push_loop(step, exit);
        self.emit_block(body)?;
        self.pop_loop();
        if !self.terminated {
            self.builder.ins().jump(step, &[]);
        }
        self.terminated = false;
        self.builder.seal_block(step);
        self.builder.switch_to_block(step);
        let cur3 = self.builder.use_var(idx_var);
        let one = self.builder.ins().iconst(types::I64, 1);
        let next = self.builder.ins().iadd(cur3, one);
        self.builder.def_var(idx_var, next);
        self.builder.ins().jump(header, &[]);
        self.builder.seal_block(header);
        self.builder.seal_block(exit);
        self.builder.switch_to_block(exit);
        self.terminated = false;
        Ok(())
    }

    fn emit_for_string(
        &mut self,
        binding: &SmolStr,
        index_binding: Option<&SmolStr>,
        iterable: &HirExpr,
        body: &HirBlock,
    ) -> Result<(), String> {
        // Convert string to list of characters via ori_string_chars, then iterate
        let str_v = self.emit_expr(iterable)?;
        let chars_ref = *self
            .func_refs
            .get("ori_string_chars")
            .ok_or_else(|| "missing runtime function `ori_string_chars`".to_string())?;
        let chars_call = self.builder.ins().call(chars_ref, &[str_v]);
        let list_v = self.builder.inst_results(chars_call)[0];

        let len_ref = *self
            .func_refs
            .get("ori_list_len")
            .ok_or_else(|| "missing runtime function `ori_list_len`".to_string())?;
        let get_ref = *self
            .func_refs
            .get("ori_list_get")
            .ok_or_else(|| "missing runtime function `ori_list_get`".to_string())?;
        let len_call = self.builder.ins().call(len_ref, &[list_v]);
        let len_v = self.builder.inst_results(len_call)[0];
        let idx_var = self.builder.declare_var(types::I64);
        let zero = self.builder.ins().iconst(types::I64, 0);
        self.builder.def_var(idx_var, zero);
        let len_var = self.builder.declare_var(types::I64);
        self.builder.def_var(len_var, len_v);
        // Bind as string (ptr type)
        let bvar = self.builder.declare_var(self.ptr_ty);
        self.vars.insert(binding.clone(), (bvar, Ty::String));

        let header = self.builder.create_block();
        let body_b = self.builder.create_block();
        let step = self.builder.create_block();
        let exit = self.builder.create_block();
        self.builder.ins().jump(header, &[]);
        self.builder.switch_to_block(header);
        let cur = self.builder.use_var(idx_var);
        let len = self.builder.use_var(len_var);
        let cond = self
            .builder
            .ins()
            .icmp(ir::condcodes::IntCC::SignedLessThan, cur, len);
        self.builder.ins().brif(cond, body_b, &[], exit, &[]);
        self.builder.seal_block(body_b);
        self.builder.switch_to_block(body_b);
        // Each element from ori_string_chars is a ptr (string)
        let cur2 = self.builder.use_var(idx_var);
        let call = self.builder.ins().call(get_ref, &[list_v, cur2]);
        let elem = self.builder.inst_results(call)[0];
        self.builder.def_var(bvar, elem);
        // Bind the index variable (0-based counter)
        if let Some(ib_name) = index_binding {
            let cur3 = self.builder.use_var(idx_var);
            let ib_var = self.builder.declare_var(types::I64);
            self.builder.def_var(ib_var, cur3);
            self.vars.insert(ib_name.clone(), (ib_var, Ty::Int));
        }
        self.terminated = false;
        self.push_loop(step, exit);
        self.emit_block(body)?;
        self.pop_loop();
        if !self.terminated {
            self.builder.ins().jump(step, &[]);
        }
        self.terminated = false;
        self.builder.seal_block(step);
        self.builder.switch_to_block(step);
        let cur3 = self.builder.use_var(idx_var);
        let one = self.builder.ins().iconst(types::I64, 1);
        let next = self.builder.ins().iadd(cur3, one);
        self.builder.def_var(idx_var, next);
        self.builder.ins().jump(header, &[]);
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
            let arm_blk = self.builder.create_block();
            let next_blk = self.builder.create_block();
            let cond = self.pattern_cond(&arm.pattern, scr, &scrutinee.ty);
            self.builder.ins().brif(cond, arm_blk, &[], next_blk, &[]);
            self.builder.seal_block(arm_blk);
            self.builder.switch_to_block(arm_blk);
            self.terminated = false;
            self.bind_pattern(&arm.pattern, scr, &scrutinee.ty);
            self.emit_scoped_stmts(&arm.body)?;
            if !self.terminated {
                self.builder.ins().jump(exit, &[]);
            }
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
            HirPattern::Wildcard | HirPattern::Binding(_, _) => {
                self.builder.ins().iconst(types::I8, 1)
            }
            HirPattern::BoolLit(b) => {
                let lit = self.builder.ins().iconst(types::I8, if *b { 1 } else { 0 });
                self.builder
                    .ins()
                    .icmp(ir::condcodes::IntCC::Equal, scr, lit)
            }
            HirPattern::IntLit(n) => {
                let cl = cl_type(scr_ty, self.ptr_ty).unwrap_or(types::I64);
                let lit = self.builder.ins().iconst(cl, *n);
                self.builder
                    .ins()
                    .icmp(ir::condcodes::IntCC::Equal, scr, lit)
            }
            HirPattern::StrLit(s) => {
                let Ok(rhs) = self.string_ptr(s.as_str()) else {
                    return self.builder.ins().iconst(types::I8, 0);
                };
                if let Some(&strcmp_ref) = self.func_refs.get("strcmp") {
                    let call = self.builder.ins().call(strcmp_ref, &[scr, rhs]);
                    let cmp = self.builder.inst_results(call)[0];
                    let zero = self.builder.ins().iconst(types::I32, 0);
                    self.builder
                        .ins()
                        .icmp(ir::condcodes::IntCC::Equal, cmp, zero)
                } else {
                    self.builder.ins().iconst(types::I8, 0)
                }
            }
            HirPattern::Variant {
                def_id,
                variant,
                fields,
            } => {
                if let Some(layout) = self.enum_layouts.get(def_id).cloned() {
                    if let Some(v_layout) = layout.variant(variant) {
                        let tag_val = self.builder.ins().load(types::I32, MemFlags::new(), scr, 0);
                        let expected = self.builder.ins().iconst(types::I32, v_layout.tag as i64);
                        let mut cond =
                            self.builder
                                .ins()
                                .icmp(ir::condcodes::IntCC::Equal, tag_val, expected);

                        for (fname, fpat) in fields {
                            if let Some(fi) = v_layout.fields.field(fname) {
                                let total_off = (layout.payload_offset + fi.offset) as i32;
                                let cl = cl_type(&fi.ty, self.ptr_ty).unwrap_or(types::I64);
                                let fval =
                                    self.builder.ins().load(cl, MemFlags::new(), scr, total_off);
                                let fcond = self.pattern_cond(fpat, fval, &fi.ty);
                                cond = self.builder.ins().band(cond, fcond);
                            }
                        }
                        return cond;
                    }
                }
                self.builder.ins().iconst(types::I8, 0)
            }
            HirPattern::None_ => {
                let has_val = self.builder.ins().load(types::I8, MemFlags::new(), scr, 0);
                let zero = self.builder.ins().iconst(types::I8, 0);
                self.builder
                    .ins()
                    .icmp(ir::condcodes::IntCC::Equal, has_val, zero)
            }
            HirPattern::Some_(inner) => {
                let has_val = self.builder.ins().load(types::I8, MemFlags::new(), scr, 0);
                let inner_ty = if let Ty::Optional(t) = scr_ty {
                    &**t
                } else {
                    &Ty::Void
                };
                let cl = cl_type(inner_ty, self.ptr_ty).unwrap_or(types::I64);
                let (val_off, _) = optional_layout(inner_ty, self.ptr_ty);
                let val = self
                    .builder
                    .ins()
                    .load(cl, MemFlags::new(), scr, val_off as i32);
                let inner_cond = self.pattern_cond(inner, val, inner_ty);
                self.builder.ins().band(has_val, inner_cond)
            }
            HirPattern::Ok_(inner) => {
                let is_ok = self.builder.ins().load(types::I8, MemFlags::new(), scr, 0);
                let inner_ty = if let Ty::Result(ok, _) = scr_ty {
                    &**ok
                } else {
                    &Ty::Void
                };
                let cl = cl_type(inner_ty, self.ptr_ty).unwrap_or(types::I64);
                let (err_ty, pay_off) = if let Ty::Result(ok, err) = scr_ty {
                    (&**err, result_layout(ok, err, self.ptr_ty).0)
                } else {
                    (&Ty::Void, 1)
                };
                let _ = err_ty;
                let val = self
                    .builder
                    .ins()
                    .load(cl, MemFlags::new(), scr, pay_off as i32);
                let inner_cond = self.pattern_cond(inner, val, inner_ty);
                self.builder.ins().band(is_ok, inner_cond)
            }
            HirPattern::Err_(inner) => {
                let is_ok = self.builder.ins().load(types::I8, MemFlags::new(), scr, 0);
                let zero = self.builder.ins().iconst(types::I8, 0);
                let is_err = self
                    .builder
                    .ins()
                    .icmp(ir::condcodes::IntCC::Equal, is_ok, zero);
                let inner_ty = if let Ty::Result(_, err) = scr_ty {
                    &**err
                } else {
                    &Ty::Void
                };
                let cl = cl_type(inner_ty, self.ptr_ty).unwrap_or(types::I64);
                let pay_off = if let Ty::Result(ok, err) = scr_ty {
                    result_layout(ok, err, self.ptr_ty).0
                } else {
                    1
                };
                let val = self
                    .builder
                    .ins()
                    .load(cl, MemFlags::new(), scr, pay_off as i32);
                let inner_cond = self.pattern_cond(inner, val, inner_ty);
                self.builder.ins().band(is_err, inner_cond)
            }
            HirPattern::Tuple(patterns) => {
                if let Ty::Tuple(elems) = scr_ty {
                    let (layout, _, _) = tuple_layout(elems, self.ptr_ty);
                    let mut cond = self.builder.ins().iconst(types::I8, 1);
                    for (pat, (offset, ty)) in patterns.iter().zip(layout.iter()) {
                        let cl = cl_type(ty, self.ptr_ty).unwrap_or(types::I64);
                        let val = self
                            .builder
                            .ins()
                            .load(cl, MemFlags::new(), scr, *offset as i32);
                        let next = self.pattern_cond(pat, val, ty);
                        cond = self.builder.ins().band(cond, next);
                    }
                    cond
                } else {
                    self.builder.ins().iconst(types::I8, 0)
                }
            }
        }
    }

    fn bind_pattern(&mut self, pat: &HirPattern, val: ir::Value, ty: &Ty) {
        match pat {
            HirPattern::Binding(name, bind_ty) => {
                let bty = if *bind_ty == Ty::Infer(0) {
                    ty
                } else {
                    bind_ty
                };
                if let Some(cl_ty) = cl_type(bty, self.ptr_ty) {
                    let var = self.builder.declare_var(cl_ty);
                    self.builder.def_var(var, val);
                    self.vars.insert(name.clone(), (var, bty.clone()));
                }
            }
            HirPattern::Variant {
                def_id,
                variant,
                fields,
            } => {
                if let Some(layout) = self.enum_layouts.get(def_id).cloned() {
                    if let Some(v_layout) = layout.variant(variant) {
                        for (fname, fpat) in fields {
                            if let Some(fi) = v_layout.fields.field(fname) {
                                let total_off = (layout.payload_offset + fi.offset) as i32;
                                let cl = cl_type(&fi.ty, self.ptr_ty).unwrap_or(types::I64);
                                let fval =
                                    self.builder.ins().load(cl, MemFlags::new(), val, total_off);
                                self.bind_pattern(fpat, fval, &fi.ty);
                            }
                        }
                    }
                }
            }
            HirPattern::Some_(inner) => {
                let inner_ty = if let Ty::Optional(t) = ty {
                    &**t
                } else {
                    &Ty::Void
                };
                let cl = cl_type(inner_ty, self.ptr_ty).unwrap_or(types::I64);
                let (val_off, _) = optional_layout(inner_ty, self.ptr_ty);
                let fval = self
                    .builder
                    .ins()
                    .load(cl, MemFlags::new(), val, val_off as i32);
                self.bind_pattern(inner, fval, inner_ty);
            }
            HirPattern::Ok_(inner) => {
                let inner_ty = if let Ty::Result(ok, _) = ty {
                    &**ok
                } else {
                    &Ty::Void
                };
                let cl = cl_type(inner_ty, self.ptr_ty).unwrap_or(types::I64);
                let pay_off = if let Ty::Result(ok, err) = ty {
                    result_layout(ok, err, self.ptr_ty).0
                } else {
                    1
                };
                let fval = self
                    .builder
                    .ins()
                    .load(cl, MemFlags::new(), val, pay_off as i32);
                self.bind_pattern(inner, fval, inner_ty);
            }
            HirPattern::Err_(inner) => {
                let inner_ty = if let Ty::Result(_, err) = ty {
                    &**err
                } else {
                    &Ty::Void
                };
                let cl = cl_type(inner_ty, self.ptr_ty).unwrap_or(types::I64);
                let pay_off = if let Ty::Result(ok, err) = ty {
                    result_layout(ok, err, self.ptr_ty).0
                } else {
                    1
                };
                let fval = self
                    .builder
                    .ins()
                    .load(cl, MemFlags::new(), val, pay_off as i32);
                self.bind_pattern(inner, fval, inner_ty);
            }
            HirPattern::Tuple(patterns) => {
                if let Ty::Tuple(elems) = ty {
                    let (layout, _, _) = tuple_layout(elems, self.ptr_ty);
                    for (pat, (offset, elem_ty)) in patterns.iter().zip(layout.iter()) {
                        let cl = cl_type(elem_ty, self.ptr_ty).unwrap_or(types::I64);
                        let fval =
                            self.builder
                                .ins()
                                .load(cl, MemFlags::new(), val, *offset as i32);
                        self.bind_pattern(pat, fval, elem_ty);
                    }
                }
            }
            _ => {}
        }
    }

    // == Expressions ==

    fn emit_expr(&mut self, expr: &HirExpr) -> Result<ir::Value, String> {
        Ok(match &expr.kind {
            HirExprKind::BoolLit(b) => self.builder.ins().iconst(types::I8, if *b { 1 } else { 0 }),
            HirExprKind::IntLit(n) => {
                let cl = cl_type(&expr.ty, self.ptr_ty).unwrap_or(types::I64);
                self.builder.ins().iconst(cl, *n)
            }
            HirExprKind::FloatLit(f) => match &expr.ty {
                Ty::Float32 => self.builder.ins().f32const(*f as f32),
                _ => self.builder.ins().f64const(*f),
            },
            HirExprKind::Unit => self.builder.ins().iconst(self.ptr_ty, 0),
            HirExprKind::None_ => {
                let inner_ty = if let Ty::Optional(t) = &expr.ty {
                    &**t
                } else {
                    &Ty::Void
                };
                let (_, total) = optional_layout(inner_ty, self.ptr_ty);
                let base = self.malloc_bytes(total)?;
                let zero8 = self.builder.ins().iconst(types::I8, 0);
                self.builder.ins().store(MemFlags::new(), zero8, base, 0);
                base
            }
            HirExprKind::Some_(inner) => {
                let val = self.emit_expr(inner)?;
                let inner_ty = if let Ty::Optional(t) = &expr.ty {
                    &**t
                } else {
                    &inner.ty
                };
                let (val_off, total) = optional_layout(inner_ty, self.ptr_ty);
                let base = self.malloc_bytes(total)?;
                let one8 = self.builder.ins().iconst(types::I8, 1);
                self.builder.ins().store(MemFlags::new(), one8, base, 0);
                self.builder
                    .ins()
                    .store(MemFlags::new(), val, base, val_off as i32);
                base
            }
            HirExprKind::Ok_(inner) => {
                let val = self.emit_expr(inner)?;
                let (ok_ty, err_ty) = if let Ty::Result(o, e) = &expr.ty {
                    (&**o, &**e)
                } else {
                    (&inner.ty, &Ty::Void)
                };
                let (pay_off, _, total) = result_layout(ok_ty, err_ty, self.ptr_ty);
                let base = self.malloc_bytes(total)?;
                let one8 = self.builder.ins().iconst(types::I8, 1);
                self.builder.ins().store(MemFlags::new(), one8, base, 0);
                self.builder
                    .ins()
                    .store(MemFlags::new(), val, base, pay_off as i32);
                base
            }
            HirExprKind::Err_(inner) => {
                let val = self.emit_expr(inner)?;
                let (ok_ty, err_ty) = if let Ty::Result(o, e) = &expr.ty {
                    (&**o, &**e)
                } else {
                    (&Ty::Void, &inner.ty)
                };
                let (pay_off, _, total) = result_layout(ok_ty, err_ty, self.ptr_ty);
                let base = self.malloc_bytes(total)?;
                let zero8 = self.builder.ins().iconst(types::I8, 0);
                self.builder.ins().store(MemFlags::new(), zero8, base, 0);
                self.builder
                    .ins()
                    .store(MemFlags::new(), val, base, pay_off as i32);
                base
            }
            HirExprKind::StrLit(s) => self.string_ptr(s.as_str())?,
            HirExprKind::InterpolatedStr(parts) => self.emit_interpolated_string(parts)?,
            HirExprKind::Var(name) => {
                if let Some((var, _)) = self.vars.get(name) {
                    let var = *var;
                    self.builder.use_var(var)
                } else if let Some(value) = self.load_global(name) {
                    value
                } else if let Some(expr) = self.const_exprs.get(name).cloned() {
                    self.emit_expr(&expr)?
                } else {
                    return Err(format!("undefined variable `{name}` in native codegen"));
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
                        self.builder
                            .ins()
                            .icmp(ir::condcodes::IntCC::Equal, v, zero)
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
                                // ori_io_print always takes (ptr, len); any string-like arg
                                // (String, Infer, or ptr type) gets strlen added
                                let is_known_string =
                                    matches!(&a.value.ty, Ty::String | Ty::Infer(_));
                                let is_ptr_like =
                                    cl_type(&a.value.ty, self.ptr_ty) == Some(self.ptr_ty);
                                if is_known_string {
                                    let parts = self.emit_as_string_parts(&a.value)?;
                                    cl_args.push(parts.ptr);
                                    cl_args.push(parts.len);
                                } else if is_ptr_like {
                                    let v = self.emit_expr(&a.value)?;
                                    let len = self.str_len_from_ptr(v)?;
                                    cl_args.push(v);
                                    cl_args.push(len);
                                } else {
                                    let v = self.emit_expr(&a.value)?;
                                    cl_args.push(v);
                                }
                            }
                            self.builder.ins().call(fref, &cl_args);
                            self.builder.ins().iconst(types::I8, 0)
                        } else {
                            return Err("missing runtime function `ori_io_print`".to_string());
                        }
                    } else {
                        if self.func_refs.get(name.as_str()).is_none()
                            && matches!(&callee.ty, Ty::Func { .. })
                        {
                            return self.emit_closure_call(callee, args);
                        }
                        // Special-case: list.map / list.filter pass closure as (fn_ptr, env_ptr)
                        if (name.as_str() == "ori_list_map"
                            || name.as_str() == "ori_list_filter")
                            && args.len() == 2
                            && matches!(&args[1].value.ty, Ty::Func { .. })
                        {
                            let list_v = self.emit_expr(&args[0].value)?;
                            let closure_ptr = self.emit_expr(&args[1].value)?;
                            let ptr_size = self.ptr_ty.bytes() as i32;
                            let fn_ptr = self.builder.ins().load(
                                self.ptr_ty,
                                MemFlags::new(),
                                closure_ptr,
                                0,
                            );
                            let env_ptr = self.builder.ins().load(
                                self.ptr_ty,
                                MemFlags::new(),
                                closure_ptr,
                                ptr_size,
                            );
                            if let Some(&fref) = self.func_refs.get(name.as_str()) {
                                let call = self.builder.ins().call(fref, &[list_v, fn_ptr, env_ptr]);
                                let res = self.builder.inst_results(call);
                                return Ok(if res.is_empty() {
                                    self.builder.ins().iconst(types::I8, 0)
                                } else {
                                    res[0]
                                });
                            }
                        }
                        let param_tys = self.func_param_tys.get(name).cloned();
                        let mut args_v = Vec::new();
                        for (index, arg) in args.iter().enumerate() {
                            if let Some(expected) = param_tys
                                .as_ref()
                                .and_then(|params| params.get(index))
                                .cloned()
                            {
                                let value = self.emit_expr_for_expected(&arg.value, &expected)?;
                                self.emit_arc_retain_if_managed(&expected, value)?;
                                args_v.push(value);
                            } else {
                                let value = self.emit_expr(&arg.value)?;
                                self.emit_arc_retain_if_managed(&arg.value.ty, value)?;
                                args_v.push(value);
                            }
                        }
                        if let Some(&fref) = self.func_refs.get(name.as_str()) {
                            let call = self.builder.ins().call(fref, &args_v);
                            let res = self.builder.inst_results(call);
                            if res.is_empty() {
                                self.builder.ins().iconst(types::I8, 0)
                            } else {
                                res[0]
                            }
                        } else {
                            return Err(format!(
                                "missing function reference `{name}` in native codegen"
                            ));
                        }
                    }
                } else {
                    return self.emit_closure_call(callee, args);
                }
            }
            HirExprKind::IfExpr { cond, then, else_ } => {
                let cv = self.emit_expr(cond)?;
                let tv = self.emit_expr(then)?;
                let ev = self.emit_expr(else_)?;
                self.builder.ins().select(cv, tv, ev)
            }
            HirExprKind::Propagate(inner) => {
                // `expr?` — load has_value/is_ok flag; if false, early return; else unwrap
                let ptr = self.emit_expr(inner)?;
                let flag = self.builder.ins().load(types::I8, MemFlags::new(), ptr, 0);
                let ok_blk = self.builder.create_block();
                let err_blk = self.builder.create_block();
                self.builder.ins().brif(flag, ok_blk, &[], err_blk, &[]);
                // Error path: return the whole tagged pointer (propagate error upward)
                self.builder.seal_block(err_blk);
                self.builder.switch_to_block(err_blk);
                self.terminated = false;
                self.emit_arc_retain_if_managed(&self.current_return_ty.clone(), ptr)?;
                self.emit_scope_cleanup_calls_from(0, 0)?;
                self.builder.ins().return_(&[ptr]);
                self.terminated = true;
                // Ok path: continue with unwrapped value
                self.builder.seal_block(ok_blk);
                self.builder.switch_to_block(ok_blk);
                self.terminated = false;

                let (pay_off, cl_ty) = match &inner.ty {
                    Ty::Result(ok, err) => {
                        let (off, _, _) = result_layout(ok, err, self.ptr_ty);
                        (off as i32, cl_type(ok, self.ptr_ty).unwrap_or(types::I64))
                    }
                    Ty::Optional(t) => {
                        let (off, _) = optional_layout(t, self.ptr_ty);
                        (off as i32, cl_type(t, self.ptr_ty).unwrap_or(types::I64))
                    }
                    _ => {
                        return Err(format!(
                            "`?` requires optional/result, got `{}`",
                            inner.ty.display()
                        ))
                    }
                };

                self.builder
                    .ins()
                    .load(cl_ty, MemFlags::new(), ptr, pay_off)
            }
            HirExprKind::StructLit { def_id, fields } => {
                if let Some(layout) = self.struct_layouts.get(def_id).cloned() {
                    let base = self.malloc_bytes(layout.size)?;
                    for (fname, fexpr) in fields {
                        let val = self.emit_expr(fexpr)?;
                        if let Some(fi) = layout.field(fname) {
                            if let Some(contract) = &fi.contract {
                                self.emit_value_contract(&fi.ty, val, contract, 3, false)?;
                            }
                            if cl_type(&fi.ty, self.ptr_ty).is_some() {
                                self.builder.ins().store(
                                    MemFlags::new(),
                                    val,
                                    base,
                                    fi.offset as i32,
                                );
                            }
                        } else {
                            return Err(format!(
                                "layout for struct literal is missing field `{fname}`"
                            ));
                        }
                    }
                    base
                } else {
                    return Err(format!(
                        "missing native layout for struct literal `{def_id:?}`"
                    ));
                }
            }
            HirExprKind::Field { object, field } => {
                let ptr = self.emit_expr(object)?;
                // Look up layout by DefId embedded in object's type
                let layout_opt = if let Ty::Named(def_id, _) = &object.ty {
                    self.struct_layouts.get(def_id).cloned()
                } else {
                    None
                };
                if let Some(layout) = layout_opt {
                    if let Some(fi) = layout.field(field) {
                        if let Some(cl_ty) = cl_type(&fi.ty, self.ptr_ty) {
                            self.builder
                                .ins()
                                .load(cl_ty, MemFlags::new(), ptr, fi.offset as i32)
                        } else {
                            return Err(format!("missing Cranelift type for field `{field}`"));
                        }
                    } else {
                        return Err(format!("layout is missing field `{field}`"));
                    }
                } else {
                    return Err(format!(
                        "field access `{field}` requires a struct value, got `{}`",
                        object.ty.display()
                    ));
                }
            }
            HirExprKind::Index { object, index } => {
                let container = self.emit_expr(object)?;
                let idx = self.emit_expr(index)?;
                match &object.ty {
                    Ty::List(elem_ty) => {
                        let get_ref = *self
                            .func_refs
                            .get("ori_list_get")
                            .ok_or_else(|| "missing runtime function `ori_list_get`".to_string())?;
                        let call = self.builder.ins().call(get_ref, &[container, idx]);
                        let stored = self.builder.inst_results(call)[0];
                        self.from_list_storage_value(stored, elem_ty)
                    }
                    Ty::String => {
                        let slice_ref =
                            *self.func_refs.get("ori_string_slice").ok_or_else(|| {
                                "missing runtime function `ori_string_slice`".to_string()
                            })?;
                        let one = self.builder.ins().iconst(types::I64, 1);
                        let end = self.builder.ins().iadd(idx, one);
                        let call = self.builder.ins().call(slice_ref, &[container, idx, end]);
                        self.builder.inst_results(call)[0]
                    }
                    _ => {
                        return Err(format!(
                            "native index codegen is missing for type `{}`",
                            object.ty.display()
                        ))
                    }
                }
            }
            HirExprKind::ListLit { elem_ty, elements } => {
                let list_ptr = self.emit_new_list()?;
                for elem in elements {
                    let value = self.emit_expr(elem)?;
                    self.emit_list_push_value(list_ptr, value, elem_ty)?;
                }
                list_ptr
            }
            HirExprKind::ListSpreadLit { elem_ty, elements } => {
                let list_ptr = self.emit_new_list()?;
                for elem in elements {
                    let value = self.emit_expr(&elem.value)?;
                    if elem.spread {
                        self.emit_list_extend_from(list_ptr, value, elem_ty)?;
                    } else {
                        self.emit_list_push_value(list_ptr, value, elem_ty)?;
                    }
                }
                list_ptr
            }
            HirExprKind::Range { start, end } => {
                let sv = self.emit_expr(start)?;
                let ev = self.emit_expr(end)?;
                let base = self.malloc_bytes(16)?;
                self.builder.ins().store(MemFlags::new(), sv, base, 0);
                self.builder.ins().store(MemFlags::new(), ev, base, 8);
                base
            }
            HirExprKind::EnumVariant {
                def_id,
                variant,
                fields,
                ..
            } => {
                if let Some(layout) = self.enum_layouts.get(def_id).cloned() {
                    let base = self.malloc_bytes(layout.size)?;

                    if let Some(v_layout) = layout.variant(variant) {
                        // Store the tag at offset 0
                        let tag = self.builder.ins().iconst(types::I32, v_layout.tag as i64);
                        self.builder.ins().store(MemFlags::new(), tag, base, 0);

                        // Store fields in the payload layout
                        for (fname, fexpr) in fields {
                            let val = self.emit_expr(fexpr)?;
                            if let Some(fi) = v_layout.fields.field(fname) {
                                let total_offset = (layout.payload_offset + fi.offset) as i32;
                                self.builder
                                    .ins()
                                    .store(MemFlags::new(), val, base, total_offset);
                            } else {
                                return Err(format!("layout for enum variant `{variant}` is missing field `{fname}`"));
                            }
                        }
                    } else {
                        return Err(format!("missing variant `{variant}` in native enum layout"));
                    }
                    base
                } else {
                    return Err(format!(
                        "missing native layout for enum variant `{def_id:?}.{variant}`"
                    ));
                }
            }
            HirExprKind::TupleLit(elems) => {
                let mut vals_and_offsets = Vec::new();
                let elem_tys: Vec<Ty> = elems.iter().map(|e| e.ty.clone()).collect();
                let (layout, total, _) = tuple_layout(&elem_tys, self.ptr_ty);

                for (e, (offset, _)) in elems.iter().zip(layout.iter()) {
                    let v = self.emit_expr(e)?;
                    vals_and_offsets.push((v, *offset));
                }
                let base = self.malloc_bytes(total)?;

                for (v, off) in vals_and_offsets {
                    self.builder
                        .ins()
                        .store(MemFlags::new(), v, base, off as i32);
                }
                base
            }
            HirExprKind::TupleIndex { object, index } => {
                let ptr = self.emit_expr(object)?;
                if let Ty::Tuple(tys) = &object.ty {
                    let (layout, _, _) = tuple_layout(tys, self.ptr_ty);
                    let Some((target_off, target_ty)) = layout.get(*index as usize) else {
                        return Err(format!(
                            "tuple index `{index}` is out of bounds for `{}`",
                            object.ty.display()
                        ));
                    };
                    if let Some(cl) = cl_type(target_ty, self.ptr_ty) {
                        self.builder
                            .ins()
                            .load(cl, MemFlags::new(), ptr, *target_off as i32)
                    } else {
                        return Err(format!("missing Cranelift type for tuple index `{index}`"));
                    }
                } else {
                    return Err(format!(
                        "tuple index on non-tuple type `{}`",
                        object.ty.display()
                    ));
                }
            }
            HirExprKind::MapLit { entries, .. } => {
                let (key_ty, value_ty) = match &expr.ty {
                    Ty::Map(key, value) => (*key.clone(), *value.clone()),
                    _ => (Ty::Infer(0), Ty::Infer(0)),
                };
                let map_ptr = if let Some(&new_ref) = self.func_refs.get("ori_map_new") {
                    let call = self.builder.ins().call(new_ref, &[]);
                    self.builder.inst_results(call)[0]
                } else {
                    return Err("missing runtime function `ori_map_new`".to_string());
                };
                if let Some(&set_ref) = self.func_refs.get("ori_map_set") {
                    for (k, v) in entries {
                        let kv = self.emit_expr(k)?;
                        let vv = self.emit_expr(v)?;
                        let kv = self.to_list_storage_value(kv, &key_ty);
                        let vv = self.to_list_storage_value(vv, &value_ty);
                        self.builder.ins().call(set_ref, &[map_ptr, kv, vv]);
                    }
                } else {
                    return Err("missing runtime function `ori_map_set`".to_string());
                }
                map_ptr
            }
            HirExprKind::SetLit { elements, .. } => {
                let set_ptr = if let Some(&new_ref) = self.func_refs.get("ori_set_new") {
                    let call = self.builder.ins().call(new_ref, &[]);
                    self.builder.inst_results(call)[0]
                } else {
                    return Err("missing runtime function `ori_set_new`".to_string());
                };
                if let Some(&add_ref) = self.func_refs.get("ori_set_add") {
                    for elem in elements {
                        let v = self.emit_expr(elem)?;
                        self.builder.ins().call(add_ref, &[set_ptr, v]);
                    }
                } else {
                    return Err("missing runtime function `ori_set_add`".to_string());
                }
                set_ptr
            }
            HirExprKind::StructUpdate {
                def_id,
                base,
                updates,
            } => {
                if let Some(layout) = self.struct_layouts.get(def_id).cloned() {
                    let base_ptr = self.emit_expr(base)?;
                    let new_ptr = self.malloc_bytes(layout.size)?;
                    // Copy all bytes from base
                    for (_fname, fl) in &layout.fields {
                        if let Some(cl) = cl_type(&fl.ty, self.ptr_ty) {
                            let val = self.builder.ins().load(
                                cl,
                                MemFlags::new(),
                                base_ptr,
                                fl.offset as i32,
                            );
                            self.builder.ins().store(
                                MemFlags::new(),
                                val,
                                new_ptr,
                                fl.offset as i32,
                            );
                        }
                    }
                    // Override updated fields
                    for (fname, fexpr) in updates {
                        let val = self.emit_expr(fexpr)?;
                        if let Some(fi) = layout.field(fname) {
                            if cl_type(&fi.ty, self.ptr_ty).is_some() {
                                self.builder.ins().store(
                                    MemFlags::new(),
                                    val,
                                    new_ptr,
                                    fi.offset as i32,
                                );
                            }
                        }
                    }
                    new_ptr
                } else {
                    return Err(format!(
                        "missing native layout for struct update `{def_id:?}`"
                    ));
                }
            }
            HirExprKind::MethodCall {
                receiver,
                method,
                args,
            } => {
                if matches!(&receiver.ty, Ty::Any(_)) {
                    return self.emit_dynamic_method_call(receiver, method, args);
                }
                let recv = self.emit_expr(receiver)?;
                match method.as_str() {
                    "__slice" if matches!(&receiver.ty, Ty::String) => {
                        // ori_string_slice(s, start, end)
                        let slice_ref =
                            *self.func_refs.get("ori_string_slice").ok_or_else(|| {
                                "missing runtime function `ori_string_slice`".to_string()
                            })?;
                        let start = self.emit_expr(&args[0])?;
                        let end = self.emit_expr(&args[1])?;
                        let call = self.builder.ins().call(slice_ref, &[recv, start, end]);
                        self.builder.inst_results(call)[0]
                    }
                    _ => {
                        // Generic method call: look up as a function `method(receiver, args...)`
                        let mut all_args = vec![recv];
                        self.emit_arc_retain_if_managed(&receiver.ty, recv)?;
                        for a in args {
                            let value = self.emit_expr(a)?;
                            self.emit_arc_retain_if_managed(&a.ty, value)?;
                            all_args.push(value);
                        }
                        if let Some(&fref) = self.func_refs.get(method.as_str()) {
                            let call = self.builder.ins().call(fref, &all_args);
                            let res = self.builder.inst_results(call);
                            if res.is_empty() {
                                self.builder.ins().iconst(types::I8, 0)
                            } else {
                                res[0]
                            }
                        } else {
                            return Err(format!(
                                "missing function reference `{method}` in native codegen"
                            ));
                        }
                    }
                }
            }
            HirExprKind::Closure {
                func_name,
                captures,
            } => self.emit_closure_value(func_name, captures)?,
            HirExprKind::IsCheck { value, check_ty } => {
                let val = self.emit_expr(value)?;
                if let Ty::Named(check_def_id, _) = check_ty {
                    if matches!(&value.ty, Ty::Any(_)) {
                        let ptr_size = self.ptr_ty.bytes() as i64;
                        let vtable = self.builder.ins().load(self.ptr_ty, MemFlags::new(), val, ptr_size as i32);
                        let actual_type_id = self.builder.ins().load(self.ptr_ty, MemFlags::new(), vtable, 0);
                        let expected_type_id = self.builder.ins().iconst(self.ptr_ty, check_def_id.0 as i64);
                        
                        let is_match = self.builder.ins().icmp(ir::condcodes::IntCC::Equal, actual_type_id, expected_type_id);
                        is_match
                    } else if let Ty::Named(actual_def_id, _) = &value.ty {
                        let is_match = actual_def_id.0 == check_def_id.0;
                        self.builder.ins().iconst(types::I8, if is_match { 1 } else { 0 })
                    } else {
                        self.builder.ins().iconst(types::I8, 0)
                    }
                } else {
                    self.builder.ins().iconst(types::I8, 0)
                }
            }
            HirExprKind::BytesLit(_) | HirExprKind::GlobalConst(_) => {
                return Err(format!(
                    "native codegen missing for expression `{:?}`",
                    expr.kind
                ));
            }
        })
    }

    /// For a null-terminated string pointer, compute its length as an i64.
    /// Uses strlen-like logic: call strlen if available, else scan bytes.
    /// For now we use the `strlen` libc function declared on demand.
    fn str_len_from_ptr(&mut self, ptr: ir::Value) -> Result<ir::Value, String> {
        if let Some(&fref) = self.func_refs.get("strlen") {
            let call = self.builder.ins().call(fref, &[ptr]);
            // strlen declared as returning I64; result is already the right type
            return Ok(self.builder.inst_results(call)[0]);
        }
        Err("missing runtime function `strlen`".to_string())
    }

    fn to_list_storage_value(&mut self, value: ir::Value, ty: &Ty) -> ir::Value {
        match ty {
            Ty::Bool | Ty::Int8 | Ty::U8 | Ty::Int16 | Ty::U16 | Ty::Int32 | Ty::U32 => {
                self.builder.ins().uextend(types::I64, value)
            }
            _ => value,
        }
    }

    fn from_list_storage_value(&mut self, value: ir::Value, ty: &Ty) -> ir::Value {
        match ty {
            Ty::Bool | Ty::Int8 | Ty::U8 => self.builder.ins().ireduce(types::I8, value),
            Ty::Int16 | Ty::U16 => self.builder.ins().ireduce(types::I16, value),
            Ty::Int32 | Ty::U32 => self.builder.ins().ireduce(types::I32, value),
            _ => value,
        }
    }

    fn emit_binary(
        &mut self,
        op: BinaryOp,
        lv: ir::Value,
        rv: ir::Value,
        ty: &Ty,
    ) -> Result<ir::Value, String> {
        use ir::condcodes::{FloatCC, IntCC};
        use BinaryOp::*;
        let float = is_float_ty(ty);
        let string = matches!(ty, Ty::String);
        Ok(match op {
            Add => {
                if string {
                    let concat_ref = *self.func_refs.get("ori_string_concat").ok_or_else(|| {
                        "missing runtime function `ori_string_concat`".to_string()
                    })?;
                    let call = self.builder.ins().call(concat_ref, &[lv, rv]);
                    self.builder.inst_results(call)[0]
                } else if float {
                    self.builder.ins().fadd(lv, rv)
                } else {
                    self.builder.ins().iadd(lv, rv)
                }
            }
            Sub => {
                if float {
                    self.builder.ins().fsub(lv, rv)
                } else {
                    self.builder.ins().isub(lv, rv)
                }
            }
            Mul => {
                if float {
                    self.builder.ins().fmul(lv, rv)
                } else {
                    self.builder.ins().imul(lv, rv)
                }
            }
            Div => {
                if float {
                    self.builder.ins().fdiv(lv, rv)
                } else {
                    self.builder.ins().sdiv(lv, rv)
                }
            }
            Rem => self.builder.ins().srem(lv, rv),
            Eq => {
                if string {
                    let strcmp_ref = *self
                        .func_refs
                        .get("strcmp")
                        .ok_or_else(|| "missing runtime function `strcmp`".to_string())?;
                    let call = self.builder.ins().call(strcmp_ref, &[lv, rv]);
                    let cmp = self.builder.inst_results(call)[0];
                    let zero = self.builder.ins().iconst(types::I32, 0);
                    self.builder.ins().icmp(IntCC::Equal, cmp, zero)
                } else if float {
                    self.builder.ins().fcmp(FloatCC::Equal, lv, rv)
                } else {
                    self.builder.ins().icmp(IntCC::Equal, lv, rv)
                }
            }
            Ne => {
                if string {
                    let strcmp_ref = *self
                        .func_refs
                        .get("strcmp")
                        .ok_or_else(|| "missing runtime function `strcmp`".to_string())?;
                    let call = self.builder.ins().call(strcmp_ref, &[lv, rv]);
                    let cmp = self.builder.inst_results(call)[0];
                    let zero = self.builder.ins().iconst(types::I32, 0);
                    self.builder.ins().icmp(IntCC::NotEqual, cmp, zero)
                } else if float {
                    self.builder.ins().fcmp(FloatCC::NotEqual, lv, rv)
                } else {
                    self.builder.ins().icmp(IntCC::NotEqual, lv, rv)
                }
            }
            Lt => {
                if float {
                    self.builder.ins().fcmp(FloatCC::LessThan, lv, rv)
                } else {
                    self.builder.ins().icmp(IntCC::SignedLessThan, lv, rv)
                }
            }
            Le => {
                if float {
                    self.builder.ins().fcmp(FloatCC::LessThanOrEqual, lv, rv)
                } else {
                    self.builder
                        .ins()
                        .icmp(IntCC::SignedLessThanOrEqual, lv, rv)
                }
            }
            Gt => {
                if float {
                    self.builder.ins().fcmp(FloatCC::GreaterThan, lv, rv)
                } else {
                    self.builder.ins().icmp(IntCC::SignedGreaterThan, lv, rv)
                }
            }
            Ge => {
                if float {
                    self.builder.ins().fcmp(FloatCC::GreaterThanOrEqual, lv, rv)
                } else {
                    self.builder
                        .ins()
                        .icmp(IntCC::SignedGreaterThanOrEqual, lv, rv)
                }
            }
            And => self.builder.ins().band(lv, rv),
            Or => self.builder.ins().bor(lv, rv),
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
    obj_path: &std::path::Path,
    exe_path: &std::path::Path,
    extra_libs: &[std::path::PathBuf],
) -> Result<(), String> {
    let mut cmd = std::process::Command::new("cc");
    cmd.arg("-o").arg(exe_path).arg(obj_path);
    for lib in extra_libs {
        cmd.arg(lib);
    }
    let status = cmd
        .status()
        .map_err(|e| format!("could not invoke cc: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "linker exited with code {}",
            status.code().unwrap_or(-1)
        ))
    }
}
