use smol_str::SmolStr;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use cranelift_codegen::ir::{self, types, AbiParam, BlockArg, InstBuilder, MemFlags};
use cranelift_codegen::settings;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{DataDescription, DataId, FuncId, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};

use ori_ast::expr::{BinaryOp, UnaryOp};
use ori_diagnostics::Span;
use ori_hir::hir::*;
use ori_types::{
    stdlib::{stdlib_func_sig, stdlib_native_abi, stdlib_runtime_functions, StdlibNativeAbiTy},
    OpaqueTy, Ty,
};

#[cfg(test)]
const INTERNAL_NATIVE_RUNTIME_IMPORTS: &[&str] = &[
    "ori_arc_collect_cycles",
    "ori_arc_register_edge",
    "ori_arc_release",
    "ori_arc_retain",
    "ori_arc_unregister_edge",
    "ori_arc_update_edge",
    "ori_alloc",
    "ori_bool_to_string_parts",
    "ori_executor_drain",
    "ori_executor_run_one",
    "ori_executor_schedule",
    "ori_float_to_string_parts",
    "ori_future_cancel",
    "ori_future_complete_f64",
    "ori_future_complete_i64",
    "ori_future_complete_ptr",
    "ori_future_complete_void",
    "ori_future_fail",
    "ori_future_on_ready",
    "ori_future_pending",
    "ori_future_poll",
    "ori_future_ready_f64",
    "ori_future_ready_i64",
    "ori_future_ready_ptr",
    "ori_future_ready_void",
    "ori_future_value_f64",
    "ori_future_value_i64",
    "ori_future_value_ptr",
    "ori_graph_add_edge_string",
    "ori_graph_add_weighted_edge_string",
    "ori_graph_add_node_string",
    "ori_graph_bfs_string",
    "ori_graph_dfs_string",
    "ori_graph_edge_weight_string",
    "ori_graph_has_edge_string",
    "ori_graph_has_node_string",
    "ori_graph_neighbors_string",
    "ori_graph_remove_edge_string",
    "ori_graph_remove_node_string",
    "ori_graph_shortest_path_string",
    "ori_graph_shortest_weighted_path_string",
    "ori_hash_table_contains_string",
    "ori_hash_table_from_entries_string",
    "ori_hash_table_get_string",
    "ori_hash_table_remove_string",
    "ori_hash_table_set_string",
    "ori_heap_from_list_custom",
    "ori_heap_from_list_string",
    "ori_heap_new_custom",
    "ori_heap_new_string",
    "ori_heap_push_custom",
    "ori_heap_push_string",
    "ori_heap_remove_custom",
    "ori_heap_remove_string",
    "ori_int_to_cstr",
    "ori_doubly_linked_list_find_string",
    "ori_linked_list_find_string",
    "ori_map_contains_string",
    "ori_map_get_string",
    "ori_map_key_at",
    "ori_map_remove_string",
    "ori_map_set_string",
    "ori_map_from_entries_string",
    "ori_map_try_get_string",
    "ori_map_try_remove_string",
    "ori_map_value_at",
    "ori_math_abs_float",
    "ori_math_max_float",
    "ori_math_min_float",
    "ori_new_result",
    "ori_os_set_args",
    "ori_set_add_string",
    "ori_set_contains_string",
    "ori_set_from_list_string",
    "ori_set_remove_string",
    "ori_set_try_remove_string",
    "ori_tree_find_string",
    "ori_string_concat_parts",
    "ori_to_string_parts",
];

struct StringCollector {
    out: Vec<SmolStr>,
    seen: std::collections::HashSet<SmolStr>,
}

impl StringCollector {
    fn new() -> Self {
        let mut seen = std::collections::HashSet::new();
        let empty = SmolStr::new("");
        seen.insert(empty.clone());
        Self {
            out: vec![empty],
            seen,
        }
    }

    fn add(&mut self, s: SmolStr) {
        if self.seen.insert(s.clone()) {
            self.out.push(s);
        }
    }
}

fn collect_strings_expr(expr: &HirExpr, out: &mut StringCollector) {
    match &expr.kind {
        HirExprKind::BoolLit(_)
        | HirExprKind::IntLit(_)
        | HirExprKind::FloatLit(_)
        | HirExprKind::BytesLit(_)
        | HirExprKind::Unit
        | HirExprKind::None_
        | HirExprKind::Var(_)
        | HirExprKind::Closure { .. } => {}
        HirExprKind::StrLit(s) => out.add(s.clone()),
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
        | HirExprKind::Await(e)
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
                    HirStrPart::Literal(s) => out.add(s.clone()),
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
        HirExprKind::IsCheck { value, .. } => collect_strings_expr(value, out),
    }
}

fn collect_strings_block(block: &HirBlock, out: &mut StringCollector) {
    for s in &block.stmts {
        collect_strings_stmt(s, out);
    }
}

fn collect_strings_stmt(stmt: &HirStmt, out: &mut StringCollector) {
    match stmt {
        HirStmt::Let { value, .. } => collect_strings_expr(value, out),
        HirStmt::Assign { lvalue, value, .. } => {
            collect_strings_lvalue(lvalue, out);
            collect_strings_expr(value, out);
        }
        HirStmt::Return(Some(e), _) => collect_strings_expr(e, out),
        HirStmt::Return(None, _) | HirStmt::Break(_) | HirStmt::Continue(_) => {}
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
        HirStmt::Check {
            condition, message, ..
        } => {
            collect_strings_expr(condition, out);
            if let Some(message) = message {
                out.add(message.clone());
            }
        }
    }
}

fn collect_strings_lvalue(lvalue: &HirLValue, out: &mut StringCollector) {
    match lvalue {
        HirLValue::Var(_) => {}
        HirLValue::Field { base, .. } => collect_strings_lvalue(base, out),
        HirLValue::Index { base, index } => {
            collect_strings_lvalue(base, out);
            collect_strings_expr(index, out);
        }
    }
}

fn collect_all_strings(hir: &HirModule) -> Vec<SmolStr> {
    let mut collector = StringCollector::new();
    for f in &hir.funcs {
        collect_strings_block(&f.body, &mut collector);
    }
    for c in &hir.consts {
        collect_strings_expr(&c.value, &mut collector);
    }
    collector.out
}

fn collect_strings_pattern(pat: &HirPattern, out: &mut StringCollector) {
    match pat {
        HirPattern::Wildcard
        | HirPattern::Binding(_, _)
        | HirPattern::BoolLit(_)
        | HirPattern::IntLit(_)
        | HirPattern::None_ => {}
        HirPattern::StrLit(s) => out.add(s.clone()),
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
        Ty::String
        | Ty::Bytes
        | Ty::Func { .. }
        | Ty::Lazy(_)
        | Ty::Future(_)
        | Ty::TaskJob(_)
        | Ty::Channel(_)
        | Ty::AtomicInt
        | Ty::TaskJoinError
        | Ty::ChannelSendError
        | Ty::ChannelReceiveError => Some(ptr_ty),
        Ty::Opaque {
            kind: OpaqueTy::NodeId,
            ..
        } => Some(types::I64),
        Ty::Opaque { .. } => Some(ptr_ty),
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

fn native_codegen_unsupported(message: impl Into<String>) -> String {
    const CODE: &str = "backend.native_unsupported";
    format!("{CODE}: {}", message.into())
}

fn cl_stdlib_abi_type(ty: StdlibNativeAbiTy, ptr_ty: types::Type) -> types::Type {
    match ty {
        StdlibNativeAbiTy::Ptr => ptr_ty,
        StdlibNativeAbiTy::I64 => types::I64,
        StdlibNativeAbiTy::I32 => types::I32,
        StdlibNativeAbiTy::I8 => types::I8,
        StdlibNativeAbiTy::F64 => types::F64,
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
            | Ty::Lazy(_)
            | Ty::Future(_)
            | Ty::TaskJob(_)
            | Ty::Channel(_)
            | Ty::AtomicInt
            | Ty::TaskJoinError
            | Ty::ChannelSendError
            | Ty::ChannelReceiveError
            | Ty::Opaque {
                kind: OpaqueTy::Deque
                    | OpaqueTy::Queue
                    | OpaqueTy::Stack
                    | OpaqueTy::LinkedList
                    | OpaqueTy::DoublyLinkedList
                    | OpaqueTy::Tree
                    | OpaqueTy::HashTable
                    | OpaqueTy::Graph
                    | OpaqueTy::Heap,
                ..
            }
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

fn needs_runtime_global_init(expr: &HirExpr, ty: &Ty) -> bool {
    const_static_bytes(expr, ty).is_none()
}

fn is_float_ty(ty: &Ty) -> bool {
    matches!(ty, Ty::Float | Ty::Float32 | Ty::Float64)
}

fn validate_native_hir(hir: &HirModule) -> Result<(), String> {
    NativeHirValidator::new()
        .module(hir)
        .map_err(|err| format!("invalid HIR for native backend: {err}"))
}

struct NativeHirValidator;

impl NativeHirValidator {
    fn new() -> Self {
        Self
    }

    fn module(&self, hir: &HirModule) -> Result<(), String> {
        for s in &hir.structs {
            for field in &s.fields {
                self.reject_error_ty(&field.ty, "struct field type", field.span)?;
                if let Some(contract) = &field.contract {
                    self.expr(contract)?;
                }
            }
        }
        for e in &hir.enums {
            for variant in &e.variants {
                for field in &variant.fields {
                    self.reject_error_ty(&field.ty, "enum variant field type", field.span)?;
                    if let Some(contract) = &field.contract {
                        self.expr(contract)?;
                    }
                }
            }
        }
        for t in &hir.traits {
            for method in &t.methods {
                for param in &method.params {
                    self.reject_error_ty(param, "trait method parameter type", Span::DUMMY)?;
                }
                self.reject_error_ty(&method.return_ty, "trait method return type", Span::DUMMY)?;
            }
        }
        for f in &hir.funcs {
            for param in &f.params {
                self.reject_error_ty(&param.ty, "function parameter type", param.span)?;
                if let Some(default) = &param.default {
                    self.expr(default)?;
                }
                if let Some(contract) = &param.contract {
                    self.expr(contract)?;
                }
            }
            self.reject_error_ty(&f.return_ty, "function return type", f.span)?;
            for capture in &f.closure_captures {
                self.reject_error_ty(&capture.ty, "closure capture type", f.span)?;
            }
            self.block(&f.body)?;
        }
        for c in &hir.consts {
            self.reject_error_ty(&c.ty, "const type", c.span)?;
            self.expr(&c.value)?;
        }
        for ext in &hir.externs {
            match ext {
                HirExtern::Func {
                    params,
                    return_ty,
                    span,
                    ..
                } => {
                    for param in params {
                        self.reject_error_ty(
                            &param.ty,
                            "extern function parameter type",
                            param.span,
                        )?;
                    }
                    self.reject_error_ty(return_ty, "extern function return type", *span)?;
                }
                HirExtern::Var { ty, span, .. } => {
                    self.reject_error_ty(ty, "extern variable type", *span)?;
                }
            }
        }
        Ok(())
    }

    fn block(&self, block: &HirBlock) -> Result<(), String> {
        for stmt in &block.stmts {
            self.stmt(stmt)?;
        }
        Ok(())
    }

    fn stmt(&self, stmt: &HirStmt) -> Result<(), String> {
        match stmt {
            HirStmt::Let {
                ty, value, span, ..
            } => {
                self.reject_error_ty(ty, "let binding type", *span)?;
                self.expr(value)?;
            }
            HirStmt::Assign { value, .. } | HirStmt::Return(Some(value), _) => {
                self.expr(value)?;
            }
            HirStmt::Return(None, _) | HirStmt::Break(_) | HirStmt::Continue(_) => {}
            HirStmt::Expr(expr) => self.expr(expr)?,
            HirStmt::If {
                cond,
                then,
                else_ifs,
                else_,
                ..
            } => {
                self.expr(cond)?;
                self.expect_bool(&cond.ty, "if condition", cond.span)?;
                self.block(then)?;
                for (else_cond, else_block) in else_ifs {
                    self.expr(else_cond)?;
                    self.expect_bool(&else_cond.ty, "else-if condition", else_cond.span)?;
                    self.block(else_block)?;
                }
                if let Some(block) = else_ {
                    self.block(block)?;
                }
            }
            HirStmt::While { cond, body, .. } => {
                self.expr(cond)?;
                self.expect_bool(&cond.ty, "while condition", cond.span)?;
                self.block(body)?;
            }
            HirStmt::For {
                elem_ty,
                iterable,
                body,
                span,
                ..
            } => {
                self.reject_error_ty(elem_ty, "for element type", *span)?;
                self.expr(iterable)?;
                self.block(body)?;
            }
            HirStmt::Loop { body, .. } => self.block(body)?,
            HirStmt::Repeat { count, body, .. } => {
                self.expr(count)?;
                self.expect_integer(&count.ty, "repeat count", count.span)?;
                self.block(body)?;
            }
            HirStmt::Match {
                scrutinee, arms, ..
            } => {
                self.expr(scrutinee)?;
                for arm in arms {
                    self.pattern(&arm.pattern, arm.span)?;
                    for stmt in &arm.body {
                        self.stmt(stmt)?;
                    }
                }
            }
            HirStmt::IfSome {
                inner_ty,
                value,
                then,
                else_,
                span,
                ..
            } => {
                self.reject_error_ty(inner_ty, "if-some binding type", *span)?;
                self.expr(value)?;
                self.block(then)?;
                if let Some(block) = else_ {
                    self.block(block)?;
                }
            }
            HirStmt::WhileSome {
                inner_ty,
                value,
                body,
                span,
                ..
            } => {
                self.reject_error_ty(inner_ty, "while-some binding type", *span)?;
                self.expr(value)?;
                self.block(body)?;
            }
            HirStmt::Using {
                ty, value, span, ..
            } => {
                self.reject_error_ty(ty, "using binding type", *span)?;
                self.expr(value)?;
            }
            HirStmt::Check { condition, .. } => {
                self.expr(condition)?;
                self.expect_bool(&condition.ty, "check condition", condition.span)?;
            }
        }
        Ok(())
    }

    fn expr(&self, expr: &HirExpr) -> Result<(), String> {
        self.reject_error_ty(&expr.ty, "expression type", expr.span)?;
        match &expr.kind {
            HirExprKind::BoolLit(_)
            | HirExprKind::IntLit(_)
            | HirExprKind::FloatLit(_)
            | HirExprKind::StrLit(_)
            | HirExprKind::BytesLit(_)
            | HirExprKind::Unit
            | HirExprKind::None_
            | HirExprKind::Var(_) => {}
            HirExprKind::InterpolatedStr(parts) => {
                for part in parts {
                    if let HirStrPart::Expr(expr) = part {
                        self.expr(expr)?;
                    }
                }
            }
            HirExprKind::Binary { op, lhs, rhs } => {
                self.expr(lhs)?;
                self.expr(rhs)?;
                match op {
                    BinaryOp::And | BinaryOp::Or => {
                        self.expect_bool(&lhs.ty, "logical operator left operand", lhs.span)?;
                        self.expect_bool(&rhs.ty, "logical operator right operand", rhs.span)?;
                        self.expect_bool(&expr.ty, "logical operator result", expr.span)?;
                    }
                    BinaryOp::Eq
                    | BinaryOp::Ne
                    | BinaryOp::Lt
                    | BinaryOp::Le
                    | BinaryOp::Gt
                    | BinaryOp::Ge => {
                        self.expect_bool(&expr.ty, "comparison result", expr.span)?;
                    }
                    BinaryOp::Add
                    | BinaryOp::Sub
                    | BinaryOp::Mul
                    | BinaryOp::Div
                    | BinaryOp::Rem => {}
                }
            }
            HirExprKind::Unary { op, operand } => {
                self.expr(operand)?;
                if matches!(op, UnaryOp::Not) {
                    self.expect_bool(&operand.ty, "not operand", operand.span)?;
                    self.expect_bool(&expr.ty, "not result", expr.span)?;
                }
            }
            HirExprKind::Field { object, .. } | HirExprKind::TupleIndex { object, .. } => {
                self.expr(object)?;
            }
            HirExprKind::Index { object, index } => {
                self.expr(object)?;
                self.expr(index)?;
                self.expect_integer(&index.ty, "index expression", index.span)?;
            }
            HirExprKind::Call { callee, args } => {
                self.expr(callee)?;
                for arg in args {
                    self.expr(&arg.value)?;
                }
            }
            HirExprKind::MethodCall { receiver, args, .. } => {
                self.expr(receiver)?;
                for arg in args {
                    self.expr(arg)?;
                }
            }
            HirExprKind::StructLit { fields, .. } | HirExprKind::EnumVariant { fields, .. } => {
                for (_, value) in fields {
                    self.expr(value)?;
                }
            }
            HirExprKind::ListLit { elem_ty, elements } => {
                self.reject_error_ty(elem_ty, "list element type", expr.span)?;
                for element in elements {
                    self.expr(element)?;
                }
            }
            HirExprKind::ListSpreadLit { elem_ty, elements } => {
                self.reject_error_ty(elem_ty, "list element type", expr.span)?;
                for element in elements {
                    self.expr(&element.value)?;
                }
            }
            HirExprKind::TupleLit(elements) => {
                for element in elements {
                    self.expr(element)?;
                }
            }
            HirExprKind::Some_(inner)
            | HirExprKind::Ok_(inner)
            | HirExprKind::Err_(inner)
            | HirExprKind::Propagate(inner)
            | HirExprKind::Await(inner) => self.expr(inner)?,
            HirExprKind::IfExpr { cond, then, else_ } => {
                self.expr(cond)?;
                self.expect_bool(&cond.ty, "if expression condition", cond.span)?;
                self.expr(then)?;
                self.expr(else_)?;
            }
            HirExprKind::Range { start, end } => {
                self.expr(start)?;
                self.expr(end)?;
                self.expect_integer(&start.ty, "range start", start.span)?;
                self.expect_integer(&end.ty, "range end", end.span)?;
            }
            HirExprKind::MapLit {
                key_ty,
                value_ty,
                entries,
            } => {
                self.reject_error_ty(key_ty, "map key type", expr.span)?;
                self.reject_error_ty(value_ty, "map value type", expr.span)?;
                for (key, value) in entries {
                    self.expr(key)?;
                    self.expr(value)?;
                }
            }
            HirExprKind::SetLit { elem_ty, elements } => {
                self.reject_error_ty(elem_ty, "set element type", expr.span)?;
                for element in elements {
                    self.expr(element)?;
                }
            }
            HirExprKind::StructUpdate { base, updates, .. } => {
                self.expr(base)?;
                for (_, value) in updates {
                    self.expr(value)?;
                }
            }
            HirExprKind::Closure { captures, .. } => {
                for capture in captures {
                    self.reject_error_ty(&capture.ty, "closure capture type", expr.span)?;
                }
            }
            HirExprKind::IsCheck { value, check_ty } => {
                self.expr(value)?;
                self.reject_error_ty(check_ty, "is-check type", expr.span)?;
                self.expect_bool(&expr.ty, "is-check result", expr.span)?;
            }
        }
        Ok(())
    }

    fn pattern(&self, pattern: &HirPattern, span: Span) -> Result<(), String> {
        match pattern {
            HirPattern::Binding(_, ty) => self.reject_error_ty(ty, "pattern binding type", span)?,
            HirPattern::Some_(inner) | HirPattern::Ok_(inner) | HirPattern::Err_(inner) => {
                self.pattern(inner, span)?;
            }
            HirPattern::Variant { fields, .. } => {
                for (_, field_pattern) in fields {
                    self.pattern(field_pattern, span)?;
                }
            }
            HirPattern::Tuple(items) => {
                for item in items {
                    self.pattern(item, span)?;
                }
            }
            HirPattern::Wildcard
            | HirPattern::BoolLit(_)
            | HirPattern::IntLit(_)
            | HirPattern::StrLit(_)
            | HirPattern::None_ => {}
        }
        Ok(())
    }

    fn expect_bool(&self, ty: &Ty, context: &str, span: Span) -> Result<(), String> {
        if matches!(ty, Ty::Bool) {
            Ok(())
        } else {
            Err(format!(
                "{context} must be bool, got `{}` at {span}",
                ty.display()
            ))
        }
    }

    fn expect_integer(&self, ty: &Ty, context: &str, span: Span) -> Result<(), String> {
        if ty.is_integer() {
            Ok(())
        } else {
            Err(format!(
                "{context} must be an integer, got `{}` at {span}",
                ty.display()
            ))
        }
    }

    fn reject_error_ty(&self, ty: &Ty, context: &str, span: Span) -> Result<(), String> {
        if contains_error_ty(ty) {
            Err(format!(
                "{context} contains unresolved error type at {span}"
            ))
        } else {
            Ok(())
        }
    }
}

fn contains_error_ty(ty: &Ty) -> bool {
    match ty {
        Ty::Error => true,
        Ty::Optional(inner)
        | Ty::List(inner)
        | Ty::Set(inner)
        | Ty::Range(inner)
        | Ty::Lazy(inner)
        | Ty::Future(inner)
        | Ty::TaskJob(inner)
        | Ty::Channel(inner) => contains_error_ty(inner),
        Ty::Result(ok, err) | Ty::Map(ok, err) => contains_error_ty(ok) || contains_error_ty(err),
        Ty::Opaque { args, .. } => args.iter().any(contains_error_ty),
        Ty::Tuple(items) => items.iter().any(contains_error_ty),
        Ty::Func { params, ret } => params.iter().any(contains_error_ty) || contains_error_ty(ret),
        Ty::Named(_, args) => args.iter().any(contains_error_ty),
        _ => false,
    }
}

fn mangle_symbol(name: &str) -> String {
    let mut out = String::with_capacity(name.len() * 2);
    for c in name.chars() {
        if c.is_ascii_alphanumeric() || c == '_' {
            out.push(c);
        } else if c == '.' {
            out.push_str("_dot_");
        } else {
            use std::fmt::Write;
            write!(&mut out, "_x{:02x}_", c as u32).unwrap();
        }
    }
    out
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
    let entry = if hir.namespace.is_empty() {
        "main".to_string()
    } else {
        format!("{}.main", hir.namespace)
    };
    f.params.is_empty() && f.name.as_str() == entry
}

fn is_synthetic_closure_func(f: &HirFunc) -> bool {
    f.params
        .first()
        .is_some_and(|param| param.name.as_str() == "__env")
        && f.name.contains(".__closure_")
}

fn async_step_name(f: &HirFunc) -> SmolStr {
    SmolStr::new(format!("{}.__async_step", f.name))
}

fn async_inner_return_ty(f: &HirFunc) -> Option<Ty> {
    match &f.return_ty {
        Ty::Future(inner) => Some(inner.as_ref().clone()),
        _ => None,
    }
}

#[derive(Clone)]
struct SimpleAsyncBinding {
    name: SmolStr,
    ty: Ty,
}

#[derive(Clone)]
struct SimpleAsyncParam {
    name: SmolStr,
    ty: Ty,
}

#[derive(Clone)]
struct SimpleAsyncLocal {
    name: SmolStr,
    ty: Ty,
    value: HirExpr,
}

#[derive(Clone)]
struct SimpleAsyncAwaitStep {
    await_future: HirExpr,
    binding: Option<SimpleAsyncBinding>,
    propagate_result_ty: Option<Ty>,
}

#[derive(Clone)]
struct SimpleAsyncStateMachinePlan {
    params: Vec<SimpleAsyncParam>,
    locals: Vec<SimpleAsyncLocal>,
    awaits: Vec<SimpleAsyncAwaitStep>,
    tail_stmts: Vec<HirStmt>,
    return_expr: Option<HirExpr>,
    tail_expr: Option<HirExpr>,
    inner_ty: Ty,
}

const ASYNC_FRAME_STATE_OFFSET: i32 = 0;
const ASYNC_FRAME_RESULT_OFFSET: i32 = 8;
const ASYNC_FRAME_AWAITED_BASE_OFFSET: u32 = 16;

fn align_u32(value: u32, align: u8) -> u32 {
    let align = align.max(1) as u32;
    (value + align - 1) & !(align - 1)
}

fn simple_async_frame_binding_offset(
    plan: &SimpleAsyncStateMachinePlan,
    step_index: usize,
    ptr_ty: types::Type,
) -> Option<u32> {
    let binding = plan.awaits.get(step_index)?.binding.as_ref()?;
    let mut offset = simple_async_frame_param_base_offset(plan, ptr_ty);
    for (param_index, param) in plan.params.iter().enumerate() {
        let param_offset = simple_async_frame_param_offset(plan, param_index, ptr_ty)
            .expect("param offset exists for param");
        let (size, _) = field_size_align(&param.ty, ptr_ty);
        offset = param_offset + size;
    }
    for (local_index, local) in plan.locals.iter().enumerate() {
        let local_offset = simple_async_frame_local_offset(plan, local_index, ptr_ty)
            .expect("local offset exists for local");
        let (size, _) = field_size_align(&local.ty, ptr_ty);
        offset = local_offset + size;
    }
    for step in plan.awaits.iter().take(step_index) {
        if let Some(binding) = &step.binding {
            let (size, align) = field_size_align(&binding.ty, ptr_ty);
            offset = align_u32(offset, align) + size;
        }
    }
    let (_, align) = field_size_align(&binding.ty, ptr_ty);
    Some(align_u32(offset, align))
}

fn simple_async_frame_local_offset(
    plan: &SimpleAsyncStateMachinePlan,
    local_index: usize,
    ptr_ty: types::Type,
) -> Option<u32> {
    let local = plan.locals.get(local_index)?;
    let mut offset = simple_async_frame_param_base_offset(plan, ptr_ty);
    for param in &plan.params {
        let (size, align) = field_size_align(&param.ty, ptr_ty);
        offset = align_u32(offset, align) + size;
    }
    for previous in plan.locals.iter().take(local_index) {
        let (size, align) = field_size_align(&previous.ty, ptr_ty);
        offset = align_u32(offset, align) + size;
    }
    let (_, align) = field_size_align(&local.ty, ptr_ty);
    Some(align_u32(offset, align))
}

fn simple_async_frame_param_base_offset(
    plan: &SimpleAsyncStateMachinePlan,
    ptr_ty: types::Type,
) -> u32 {
    ASYNC_FRAME_AWAITED_BASE_OFFSET + (plan.awaits.len() as u32 * ptr_ty.bytes() as u32)
}

fn simple_async_frame_param_offset(
    plan: &SimpleAsyncStateMachinePlan,
    param_index: usize,
    ptr_ty: types::Type,
) -> Option<u32> {
    let param = plan.params.get(param_index)?;
    let mut offset = simple_async_frame_param_base_offset(plan, ptr_ty);
    for previous in plan.params.iter().take(param_index) {
        let (size, align) = field_size_align(&previous.ty, ptr_ty);
        offset = align_u32(offset, align) + size;
    }
    let (_, align) = field_size_align(&param.ty, ptr_ty);
    Some(align_u32(offset, align))
}

fn simple_async_frame_size(plan: &SimpleAsyncStateMachinePlan, ptr_ty: types::Type) -> u32 {
    let mut offset = simple_async_frame_param_base_offset(plan, ptr_ty);
    let mut max_align = ptr_ty.bytes().min(8).max(1) as u8;
    for param in &plan.params {
        let (size, align) = field_size_align(&param.ty, ptr_ty);
        offset = align_u32(offset, align) + size;
        max_align = max_align.max(align);
    }
    for local in &plan.locals {
        let (size, align) = field_size_align(&local.ty, ptr_ty);
        offset = align_u32(offset, align) + size;
        max_align = max_align.max(align);
    }
    for step in &plan.awaits {
        if let Some(binding) = &step.binding {
            let (size, align) = field_size_align(&binding.ty, ptr_ty);
            offset = align_u32(offset, align) + size;
            max_align = max_align.max(align);
        }
    }
    align_u32(offset, max_align).max(ASYNC_FRAME_AWAITED_BASE_OFFSET + ptr_ty.bytes() as u32)
}

fn simple_async_frame_awaited_offset(step_index: usize, ptr_ty: types::Type) -> i32 {
    (ASYNC_FRAME_AWAITED_BASE_OFFSET + (step_index as u32 * ptr_ty.bytes() as u32)) as i32
}

fn expr_contains_await(expr: &HirExpr) -> bool {
    match &expr.kind {
        HirExprKind::Await(_) => true,
        HirExprKind::Unary { operand, .. } => expr_contains_await(operand),
        HirExprKind::Binary { lhs, rhs, .. } => {
            expr_contains_await(lhs) || expr_contains_await(rhs)
        }
        HirExprKind::Call { callee, args } => {
            expr_contains_await(callee) || args.iter().any(|arg| expr_contains_await(&arg.value))
        }
        HirExprKind::MethodCall { receiver, args, .. } => {
            expr_contains_await(receiver) || args.iter().any(expr_contains_await)
        }
        HirExprKind::Field { object, .. } | HirExprKind::TupleIndex { object, .. } => {
            expr_contains_await(object)
        }
        HirExprKind::Index { object, index } => {
            expr_contains_await(object) || expr_contains_await(index)
        }
        HirExprKind::ListLit { elements, .. }
        | HirExprKind::TupleLit(elements)
        | HirExprKind::SetLit { elements, .. } => elements.iter().any(expr_contains_await),
        HirExprKind::ListSpreadLit { elements, .. } => elements
            .iter()
            .any(|element| expr_contains_await(&element.value)),
        HirExprKind::MapLit { entries, .. } => entries
            .iter()
            .any(|(key, value)| expr_contains_await(key) || expr_contains_await(value)),
        HirExprKind::StructLit { fields, .. } => {
            fields.iter().any(|(_, value)| expr_contains_await(value))
        }
        HirExprKind::StructUpdate { base, updates, .. } => {
            expr_contains_await(base) || updates.iter().any(|(_, value)| expr_contains_await(value))
        }
        HirExprKind::EnumVariant { fields, .. } => {
            fields.iter().any(|(_, value)| expr_contains_await(value))
        }
        HirExprKind::Some_(inner) | HirExprKind::Ok_(inner) | HirExprKind::Err_(inner) => {
            expr_contains_await(inner)
        }
        HirExprKind::IfExpr { cond, then, else_ } => {
            expr_contains_await(cond) || expr_contains_await(then) || expr_contains_await(else_)
        }
        HirExprKind::Range { start, end, .. } => {
            expr_contains_await(start) || expr_contains_await(end)
        }
        HirExprKind::Propagate(inner) => expr_contains_await(inner),
        HirExprKind::InterpolatedStr(parts) => parts.iter().any(|part| match part {
            HirStrPart::Literal(_) => false,
            HirStrPart::Expr(expr) => expr_contains_await(expr),
        }),
        HirExprKind::IntLit(_)
        | HirExprKind::FloatLit(_)
        | HirExprKind::BoolLit(_)
        | HirExprKind::StrLit(_)
        | HirExprKind::BytesLit(_)
        | HirExprKind::Unit
        | HirExprKind::None_
        | HirExprKind::Var(_)
        | HirExprKind::Closure { .. }
        | HirExprKind::IsCheck { .. } => false,
    }
}

fn block_contains_await(block: &HirBlock) -> bool {
    block.stmts.iter().any(stmt_contains_await)
}

fn arm_body_contains_await(stmts: &[HirStmt]) -> bool {
    stmts.iter().any(stmt_contains_await)
}

fn stmt_contains_await(stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Let { value, .. } | HirStmt::Using { value, .. } => expr_contains_await(value),
        HirStmt::Assign { value, .. } => expr_contains_await(value),
        HirStmt::Return(Some(value), _) | HirStmt::Expr(value) => expr_contains_await(value),
        HirStmt::Return(None, _) | HirStmt::Break(_) | HirStmt::Continue(_) => false,
        HirStmt::If {
            cond,
            then,
            else_ifs,
            else_,
            ..
        } => {
            expr_contains_await(cond)
                || block_contains_await(then)
                || else_ifs
                    .iter()
                    .any(|(cond, block)| expr_contains_await(cond) || block_contains_await(block))
                || else_.as_ref().is_some_and(block_contains_await)
        }
        HirStmt::While { cond, body, .. } => {
            expr_contains_await(cond) || block_contains_await(body)
        }
        HirStmt::For { iterable, body, .. } => {
            expr_contains_await(iterable) || block_contains_await(body)
        }
        HirStmt::Loop { body, .. } => block_contains_await(body),
        HirStmt::Repeat { count, body, .. } => {
            expr_contains_await(count) || block_contains_await(body)
        }
        HirStmt::Match {
            scrutinee, arms, ..
        } => {
            expr_contains_await(scrutinee)
                || arms.iter().any(|arm| arm_body_contains_await(&arm.body))
        }
        HirStmt::IfSome {
            value, then, else_, ..
        } => {
            expr_contains_await(value)
                || block_contains_await(then)
                || else_.as_ref().is_some_and(block_contains_await)
        }
        HirStmt::WhileSome { value, body, .. } => {
            expr_contains_await(value) || block_contains_await(body)
        }
        HirStmt::Check { condition, .. } => expr_contains_await(condition),
    }
}

fn block_contains_return(block: &HirBlock) -> bool {
    block.stmts.iter().any(stmt_contains_return)
}

fn arm_body_contains_return(stmts: &[HirStmt]) -> bool {
    stmts.iter().any(stmt_contains_return)
}

fn stmt_contains_return(stmt: &HirStmt) -> bool {
    match stmt {
        HirStmt::Return(_, _) => true,
        HirStmt::Let { .. }
        | HirStmt::Assign { .. }
        | HirStmt::Break(_)
        | HirStmt::Continue(_)
        | HirStmt::Expr(_)
        | HirStmt::Using { .. }
        | HirStmt::Check { .. } => false,
        HirStmt::If {
            then,
            else_ifs,
            else_,
            ..
        } => {
            block_contains_return(then)
                || else_ifs
                    .iter()
                    .any(|(_, block)| block_contains_return(block))
                || else_.as_ref().is_some_and(block_contains_return)
        }
        HirStmt::While { body, .. }
        | HirStmt::For { body, .. }
        | HirStmt::Loop { body, .. }
        | HirStmt::Repeat { body, .. }
        | HirStmt::WhileSome { body, .. } => block_contains_return(body),
        HirStmt::Match { arms, .. } => arms.iter().any(|arm| arm_body_contains_return(&arm.body)),
        HirStmt::IfSome { then, else_, .. } => {
            block_contains_return(then) || else_.as_ref().is_some_and(block_contains_return)
        }
    }
}

fn expr_collect_var_uses(expr: &HirExpr, uses: &mut HashSet<SmolStr>) {
    match &expr.kind {
        HirExprKind::Var(name) => {
            uses.insert(name.clone());
        }
        HirExprKind::Unary { operand, .. }
        | HirExprKind::Some_(operand)
        | HirExprKind::Ok_(operand)
        | HirExprKind::Err_(operand)
        | HirExprKind::Propagate(operand)
        | HirExprKind::Await(operand) => expr_collect_var_uses(operand, uses),
        HirExprKind::Binary { lhs, rhs, .. } => {
            expr_collect_var_uses(lhs, uses);
            expr_collect_var_uses(rhs, uses);
        }
        HirExprKind::Field { object, .. } | HirExprKind::TupleIndex { object, .. } => {
            expr_collect_var_uses(object, uses);
        }
        HirExprKind::Index { object, index } => {
            expr_collect_var_uses(object, uses);
            expr_collect_var_uses(index, uses);
        }
        HirExprKind::Call { callee, args } => {
            expr_collect_var_uses(callee, uses);
            for arg in args {
                expr_collect_var_uses(&arg.value, uses);
            }
        }
        HirExprKind::MethodCall { receiver, args, .. } => {
            expr_collect_var_uses(receiver, uses);
            for arg in args {
                expr_collect_var_uses(arg, uses);
            }
        }
        HirExprKind::StructLit { fields, .. } | HirExprKind::EnumVariant { fields, .. } => {
            for (_, value) in fields {
                expr_collect_var_uses(value, uses);
            }
        }
        HirExprKind::ListLit { elements, .. }
        | HirExprKind::TupleLit(elements)
        | HirExprKind::SetLit { elements, .. } => {
            for element in elements {
                expr_collect_var_uses(element, uses);
            }
        }
        HirExprKind::ListSpreadLit { elements, .. } => {
            for element in elements {
                expr_collect_var_uses(&element.value, uses);
            }
        }
        HirExprKind::MapLit { entries, .. } => {
            for (key, value) in entries {
                expr_collect_var_uses(key, uses);
                expr_collect_var_uses(value, uses);
            }
        }
        HirExprKind::IfExpr { cond, then, else_ } => {
            expr_collect_var_uses(cond, uses);
            expr_collect_var_uses(then, uses);
            expr_collect_var_uses(else_, uses);
        }
        HirExprKind::Range { start, end, .. } => {
            expr_collect_var_uses(start, uses);
            expr_collect_var_uses(end, uses);
        }
        HirExprKind::StructUpdate { base, updates, .. } => {
            expr_collect_var_uses(base, uses);
            for (_, value) in updates {
                expr_collect_var_uses(value, uses);
            }
        }
        HirExprKind::InterpolatedStr(parts) => {
            for part in parts {
                if let HirStrPart::Expr(expr) = part {
                    expr_collect_var_uses(expr, uses);
                }
            }
        }
        HirExprKind::Closure { captures, .. } => {
            for capture in captures {
                uses.insert(capture.name.clone());
            }
        }
        HirExprKind::IsCheck { value, .. } => expr_collect_var_uses(value, uses),
        HirExprKind::IntLit(_)
        | HirExprKind::FloatLit(_)
        | HirExprKind::BoolLit(_)
        | HirExprKind::StrLit(_)
        | HirExprKind::BytesLit(_)
        | HirExprKind::Unit
        | HirExprKind::None_ => {}
    }
}

fn lvalue_collect_var_uses(lvalue: &HirLValue, uses: &mut HashSet<SmolStr>) {
    match lvalue {
        HirLValue::Var(name) => {
            uses.insert(name.clone());
        }
        HirLValue::Field { base, .. } => lvalue_collect_var_uses(base, uses),
        HirLValue::Index { base, index } => {
            lvalue_collect_var_uses(base, uses);
            expr_collect_var_uses(index, uses);
        }
    }
}

fn stmt_collect_var_uses(stmt: &HirStmt, uses: &mut HashSet<SmolStr>) {
    match stmt {
        HirStmt::Let { value, .. } | HirStmt::Using { value, .. } => {
            expr_collect_var_uses(value, uses);
        }
        HirStmt::Assign { lvalue, value, .. } => {
            lvalue_collect_var_uses(lvalue, uses);
            expr_collect_var_uses(value, uses);
        }
        HirStmt::Return(Some(value), _) | HirStmt::Expr(value) => {
            expr_collect_var_uses(value, uses)
        }
        HirStmt::Return(None, _) | HirStmt::Break(_) | HirStmt::Continue(_) => {}
        HirStmt::If {
            cond,
            then,
            else_ifs,
            else_,
            ..
        } => {
            expr_collect_var_uses(cond, uses);
            for stmt in &then.stmts {
                stmt_collect_var_uses(stmt, uses);
            }
            for (cond, block) in else_ifs {
                expr_collect_var_uses(cond, uses);
                for stmt in &block.stmts {
                    stmt_collect_var_uses(stmt, uses);
                }
            }
            if let Some(block) = else_ {
                for stmt in &block.stmts {
                    stmt_collect_var_uses(stmt, uses);
                }
            }
        }
        HirStmt::While { cond, body, .. } => {
            expr_collect_var_uses(cond, uses);
            for stmt in &body.stmts {
                stmt_collect_var_uses(stmt, uses);
            }
        }
        HirStmt::For { iterable, body, .. } => {
            expr_collect_var_uses(iterable, uses);
            for stmt in &body.stmts {
                stmt_collect_var_uses(stmt, uses);
            }
        }
        HirStmt::Loop { body, .. } => {
            for stmt in &body.stmts {
                stmt_collect_var_uses(stmt, uses);
            }
        }
        HirStmt::Repeat { count, body, .. } => {
            expr_collect_var_uses(count, uses);
            for stmt in &body.stmts {
                stmt_collect_var_uses(stmt, uses);
            }
        }
        HirStmt::Match {
            scrutinee, arms, ..
        } => {
            expr_collect_var_uses(scrutinee, uses);
            for arm in arms {
                for stmt in &arm.body {
                    stmt_collect_var_uses(stmt, uses);
                }
            }
        }
        HirStmt::IfSome {
            value, then, else_, ..
        } => {
            expr_collect_var_uses(value, uses);
            for stmt in &then.stmts {
                stmt_collect_var_uses(stmt, uses);
            }
            if let Some(block) = else_ {
                for stmt in &block.stmts {
                    stmt_collect_var_uses(stmt, uses);
                }
            }
        }
        HirStmt::WhileSome { value, body, .. } => {
            expr_collect_var_uses(value, uses);
            for stmt in &body.stmts {
                stmt_collect_var_uses(stmt, uses);
            }
        }
        HirStmt::Check { condition, .. } => expr_collect_var_uses(condition, uses),
    }
}

fn simple_async_uses_after_await(
    plan: &SimpleAsyncStateMachinePlan,
    await_index: usize,
) -> HashSet<SmolStr> {
    let mut uses = HashSet::new();
    for step in plan.awaits.iter().skip(await_index + 1) {
        expr_collect_var_uses(&step.await_future, &mut uses);
    }
    for stmt in &plan.tail_stmts {
        stmt_collect_var_uses(stmt, &mut uses);
    }
    if let Some(expr) = &plan.return_expr {
        expr_collect_var_uses(expr, &mut uses);
    }
    if let Some(expr) = &plan.tail_expr {
        expr_collect_var_uses(expr, &mut uses);
    }
    uses
}

#[cfg(test)]
fn simple_async_name_used_after_await(
    plan: &SimpleAsyncStateMachinePlan,
    name: &SmolStr,
    await_index: usize,
) -> bool {
    simple_async_uses_after_await(plan, await_index).contains(name)
}

fn simple_async_lift_expr_awaits(
    expr: &HirExpr,
    awaits: &mut Vec<SimpleAsyncAwaitStep>,
    first_index: usize,
) -> Option<HirExpr> {
    match &expr.kind {
        HirExprKind::Await(await_future) => {
            cl_type(&expr.ty, types::I64)?;
            let Ty::Future(await_ty) = &await_future.ty else {
                return None;
            };
            if await_ty.as_ref() != &expr.ty {
                return None;
            }
            let name = SmolStr::new(format!(
                ".__async_expr_await_{}",
                first_index + awaits.len()
            ));
            awaits.push(SimpleAsyncAwaitStep {
                await_future: await_future.as_ref().clone(),
                binding: Some(SimpleAsyncBinding {
                    name: name.clone(),
                    ty: expr.ty.clone(),
                }),
                propagate_result_ty: None,
            });
            Some(HirExpr {
                kind: HirExprKind::Var(name),
                ty: expr.ty.clone(),
                span: expr.span,
            })
        }
        HirExprKind::Binary { op, lhs, rhs } => Some(HirExpr {
            kind: HirExprKind::Binary {
                op: *op,
                lhs: Box::new(simple_async_lift_expr_awaits(lhs, awaits, first_index)?),
                rhs: Box::new(simple_async_lift_expr_awaits(rhs, awaits, first_index)?),
            },
            ty: expr.ty.clone(),
            span: expr.span,
        }),
        HirExprKind::Unary { op, operand } => Some(HirExpr {
            kind: HirExprKind::Unary {
                op: *op,
                operand: Box::new(simple_async_lift_expr_awaits(operand, awaits, first_index)?),
            },
            ty: expr.ty.clone(),
            span: expr.span,
        }),
        HirExprKind::Field { object, field } => Some(HirExpr {
            kind: HirExprKind::Field {
                object: Box::new(simple_async_lift_expr_awaits(object, awaits, first_index)?),
                field: field.clone(),
            },
            ty: expr.ty.clone(),
            span: expr.span,
        }),
        HirExprKind::Index { object, index } => Some(HirExpr {
            kind: HirExprKind::Index {
                object: Box::new(simple_async_lift_expr_awaits(object, awaits, first_index)?),
                index: Box::new(simple_async_lift_expr_awaits(index, awaits, first_index)?),
            },
            ty: expr.ty.clone(),
            span: expr.span,
        }),
        HirExprKind::TupleIndex { object, index } => Some(HirExpr {
            kind: HirExprKind::TupleIndex {
                object: Box::new(simple_async_lift_expr_awaits(object, awaits, first_index)?),
                index: *index,
            },
            ty: expr.ty.clone(),
            span: expr.span,
        }),
        HirExprKind::Call { callee, args } => Some(HirExpr {
            kind: HirExprKind::Call {
                callee: Box::new(simple_async_lift_expr_awaits(callee, awaits, first_index)?),
                args: args
                    .iter()
                    .map(|arg| {
                        Some(HirArg {
                            label: arg.label.clone(),
                            spread: arg.spread,
                            value: simple_async_lift_expr_awaits(&arg.value, awaits, first_index)?,
                        })
                    })
                    .collect::<Option<Vec<_>>>()?,
            },
            ty: expr.ty.clone(),
            span: expr.span,
        }),
        HirExprKind::MethodCall {
            receiver,
            method,
            args,
        } => Some(HirExpr {
            kind: HirExprKind::MethodCall {
                receiver: Box::new(simple_async_lift_expr_awaits(
                    receiver,
                    awaits,
                    first_index,
                )?),
                method: method.clone(),
                args: args
                    .iter()
                    .map(|arg| simple_async_lift_expr_awaits(arg, awaits, first_index))
                    .collect::<Option<Vec<_>>>()?,
            },
            ty: expr.ty.clone(),
            span: expr.span,
        }),
        HirExprKind::StructLit { def_id, fields } => Some(HirExpr {
            kind: HirExprKind::StructLit {
                def_id: *def_id,
                fields: fields
                    .iter()
                    .map(|(name, value)| {
                        Some((
                            name.clone(),
                            simple_async_lift_expr_awaits(value, awaits, first_index)?,
                        ))
                    })
                    .collect::<Option<Vec<_>>>()?,
            },
            ty: expr.ty.clone(),
            span: expr.span,
        }),
        HirExprKind::EnumVariant {
            def_id,
            variant,
            fields,
        } => Some(HirExpr {
            kind: HirExprKind::EnumVariant {
                def_id: *def_id,
                variant: variant.clone(),
                fields: fields
                    .iter()
                    .map(|(name, value)| {
                        Some((
                            name.clone(),
                            simple_async_lift_expr_awaits(value, awaits, first_index)?,
                        ))
                    })
                    .collect::<Option<Vec<_>>>()?,
            },
            ty: expr.ty.clone(),
            span: expr.span,
        }),
        HirExprKind::ListLit { elem_ty, elements } => Some(HirExpr {
            kind: HirExprKind::ListLit {
                elem_ty: elem_ty.clone(),
                elements: elements
                    .iter()
                    .map(|element| simple_async_lift_expr_awaits(element, awaits, first_index))
                    .collect::<Option<Vec<_>>>()?,
            },
            ty: expr.ty.clone(),
            span: expr.span,
        }),
        HirExprKind::ListSpreadLit { elem_ty, elements } => Some(HirExpr {
            kind: HirExprKind::ListSpreadLit {
                elem_ty: elem_ty.clone(),
                elements: elements
                    .iter()
                    .map(|element| {
                        Some(HirListElement {
                            spread: element.spread,
                            value: simple_async_lift_expr_awaits(
                                &element.value,
                                awaits,
                                first_index,
                            )?,
                        })
                    })
                    .collect::<Option<Vec<_>>>()?,
            },
            ty: expr.ty.clone(),
            span: expr.span,
        }),
        HirExprKind::TupleLit(elements) => Some(HirExpr {
            kind: HirExprKind::TupleLit(
                elements
                    .iter()
                    .map(|element| simple_async_lift_expr_awaits(element, awaits, first_index))
                    .collect::<Option<Vec<_>>>()?,
            ),
            ty: expr.ty.clone(),
            span: expr.span,
        }),
        HirExprKind::Some_(inner) => Some(HirExpr {
            kind: HirExprKind::Some_(Box::new(simple_async_lift_expr_awaits(
                inner,
                awaits,
                first_index,
            )?)),
            ty: expr.ty.clone(),
            span: expr.span,
        }),
        HirExprKind::Ok_(inner) => Some(HirExpr {
            kind: HirExprKind::Ok_(Box::new(simple_async_lift_expr_awaits(
                inner,
                awaits,
                first_index,
            )?)),
            ty: expr.ty.clone(),
            span: expr.span,
        }),
        HirExprKind::Err_(inner) => Some(HirExpr {
            kind: HirExprKind::Err_(Box::new(simple_async_lift_expr_awaits(
                inner,
                awaits,
                first_index,
            )?)),
            ty: expr.ty.clone(),
            span: expr.span,
        }),
        HirExprKind::Propagate(inner) => {
            if expr_contains_await(inner) {
                None
            } else {
                Some(expr.clone())
            }
        }
        HirExprKind::IfExpr { cond, then, else_ } => {
            if expr_contains_await(then) || expr_contains_await(else_) {
                return None;
            }
            Some(HirExpr {
                kind: HirExprKind::IfExpr {
                    cond: Box::new(simple_async_lift_expr_awaits(cond, awaits, first_index)?),
                    then: then.clone(),
                    else_: else_.clone(),
                },
                ty: expr.ty.clone(),
                span: expr.span,
            })
        }
        HirExprKind::Range { start, end } => Some(HirExpr {
            kind: HirExprKind::Range {
                start: Box::new(simple_async_lift_expr_awaits(start, awaits, first_index)?),
                end: Box::new(simple_async_lift_expr_awaits(end, awaits, first_index)?),
            },
            ty: expr.ty.clone(),
            span: expr.span,
        }),
        HirExprKind::MapLit {
            key_ty,
            value_ty,
            entries,
        } => Some(HirExpr {
            kind: HirExprKind::MapLit {
                key_ty: key_ty.clone(),
                value_ty: value_ty.clone(),
                entries: entries
                    .iter()
                    .map(|(key, value)| {
                        Some((
                            simple_async_lift_expr_awaits(key, awaits, first_index)?,
                            simple_async_lift_expr_awaits(value, awaits, first_index)?,
                        ))
                    })
                    .collect::<Option<Vec<_>>>()?,
            },
            ty: expr.ty.clone(),
            span: expr.span,
        }),
        HirExprKind::SetLit { elem_ty, elements } => Some(HirExpr {
            kind: HirExprKind::SetLit {
                elem_ty: elem_ty.clone(),
                elements: elements
                    .iter()
                    .map(|element| simple_async_lift_expr_awaits(element, awaits, first_index))
                    .collect::<Option<Vec<_>>>()?,
            },
            ty: expr.ty.clone(),
            span: expr.span,
        }),
        HirExprKind::StructUpdate {
            def_id,
            base,
            updates,
        } => Some(HirExpr {
            kind: HirExprKind::StructUpdate {
                def_id: *def_id,
                base: Box::new(simple_async_lift_expr_awaits(base, awaits, first_index)?),
                updates: updates
                    .iter()
                    .map(|(name, value)| {
                        Some((
                            name.clone(),
                            simple_async_lift_expr_awaits(value, awaits, first_index)?,
                        ))
                    })
                    .collect::<Option<Vec<_>>>()?,
            },
            ty: expr.ty.clone(),
            span: expr.span,
        }),
        HirExprKind::InterpolatedStr(parts) => Some(HirExpr {
            kind: HirExprKind::InterpolatedStr(
                parts
                    .iter()
                    .map(|part| match part {
                        HirStrPart::Literal(text) => Some(HirStrPart::Literal(text.clone())),
                        HirStrPart::Expr(expr) => Some(HirStrPart::Expr(
                            simple_async_lift_expr_awaits(expr, awaits, first_index)?,
                        )),
                    })
                    .collect::<Option<Vec<_>>>()?,
            ),
            ty: expr.ty.clone(),
            span: expr.span,
        }),
        HirExprKind::IsCheck { value, check_ty } => Some(HirExpr {
            kind: HirExprKind::IsCheck {
                value: Box::new(simple_async_lift_expr_awaits(value, awaits, first_index)?),
                check_ty: check_ty.clone(),
            },
            ty: expr.ty.clone(),
            span: expr.span,
        }),
        HirExprKind::BoolLit(_)
        | HirExprKind::IntLit(_)
        | HirExprKind::FloatLit(_)
        | HirExprKind::StrLit(_)
        | HirExprKind::BytesLit(_)
        | HirExprKind::Unit
        | HirExprKind::None_
        | HirExprKind::Var(_)
        | HirExprKind::Closure { .. } => Some(expr.clone()),
    }
}

fn simple_async_lift_stmt_awaits(
    stmt: &HirStmt,
    first_index: usize,
) -> Option<(HirStmt, Vec<SimpleAsyncAwaitStep>)> {
    let mut awaits = Vec::new();
    let lifted = match stmt {
        HirStmt::Let {
            name,
            ty,
            mutable,
            value,
            span,
        } => HirStmt::Let {
            name: name.clone(),
            ty: ty.clone(),
            mutable: *mutable,
            value: simple_async_lift_expr_awaits(value, &mut awaits, first_index)?,
            span: *span,
        },
        HirStmt::Assign {
            lvalue,
            value,
            span,
        } => HirStmt::Assign {
            lvalue: lvalue.clone(),
            value: simple_async_lift_expr_awaits(value, &mut awaits, first_index)?,
            span: *span,
        },
        HirStmt::Return(Some(expr), span) => HirStmt::Return(
            Some(simple_async_lift_expr_awaits(
                expr,
                &mut awaits,
                first_index,
            )?),
            *span,
        ),
        HirStmt::Expr(expr) => HirStmt::Expr(simple_async_lift_expr_awaits(
            expr,
            &mut awaits,
            first_index,
        )?),
        HirStmt::If {
            cond,
            then,
            else_ifs,
            else_,
            span,
        } => {
            if block_contains_await(then)
                || else_ifs
                    .iter()
                    .any(|(_, block)| block_contains_await(block))
                || else_.as_ref().is_some_and(block_contains_await)
            {
                return None;
            }
            HirStmt::If {
                cond: simple_async_lift_expr_awaits(cond, &mut awaits, first_index)?,
                then: then.clone(),
                else_ifs: else_ifs
                    .iter()
                    .map(|(cond, block)| {
                        Some((
                            simple_async_lift_expr_awaits(cond, &mut awaits, first_index)?,
                            block.clone(),
                        ))
                    })
                    .collect::<Option<Vec<_>>>()?,
                else_: else_.clone(),
                span: *span,
            }
        }
        HirStmt::IfSome {
            binding,
            inner_ty,
            value,
            then,
            else_,
            span,
        } => {
            if block_contains_await(then) || else_.as_ref().is_some_and(block_contains_await) {
                return None;
            }
            HirStmt::IfSome {
                binding: binding.clone(),
                inner_ty: inner_ty.clone(),
                value: simple_async_lift_expr_awaits(value, &mut awaits, first_index)?,
                then: then.clone(),
                else_: else_.clone(),
                span: *span,
            }
        }
        HirStmt::Match {
            scrutinee,
            arms,
            span,
        } => {
            if arms.iter().any(|arm| arm_body_contains_await(&arm.body)) {
                return None;
            }
            HirStmt::Match {
                scrutinee: simple_async_lift_expr_awaits(scrutinee, &mut awaits, first_index)?,
                arms: arms.clone(),
                span: *span,
            }
        }
        HirStmt::Check {
            condition,
            message,
            span,
        } => HirStmt::Check {
            condition: simple_async_lift_expr_awaits(condition, &mut awaits, first_index)?,
            message: message.clone(),
            span: *span,
        },
        HirStmt::For {
            binding,
            index_binding,
            elem_ty,
            iterable,
            body,
            span,
        } => {
            if block_contains_await(body) {
                return None;
            }
            HirStmt::For {
                binding: binding.clone(),
                index_binding: index_binding.clone(),
                elem_ty: elem_ty.clone(),
                iterable: simple_async_lift_expr_awaits(iterable, &mut awaits, first_index)?,
                body: body.clone(),
                span: *span,
            }
        }
        HirStmt::Return(None, _)
        | HirStmt::Break(_)
        | HirStmt::Continue(_)
        | HirStmt::While { .. }
        | HirStmt::Loop { .. }
        | HirStmt::Repeat { .. }
        | HirStmt::WhileSome { .. }
        | HirStmt::Using { .. } => return None,
    };
    if awaits.is_empty() {
        return None;
    }
    Some((lifted, awaits))
}

fn simple_async_state_machine_plan(f: &HirFunc) -> Option<SimpleAsyncStateMachinePlan> {
    if f.body.stmts.is_empty() {
        return None;
    }
    let mut params = Vec::with_capacity(f.params.len());
    for param in &f.params {
        cl_type(&param.ty, types::I64)?;
        params.push(SimpleAsyncParam {
            name: param.name.clone(),
            ty: param.ty.clone(),
        });
    }
    let inner_ty = async_inner_return_ty(f)?;
    let mut locals = Vec::new();
    let mut awaits = Vec::new();
    let mut tail_stmts = Vec::new();
    let mut return_expr = None;
    let mut tail_expr = None;
    let mut saw_await = false;
    let mut terminal_return_await = false;

    for stmt in &f.body.stmts {
        if terminal_return_await {
            return None;
        }

        let parsed_await = match stmt {
            HirStmt::Expr(await_expr) => match &await_expr.kind {
                HirExprKind::Await(await_future) => {
                    Some((await_future.as_ref().clone(), None, None, None::<HirExpr>))
                }
                _ => None,
            },
            HirStmt::Let {
                name, ty, value, ..
            } => {
                if let HirExprKind::Await(await_future) = &value.kind {
                    cl_type(ty, types::I64)?;
                    Some((
                        await_future.as_ref().clone(),
                        Some(SimpleAsyncBinding {
                            name: name.clone(),
                            ty: ty.clone(),
                        }),
                        None,
                        None,
                    ))
                } else if let HirExprKind::Propagate(inner) = &value.kind {
                    let HirExprKind::Await(await_future) = &inner.kind else {
                        return None;
                    };
                    cl_type(ty, types::I64)?;
                    let Ty::Future(await_result_ty) = &await_future.ty else {
                        return None;
                    };
                    let Ty::Result(ok_ty, _) = await_result_ty.as_ref() else {
                        return None;
                    };
                    if ok_ty.as_ref() != ty || await_result_ty.as_ref() != &inner_ty {
                        return None;
                    }
                    Some((
                        await_future.as_ref().clone(),
                        Some(SimpleAsyncBinding {
                            name: name.clone(),
                            ty: ty.clone(),
                        }),
                        Some(await_result_ty.as_ref().clone()),
                        None,
                    ))
                } else {
                    None
                }
            }
            HirStmt::Return(Some(expr), _) if matches!(expr.kind, HirExprKind::Await(_)) => {
                let HirExprKind::Await(await_future) = &expr.kind else {
                    unreachable!("guarded by matches")
                };
                let binding_name = SmolStr::new(".__async_return_value");
                let return_value = HirExpr {
                    kind: HirExprKind::Var(binding_name.clone()),
                    ty: expr.ty.clone(),
                    span: expr.span,
                };
                Some((
                    await_future.as_ref().clone(),
                    Some(SimpleAsyncBinding {
                        name: binding_name,
                        ty: expr.ty.clone(),
                    }),
                    None,
                    Some(return_value),
                ))
            }
            _ => None,
        };

        if let Some((await_future, binding, propagate_result_ty, return_value)) = parsed_await {
            if !tail_stmts.is_empty() {
                return None;
            }
            saw_await = true;
            awaits.push(SimpleAsyncAwaitStep {
                await_future,
                binding,
                propagate_result_ty,
            });
            if let Some(return_value) = return_value {
                return_expr = Some(return_value);
                terminal_return_await = true;
            }
            continue;
        }

        if stmt_contains_await(stmt) {
            if !tail_stmts.is_empty() {
                return None;
            }
            let (lifted_stmt, lifted_awaits) = simple_async_lift_stmt_awaits(stmt, awaits.len())?;
            saw_await = true;
            awaits.extend(lifted_awaits);
            tail_stmts.push(lifted_stmt);
            continue;
        }

        if !saw_await {
            let HirStmt::Let {
                name, ty, value, ..
            } = stmt
            else {
                return None;
            };
            cl_type(ty, types::I64)?;
            locals.push(SimpleAsyncLocal {
                name: name.clone(),
                ty: ty.clone(),
                value: value.clone(),
            });
        } else {
            tail_stmts.push(stmt.clone());
        }
    }

    if awaits.is_empty() {
        return None;
    }
    if !terminal_return_await {
        match tail_stmts.pop() {
            Some(HirStmt::Return(expr, _)) => {
                if expr.as_ref().is_some_and(expr_contains_await) {
                    return None;
                }
                return_expr = expr;
            }
            Some(HirStmt::Expr(expr)) if matches!(inner_ty, Ty::Void | Ty::Never) => {
                if expr_contains_await(&expr) {
                    return None;
                }
                tail_expr = Some(expr);
            }
            Some(stmt) if matches!(inner_ty, Ty::Void | Ty::Never) => {
                if stmt_contains_return(&stmt) {
                    return None;
                }
                tail_stmts.push(stmt);
                tail_expr = Some(HirExpr {
                    kind: HirExprKind::Unit,
                    ty: Ty::Void,
                    span: f.span,
                });
            }
            None if matches!(inner_ty, Ty::Void | Ty::Never) => {
                tail_expr = Some(HirExpr {
                    kind: HirExprKind::Unit,
                    ty: Ty::Void,
                    span: f.span,
                });
            }
            _ => return None,
        }
    }
    if tail_stmts.iter().any(stmt_contains_return) {
        return None;
    }
    Some(SimpleAsyncStateMachinePlan {
        params,
        locals,
        awaits,
        tail_stmts,
        return_expr,
        tail_expr,
        inner_ty,
    })
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

/// Layout of `lazy<T>`: `{ thunk: ptr, forced: i8, [padding], value: T }`.
fn lazy_layout(inner: &Ty, ptr_ty: types::Type) -> (u32, u32) {
    let ptr_size = ptr_ty.bytes() as u32;
    let (val_size, val_align) = field_size_align(inner, ptr_ty);
    let val_offset = (ptr_size + 1 + val_align as u32 - 1) & !(val_align as u32 - 1);
    let max_align = (ptr_ty.bytes() as u8).max(val_align).max(1);
    let total = ((val_offset + val_size + max_align as u32 - 1) & !(max_align as u32 - 1))
        .max(ptr_size + 1);
    (val_offset, total)
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
        validate_native_hir(hir)?;
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
            let Some(cl_ty) = cl_type(&c.ty, self.ptr_ty) else {
                continue;
            };
            let static_bytes = const_static_bytes(&c.value, &c.ty);
            let runtime_init = static_bytes.is_none();
            let bytes = static_bytes.unwrap_or_else(|| {
                let size = cl_ty.bytes().max(1) as usize;
                vec![0; size]
            });
            let mut desc = DataDescription::new();
            desc.define(bytes.into_boxed_slice());
            let link = if c.is_public {
                Linkage::Export
            } else {
                Linkage::Local
            };
            let writable = c.mutable || runtime_init;
            let id = self
                .module
                .declare_data(&native_global_symbol(&c.name), link, writable, false)
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
        let mut declared_imports = HashMap::new();
        let mut decl = |name: &'static str,
                        params: &[types::Type],
                        params_ty: Vec<Ty>,
                        ret: Option<types::Type>|
         -> Result<FuncId, String> {
            if let Some(existing) = declared_imports.get(name).copied() {
                return Ok(existing);
            }
            let mut sig = self.module.make_signature();
            for &p in params {
                sig.params.push(AbiParam::new(p));
            }
            if let Some(r) = ret {
                sig.returns.push(AbiParam::new(r));
            }
            self.func_param_tys.insert(SmolStr::new(name), params_ty);
            let id = self
                .module
                .declare_function(name, Linkage::Import, &sig)
                .map_err(|e| format!("declare {name}: {e}"))?;
            declared_imports.insert(name, id);
            Ok(id)
        };
        let mut declared_manifest_symbols = std::collections::HashSet::new();
        for entry in stdlib_runtime_functions()
            .iter()
            .filter(|entry| entry.native_runtime)
        {
            if !declared_manifest_symbols.insert(entry.runtime_symbol) {
                continue;
            }
            let (abi_params, abi_ret) =
                stdlib_native_abi(entry.runtime_symbol).ok_or_else(|| {
                    format!(
                        "stdlib manifest entry `{}` is missing native ABI metadata",
                        entry.runtime_symbol
                    )
                })?;
            let cl_params: Vec<_> = abi_params
                .into_iter()
                .map(|ty| cl_stdlib_abi_type(ty, pt))
                .collect();
            let cl_ret = abi_ret.map(|ty| cl_stdlib_abi_type(ty, pt));
            let semantic_params = stdlib_func_sig(entry.canonical_path)
                .map(|(params, _)| params)
                .unwrap_or_default();
            let id = decl(entry.runtime_symbol, &cl_params, semantic_params, cl_ret)?;
            self.stdlib_ids
                .insert(SmolStr::new(entry.runtime_symbol), id);
        }
        // ori_io_print(ptr: *u8, len: i64) -- prints len bytes from ptr
        let id = decl(
            "ori_io_print",
            &[pt, types::I64],
            vec![Ty::String, Ty::Int64],
            None,
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_io_print"), id);
        // ori_io_eprint(ptr: *u8, len: i64) -- stderr print
        let id = decl(
            "ori_io_eprint",
            &[pt, types::I64],
            vec![Ty::String, Ty::Int64],
            None,
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_io_eprint"), id);
        let id = decl("ori_io_read_line", &[], vec![], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_io_read_line"), id);
        let id = decl("ori_future_ready_i64", &[types::I64], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_future_ready_i64"), id);
        let id = decl("ori_future_ready_f64", &[types::F64], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_future_ready_f64"), id);
        let id = decl("ori_future_ready_ptr", &[pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_future_ready_ptr"), id);
        let id = decl("ori_future_ready_void", &[], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_future_ready_void"), id);
        let id = decl("ori_future_pending", &[], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_future_pending"), id);
        let id = decl("ori_future_poll", &[pt], vec![], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_future_poll"), id);
        let id = decl("ori_future_value_i64", &[pt], vec![], Some(types::I64))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_future_value_i64"), id);
        let id = decl("ori_future_value_f64", &[pt], vec![], Some(types::F64))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_future_value_f64"), id);
        let id = decl("ori_future_value_ptr", &[pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_future_value_ptr"), id);
        let id = decl("ori_future_on_ready", &[pt, pt], vec![], None)?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_future_on_ready"), id);
        let id = decl("ori_future_complete_i64", &[pt, types::I64], vec![], None)?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_future_complete_i64"), id);
        let id = decl("ori_future_complete_f64", &[pt, types::F64], vec![], None)?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_future_complete_f64"), id);
        let id = decl("ori_future_complete_ptr", &[pt, pt], vec![], None)?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_future_complete_ptr"), id);
        let id = decl("ori_future_complete_void", &[pt], vec![], None)?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_future_complete_void"), id);
        let id = decl("ori_future_fail", &[pt], vec![], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_future_fail"), id);
        let id = decl("ori_future_cancel", &[pt], vec![], None)?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_future_cancel"), id);
        let id = decl("ori_executor_schedule", &[pt], vec![], None)?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_executor_schedule"), id);
        let id = decl("ori_executor_run_one", &[], vec![], Some(types::I64))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_executor_run_one"), id);
        let id = decl("ori_executor_drain", &[], vec![], Some(types::I64))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_executor_drain"), id);
        // Compatibility pointer return for stored `string(n)` values.
        let id = decl("ori_int_to_cstr", &[types::I64], vec![], Some(pt))?;
        self.stdlib_ids
            .entry(SmolStr::new("ori_to_string"))
            .or_insert(id);
        // Length-aware numeric conversion used by direct print/interpolation paths.
        let id = decl("ori_to_string_parts", &[types::I64, pt, pt], vec![], None)?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_to_string_parts"), id);
        // strlen(ptr: *u8) -> i64
        let id = decl("strlen", &[pt], vec![], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("strlen"), id);
        let id = decl("strcmp", &[pt, pt], vec![], Some(types::I32))?;
        self.stdlib_ids.insert(SmolStr::new("strcmp"), id);
        let id = decl("ori_string_len", &[pt], vec![], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_string_len"), id);
        let id = decl("ori_string_concat", &[pt, pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_concat"), id);
        let id = decl(
            "ori_string_concat_parts",
            &[pt, types::I64, pt, types::I64],
            vec![],
            Some(pt),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_concat_parts"), id);
        let id = decl("ori_string_split", &[pt, pt], vec![], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_string_split"), id);
        let id = decl(
            "ori_string_slice",
            &[pt, types::I64, types::I64],
            vec![],
            Some(pt),
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_string_slice"), id);
        let id = decl("ori_string_contains", &[pt, pt], vec![], Some(types::I8))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_contains"), id);
        let id = decl("ori_string_starts_with", &[pt, pt], vec![], Some(types::I8))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_starts_with"), id);
        let id = decl("ori_string_ends_with", &[pt, pt], vec![], Some(types::I8))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_ends_with"), id);
        let id = decl("ori_string_trim", &[pt], vec![], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_string_trim"), id);
        let id = decl("ori_string_trim_start", &[pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_trim_start"), id);
        let id = decl("ori_string_trim_end", &[pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_trim_end"), id);
        let id = decl("ori_string_to_upper", &[pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_to_upper"), id);
        let id = decl("ori_string_to_lower", &[pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_to_lower"), id);
        let id = decl("ori_string_replace", &[pt, pt, pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_replace"), id);
        let id = decl("ori_string_chars", &[pt], vec![], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_string_chars"), id);
        let id = decl("ori_string_index_of", &[pt, pt], vec![], Some(types::I64))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_index_of"), id);
        let id = decl("ori_string_join", &[pt, pt], vec![], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_string_join"), id);
        let id = decl("ori_string_repeat", &[pt, types::I64], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_repeat"), id);
        let id = decl(
            "ori_string_pad_left",
            &[pt, types::I64, pt],
            vec![],
            Some(pt),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_pad_left"), id);
        let id = decl(
            "ori_string_pad_right",
            &[pt, types::I64, pt],
            vec![],
            Some(pt),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_pad_right"), id);
        // ori_len(ptr: *u8) -> i64
        let id = decl("ori_len", &[pt], vec![], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_len"), id);
        // ori_math_abs(n: i64) -> i64
        let id = decl("ori_math_sqrt", &[types::F64], vec![], Some(types::F64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_sqrt"), id);
        let id = decl("ori_math_abs", &[types::I64], vec![], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_abs"), id);
        let id = decl(
            "ori_math_abs_float",
            &[types::F64],
            vec![],
            Some(types::F64),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_math_abs_float"), id);
        // ori_math_min / ori_math_max
        let id = decl(
            "ori_math_min",
            &[types::I64, types::I64],
            vec![],
            Some(types::I64),
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_min"), id);
        let id = decl(
            "ori_math_min_float",
            &[types::F64, types::F64],
            vec![],
            Some(types::F64),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_math_min_float"), id);
        let id = decl(
            "ori_math_max",
            &[types::I64, types::I64],
            vec![],
            Some(types::I64),
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_max"), id);
        let id = decl(
            "ori_math_max_float",
            &[types::F64, types::F64],
            vec![],
            Some(types::F64),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_math_max_float"), id);
        let id = decl(
            "ori_math_clamp",
            &[types::I64, types::I64, types::I64],
            vec![],
            Some(types::I64),
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_clamp"), id);
        let id = decl(
            "ori_math_pow",
            &[types::F64, types::F64],
            vec![],
            Some(types::F64),
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_pow"), id);
        let id = decl("ori_math_floor", &[types::F64], vec![], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_floor"), id);
        let id = decl("ori_math_ceil", &[types::F64], vec![], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_ceil"), id);
        let id = decl("ori_math_round", &[types::F64], vec![], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_round"), id);
        let id = decl("ori_time_now", &[], vec![], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_time_now"), id);
        let id = decl("ori_time_sleep", &[types::I64], vec![], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_time_sleep"), id);
        let id = decl(
            "ori_time_duration_ms",
            &[types::I64, types::I64],
            vec![],
            Some(types::I64),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_time_duration_ms"), id);
        let id = decl(
            "ori_format_number",
            &[types::F64, types::I64],
            vec![],
            Some(pt),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_format_number"), id);
        let id = decl(
            "ori_format_percent",
            &[types::F64, types::I64],
            vec![],
            Some(pt),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_format_percent"), id);
        let id = decl("ori_format_hex", &[types::I64], vec![], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_format_hex"), id);
        let id = decl("ori_format_binary", &[types::I64], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_format_binary"), id);
        let id = decl(
            "ori_format_date",
            &[types::I64, pt],
            vec![Ty::Int64, Ty::String],
            Some(pt),
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_format_date"), id);
        let id = decl(
            "ori_format_datetime",
            &[types::I64, pt, pt],
            vec![Ty::Int64, Ty::String, Ty::String],
            Some(pt),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_format_datetime"), id);
        let id = decl(
            "ori_format_bytes_size",
            &[types::I64, pt],
            vec![Ty::Int64, Ty::String],
            Some(pt),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_format_bytes_size"), id);
        let id = decl("ori_os_set_args", &[types::I32, pt], vec![], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_os_set_args"), id);
        let id = decl("ori_os_args", &[], vec![], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_os_args"), id);
        let id = decl("ori_os_env", &[pt], vec![Ty::String], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_os_env"), id);
        let id = decl("ori_os_exit", &[types::I64], vec![], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_os_exit"), id);
        let id = decl("ori_os_pid", &[], vec![], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_os_pid"), id);
        let id = decl("ori_os_platform", &[], vec![], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_os_platform"), id);
        let id = decl("ori_os_arch", &[], vec![], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_os_arch"), id);
        let id = decl(
            "ori_random_int",
            &[types::I64, types::I64],
            vec![],
            Some(types::I64),
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_random_int"), id);
        let id = decl(
            "ori_random_float",
            &[types::F64, types::F64],
            vec![],
            Some(types::F64),
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_random_float"), id);
        let id = decl("ori_random_bool", &[], vec![], Some(types::I8))?;
        self.stdlib_ids.insert(SmolStr::new("ori_random_bool"), id);
        let id = decl(
            "ori_test_assert",
            &[types::I8, pt],
            vec![Ty::Bool, Ty::String],
            None,
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_test_assert"), id);
        let id = decl("ori_test_fail", &[pt], vec![Ty::String], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_test_fail"), id);
        let test_assert_overloads = [
            ("ori_test_assert_eq_float", vec![types::F64, types::F64]),
            ("ori_test_assert_ne_float", vec![types::F64, types::F64]),
            ("ori_test_assert_eq_bool", vec![types::I8, types::I8]),
            ("ori_test_assert_ne_bool", vec![types::I8, types::I8]),
            ("ori_test_assert_eq_string", vec![pt, pt]),
            ("ori_test_assert_ne_string", vec![pt, pt]),
        ];
        for (name, abi_params) in test_assert_overloads {
            let id = decl(name, &abi_params, Vec::new(), None)?;
            self.stdlib_ids.insert(SmolStr::new(name), id);
        }
        let iter_overloads = [
            ("ori_iter_sort_string", vec![pt], Some(pt)),
            ("ori_iter_unique_string", vec![pt], Some(pt)),
            ("ori_iter_group_by_string", vec![pt, pt, pt], Some(pt)),
        ];
        for (name, abi_params, ret) in iter_overloads {
            let id = decl(name, &abi_params, Vec::new(), ret)?;
            self.stdlib_ids.insert(SmolStr::new(name), id);
        }

        // Bytes methods
        let id = decl(
            "ori_bytes_len",
            &[self.ptr_ty],
            vec![Ty::Bytes],
            Some(types::I64),
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_bytes_len"), id);
        let id = decl(
            "ori_bytes_get",
            &[self.ptr_ty, types::I64],
            vec![Ty::Bytes, Ty::Int64],
            Some(types::I8),
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_bytes_get"), id);
        let id = decl(
            "ori_bytes_concat",
            &[self.ptr_ty, self.ptr_ty],
            vec![Ty::Bytes, Ty::Bytes],
            Some(self.ptr_ty),
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_bytes_concat"), id);
        let id = decl(
            "ori_bytes_slice",
            &[self.ptr_ty, types::I64, types::I64],
            vec![Ty::Bytes, Ty::Int64, Ty::Int64],
            Some(self.ptr_ty),
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_bytes_slice"), id);
        let id = decl(
            "ori_bytes_from_hex",
            &[self.ptr_ty],
            vec![Ty::String],
            Some(self.ptr_ty),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_bytes_from_hex"), id);
        let id = decl(
            "ori_bytes_to_hex",
            &[self.ptr_ty],
            vec![Ty::Bytes],
            Some(self.ptr_ty),
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_bytes_to_hex"), id);
        let id = decl(
            "ori_bytes_decode_utf8",
            &[self.ptr_ty],
            vec![Ty::Bytes],
            Some(self.ptr_ty),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_bytes_decode_utf8"), id);
        let id = decl(
            "ori_string_to_bytes",
            &[self.ptr_ty],
            vec![Ty::String],
            Some(self.ptr_ty),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_to_bytes"), id);
        let id = decl(
            "ori_string_from_bytes",
            &[self.ptr_ty],
            vec![Ty::Bytes],
            Some(self.ptr_ty),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_from_bytes"), id);

        // Primitive conversions
        let id = decl(
            "ori_to_int",
            &[types::I64],
            vec![Ty::Int64],
            Some(types::I64),
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_to_int"), id);
        let id = decl(
            "ori_to_float",
            &[types::I64],
            vec![Ty::Int64],
            Some(types::F64),
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_to_float"), id);
        let id = decl(
            "ori_new_result",
            &[types::I8, self.ptr_ty],
            vec![Ty::Bool, Ty::Int64],
            Some(self.ptr_ty),
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_new_result"), id);
        let id = decl("ori_math_log", &[types::F64], vec![], Some(types::F64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_log"), id);
        let id = decl("ori_math_log2", &[types::F64], vec![], Some(types::F64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_log2"), id);
        let id = decl("ori_math_sin", &[types::F64], vec![], Some(types::F64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_sin"), id);
        let id = decl("ori_math_cos", &[types::F64], vec![], Some(types::F64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_cos"), id);
        let id = decl("ori_math_tan", &[types::F64], vec![], Some(types::F64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_tan"), id);
        let id = decl("ori_math_is_nan", &[types::F64], vec![], Some(types::I8))?;
        self.stdlib_ids.insert(SmolStr::new("ori_math_is_nan"), id);
        let id = decl(
            "ori_math_is_infinite",
            &[types::F64],
            vec![],
            Some(types::I8),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_math_is_infinite"), id);
        let id = decl("ori_float_to_string", &[types::F64], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_float_to_string"), id);
        let id = decl(
            "ori_float_to_string_parts",
            &[types::F64, pt, pt],
            vec![],
            None,
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_float_to_string_parts"), id);
        let id = decl("ori_bool_to_string", &[types::I8], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_bool_to_string"), id);
        let id = decl(
            "ori_bool_to_string_parts",
            &[types::I8, pt, pt],
            vec![],
            None,
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_bool_to_string_parts"), id);
        let id = decl("ori_string_to_int", &[pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_to_int"), id);
        let id = decl("ori_string_to_float", &[pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_to_float"), id);
        let id = decl("ori_string_parse_int", &[pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_parse_int"), id);
        let id = decl("ori_string_parse_float", &[pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_string_parse_float"), id);
        // list<T> runtime
        let id = decl("ori_list_new", &[], vec![], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_list_new"), id);
        let id = decl("ori_list_push", &[pt, types::I64], vec![], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_list_push"), id);
        let id = decl("ori_list_get", &[pt, types::I64], vec![], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_list_get"), id);
        let id = decl("ori_list_set", &[pt, types::I64, types::I64], vec![], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_list_set"), id);
        let id = decl("ori_list_len", &[pt], vec![], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_list_len"), id);
        let id = decl("ori_list_free", &[pt], vec![], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_list_free"), id);
        let id = decl("ori_list_pop", &[pt], vec![], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_list_pop"), id);
        let id = decl("ori_list_remove", &[pt, types::I64], vec![], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_list_remove"), id);
        let id = decl(
            "ori_list_insert",
            &[pt, types::I64, types::I64],
            vec![],
            None,
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_list_insert"), id);
        let id = decl(
            "ori_list_contains",
            &[pt, types::I64],
            vec![],
            Some(types::I8),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_list_contains"), id);
        let id = decl(
            "ori_list_index_of",
            &[pt, types::I64],
            vec![],
            Some(types::I64),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_list_index_of"), id);
        let id = decl("ori_list_sort", &[pt], vec![], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_list_sort"), id);
        let id = decl("ori_list_reverse", &[pt], vec![], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_list_reverse"), id);
        let id = decl(
            "ori_list_slice",
            &[pt, types::I64, types::I64],
            vec![],
            Some(pt),
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_list_slice"), id);
        let id = decl("ori_set_new", &[], vec![], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_set_new"), id);
        let id = decl("ori_set_add", &[pt, types::I64], vec![], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_set_add"), id);
        let id = decl("ori_set_add_string", &[pt, pt], vec![], None)?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_set_add_string"), id);
        let id = decl(
            "ori_set_contains",
            &[pt, types::I64],
            vec![],
            Some(types::I8),
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_set_contains"), id);
        let id = decl(
            "ori_set_contains_string",
            &[pt, pt],
            vec![],
            Some(types::I8),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_set_contains_string"), id);
        let id = decl("ori_set_len", &[pt], vec![], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_set_len"), id);
        let id = decl("ori_set_capacity", &[pt], vec![], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_set_capacity"), id);
        let id = decl("ori_set_reserve", &[pt, types::I64], vec![], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_set_reserve"), id);
        let id = decl("ori_set_clear", &[pt], vec![], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_set_clear"), id);
        let id = decl("ori_set_free", &[pt], vec![], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_set_free"), id);
        let id = decl("ori_set_remove", &[pt, types::I64], vec![], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_set_remove"), id);
        let id = decl("ori_set_remove_string", &[pt, pt], vec![], None)?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_set_remove_string"), id);
        let id = decl(
            "ori_set_try_remove_string",
            &[pt, pt],
            vec![],
            Some(types::I8),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_set_try_remove_string"), id);
        let id = decl("ori_set_from_list_string", &[pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_set_from_list_string"), id);
        let id = decl("ori_set_union", &[pt, pt], vec![], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_set_union"), id);
        let id = decl("ori_set_intersection", &[pt, pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_set_intersection"), id);
        let id = decl("ori_set_difference", &[pt, pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_set_difference"), id);
        let id = decl("ori_list_map", &[pt, pt, pt], vec![], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_list_map"), id);
        let id = decl("ori_list_filter", &[pt, pt, pt], vec![], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_list_filter"), id);
        let id = decl("ori_map_new", &[], vec![], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_new"), id);
        let id = decl("ori_map_set", &[pt, types::I64, types::I64], vec![], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_set"), id);
        let id = decl("ori_map_set_string", &[pt, pt, types::I64], vec![], None)?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_map_set_string"), id);
        let id = decl("ori_map_get", &[pt, types::I64], vec![], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_get"), id);
        let id = decl("ori_map_get_string", &[pt, pt], vec![], Some(types::I64))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_map_get_string"), id);
        let id = decl("ori_map_try_get_string", &[pt, pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_map_try_get_string"), id);
        let id = decl(
            "ori_map_contains",
            &[pt, types::I64],
            vec![],
            Some(types::I8),
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_contains"), id);
        let id = decl(
            "ori_map_contains_string",
            &[pt, pt],
            vec![],
            Some(types::I8),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_map_contains_string"), id);
        let id = decl("ori_map_len", &[pt], vec![], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_len"), id);
        let id = decl("ori_map_capacity", &[pt], vec![], Some(types::I64))?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_capacity"), id);
        let id = decl("ori_map_reserve", &[pt, types::I64], vec![], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_reserve"), id);
        let id = decl("ori_map_clear", &[pt], vec![], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_clear"), id);
        let id = decl(
            "ori_map_key_at",
            &[pt, types::I64],
            vec![],
            Some(types::I64),
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_key_at"), id);
        let id = decl(
            "ori_map_value_at",
            &[pt, types::I64],
            vec![],
            Some(types::I64),
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_value_at"), id);
        let id = decl("ori_map_free", &[pt], vec![], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_free"), id);
        let id = decl("ori_map_remove", &[pt, types::I64], vec![], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_remove"), id);
        let id = decl("ori_map_remove_string", &[pt, pt], vec![], None)?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_map_remove_string"), id);
        let id = decl("ori_map_try_remove_string", &[pt, pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_map_try_remove_string"), id);
        let id = decl("ori_map_from_entries_string", &[pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_map_from_entries_string"), id);
        let id = decl("ori_map_keys", &[pt], vec![], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_keys"), id);
        let id = decl("ori_map_values", &[pt], vec![], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_values"), id);
        let id = decl("ori_map_entries", &[pt], vec![], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("ori_map_entries"), id);
        let id = decl(
            "ori_hash_table_set_string",
            &[pt, pt, types::I64],
            vec![],
            None,
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_hash_table_set_string"), id);
        let id = decl("ori_hash_table_get_string", &[pt, pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_hash_table_get_string"), id);
        let id = decl("ori_hash_table_remove_string", &[pt, pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_hash_table_remove_string"), id);
        let id = decl(
            "ori_hash_table_contains_string",
            &[pt, pt],
            vec![],
            Some(types::I8),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_hash_table_contains_string"), id);
        let id = decl(
            "ori_hash_table_from_entries_string",
            &[pt],
            vec![],
            Some(pt),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_hash_table_from_entries_string"), id);
        let id = decl("ori_tree_find_string", &[pt, pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_tree_find_string"), id);
        let id = decl("ori_linked_list_find_string", &[pt, pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_linked_list_find_string"), id);
        let id = decl(
            "ori_doubly_linked_list_find_string",
            &[pt, pt],
            vec![],
            Some(pt),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_doubly_linked_list_find_string"), id);
        for name in ["ori_graph_add_node_string", "ori_graph_remove_node_string"] {
            let id = decl(name, &[pt, pt], vec![], None)?;
            self.stdlib_ids.insert(SmolStr::new(name), id);
        }
        for name in ["ori_graph_add_edge_string", "ori_graph_remove_edge_string"] {
            let id = decl(name, &[pt, pt, pt], vec![], None)?;
            self.stdlib_ids.insert(SmolStr::new(name), id);
        }
        let id = decl(
            "ori_graph_add_weighted_edge_string",
            &[pt, pt, pt, types::I64],
            vec![],
            None,
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_graph_add_weighted_edge_string"), id);
        for name in ["ori_graph_has_node_string"] {
            let id = decl(name, &[pt, pt], vec![], Some(types::I8))?;
            self.stdlib_ids.insert(SmolStr::new(name), id);
        }
        for name in ["ori_graph_has_edge_string"] {
            let id = decl(name, &[pt, pt, pt], vec![], Some(types::I8))?;
            self.stdlib_ids.insert(SmolStr::new(name), id);
        }
        for name in [
            "ori_graph_edge_weight_string",
            "ori_graph_shortest_path_string",
            "ori_graph_shortest_weighted_path_string",
        ] {
            let id = decl(name, &[pt, pt, pt], vec![], Some(pt))?;
            self.stdlib_ids.insert(SmolStr::new(name), id);
        }
        for name in [
            "ori_graph_neighbors_string",
            "ori_graph_bfs_string",
            "ori_graph_dfs_string",
        ] {
            let id = decl(name, &[pt, pt], vec![], Some(pt))?;
            self.stdlib_ids.insert(SmolStr::new(name), id);
        }
        let id = decl("ori_heap_new_string", &[], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_heap_new_string"), id);
        let id = decl("ori_heap_new_custom", &[pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_heap_new_custom"), id);
        let id = decl("ori_heap_push_string", &[pt, pt], vec![], None)?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_heap_push_string"), id);
        let id = decl("ori_heap_push_custom", &[pt, types::I64, pt], vec![], None)?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_heap_push_custom"), id);
        let id = decl("ori_heap_from_list_string", &[pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_heap_from_list_string"), id);
        let id = decl("ori_heap_from_list_custom", &[pt, pt], vec![], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_heap_from_list_custom"), id);
        let id = decl("ori_heap_remove_string", &[pt, pt], vec![], Some(types::I8))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_heap_remove_string"), id);
        let id = decl(
            "ori_heap_remove_custom",
            &[pt, types::I64, pt],
            vec![],
            Some(types::I8),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_heap_remove_custom"), id);
        let id = decl("ori_files_read_text", &[pt], vec![Ty::String], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_files_read_text"), id);
        let id = decl(
            "ori_files_write_text",
            &[pt, pt],
            vec![Ty::String, Ty::String],
            Some(pt),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_files_write_text"), id);
        let id = decl(
            "ori_files_append_text",
            &[pt, pt],
            vec![Ty::String, Ty::String],
            Some(types::I8),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_files_append_text"), id);
        let id = decl("ori_files_exists", &[pt], vec![Ty::String], Some(types::I8))?;
        self.stdlib_ids.insert(SmolStr::new("ori_files_exists"), id);
        let id = decl("ori_files_delete", &[pt], vec![Ty::String], Some(types::I8))?;
        self.stdlib_ids.insert(SmolStr::new("ori_files_delete"), id);
        let id = decl("ori_files_list_dir", &[pt], vec![Ty::String], Some(pt))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_files_list_dir"), id);
        let id = decl(
            "ori_files_create_dir",
            &[pt],
            vec![Ty::String],
            Some(types::I8),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_files_create_dir"), id);
        let id = decl(
            "ori_files_is_file",
            &[pt],
            vec![Ty::String],
            Some(types::I8),
        )?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_files_is_file"), id);
        let id = decl("ori_files_is_dir", &[pt], vec![Ty::String], Some(types::I8))?;
        self.stdlib_ids.insert(SmolStr::new("ori_files_is_dir"), id);
        let id = decl(
            "ori_files_copy",
            &[pt, pt],
            vec![Ty::String, Ty::String],
            Some(types::I8),
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_files_copy"), id);
        let id = decl(
            "ori_files_rename",
            &[pt, pt],
            vec![Ty::String, Ty::String],
            Some(types::I8),
        )?;
        self.stdlib_ids.insert(SmolStr::new("ori_files_rename"), id);
        let id = decl("ori_arc_retain", &[pt], vec![], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_arc_retain"), id);
        let id = decl("ori_arc_release", &[pt], vec![], None)?;
        self.stdlib_ids.insert(SmolStr::new("ori_arc_release"), id);
        let id = decl("ori_arc_register_edge", &[pt, pt], vec![], None)?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_arc_register_edge"), id);
        let id = decl("ori_arc_unregister_edge", &[pt, pt], vec![], None)?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_arc_unregister_edge"), id);
        let id = decl("ori_arc_update_edge", &[pt, pt, pt], vec![], None)?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_arc_update_edge"), id);
        let id = decl("ori_arc_collect_cycles", &[], vec![], Some(types::I64))?;
        self.stdlib_ids
            .insert(SmolStr::new("ori_arc_collect_cycles"), id);
        // malloc / free for runtime allocation
        let id = decl("malloc", &[types::I64], vec![], Some(pt))?;
        self.stdlib_ids.insert(SmolStr::new("malloc"), id);
        let id = decl("free", &[pt], vec![], None)?;
        self.stdlib_ids.insert(SmolStr::new("free"), id);
        let id = decl("ori_alloc", &[types::I64, pt], vec![], Some(pt))?;
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

    fn make_async_step_sig(&self) -> ir::Signature {
        let mut sig = self.module.make_signature();
        sig.params.push(AbiParam::new(self.ptr_ty));
        sig.returns.push(AbiParam::new(types::I64));
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
            if f.is_async && simple_async_state_machine_plan(f).is_some() {
                let step_name = async_step_name(f);
                let sig = self.make_async_step_sig();
                let id = self
                    .module
                    .declare_function(&native_func_symbol(&step_name), Linkage::Local, &sig)
                    .map_err(|e| format!("declare async step '{}': {e}", step_name))?;
                self.func_ids.insert(step_name, id);
            }
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
            sig.params.push(AbiParam::new(types::I32));
            sig.params.push(AbiParam::new(self.ptr_ty));
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
        let global_init_id = if hir.consts.iter().any(|c| {
            cl_type(&c.ty, self.ptr_ty).is_some() && needs_runtime_global_init(&c.value, &c.ty)
        }) {
            let sig = self.module.make_signature();
            Some(
                self.module
                    .declare_function("__ori_init_globals", Linkage::Local, &sig)
                    .map_err(|e| format!("declare global initializer: {e}"))?,
            )
        } else {
            None
        };
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
                .emit_user_func(f)?;
            }
            self.module
                .define_function(func_id, &mut ctx)
                .map_err(|e| format!("define '{}': {e}", f.name))?;
        }

        for f in &hir.funcs {
            let Some(plan) = simple_async_state_machine_plan(f) else {
                continue;
            };
            self.define_simple_async_step(f, &plan, &const_exprs)?;
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

        if let Some(global_init_id) = global_init_id {
            self.define_global_initializer(hir, global_init_id)?;
        }

        // Define C main wrapper
        if let Some(entry_main) = hir.funcs.iter().find(|f| is_entry_main(hir, f)) {
            let ori_main_id = self.func_ids[&entry_main.name];
            let mut sig = self.module.make_signature();
            sig.params.push(AbiParam::new(types::I32));
            sig.params.push(AbiParam::new(self.ptr_ty));
            sig.returns.push(AbiParam::new(types::I32));
            let main_id = self
                .module
                .declare_function("main", Linkage::Export, &sig)
                .map_err(|e| format!("re-declare main: {e}"))?;
            let mut ctx = self.module.make_context();
            ctx.func.signature = sig;
            let ori_ref = self.module.declare_func_in_func(ori_main_id, &mut ctx.func);
            let init_ref =
                global_init_id.map(|id| self.module.declare_func_in_func(id, &mut ctx.func));
            let set_args_ref = self
                .stdlib_ids
                .get("ori_os_set_args")
                .copied()
                .map(|id| self.module.declare_func_in_func(id, &mut ctx.func));
            let block_on_ref = if matches!(entry_main.return_ty, Ty::Future(_)) {
                Some(
                    self.stdlib_ids
                        .get("ori_task_block_on")
                        .copied()
                        .map(|id| self.module.declare_func_in_func(id, &mut ctx.func))
                        .ok_or_else(|| {
                            "missing runtime function `ori_task_block_on` for async main"
                                .to_string()
                        })?,
                )
            } else {
                None
            };
            let mut bctx = FunctionBuilderContext::new();
            {
                let mut b = FunctionBuilder::new(&mut ctx.func, &mut bctx);
                let blk = b.create_block();
                b.append_block_params_for_function_params(blk);
                b.switch_to_block(blk);
                b.seal_block(blk);
                if let Some(init_ref) = init_ref {
                    b.ins().call(init_ref, &[]);
                }
                if let Some(set_args_ref) = set_args_ref {
                    let (argc, argv) = {
                        let params = b.block_params(blk);
                        (params[0], params[1])
                    };
                    b.ins().call(set_args_ref, &[argc, argv]);
                }
                let call = b.ins().call(ori_ref, &[]);
                if let Some(block_on_ref) = block_on_ref {
                    let future = b.inst_results(call)[0];
                    b.ins().call(block_on_ref, &[future]);
                }
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

    fn define_simple_async_step(
        &mut self,
        f: &HirFunc,
        plan: &SimpleAsyncStateMachinePlan,
        const_exprs: &HashMap<SmolStr, HirExpr>,
    ) -> Result<(), String> {
        let step_name = async_step_name(f);
        let sig = self.make_async_step_sig();
        let func_id = self.func_ids[&step_name];
        let mut ctx = self.module.make_context();
        ctx.func.signature = sig;

        let mut func_refs: HashMap<SmolStr, ir::FuncRef> = HashMap::new();
        for (name, &id) in self.func_ids.iter().chain(self.stdlib_ids.iter()) {
            let fref = self.module.declare_func_in_func(id, &mut ctx.func);
            func_refs.insert(name.clone(), fref);
        }
        for (name, &id) in &self.func_wrapper_ids {
            let fref = self.module.declare_func_in_func(id, &mut ctx.func);
            func_refs.insert(SmolStr::new(format!("{name}.__fnptr_wrapper")), fref);
        }

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
                const_exprs,
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
                current_return_ty: plan.inner_ty.clone(),
                terminated: false,
            }
            .emit_simple_async_step(f, plan)?;
        }
        self.module
            .define_function(func_id, &mut ctx)
            .map_err(|e| format!("define async step '{}': {e}", step_name))?;
        Ok(())
    }

    fn define_global_initializer(
        &mut self,
        hir: &HirModule,
        init_id: FuncId,
    ) -> Result<(), String> {
        let mut ctx = self.module.make_context();
        ctx.func.signature = self.module.make_signature();

        let mut func_refs: HashMap<SmolStr, ir::FuncRef> = HashMap::new();
        for (name, &id) in self.func_ids.iter().chain(self.stdlib_ids.iter()) {
            let fref = self.module.declare_func_in_func(id, &mut ctx.func);
            func_refs.insert(name.clone(), fref);
        }
        for (name, &id) in &self.func_wrapper_ids {
            let fref = self.module.declare_func_in_func(id, &mut ctx.func);
            func_refs.insert(SmolStr::new(format!("{name}.__fnptr_wrapper")), fref);
        }

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

        let const_exprs: HashMap<SmolStr, HirExpr> = hir
            .consts
            .iter()
            .map(|c| (c.name.clone(), c.value.clone()))
            .collect();
        let mut bctx = FunctionBuilderContext::new();
        {
            let mut codegen = FuncCodegen {
                builder: FunctionBuilder::new(&mut ctx.func, &mut bctx),
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
                current_return_ty: Ty::Void,
                terminated: false,
            };
            let block = codegen.builder.create_block();
            codegen.builder.switch_to_block(block);
            codegen.builder.seal_block(block);
            for global in &hir.consts {
                if cl_type(&global.ty, self.ptr_ty).is_none()
                    || !needs_runtime_global_init(&global.value, &global.ty)
                {
                    continue;
                }
                let value = codegen.emit_expr_for_expected(&global.value, &global.ty)?;
                codegen.emit_arc_retain_if_managed(&global.ty, value)?;
                if !codegen.initialize_global(&global.name, value) {
                    return Err(format!(
                        "global `{}` is not available during native initialization",
                        global.name
                    ));
                }
            }
            codegen.builder.ins().return_(&[]);
            codegen.builder.seal_all_blocks();
            codegen.builder.finalize();
        }

        self.module
            .define_function(init_id, &mut ctx)
            .map_err(|e| format!("define global initializer: {e}"))?;
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
    fn emit_user_func(self, f: &HirFunc) -> Result<(), String> {
        if f.is_async {
            self.emit_async_wrapper(f)
        } else {
            self.emit(f)
        }
    }

    fn emit_async_wrapper(self, f: &HirFunc) -> Result<(), String> {
        if let Some(plan) = simple_async_state_machine_plan(f) {
            return self.emit_simple_async_state_machine_wrapper(f, &plan);
        }
        if !block_contains_await(&f.body) {
            return self.emit(f);
        }
        Err(native_codegen_unsupported(format!(
            "async function `{}` contains an `await` shape not yet covered by the native state machine; supported forms include top-level `await value`, `const x: T = await value`, `return await value`, expression-level awaits in calls/operators/conditions, and `const x = (await value)?`; awaits inside nested statement bodies are still blocked",
            f.name
        )))
    }

    fn emit_simple_async_state_machine_wrapper(
        mut self,
        f: &HirFunc,
        plan: &SimpleAsyncStateMachinePlan,
    ) -> Result<(), String> {
        let entry = self.builder.create_block();
        self.builder.append_block_params_for_function_params(entry);
        self.builder.switch_to_block(entry);
        self.builder.seal_block(entry);

        self.emit_param_contracts(&f.params)?;

        let pending_ref = *self
            .func_refs
            .get("ori_future_pending")
            .ok_or_else(|| "missing runtime function `ori_future_pending`".to_string())?;
        let pending_call = self.builder.ins().call(pending_ref, &[]);
        let future = self.builder.inst_results(pending_call)[0];

        let frame = self.malloc_bytes(simple_async_frame_size(plan, self.ptr_ty))?;
        let zero = self.builder.ins().iconst(types::I64, 0);
        self.builder
            .ins()
            .store(MemFlags::new(), zero, frame, ASYNC_FRAME_STATE_OFFSET);
        self.builder
            .ins()
            .store(MemFlags::new(), future, frame, ASYNC_FRAME_RESULT_OFFSET);
        self.emit_arc_register_edge(frame, future)?;
        for (index, param) in plan.params.iter().enumerate() {
            let cl_ty = cl_type(&param.ty, self.ptr_ty)
                .ok_or_else(|| format!("async param `{}` has no native value", param.name))?;
            let value = self.builder.block_params(entry)[index];
            debug_assert_eq!(self.builder.func.dfg.value_type(value), cl_ty);
            let offset = simple_async_frame_param_offset(plan, index, self.ptr_ty)
                .expect("async frame param offset");
            self.builder
                .ins()
                .store(MemFlags::new(), value, frame, offset as i32);
            self.emit_arc_register_edge_if_managed(&param.ty, frame, value)?;
        }
        for (index, local) in plan.locals.iter().enumerate() {
            let value = self.zero_val(&local.ty);
            let offset = simple_async_frame_local_offset(plan, index, self.ptr_ty)
                .expect("async frame local offset");
            self.builder
                .ins()
                .store(MemFlags::new(), value, frame, offset as i32);
        }

        let step_name = async_step_name(f);
        let step_ref = *self
            .func_refs
            .get(step_name.as_str())
            .ok_or_else(|| format!("missing async step function `{step_name}`"))?;
        let step_closure = self.emit_closure_object(step_ref, Some(frame))?;
        let schedule_ref = *self
            .func_refs
            .get("ori_executor_schedule")
            .ok_or_else(|| "missing runtime function `ori_executor_schedule`".to_string())?;
        self.builder.ins().call(schedule_ref, &[step_closure]);
        self.emit_arc_release_if_managed(&Ty::Bytes, frame)?;

        self.emit_arc_retain_if_managed(&f.return_ty, future)?;
        self.emit_scope_cleanup_calls_from(0, 0)?;
        self.builder.ins().return_(&[future]);
        self.terminated = true;

        self.builder.seal_all_blocks();
        self.builder.finalize();
        Ok(())
    }

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
            if let Ty::Future(inner) = &f.return_ty {
                let future = self.emit_future_ready(inner, None)?;
                self.builder.ins().return_(&[future]);
            } else if cl_type(&f.return_ty, self.ptr_ty).is_none() {
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
        let trap = ir::TrapCode::user(trap_code)
            .ok_or_else(|| format!("invalid runtime contract trap code `{trap_code}`"))?;
        self.emit_trap_unless(condition, trap, run_cleanup)
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
            return self.emit_any_box(*trait_def_id, *type_def_id, value);
        }

        // Numeric widening casts
        let actual_ty = self.builder.func.dfg.value_type(value);
        let expected_ty = cl_type(expected, self.ptr_ty).unwrap_or(self.ptr_ty);
        if actual_ty != expected_ty {
            if actual_ty == types::F32 && expected_ty == types::F64 {
                return Ok(self.builder.ins().fpromote(expected_ty, value));
            }
            if actual_ty == types::F64 && expected_ty == types::F32 {
                return Ok(self.builder.ins().fdemote(expected_ty, value));
            }

            let actual_size = match actual_ty {
                types::I8 => 1,
                types::I16 => 2,
                types::I32 => 4,
                types::I64 => 8,
                _ => 0,
            };
            let expected_size = match expected_ty {
                types::I8 => 1,
                types::I16 => 2,
                types::I32 => 4,
                types::I64 => 8,
                _ => 0,
            };
            if actual_size > 0 && expected_size > actual_size {
                // Determine if we should sign-extend or zero-extend
                let is_unsigned = matches!(expr.ty, Ty::U8 | Ty::U16 | Ty::U32 | Ty::U64);
                if is_unsigned {
                    return Ok(self.builder.ins().uextend(expected_ty, value));
                } else {
                    return Ok(self.builder.ins().sextend(expected_ty, value));
                }
            }
        }

        Ok(value)
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
        if env_ptr.is_some() {
            self.emit_arc_register_edge(object, env)?;
        }
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
        self.builder
            .ins()
            .store(MemFlags::new(), type_id_val, vtable, 0);
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
        self.emit_arc_register_edge(object, data_ptr)?;
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
                if let Some(cl_ty) = cl_type(&capture.ty, self.ptr_ty) {
                    let stored = if cl_ty == self.ptr_ty || cl_ty == types::I64 {
                        value
                    } else {
                        value
                    };
                    self.builder
                        .ins()
                        .store(MemFlags::new(), stored, env, offset as i32);
                    self.emit_arc_register_edge_if_managed(&capture.ty, env, value)?;
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

    fn emit_lazy_once(&mut self, thunk: &HirExpr, lazy_ty: &Ty) -> Result<ir::Value, String> {
        let Ty::Lazy(inner) = lazy_ty else {
            return Err(format!(
                "lazy.once expected lazy result type, got `{}`",
                lazy_ty.display()
            ));
        };
        let thunk_value = self.emit_expr(thunk)?;
        let ptr_size = self.ptr_ty.bytes() as i32;
        let (_, total) = lazy_layout(inner, self.ptr_ty);
        let object = self.malloc_bytes(total)?;
        let zero8 = self.builder.ins().iconst(types::I8, 0);
        self.builder
            .ins()
            .store(MemFlags::new(), thunk_value, object, 0);
        self.builder
            .ins()
            .store(MemFlags::new(), zero8, object, ptr_size);
        self.emit_arc_register_edge(object, thunk_value)?;
        self.emit_arc_release_if_managed(&thunk.ty, thunk_value)?;
        Ok(object)
    }

    fn emit_lazy_force(&mut self, value: &HirExpr, ret_ty: &Ty) -> Result<ir::Value, String> {
        let Ty::Lazy(inner) = &value.ty else {
            return Err(format!(
                "lazy.force expected lazy value, got `{}`",
                value.ty.display()
            ));
        };
        let ret_cl = cl_type(ret_ty, self.ptr_ty).ok_or_else(|| {
            format!(
                "native backend cannot force lazy value returning `{}`",
                ret_ty.display()
            )
        })?;
        let lazy_value = self.emit_expr(value)?;
        let ptr_size = self.ptr_ty.bytes() as i32;
        let (value_offset, _) = lazy_layout(inner, self.ptr_ty);
        let forced = self
            .builder
            .ins()
            .load(types::I8, MemFlags::new(), lazy_value, ptr_size);
        let forced_block = self.builder.create_block();
        let compute_block = self.builder.create_block();
        let merge_block = self.builder.create_block();
        let slot = self.builder.create_sized_stack_slot(ir::StackSlotData::new(
            ir::StackSlotKind::ExplicitSlot,
            ret_cl.bytes(),
            3,
        ));
        let slot_addr = self.builder.ins().stack_addr(self.ptr_ty, slot, 0);
        self.builder
            .ins()
            .brif(forced, forced_block, &[], compute_block, &[]);

        self.builder.seal_block(forced_block);
        self.builder.switch_to_block(forced_block);
        let cached =
            self.builder
                .ins()
                .load(ret_cl, MemFlags::new(), lazy_value, value_offset as i32);
        self.builder
            .ins()
            .store(MemFlags::new(), cached, slot_addr, 0);
        self.builder.ins().jump(merge_block, &[]);

        self.builder.seal_block(compute_block);
        self.builder.switch_to_block(compute_block);
        let thunk = self
            .builder
            .ins()
            .load(self.ptr_ty, MemFlags::new(), lazy_value, 0);
        let fn_ptr = self
            .builder
            .ins()
            .load(self.ptr_ty, MemFlags::new(), thunk, 0);
        let env_ptr = self
            .builder
            .ins()
            .load(self.ptr_ty, MemFlags::new(), thunk, ptr_size);
        let mut sig = ir::Signature::new(self.builder.func.signature.call_conv);
        sig.params.push(AbiParam::new(self.ptr_ty));
        sig.returns.push(AbiParam::new(ret_cl));
        let sig_ref = self.builder.func.import_signature(sig);
        let call = self
            .builder
            .ins()
            .call_indirect(sig_ref, fn_ptr, &[env_ptr]);
        let computed = self.builder.inst_results(call)[0];
        self.builder
            .ins()
            .store(MemFlags::new(), computed, lazy_value, value_offset as i32);
        let one8 = self.builder.ins().iconst(types::I8, 1);
        self.builder
            .ins()
            .store(MemFlags::new(), one8, lazy_value, ptr_size);
        self.emit_arc_register_edge_if_managed(inner, lazy_value, computed)?;
        self.emit_arc_release_if_managed(inner, computed)?;
        self.builder
            .ins()
            .store(MemFlags::new(), computed, slot_addr, 0);
        self.builder.ins().jump(merge_block, &[]);

        self.builder.seal_block(merge_block);
        self.builder.switch_to_block(merge_block);
        Ok(self
            .builder
            .ins()
            .load(ret_cl, MemFlags::new(), slot_addr, 0))
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
        self.emit_arc_register_edge_if_managed(elem_ty, list, value)?;
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
        self.store_global_inner(name, value, true)
    }

    fn initialize_global(&mut self, name: &str, value: ir::Value) -> bool {
        self.store_global_inner(name, value, false)
    }

    fn store_global_inner(&mut self, name: &str, value: ir::Value, require_mutable: bool) -> bool {
        let Some(gv) = self.global_gvs.get(name).copied() else {
            return false;
        };
        let Some(info) = self.global_data.get(name) else {
            return false;
        };
        if require_mutable && !info.mutable {
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
            HirLValue::Field { base, field } => {
                let (addr, field_layout, _) = self.emit_field_lvalue_addr(base, field)?;
                let cl_ty = cl_type(&field_layout.ty, self.ptr_ty)
                    .ok_or_else(|| format!("missing Cranelift type for field `{field}`"))?;
                let value = self.builder.ins().load(cl_ty, MemFlags::new(), addr, 0);
                Ok((value, field_layout.ty))
            }
            _ => Err(native_codegen_unsupported(
                "indexed assignment base in native codegen",
            )),
        }
    }

    fn emit_field_lvalue_addr(
        &mut self,
        base: &HirLValue,
        field: &str,
    ) -> Result<(ir::Value, FieldLayout, ir::Value), String> {
        let (container, container_ty) = self.emit_lvalue_value(base)?;
        let Ty::Named(def_id, _) = &container_ty else {
            return Err(format!(
                "field assignment `{field}` requires a struct value, got `{}`",
                container_ty.display()
            ));
        };
        let layout = self
            .struct_layouts
            .get(def_id)
            .cloned()
            .ok_or_else(|| format!("missing native layout for `{}`", container_ty.display()))?;
        let field_layout = layout
            .field(field)
            .cloned()
            .ok_or_else(|| format!("layout is missing field `{field}`"))?;
        let addr = self
            .builder
            .ins()
            .iadd_imm(container, i64::from(field_layout.offset));
        Ok((addr, field_layout, container))
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
        let gv = *self
            .string_gvs
            .get(value)
            .ok_or_else(|| format!("string literal `{value}` was not emitted in native codegen"))?;
        let base = self.builder.ins().global_value(self.ptr_ty, gv);
        // Skip the 16-byte header to point to the string payload
        Ok(self.builder.ins().iadd_imm(base, 16))
    }

    fn bytes_ptr(&mut self, bytes: &[u8]) -> Result<ir::Value, String> {
        let total = bytes.len() + 1;
        let base = self.malloc_bytes(total as u32)?;

        for (idx, byte) in bytes.iter().enumerate() {
            let value = self.builder.ins().iconst(types::I8, i64::from(*byte as i8));
            self.builder
                .ins()
                .store(MemFlags::new(), value, base, idx as i32);
        }
        let nul = self.builder.ins().iconst(types::I8, 0);
        self.builder
            .ins()
            .store(MemFlags::new(), nul, base, bytes.len() as i32);

        Ok(base)
    }

    fn call_string_parts_function(
        &mut self,
        function_name: &str,
        value: ir::Value,
    ) -> Result<StringParts, String> {
        let fref = *self
            .func_refs
            .get(function_name)
            .ok_or_else(|| format!("missing runtime function `{function_name}`"))?;
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

    fn int_to_string_parts(&mut self, value: ir::Value) -> Result<StringParts, String> {
        self.call_string_parts_function("ori_to_string_parts", value)
    }

    fn float_to_string_parts(&mut self, value: ir::Value) -> Result<StringParts, String> {
        self.call_string_parts_function("ori_float_to_string_parts", value)
    }

    fn bool_to_string_parts(&mut self, value: ir::Value) -> Result<StringParts, String> {
        self.call_string_parts_function("ori_bool_to_string_parts", value)
    }

    fn emit_to_string_call_parts(&mut self, expr: &HirExpr) -> Result<Option<StringParts>, String> {
        let HirExprKind::Call { callee, args } = &expr.kind else {
            return Ok(None);
        };
        let HirExprKind::Var(name) = &callee.kind else {
            return Ok(None);
        };
        let parts_function = match name.as_str() {
            "ori_to_string" => "ori_to_string_parts",
            "ori_float_to_string" => "ori_float_to_string_parts",
            "ori_bool_to_string" => "ori_bool_to_string_parts",
            _ => return Ok(None),
        };
        let Some(arg) = args.first() else {
            return Ok(None);
        };
        let value = self.emit_expr(&arg.value)?;
        let value = match &arg.value.ty {
            Ty::Int8 | Ty::Int16 | Ty::Int32 => self.builder.ins().sextend(types::I64, value),
            Ty::U8 | Ty::U16 | Ty::U32 => self.builder.ins().uextend(types::I64, value),
            Ty::Float32 if parts_function == "ori_float_to_string_parts" => {
                self.builder.ins().fpromote(types::F64, value)
            }
            _ => value,
        };
        self.call_string_parts_function(parts_function, value)
            .map(Some)
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
            Ty::Float | Ty::Float64 => self.float_to_string_parts(value),
            Ty::Float32 => {
                let widened = self.builder.ins().fpromote(types::F64, value);
                self.float_to_string_parts(widened)
            }
            Ty::Bool => self.bool_to_string_parts(value),
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
            return Err(format!(
                "using cleanup for `{}` has no disposable function name",
                cleanup.ty.display()
            ));
        };
        let Some(&dispose_ref) = self.func_refs.get(func_name.as_str()) else {
            return Err(format!(
                "using cleanup for `{}` could not find native function `{}`",
                cleanup.ty.display(),
                func_name
            ));
        };
        let value = self.builder.use_var(cleanup.var);
        self.builder.ins().call(dispose_ref, &[value]);
        Ok(())
    }

    fn dispose_func_name_for_ty(&self, ty: &Ty) -> Option<SmolStr> {
        match ty {
            Ty::Named(def_id, _) => {
                let mut matches = Vec::new();
                for ((trait_def_id, impl_type_def_id), impl_sig) in self.trait_impls {
                    if impl_type_def_id != def_id {
                        continue;
                    }
                    let Some(trait_sig) = self.trait_layouts.get(trait_def_id) else {
                        continue;
                    };
                    if !trait_sig.name.ends_with(".Disposable") {
                        continue;
                    }
                    if let Some(method) = impl_sig
                        .methods
                        .iter()
                        .find(|method| method.name == "dispose")
                    {
                        matches.push(method.func_name.clone());
                    }
                }
                if matches.len() == 1 {
                    return matches.pop();
                }
                self.type_names
                    .get(def_id)
                    .map(|name| SmolStr::new(format!("{name}.dispose")))
            }
            _ => None,
        }
    }

    fn trait_method_func_name_for_type(
        &self,
        type_def_id: ori_types::DefId,
        method_name: &SmolStr,
    ) -> Option<SmolStr> {
        let mut matches = Vec::new();
        for ((_, impl_type_def_id), impl_sig) in self.trait_impls {
            if *impl_type_def_id != type_def_id {
                continue;
            }
            if let Some(method) = impl_sig
                .methods
                .iter()
                .find(|method| method.name == *method_name)
            {
                matches.push(method.func_name.clone());
            }
        }
        (matches.len() == 1).then(|| matches.remove(0))
    }

    fn iterable_next_func_name_for_type(&self, ty: &Ty) -> Option<SmolStr> {
        let Ty::Named(type_def_id, _) = ty else {
            return None;
        };
        self.trait_impls
            .iter()
            .filter(|((_, impl_type_def_id), _)| impl_type_def_id == type_def_id)
            .find_map(|((trait_def_id, _), impl_sig)| {
                let trait_sig = self.trait_layouts.get(trait_def_id)?;
                if !trait_sig.name.ends_with(".Iterable") {
                    return None;
                }
                impl_sig
                    .methods
                    .iter()
                    .find(|method| method.name == "next")
                    .map(|method| method.func_name.clone())
            })
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

    fn emit_arc_register_edge(&mut self, owner: ir::Value, child: ir::Value) -> Result<(), String> {
        let Some(&register_ref) = self.func_refs.get("ori_arc_register_edge") else {
            return Ok(());
        };
        self.builder.ins().call(register_ref, &[owner, child]);
        Ok(())
    }

    fn emit_arc_unregister_edge(
        &mut self,
        owner: ir::Value,
        child: ir::Value,
    ) -> Result<(), String> {
        let Some(&unregister_ref) = self.func_refs.get("ori_arc_unregister_edge") else {
            return Ok(());
        };
        self.builder.ins().call(unregister_ref, &[owner, child]);
        Ok(())
    }

    fn emit_arc_register_edge_if_managed(
        &mut self,
        ty: &Ty,
        owner: ir::Value,
        child: ir::Value,
    ) -> Result<(), String> {
        if !is_managed_ty(ty) {
            return Ok(());
        }
        self.emit_arc_register_edge(owner, child)
    }

    fn emit_arc_update_edge_if_managed(
        &mut self,
        ty: &Ty,
        owner: ir::Value,
        old_child: ir::Value,
        new_child: ir::Value,
    ) -> Result<(), String> {
        if !is_managed_ty(ty) {
            return Ok(());
        }
        let Some(&update_ref) = self.func_refs.get("ori_arc_update_edge") else {
            return Ok(());
        };
        self.builder
            .ins()
            .call(update_ref, &[owner, old_child, new_child]);
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
        if let Ty::Future(inner) = return_ty {
            let inner_ty = inner.as_ref().clone();
            let value = match val {
                Some(expr) if inner_ty == Ty::Void => {
                    self.emit_expr(expr)?;
                    None
                }
                Some(expr) => Some(self.emit_expr_for_expected(expr, &inner_ty)?),
                None => None,
            };
            let future = self.emit_future_ready(&inner_ty, value)?;
            self.emit_arc_retain_if_managed(&Ty::Future(Box::new(inner_ty)), future)?;
            self.emit_scope_cleanup_calls_from(0, 0)?;
            self.builder.ins().return_(&[future]);
            self.terminated = true;
            return Ok(());
        }
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

    fn emit_future_ready(
        &mut self,
        inner_ty: &Ty,
        value: Option<ir::Value>,
    ) -> Result<ir::Value, String> {
        let runtime_name = match inner_ty {
            Ty::Void | Ty::Never => "ori_future_ready_void",
            Ty::Float | Ty::Float64 | Ty::Float32 => "ori_future_ready_f64",
            Ty::String
            | Ty::Bytes
            | Ty::Func { .. }
            | Ty::Lazy(_)
            | Ty::Future(_)
            | Ty::TaskJob(_)
            | Ty::Channel(_)
            | Ty::AtomicInt
            | Ty::TaskJoinError
            | Ty::ChannelSendError
            | Ty::ChannelReceiveError
            | Ty::Any(_)
            | Ty::Optional(_)
            | Ty::Result(_, _)
            | Ty::List(_)
            | Ty::Map(_, _)
            | Ty::Set(_)
            | Ty::Range(_)
            | Ty::Tuple(_)
            | Ty::Named(_, _) => "ori_future_ready_ptr",
            _ => "ori_future_ready_i64",
        };
        let fref = *self
            .func_refs
            .get(runtime_name)
            .ok_or_else(|| format!("missing runtime function `{runtime_name}`"))?;
        let args = match runtime_name {
            "ori_future_ready_void" => Vec::new(),
            "ori_future_ready_f64" => {
                let mut value = value.unwrap_or_else(|| self.builder.ins().f64const(0.0));
                if self.builder.func.dfg.value_type(value) == types::F32 {
                    value = self.builder.ins().fpromote(types::F64, value);
                }
                vec![value]
            }
            "ori_future_ready_ptr" => {
                vec![value.unwrap_or_else(|| self.builder.ins().iconst(self.ptr_ty, 0))]
            }
            _ => {
                let mut value = value.unwrap_or_else(|| self.builder.ins().iconst(types::I64, 0));
                let actual_ty = self.builder.func.dfg.value_type(value);
                if actual_ty != types::I64 {
                    value = match inner_ty {
                        Ty::Bool | Ty::U8 | Ty::U16 | Ty::U32 => {
                            self.builder.ins().uextend(types::I64, value)
                        }
                        Ty::Int8 | Ty::Int16 | Ty::Int32 => {
                            self.builder.ins().sextend(types::I64, value)
                        }
                        _ => value,
                    };
                }
                vec![value]
            }
        };
        let call = self.builder.ins().call(fref, &args);
        Ok(self.builder.inst_results(call)[0])
    }

    fn emit_future_complete(
        &mut self,
        future: ir::Value,
        inner_ty: &Ty,
        value: Option<ir::Value>,
    ) -> Result<(), String> {
        let runtime_name = match inner_ty {
            Ty::Void | Ty::Never => "ori_future_complete_void",
            Ty::Float | Ty::Float64 | Ty::Float32 => "ori_future_complete_f64",
            Ty::String
            | Ty::Bytes
            | Ty::Func { .. }
            | Ty::Lazy(_)
            | Ty::Future(_)
            | Ty::TaskJob(_)
            | Ty::Channel(_)
            | Ty::AtomicInt
            | Ty::TaskJoinError
            | Ty::ChannelSendError
            | Ty::ChannelReceiveError
            | Ty::Any(_)
            | Ty::Optional(_)
            | Ty::Result(_, _)
            | Ty::List(_)
            | Ty::Map(_, _)
            | Ty::Set(_)
            | Ty::Range(_)
            | Ty::Tuple(_)
            | Ty::Named(_, _) => "ori_future_complete_ptr",
            _ => "ori_future_complete_i64",
        };
        let fref = *self
            .func_refs
            .get(runtime_name)
            .ok_or_else(|| format!("missing runtime function `{runtime_name}`"))?;
        let mut args = vec![future];
        match runtime_name {
            "ori_future_complete_void" => {}
            "ori_future_complete_f64" => {
                let mut value = value.unwrap_or_else(|| self.builder.ins().f64const(0.0));
                if self.builder.func.dfg.value_type(value) == types::F32 {
                    value = self.builder.ins().fpromote(types::F64, value);
                }
                args.push(value);
            }
            "ori_future_complete_ptr" => {
                args.push(value.unwrap_or_else(|| self.builder.ins().iconst(self.ptr_ty, 0)));
            }
            _ => {
                let mut value = value.unwrap_or_else(|| self.builder.ins().iconst(types::I64, 0));
                let actual_ty = self.builder.func.dfg.value_type(value);
                if actual_ty != types::I64 {
                    value = match inner_ty {
                        Ty::Bool | Ty::U8 | Ty::U16 | Ty::U32 => {
                            self.builder.ins().uextend(types::I64, value)
                        }
                        Ty::Int8 | Ty::Int16 | Ty::Int32 => {
                            self.builder.ins().sextend(types::I64, value)
                        }
                        _ => value,
                    };
                }
                args.push(value);
            }
        }
        self.builder.ins().call(fref, &args);
        Ok(())
    }

    fn emit_future_value_read(
        &mut self,
        future: ir::Value,
        result_ty: &Ty,
    ) -> Result<ir::Value, String> {
        let runtime_name = match result_ty {
            Ty::Float | Ty::Float64 | Ty::Float32 => "ori_future_value_f64",
            Ty::String
            | Ty::Bytes
            | Ty::Func { .. }
            | Ty::Lazy(_)
            | Ty::Future(_)
            | Ty::TaskJob(_)
            | Ty::Channel(_)
            | Ty::AtomicInt
            | Ty::TaskJoinError
            | Ty::ChannelSendError
            | Ty::ChannelReceiveError
            | Ty::Any(_)
            | Ty::Optional(_)
            | Ty::Result(_, _)
            | Ty::List(_)
            | Ty::Map(_, _)
            | Ty::Set(_)
            | Ty::Range(_)
            | Ty::Tuple(_)
            | Ty::Named(_, _) => "ori_future_value_ptr",
            _ => "ori_future_value_i64",
        };
        let fref = *self
            .func_refs
            .get(runtime_name)
            .ok_or_else(|| format!("missing runtime function `{runtime_name}`"))?;
        let call = self.builder.ins().call(fref, &[future]);
        let mut value = self.builder.inst_results(call)[0];
        if matches!(runtime_name, "ori_future_value_i64") {
            value = self.from_list_storage_value(value, result_ty);
        } else if matches!(result_ty, Ty::Float32) {
            value = self.builder.ins().fdemote(types::F32, value);
        }
        Ok(value)
    }

    fn emit_simple_async_step(
        mut self,
        f: &HirFunc,
        plan: &SimpleAsyncStateMachinePlan,
    ) -> Result<(), String> {
        let entry = self.builder.create_block();
        self.builder.append_block_params_for_function_params(entry);
        self.builder.switch_to_block(entry);
        self.builder.seal_block(entry);
        let frame = self.builder.block_params(entry)[0];
        let state =
            self.builder
                .ins()
                .load(types::I64, MemFlags::new(), frame, ASYNC_FRAME_STATE_OFFSET);
        for (index, param) in plan.params.iter().enumerate() {
            let cl_ty = cl_type(&param.ty, self.ptr_ty)
                .ok_or_else(|| format!("async param `{}` has no native value", param.name))?;
            let offset = simple_async_frame_param_offset(plan, index, self.ptr_ty)
                .expect("async frame param offset");
            let value = self
                .builder
                .ins()
                .load(cl_ty, MemFlags::new(), frame, offset as i32);
            let var = self.builder.declare_var(cl_ty);
            self.builder.def_var(var, value);
            self.vars
                .insert(param.name.clone(), (var, param.ty.clone()));
        }
        for (index, local) in plan.locals.iter().enumerate() {
            let cl_ty = cl_type(&local.ty, self.ptr_ty)
                .ok_or_else(|| format!("async local `{}` has no native value", local.name))?;
            let offset = simple_async_frame_local_offset(plan, index, self.ptr_ty)
                .expect("async frame local offset");
            let value = self
                .builder
                .ins()
                .load(cl_ty, MemFlags::new(), frame, offset as i32);
            let var = self.builder.declare_var(cl_ty);
            self.builder.def_var(var, value);
            self.vars
                .insert(local.name.clone(), (var, local.ty.clone()));
        }
        let await_count = plan.awaits.len();
        let eval_blocks: Vec<_> = (0..await_count)
            .map(|_| self.builder.create_block())
            .collect();
        let poll_blocks: Vec<_> = (0..await_count)
            .map(|_| self.builder.create_block())
            .collect();
        let pending_blocks: Vec<_> = (0..await_count)
            .map(|_| self.builder.create_block())
            .collect();
        let status_blocks: Vec<_> = (0..await_count)
            .map(|_| self.builder.create_block())
            .collect();
        let ready_blocks: Vec<_> = (0..await_count)
            .map(|_| self.builder.create_block())
            .collect();
        let non_pending_status_blocks: Vec<_> = (0..await_count)
            .map(|_| self.builder.create_block())
            .collect();
        let failed_blocks: Vec<_> = (0..await_count)
            .map(|_| self.builder.create_block())
            .collect();
        let cancelled_blocks: Vec<_> = (0..await_count)
            .map(|_| self.builder.create_block())
            .collect();
        let dispatch_blocks: Vec<_> = (0..=await_count)
            .map(|_| self.builder.create_block())
            .collect();
        let invalid_state_block = self.builder.create_block();
        let complete_block = self.builder.create_block();

        self.builder.ins().jump(dispatch_blocks[0], &[]);
        for state_index in 0..=await_count {
            self.builder.switch_to_block(dispatch_blocks[state_index]);
            let is_state =
                self.builder
                    .ins()
                    .icmp_imm(ir::condcodes::IntCC::Equal, state, state_index as i64);
            let target = if state_index == 0 {
                eval_blocks[0]
            } else {
                poll_blocks[state_index - 1]
            };
            let next = if state_index < await_count {
                dispatch_blocks[state_index + 1]
            } else {
                invalid_state_block
            };
            self.builder.ins().brif(is_state, target, &[], next, &[]);
        }

        for (index, step) in plan.awaits.iter().enumerate() {
            let awaited_offset = simple_async_frame_awaited_offset(index, self.ptr_ty);

            self.builder.switch_to_block(eval_blocks[index]);
            if index == 0 {
                for (local_index, local) in plan.locals.iter().enumerate() {
                    let value = self.emit_expr_for_expected(&local.value, &local.ty)?;
                    let offset = simple_async_frame_local_offset(plan, local_index, self.ptr_ty)
                        .expect("async frame local offset");
                    self.builder
                        .ins()
                        .store(MemFlags::new(), value, frame, offset as i32);
                    self.emit_arc_register_edge_if_managed(&local.ty, frame, value)?;
                    if let Some((var, _)) = self.vars.get(&local.name).cloned() {
                        self.builder.def_var(var, value);
                    }
                }
            }
            let awaited = self.emit_expr(&step.await_future)?;
            self.builder
                .ins()
                .store(MemFlags::new(), awaited, frame, awaited_offset);
            self.emit_arc_register_edge(frame, awaited)?;
            self.emit_arc_release_if_managed(&step.await_future.ty, awaited)?;
            self.builder.ins().jump(poll_blocks[index], &[]);

            self.builder.switch_to_block(poll_blocks[index]);
            let awaited =
                self.builder
                    .ins()
                    .load(self.ptr_ty, MemFlags::new(), frame, awaited_offset);
            let poll_ref = *self
                .func_refs
                .get("ori_future_poll")
                .ok_or_else(|| "missing runtime function `ori_future_poll`".to_string())?;
            let poll_call = self.builder.ins().call(poll_ref, &[awaited]);
            let status = self.builder.inst_results(poll_call)[0];
            let ready = self
                .builder
                .ins()
                .icmp_imm(ir::condcodes::IntCC::Equal, status, 1);
            self.builder
                .ins()
                .brif(ready, ready_blocks[index], &[], status_blocks[index], &[]);

            self.builder.switch_to_block(ready_blocks[index]);
            let ready_continue_block = self.builder.create_block();
            if let Some(result_ty) = &step.propagate_result_ty {
                let result_value = self.emit_future_value_read(awaited, result_ty)?;
                let flag = self
                    .builder
                    .ins()
                    .load(types::I8, MemFlags::new(), result_value, 0);
                let ok_block = self.builder.create_block();
                let err_block = self.builder.create_block();
                self.builder.ins().brif(flag, ok_block, &[], err_block, &[]);

                self.builder.switch_to_block(err_block);
                let result_future = self.builder.ins().load(
                    self.ptr_ty,
                    MemFlags::new(),
                    frame,
                    ASYNC_FRAME_RESULT_OFFSET,
                );
                self.emit_future_complete(result_future, &plan.inner_ty, Some(result_value))?;
                self.emit_arc_unregister_edge(frame, awaited)?;
                self.emit_simple_async_frame_cleanup(plan, frame, index, true)?;
                let zero = self.builder.ins().iconst(types::I64, 0);
                self.builder.ins().return_(&[zero]);

                self.builder.switch_to_block(ok_block);
                if step.binding.is_none() {
                    return Err("propagating await step has no binding".to_string());
                }
                let Ty::Result(ok_ty, err_ty) = result_ty else {
                    return Err("propagating await step requires result type".to_string());
                };
                let (pay_off, _, _) = result_layout(ok_ty, err_ty, self.ptr_ty);
                let cl_ty = cl_type(ok_ty, self.ptr_ty).unwrap_or(types::I64);
                let value =
                    self.builder
                        .ins()
                        .load(cl_ty, MemFlags::new(), result_value, pay_off as i32);
                let offset = simple_async_frame_binding_offset(plan, index, self.ptr_ty)
                    .expect("async frame binding offset");
                self.builder
                    .ins()
                    .store(MemFlags::new(), value, frame, offset as i32);
                self.emit_arc_register_edge_if_managed(ok_ty, frame, value)?;
                self.builder.ins().jump(ready_continue_block, &[]);
            } else if let Some(binding) = &step.binding {
                let value = self.emit_future_value_read(awaited, &binding.ty)?;
                let offset = simple_async_frame_binding_offset(plan, index, self.ptr_ty)
                    .expect("async frame binding offset");
                self.builder
                    .ins()
                    .store(MemFlags::new(), value, frame, offset as i32);
                self.emit_arc_register_edge_if_managed(&binding.ty, frame, value)?;
                self.builder.ins().jump(ready_continue_block, &[]);
            } else {
                self.builder.ins().jump(ready_continue_block, &[]);
            }
            self.builder.switch_to_block(ready_continue_block);
            self.emit_arc_unregister_edge(frame, awaited)?;
            self.emit_simple_async_drop_dead_frame_values_after_await(
                plan,
                frame,
                index,
                index + 1,
            )?;
            if index + 1 < await_count {
                self.builder.ins().jump(eval_blocks[index + 1], &[]);
            } else {
                self.builder.ins().jump(complete_block, &[]);
            }

            self.builder.switch_to_block(status_blocks[index]);
            let pending = self
                .builder
                .ins()
                .icmp_imm(ir::condcodes::IntCC::Equal, status, 0);
            self.builder.ins().brif(
                pending,
                pending_blocks[index],
                &[],
                non_pending_status_blocks[index],
                &[],
            );

            self.builder.switch_to_block(pending_blocks[index]);
            let next_state = self.builder.ins().iconst(types::I64, (index + 1) as i64);
            self.builder
                .ins()
                .store(MemFlags::new(), next_state, frame, ASYNC_FRAME_STATE_OFFSET);
            let step_name = async_step_name(f);
            let step_ref = *self
                .func_refs
                .get(step_name.as_str())
                .ok_or_else(|| format!("missing async step function `{step_name}`"))?;
            let continuation = self.emit_closure_object(step_ref, Some(frame))?;
            let on_ready_ref = *self
                .func_refs
                .get("ori_future_on_ready")
                .ok_or_else(|| "missing runtime function `ori_future_on_ready`".to_string())?;
            self.builder
                .ins()
                .call(on_ready_ref, &[awaited, continuation]);
            self.emit_simple_async_drop_dead_frame_values_after_await(plan, frame, index, index)?;
            let zero = self.builder.ins().iconst(types::I64, 0);
            self.builder.ins().return_(&[zero]);

            self.builder
                .switch_to_block(non_pending_status_blocks[index]);
            let failed = self
                .builder
                .ins()
                .icmp_imm(ir::condcodes::IntCC::Equal, status, 2);
            self.builder.ins().brif(
                failed,
                failed_blocks[index],
                &[],
                cancelled_blocks[index],
                &[],
            );

            self.builder.switch_to_block(cancelled_blocks[index]);
            let result_future = self.builder.ins().load(
                self.ptr_ty,
                MemFlags::new(),
                frame,
                ASYNC_FRAME_RESULT_OFFSET,
            );
            let cancel_ref = *self
                .func_refs
                .get("ori_future_cancel")
                .ok_or_else(|| "missing runtime function `ori_future_cancel`".to_string())?;
            self.emit_arc_unregister_edge(frame, awaited)?;
            self.builder.ins().call(cancel_ref, &[result_future]);
            self.emit_simple_async_frame_cleanup(plan, frame, index, true)?;
            let zero = self.builder.ins().iconst(types::I64, 0);
            self.builder.ins().return_(&[zero]);

            self.builder.switch_to_block(failed_blocks[index]);
            let result_future = self.builder.ins().load(
                self.ptr_ty,
                MemFlags::new(),
                frame,
                ASYNC_FRAME_RESULT_OFFSET,
            );
            let fail_ref = *self
                .func_refs
                .get("ori_future_fail")
                .ok_or_else(|| "missing runtime function `ori_future_fail`".to_string())?;
            self.emit_arc_unregister_edge(frame, awaited)?;
            self.builder.ins().call(fail_ref, &[result_future]);
            self.emit_simple_async_frame_cleanup(plan, frame, index, true)?;
            let zero = self.builder.ins().iconst(types::I64, 0);
            self.builder.ins().return_(&[zero]);
        }

        self.builder.switch_to_block(invalid_state_block);
        let result_future = self.builder.ins().load(
            self.ptr_ty,
            MemFlags::new(),
            frame,
            ASYNC_FRAME_RESULT_OFFSET,
        );
        let cancel_ref = *self
            .func_refs
            .get("ori_future_cancel")
            .ok_or_else(|| "missing runtime function `ori_future_cancel`".to_string())?;
        self.builder.ins().call(cancel_ref, &[result_future]);
        self.emit_simple_async_frame_cleanup(plan, frame, await_count, true)?;
        let zero = self.builder.ins().iconst(types::I64, 0);
        self.builder.ins().return_(&[zero]);

        self.builder.switch_to_block(complete_block);
        let result_future = self.builder.ins().load(
            self.ptr_ty,
            MemFlags::new(),
            frame,
            ASYNC_FRAME_RESULT_OFFSET,
        );
        for (index, step) in plan.awaits.iter().enumerate() {
            if let Some(binding) = &step.binding {
                let cl_ty = cl_type(&binding.ty, self.ptr_ty).ok_or_else(|| {
                    format!("async binding `{}` has no native value", binding.name)
                })?;
                let offset = simple_async_frame_binding_offset(plan, index, self.ptr_ty)
                    .expect("async frame binding offset");
                let value = self
                    .builder
                    .ins()
                    .load(cl_ty, MemFlags::new(), frame, offset as i32);
                let var = self.builder.declare_var(cl_ty);
                self.builder.def_var(var, value);
                self.vars
                    .insert(binding.name.clone(), (var, binding.ty.clone()));
            }
        }
        for stmt in &plan.tail_stmts {
            self.emit_stmt(stmt)?;
        }
        if let Some(expr) = &plan.tail_expr {
            self.emit_expr(expr)?;
            self.emit_future_complete(result_future, &plan.inner_ty, None)?;
        } else {
            let return_value = plan
                .return_expr
                .as_ref()
                .map(|expr| self.emit_expr_for_expected(expr, &plan.inner_ty))
                .transpose()?;
            if let Some(value) = return_value {
                self.emit_arc_retain_if_managed(&plan.inner_ty, value)?;
                self.emit_future_complete(result_future, &plan.inner_ty, Some(value))?;
                self.emit_arc_release_if_managed(&plan.inner_ty, value)?;
            } else {
                self.emit_future_complete(result_future, &plan.inner_ty, None)?;
            }
        }
        self.emit_scope_cleanup_calls_from(0, 0)?;
        self.emit_simple_async_frame_cleanup(plan, frame, await_count, true)?;
        let zero = self.builder.ins().iconst(types::I64, 0);
        self.builder.ins().return_(&[zero]);

        self.builder.seal_all_blocks();
        self.builder.finalize();
        Ok(())
    }

    fn emit_simple_async_frame_cleanup(
        &mut self,
        plan: &SimpleAsyncStateMachinePlan,
        frame: ir::Value,
        initialized_bindings: usize,
        include_result_future: bool,
    ) -> Result<(), String> {
        if include_result_future {
            let result_future = self.builder.ins().load(
                self.ptr_ty,
                MemFlags::new(),
                frame,
                ASYNC_FRAME_RESULT_OFFSET,
            );
            self.emit_arc_unregister_edge(frame, result_future)?;
        }
        for (index, param) in plan.params.iter().enumerate() {
            if !is_managed_ty(&param.ty) {
                continue;
            }
            let offset = simple_async_frame_param_offset(plan, index, self.ptr_ty)
                .expect("async frame param offset");
            let value = self
                .builder
                .ins()
                .load(self.ptr_ty, MemFlags::new(), frame, offset as i32);
            self.emit_arc_unregister_edge(frame, value)?;
        }
        for (index, local) in plan.locals.iter().enumerate() {
            if !is_managed_ty(&local.ty) {
                continue;
            }
            let offset = simple_async_frame_local_offset(plan, index, self.ptr_ty)
                .expect("async frame local offset");
            let value = self
                .builder
                .ins()
                .load(self.ptr_ty, MemFlags::new(), frame, offset as i32);
            self.emit_arc_unregister_edge(frame, value)?;
        }
        for (index, step) in plan.awaits.iter().take(initialized_bindings).enumerate() {
            let Some(binding) = &step.binding else {
                continue;
            };
            if !is_managed_ty(&binding.ty) {
                continue;
            }
            let offset = simple_async_frame_binding_offset(plan, index, self.ptr_ty)
                .expect("async frame binding offset");
            let value = self
                .builder
                .ins()
                .load(self.ptr_ty, MemFlags::new(), frame, offset as i32);
            self.emit_arc_unregister_edge(frame, value)?;
        }
        Ok(())
    }

    fn emit_simple_async_drop_dead_frame_values_after_await(
        &mut self,
        plan: &SimpleAsyncStateMachinePlan,
        frame: ir::Value,
        await_index: usize,
        initialized_bindings: usize,
    ) -> Result<(), String> {
        let live_after = simple_async_uses_after_await(plan, await_index);
        let mut dropped_any = false;

        for (index, param) in plan.params.iter().enumerate() {
            if !is_managed_ty(&param.ty) || live_after.contains(&param.name) {
                continue;
            }
            let offset = simple_async_frame_param_offset(plan, index, self.ptr_ty)
                .expect("async frame param offset");
            self.emit_simple_async_drop_frame_edge(frame, offset as i32)?;
            dropped_any = true;
        }

        for (index, local) in plan.locals.iter().enumerate() {
            if !is_managed_ty(&local.ty) || live_after.contains(&local.name) {
                continue;
            }
            let offset = simple_async_frame_local_offset(plan, index, self.ptr_ty)
                .expect("async frame local offset");
            self.emit_simple_async_drop_frame_edge(frame, offset as i32)?;
            dropped_any = true;
        }

        for (index, step) in plan.awaits.iter().take(initialized_bindings).enumerate() {
            let Some(binding) = &step.binding else {
                continue;
            };
            if !is_managed_ty(&binding.ty) || live_after.contains(&binding.name) {
                continue;
            }
            let offset = simple_async_frame_binding_offset(plan, index, self.ptr_ty)
                .expect("async frame binding offset");
            self.emit_simple_async_drop_frame_edge(frame, offset as i32)?;
            dropped_any = true;
        }

        if dropped_any {
            self.emit_arc_collect_cycles()?;
        }
        Ok(())
    }

    fn emit_simple_async_drop_frame_edge(
        &mut self,
        frame: ir::Value,
        offset: i32,
    ) -> Result<(), String> {
        let value = self
            .builder
            .ins()
            .load(self.ptr_ty, MemFlags::new(), frame, offset);
        self.emit_arc_unregister_edge(frame, value)?;
        let zero = self.builder.ins().iconst(self.ptr_ty, 0);
        self.builder
            .ins()
            .store(MemFlags::new(), zero, frame, offset);
        Ok(())
    }

    fn emit_await(&mut self, _future: &HirExpr, _result_ty: &Ty) -> Result<ir::Value, String> {
        Err(native_codegen_unsupported(
            "`await` must be lowered by the native async state machine; use `task.block_on(...)` only as an explicit synchronous bridge outside async lowering",
        ))
    }

    fn emit_never_call_stmt(&mut self, expr: &HirExpr) -> Result<bool, String> {
        if !expr.ty.is_never() {
            return Ok(false);
        }
        let HirExprKind::Call { callee, args } = &expr.kind else {
            return Ok(false);
        };
        let HirExprKind::Var(name) = &callee.kind else {
            return Ok(false);
        };
        if !matches!(name.as_str(), "ori_panic" | "ori_test_fail") {
            return Ok(false);
        }

        let mut args_v = Vec::new();
        for arg in args {
            args_v.push(self.emit_expr_for_expected(&arg.value, &Ty::String)?);
        }
        self.emit_scope_cleanup_calls_from(0, 0)?;
        let fref = *self
            .func_refs
            .get(name.as_str())
            .ok_or_else(|| format!("missing runtime function `{name}`"))?;
        self.builder.ins().call(fref, &args_v);
        let trap = ir::TrapCode::user(2)
            .ok_or_else(|| "invalid panic fallback trap code `2`".to_string())?;
        self.builder.ins().trap(trap);
        self.terminated = true;
        Ok(true)
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
                        if is_managed_ty(&elem_ty) {
                            let get_ref = *self.func_refs.get("ori_list_get").ok_or_else(|| {
                                "missing runtime function `ori_list_get`".to_string()
                            })?;
                            let old_call = self.builder.ins().call(get_ref, &[container, idx]);
                            let old = self.builder.inst_results(old_call)[0];
                            let old = self.from_list_storage_value(old, &elem_ty);
                            self.emit_arc_update_edge_if_managed(&elem_ty, container, old, val)?;
                        }
                        let stored = self.to_list_storage_value(val, &elem_ty);
                        let set_ref = *self
                            .func_refs
                            .get("ori_list_set")
                            .ok_or_else(|| "missing runtime function `ori_list_set`".to_string())?;
                        self.builder.ins().call(set_ref, &[container, idx, stored]);
                    }
                } else if let HirLValue::Field { base, field } = lvalue {
                    let (addr, field_layout, owner) = self.emit_field_lvalue_addr(base, field)?;
                    let val = self.emit_expr_for_expected(value, &field_layout.ty)?;
                    if let Some(contract) = &field_layout.contract {
                        self.emit_value_contract(&field_layout.ty, val, contract, 3, false)?;
                    }
                    let cl_ty = cl_type(&field_layout.ty, self.ptr_ty)
                        .ok_or_else(|| format!("missing Cranelift type for field `{field}`"))?;
                    let old = self.builder.ins().load(cl_ty, MemFlags::new(), addr, 0);
                    self.emit_arc_update_edge_if_managed(&field_layout.ty, owner, old, val)?;
                    self.builder.ins().store(MemFlags::new(), val, addr, 0);
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
                if !self.emit_never_call_stmt(e)? {
                    self.emit_expr(e)?;
                }
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
                let trap = ir::TrapCode::user(1)
                    .ok_or_else(|| "invalid runtime check trap code `1`".to_string())?;
                self.emit_trap_unless(cv, trap, true)?;
            }
            HirStmt::Repeat { count, body, .. } => {
                // Desugar: for (int64_t _i = 0; _i < count; _i++) { body }
                let count_v = self.emit_expr(count)?;
                let zero = self.builder.ins().iconst(types::I64, 0);
                let non_negative = self.builder.ins().icmp(
                    ir::condcodes::IntCC::SignedGreaterThanOrEqual,
                    count_v,
                    zero,
                );
                let trap = ir::TrapCode::user(4)
                    .ok_or_else(|| "invalid repeat-count trap code `4`".to_string())?;
                self.emit_trap_unless(non_negative, trap, true)?;

                let idx_var = self.builder.declare_var(types::I64);
                let loop_zero = self.builder.ins().iconst(types::I64, 0);
                self.builder.def_var(idx_var, loop_zero);

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
            _ if matches!(&iterable.ty, Ty::Bytes) => {
                self.emit_for_bytes(binding, index_binding, iterable, body)
            }
            _ if matches!(&iterable.ty, Ty::Opaque { kind, .. } if kind.is_list_backed_collection()) =>
            {
                let Ty::Opaque { kind, args } = &iterable.ty else {
                    unreachable!();
                };
                let runtime_name = match kind {
                    OpaqueTy::Deque => "ori_deque_to_list",
                    OpaqueTy::Queue => "ori_queue_to_list",
                    OpaqueTy::Stack => "ori_stack_to_list",
                    OpaqueTy::LinkedList => "ori_linked_list_to_list",
                    OpaqueTy::DoublyLinkedList => "ori_doubly_linked_list_to_list",
                    _ => unreachable!(),
                };
                let fref = *self
                    .func_refs
                    .get(runtime_name)
                    .ok_or_else(|| format!("missing runtime function `{runtime_name}`"))?;
                let handle = self.emit_expr(iterable)?;
                let call = self.builder.ins().call(fref, &[handle]);
                let snapshot = self.builder.inst_results(call)[0];
                let elem = args.first().cloned().unwrap_or(Ty::Infer(0));
                self.emit_for_list_value(binding, index_binding, &elem, snapshot, body)?;
                let free_ref = *self
                    .func_refs
                    .get("ori_list_free")
                    .ok_or_else(|| "missing runtime function `ori_list_free`".to_string())?;
                self.builder.ins().call(free_ref, &[snapshot]);
                Ok(())
            }
            _ if matches!(
                &iterable.ty,
                Ty::Opaque {
                    kind: OpaqueTy::Heap,
                    ..
                }
            ) =>
            {
                let Ty::Opaque { args, .. } = &iterable.ty else {
                    unreachable!();
                };
                let fref = *self
                    .func_refs
                    .get("ori_heap_to_list")
                    .ok_or_else(|| "missing runtime function `ori_heap_to_list`".to_string())?;
                let handle = self.emit_expr(iterable)?;
                let call = self.builder.ins().call(fref, &[handle]);
                let snapshot = self.builder.inst_results(call)[0];
                let elem = args.first().cloned().unwrap_or(Ty::Infer(0));
                self.emit_for_list_value(binding, index_binding, &elem, snapshot, body)?;
                let free_ref = *self
                    .func_refs
                    .get("ori_list_free")
                    .ok_or_else(|| "missing runtime function `ori_list_free`".to_string())?;
                self.builder.ins().call(free_ref, &[snapshot]);
                Ok(())
            }
            _ if matches!(
                &iterable.ty,
                Ty::Opaque {
                    kind: OpaqueTy::Graph,
                    ..
                }
            ) =>
            {
                let Ty::Opaque { args, .. } = &iterable.ty else {
                    unreachable!();
                };
                let fref = *self
                    .func_refs
                    .get("ori_graph_nodes")
                    .ok_or_else(|| "missing runtime function `ori_graph_nodes`".to_string())?;
                let handle = self.emit_expr(iterable)?;
                let call = self.builder.ins().call(fref, &[handle]);
                let snapshot = self.builder.inst_results(call)[0];
                let elem = args.first().cloned().unwrap_or(Ty::Infer(0));
                self.emit_for_list_value(binding, index_binding, &elem, snapshot, body)?;
                let free_ref = *self
                    .func_refs
                    .get("ori_list_free")
                    .ok_or_else(|| "missing runtime function `ori_list_free`".to_string())?;
                self.builder.ins().call(free_ref, &[snapshot]);
                Ok(())
            }
            _ if matches!(
                &iterable.ty,
                Ty::Opaque {
                    kind: OpaqueTy::HashTable,
                    ..
                }
            ) =>
            {
                let Ty::Opaque { args, .. } = &iterable.ty else {
                    unreachable!();
                };
                let fref = *self
                    .func_refs
                    .get("ori_hash_table_keys")
                    .ok_or_else(|| "missing runtime function `ori_hash_table_keys`".to_string())?;
                let handle = self.emit_expr(iterable)?;
                let call = self.builder.ins().call(fref, &[handle]);
                let snapshot = self.builder.inst_results(call)[0];
                let elem = args.first().cloned().unwrap_or(Ty::Infer(0));
                self.emit_for_list_value(binding, index_binding, &elem, snapshot, body)?;
                let free_ref = *self
                    .func_refs
                    .get("ori_list_free")
                    .ok_or_else(|| "missing runtime function `ori_list_free`".to_string())?;
                self.builder.ins().call(free_ref, &[snapshot]);
                Ok(())
            }
            _ => {
                if let Some(next_func) = self.iterable_next_func_name_for_type(&iterable.ty) {
                    self.emit_for_custom_iterable(
                        binding,
                        index_binding,
                        elem_ty,
                        iterable,
                        body,
                        &next_func,
                    )
                } else {
                    Err(native_codegen_unsupported(format!(
                        "`for` iterable type `{}`",
                        iterable.ty.display()
                    )))
                }
            }
        }
    }

    fn emit_for_custom_iterable(
        &mut self,
        binding: &SmolStr,
        index_binding: Option<&SmolStr>,
        elem_ty: &Ty,
        iterable: &HirExpr,
        body: &HirBlock,
        next_func: &SmolStr,
    ) -> Result<(), String> {
        let Some(iter_cl_ty) = cl_type(&iterable.ty, self.ptr_ty) else {
            return Err(native_codegen_unsupported(format!(
                "`for` iterable type `{}`",
                iterable.ty.display()
            )));
        };
        let Some(elem_cl_ty) = cl_type(elem_ty, self.ptr_ty) else {
            return Err(native_codegen_unsupported(format!(
                "`for` element type `{}`",
                elem_ty.display()
            )));
        };
        let next_ref = *self.func_refs.get(next_func.as_str()).ok_or_else(|| {
            format!("missing function reference `{next_func}` for custom Iterable")
        })?;
        let iter_value = self.emit_expr(iterable)?;
        let iter_var = self.builder.declare_var(iter_cl_ty);
        self.builder.def_var(iter_var, iter_value);
        let idx_var = self.builder.declare_var(types::I64);
        let zero = self.builder.ins().iconst(types::I64, 0);
        self.builder.def_var(idx_var, zero);
        let binding_var = self.builder.declare_var(elem_cl_ty);
        self.vars
            .insert(binding.clone(), (binding_var, elem_ty.clone()));

        let header = self.builder.create_block();
        let body_b = self.builder.create_block();
        let step = self.builder.create_block();
        let exit = self.builder.create_block();
        self.builder.ins().jump(header, &[]);
        self.builder.switch_to_block(header);
        let iter_current = self.builder.use_var(iter_var);
        let call = self.builder.ins().call(next_ref, &[iter_current]);
        let opt_ptr = self.builder.inst_results(call)[0];
        let has_val = self
            .builder
            .ins()
            .load(types::I8, MemFlags::new(), opt_ptr, 0);
        let has_val_bool = self
            .builder
            .ins()
            .icmp_imm(ir::condcodes::IntCC::NotEqual, has_val, 0);
        let idx_current = self.builder.use_var(idx_var);
        let idx_non_negative = self.builder.ins().icmp_imm(
            ir::condcodes::IntCC::SignedGreaterThanOrEqual,
            idx_current,
            0,
        );
        let keep_going = self.builder.ins().band(has_val_bool, idx_non_negative);
        self.builder.ins().brif(keep_going, body_b, &[], exit, &[]);

        self.builder.seal_block(body_b);
        self.builder.switch_to_block(body_b);
        let (val_off, _) = optional_layout(elem_ty, self.ptr_ty);
        let item = self
            .builder
            .ins()
            .load(elem_cl_ty, MemFlags::new(), opt_ptr, val_off as i32);
        self.builder.def_var(binding_var, item);
        if let Some(ib_name) = index_binding {
            let cur = self.builder.use_var(idx_var);
            let ib_var = self.builder.declare_var(types::I64);
            self.builder.def_var(ib_var, cur);
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
        self.emit_for_list_value(binding, index_binding, elem_ty, list_v, body)
    }

    fn emit_for_list_value(
        &mut self,
        binding: &SmolStr,
        index_binding: Option<&SmolStr>,
        elem_ty: &Ty,
        list_v: ir::Value,
        body: &HirBlock,
    ) -> Result<(), String> {
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

    fn emit_for_bytes(
        &mut self,
        binding: &SmolStr,
        index_binding: Option<&SmolStr>,
        iterable: &HirExpr,
        body: &HirBlock,
    ) -> Result<(), String> {
        let bytes_v = self.emit_expr(iterable)?;
        let len_ref = *self
            .func_refs
            .get("ori_bytes_len")
            .ok_or_else(|| "missing runtime function `ori_bytes_len`".to_string())?;
        let get_ref = *self
            .func_refs
            .get("ori_bytes_get")
            .ok_or_else(|| "missing runtime function `ori_bytes_get`".to_string())?;

        let len_call = self.builder.ins().call(len_ref, &[bytes_v]);
        let len_v = self.builder.inst_results(len_call)[0];
        let idx_var = self.builder.declare_var(types::I64);
        let zero = self.builder.ins().iconst(types::I64, 0);
        self.builder.def_var(idx_var, zero);
        let len_var = self.builder.declare_var(types::I64);
        self.builder.def_var(len_var, len_v);

        // Bind as U8 (i8 in cranelift)
        let bvar = self.builder.declare_var(types::I8);
        self.vars.insert(binding.clone(), (bvar, Ty::U8));

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
        let call = self.builder.ins().call(get_ref, &[bytes_v, cur2]);
        let elem = self.builder.inst_results(call)[0];
        self.builder.def_var(bvar, elem);

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
                self.emit_arc_register_edge_if_managed(inner_ty, base, val)?;
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
                self.emit_arc_register_edge_if_managed(ok_ty, base, val)?;
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
                self.emit_arc_register_edge_if_managed(err_ty, base, val)?;
                base
            }
            HirExprKind::StrLit(s) => self.string_ptr(s.as_str())?,
            HirExprKind::BytesLit(bytes) => self.bytes_ptr(bytes)?,
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
                if matches!(op, BinaryOp::And | BinaryOp::Or) {
                    return self.emit_short_circuit_binary(*op, lhs, rhs);
                }
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
                    if name.as_str() == "ori_lazy_once" && args.len() == 1 {
                        return self.emit_lazy_once(&args[0].value, &expr.ty);
                    }
                    if name.as_str() == "ori_lazy_force" && args.len() == 1 {
                        return self.emit_lazy_force(&args[0].value, &expr.ty);
                    }
                    // ori_io_print takes (ptr: *u8, len: i64) — build args accordingly
                    if matches!(name.as_str(), "ori_test_assert_eq" | "ori_test_assert_ne") {
                        return self.emit_test_assert_equality_call(name.as_str(), args);
                    }
                    if matches!(
                        name.as_str(),
                        "ori_map_set"
                            | "ori_map_get"
                            | "ori_map_try_get"
                            | "ori_map_contains"
                            | "ori_map_remove"
                            | "ori_map_try_remove"
                            | "ori_map_from_entries"
                    ) {
                        return self.emit_map_runtime_call(name.as_str(), args);
                    }
                    if matches!(
                        name.as_str(),
                        "ori_hash_table_set"
                            | "ori_hash_table_get"
                            | "ori_hash_table_remove"
                            | "ori_hash_table_contains"
                    ) {
                        return self.emit_hash_table_runtime_call(name.as_str(), args);
                    }
                    if matches!(
                        name.as_str(),
                        "ori_linked_list_find" | "ori_doubly_linked_list_find"
                    ) {
                        return self.emit_linked_list_find_runtime_call(name.as_str(), args);
                    }
                    if matches!(
                        name.as_str(),
                        "ori_graph_add_node"
                            | "ori_graph_remove_node"
                            | "ori_graph_add_edge"
                            | "ori_graph_add_weighted_edge"
                            | "ori_graph_remove_edge"
                            | "ori_graph_has_node"
                            | "ori_graph_has_edge"
                            | "ori_graph_edge_weight"
                            | "ori_graph_neighbors"
                            | "ori_graph_bfs"
                            | "ori_graph_dfs"
                            | "ori_graph_shortest_path"
                            | "ori_graph_shortest_weighted_path"
                    ) {
                        return self.emit_graph_runtime_call(name.as_str(), args);
                    }
                    if matches!(
                        name.as_str(),
                        "ori_set_add"
                            | "ori_set_contains"
                            | "ori_set_remove"
                            | "ori_set_try_remove"
                            | "ori_set_from_list"
                    ) {
                        return self.emit_set_runtime_call(name.as_str(), args);
                    }
                    if matches!(
                        name.as_str(),
                        "ori_tree_new"
                            | "ori_tree_root"
                            | "ori_tree_value"
                            | "ori_tree_try_value"
                            | "ori_tree_contains_node"
                            | "ori_tree_set_value"
                            | "ori_tree_add_child"
                            | "ori_tree_children"
                            | "ori_tree_parent"
                            | "ori_tree_remove_subtree"
                            | "ori_tree_move_subtree"
                            | "ori_tree_find"
                            | "ori_tree_len"
                            | "ori_tree_depth"
                            | "ori_tree_pre_order"
                            | "ori_tree_post_order"
                            | "ori_tree_breadth_first"
                            | "ori_tree_clone"
                            | "ori_tree_clone_subtree"
                    ) {
                        return self.emit_tree_runtime_call(name.as_str(), args);
                    }
                    if matches!(
                        name.as_str(),
                        "ori_heap_new"
                            | "ori_heap_push"
                            | "ori_heap_pop"
                            | "ori_heap_peek"
                            | "ori_heap_len"
                            | "ori_heap_is_empty"
                            | "ori_heap_clear"
                            | "ori_heap_clone"
                            | "ori_heap_to_list"
                            | "ori_heap_from_list"
                            | "ori_heap_merge"
                            | "ori_heap_remove"
                            | "ori_heap_into_sorted_list"
                    ) {
                        return self.emit_heap_runtime_call(name.as_str(), args, &expr.ty);
                    }
                    if name == "ori_io_print" || name == "ori_io_eprint" {
                        if let Some(&fref) = self.func_refs.get(name.as_str()) {
                            let mut cl_args = Vec::new();
                            for a in args {
                                // ori_io_print always takes (ptr, len); string-like args
                                // use length-aware parts or the Ori runtime length helper.
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
                        if matches!(name.as_str(), "ori_iter_sort" | "ori_iter_unique")
                            && args.len() == 1
                            && matches!(
                                &args[0].value.ty,
                                Ty::List(elem) if matches!(elem.as_ref(), Ty::String)
                            )
                        {
                            let runtime_name = if name.as_str() == "ori_iter_sort" {
                                "ori_iter_sort_string"
                            } else {
                                "ori_iter_unique_string"
                            };
                            let list_v = self.emit_expr(&args[0].value)?;
                            let fref = *self.func_refs.get(runtime_name).ok_or_else(|| {
                                format!("missing runtime function `{runtime_name}`")
                            })?;
                            let call = self.builder.ins().call(fref, &[list_v]);
                            return Ok(self.builder.inst_results(call)[0]);
                        }
                        // Special-case: iter/list helpers pass closure as (fn_ptr, env_ptr).
                        if matches!(
                            name.as_str(),
                            "ori_list_map"
                                | "ori_list_filter"
                                | "ori_iter_flat_map"
                                | "ori_iter_any"
                                | "ori_iter_all"
                                | "ori_iter_count_where"
                                | "ori_iter_find"
                                | "ori_iter_partition"
                                | "ori_iter_group_by"
                        ) && args.len() == 2
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
                            if self.func_refs.get(name.as_str()).is_some() {
                                let runtime_name = if name.as_str() == "ori_iter_group_by"
                                    && matches!(
                                        &expr.ty,
                                        Ty::Map(key, _) if matches!(key.as_ref(), Ty::String)
                                    ) {
                                    "ori_iter_group_by_string"
                                } else {
                                    name.as_str()
                                };
                                let fref = *self.func_refs.get(runtime_name).ok_or_else(|| {
                                    format!("missing runtime function `{runtime_name}`")
                                })?;
                                let call =
                                    self.builder.ins().call(fref, &[list_v, fn_ptr, env_ptr]);
                                let res = self.builder.inst_results(call);
                                return Ok(if res.is_empty() {
                                    self.builder.ins().iconst(types::I8, 0)
                                } else {
                                    res[0]
                                });
                            }
                        }
                        if name.as_str() == "ori_iter_reduce"
                            && args.len() == 3
                            && matches!(&args[2].value.ty, Ty::Func { .. })
                        {
                            let list_v = self.emit_expr(&args[0].value)?;
                            let initial_v = self.emit_expr(&args[1].value)?;
                            let closure_ptr = self.emit_expr(&args[2].value)?;
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
                                let call = self
                                    .builder
                                    .ins()
                                    .call(fref, &[list_v, initial_v, fn_ptr, env_ptr]);
                                let res = self.builder.inst_results(call);
                                return Ok(if res.is_empty() {
                                    self.builder.ins().iconst(types::I8, 0)
                                } else {
                                    res[0]
                                });
                            }
                        }
                        if name.as_str() == "ori_iter_sort_by"
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
                                let call =
                                    self.builder.ins().call(fref, &[list_v, fn_ptr, env_ptr]);
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
                                let retain_ty = if expected.contains_infer() {
                                    &arg.value.ty
                                } else {
                                    &expected
                                };
                                self.emit_arc_retain_if_managed(retain_ty, value)?;
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
            HirExprKind::Await(inner) => self.emit_await(inner, &expr.ty)?,
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
                                self.emit_arc_register_edge_if_managed(&fi.ty, base, val)?;
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
                    Ty::Bytes => {
                        let get_ref = *self.func_refs.get("ori_bytes_get").ok_or_else(|| {
                            "missing runtime function `ori_bytes_get`".to_string()
                        })?;
                        let call = self.builder.ins().call(get_ref, &[container, idx]);
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
                                self.emit_arc_register_edge_if_managed(&fi.ty, base, val)?;
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

                for (e, (offset, elem_ty)) in elems.iter().zip(layout.iter()) {
                    let v = self.emit_expr(e)?;
                    vals_and_offsets.push((v, *offset, elem_ty.clone()));
                }
                let base = self.malloc_bytes(total)?;

                for (v, off, elem_ty) in vals_and_offsets {
                    self.builder
                        .ins()
                        .store(MemFlags::new(), v, base, off as i32);
                    self.emit_arc_register_edge_if_managed(&elem_ty, base, v)?;
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
                let set_symbol = if matches!(&key_ty, Ty::String) {
                    "ori_map_set_string"
                } else {
                    "ori_map_set"
                };
                if let Some(&set_ref) = self.func_refs.get(set_symbol) {
                    for (k, v) in entries {
                        let key_value = self.emit_expr(k)?;
                        let map_value = self.emit_expr(v)?;
                        let kv = self.to_list_storage_value(key_value, &key_ty);
                        let vv = self.to_list_storage_value(map_value, &value_ty);
                        self.builder.ins().call(set_ref, &[map_ptr, kv, vv]);
                        self.emit_arc_register_edge_if_managed(&key_ty, map_ptr, key_value)?;
                        self.emit_arc_register_edge_if_managed(&value_ty, map_ptr, map_value)?;
                    }
                } else {
                    return Err(format!("missing runtime function `{set_symbol}`"));
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
                let elem_ty = if let Ty::Set(elem_ty) = &expr.ty {
                    elem_ty.as_ref()
                } else {
                    &Ty::Int
                };
                let add_symbol = if matches!(elem_ty, Ty::String) {
                    "ori_set_add_string"
                } else {
                    "ori_set_add"
                };
                if let Some(&add_ref) = self.func_refs.get(add_symbol) {
                    for elem in elements {
                        let v = self.emit_expr_for_expected(elem, elem_ty)?;
                        let stored = self.to_list_storage_value(v, elem_ty);
                        self.builder.ins().call(add_ref, &[set_ptr, stored]);
                        self.emit_arc_register_edge_if_managed(&elem.ty, set_ptr, v)?;
                    }
                } else {
                    return Err(format!("missing runtime function `{add_symbol}`"));
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
                    let updated_names: Vec<_> =
                        updates.iter().map(|(name, _)| name.clone()).collect();
                    // Copy all bytes from base
                    for (fname, fl) in &layout.fields {
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
                            if !updated_names.iter().any(|name| name == fname) {
                                self.emit_arc_register_edge_if_managed(&fl.ty, new_ptr, val)?;
                            }
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
                                self.emit_arc_register_edge_if_managed(&fi.ty, new_ptr, val)?;
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
                    "__slice" => {
                        let runtime_name = match &receiver.ty {
                            Ty::String => "ori_string_slice",
                            Ty::List(_) => "ori_list_slice",
                            Ty::Bytes => "ori_bytes_slice",
                            other => {
                                return Err(format!(
                                    "native range slice codegen is missing for type `{}`",
                                    other.display()
                                ))
                            }
                        };
                        let slice_ref = *self
                            .func_refs
                            .get(runtime_name)
                            .ok_or_else(|| format!("missing runtime function `{runtime_name}`"))?;
                        let start = self.emit_expr(&args[0])?;
                        let end = self.emit_expr(&args[1])?;
                        let call = self.builder.ins().call(slice_ref, &[recv, start, end]);
                        self.builder.inst_results(call)[0]
                    }
                    _ => {
                        // Generic trait calls are lowered while the receiver may still be
                        // a type parameter. After monomorphization the receiver is concrete,
                        // so resolve the trait implementation symbol for that concrete type.
                        let target_method = match &receiver.ty {
                            Ty::Named(type_def_id, _) => self
                                .trait_method_func_name_for_type(*type_def_id, method)
                                .unwrap_or_else(|| method.clone()),
                            _ => method.clone(),
                        };
                        let mut all_args = vec![recv];
                        self.emit_arc_retain_if_managed(&receiver.ty, recv)?;
                        for a in args {
                            let value = self.emit_expr(a)?;
                            self.emit_arc_retain_if_managed(&a.ty, value)?;
                            all_args.push(value);
                        }
                        if let Some(&fref) = self.func_refs.get(target_method.as_str()) {
                            let call = self.builder.ins().call(fref, &all_args);
                            let res = self.builder.inst_results(call);
                            if res.is_empty() {
                                self.builder.ins().iconst(types::I8, 0)
                            } else {
                                res[0]
                            }
                        } else {
                            return Err(format!(
                                "missing function reference `{target_method}` in native codegen"
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
                        let vtable = self.builder.ins().load(
                            self.ptr_ty,
                            MemFlags::new(),
                            val,
                            ptr_size as i32,
                        );
                        let actual_type_id =
                            self.builder
                                .ins()
                                .load(self.ptr_ty, MemFlags::new(), vtable, 0);
                        let expected_type_id = self
                            .builder
                            .ins()
                            .iconst(self.ptr_ty, check_def_id.0 as i64);

                        let is_match = self.builder.ins().icmp(
                            ir::condcodes::IntCC::Equal,
                            actual_type_id,
                            expected_type_id,
                        );
                        is_match
                    } else if let Ty::Named(actual_def_id, _) = &value.ty {
                        let is_match = actual_def_id.0 == check_def_id.0;
                        self.builder
                            .ins()
                            .iconst(types::I8, if is_match { 1 } else { 0 })
                    } else {
                        self.builder.ins().iconst(types::I8, 0)
                    }
                } else {
                    self.builder
                        .ins()
                        .iconst(types::I8, if value.ty == *check_ty { 1 } else { 0 })
                }
            }
        })
    }

    /// Compute the byte length of a nul-terminated Ori string pointer as an i64.
    /// This is used by print/interpolation paths that write raw bytes.
    fn str_len_from_ptr(&mut self, ptr: ir::Value) -> Result<ir::Value, String> {
        if let Some(&fref) = self.func_refs.get("strlen") {
            let call = self.builder.ins().call(fref, &[ptr]);
            return Ok(self.builder.inst_results(call)[0]);
        }
        if let Some(&fref) = self.func_refs.get("ori_string_len") {
            let call = self.builder.ins().call(fref, &[ptr]);
            return Ok(self.builder.inst_results(call)[0]);
        }
        Err("missing runtime function `ori_string_len` or `strlen`".to_string())
    }

    fn emit_test_assert_equality_call(
        &mut self,
        name: &str,
        args: &[HirArg],
    ) -> Result<ir::Value, String> {
        if args.len() != 2 {
            return Err(format!("{name} expects two arguments"));
        }
        let ty = &args[0].value.ty;
        let is_ne = name == "ori_test_assert_ne";
        let runtime_name = match ty {
            Ty::String => {
                if is_ne {
                    "ori_test_assert_ne_string"
                } else {
                    "ori_test_assert_eq_string"
                }
            }
            Ty::Float | Ty::Float32 | Ty::Float64 => {
                if is_ne {
                    "ori_test_assert_ne_float"
                } else {
                    "ori_test_assert_eq_float"
                }
            }
            Ty::Bool => {
                if is_ne {
                    "ori_test_assert_ne_bool"
                } else {
                    "ori_test_assert_eq_bool"
                }
            }
            _ => name,
        };
        let fref = *self
            .func_refs
            .get(runtime_name)
            .ok_or_else(|| format!("missing runtime function `{runtime_name}`"))?;
        let mut left = self.emit_expr_for_expected(&args[0].value, ty)?;
        let mut right = self.emit_expr_for_expected(&args[1].value, ty)?;
        match ty {
            Ty::Float32 => {
                left = self.builder.ins().fpromote(types::F64, left);
                right = self.builder.ins().fpromote(types::F64, right);
            }
            Ty::Int8 | Ty::Int16 | Ty::Int32 => {
                left = self.builder.ins().sextend(types::I64, left);
                right = self.builder.ins().sextend(types::I64, right);
            }
            Ty::U8 | Ty::U16 | Ty::U32 => {
                left = self.builder.ins().uextend(types::I64, left);
                right = self.builder.ins().uextend(types::I64, right);
            }
            _ => {}
        }
        self.builder.ins().call(fref, &[left, right]);
        Ok(self.builder.ins().iconst(types::I8, 0))
    }

    fn emit_map_runtime_call(&mut self, name: &str, args: &[HirArg]) -> Result<ir::Value, String> {
        if name == "ori_map_from_entries" {
            if args.len() != 1 {
                return Err("ori_map_from_entries expects one entries list".to_string());
            }
            let key_ty = match &args[0].value.ty {
                Ty::List(inner) => match inner.as_ref() {
                    Ty::Tuple(items) if items.len() == 2 => items[0].clone(),
                    _ => Ty::Infer(0),
                },
                _ => Ty::Infer(0),
            };
            let runtime_name = if matches!(key_ty, Ty::String) {
                "ori_map_from_entries_string"
            } else {
                name
            };
            let fref = *self
                .func_refs
                .get(runtime_name)
                .ok_or_else(|| format!("missing runtime function `{runtime_name}`"))?;
            let entries = self.emit_expr(&args[0].value)?;
            let call = self.builder.ins().call(fref, &[entries]);
            return Ok(self.builder.inst_results(call)[0]);
        }
        let Some(first_arg) = args.first() else {
            return Err(format!("map runtime call `{name}` expects a map argument"));
        };
        let Ty::Map(key_ty, value_ty) = &first_arg.value.ty else {
            return Err(format!(
                "map runtime call `{name}` received `{}`",
                first_arg.value.ty.display()
            ));
        };
        let key_ty = key_ty.as_ref();
        let value_ty = value_ty.as_ref();
        let map_v = self.emit_expr(&first_arg.value)?;
        let runtime_name = match (name, key_ty) {
            ("ori_map_set", Ty::String) => "ori_map_set_string",
            ("ori_map_get", Ty::String) => "ori_map_get_string",
            ("ori_map_try_get", Ty::String) => "ori_map_try_get_string",
            ("ori_map_contains", Ty::String) => "ori_map_contains_string",
            ("ori_map_remove", Ty::String) => "ori_map_remove_string",
            ("ori_map_try_remove", Ty::String) => "ori_map_try_remove_string",
            ("ori_map_from_entries", Ty::String) => "ori_map_from_entries_string",
            _ => name,
        };
        let fref = *self
            .func_refs
            .get(runtime_name)
            .ok_or_else(|| format!("missing runtime function `{runtime_name}`"))?;
        match name {
            "ori_map_set" => {
                if args.len() != 3 {
                    return Err("ori_map_set expects map, key, and value".to_string());
                }
                let key_value = self.emit_expr_for_expected(&args[1].value, key_ty)?;
                let map_value = self.emit_expr_for_expected(&args[2].value, value_ty)?;
                let stored_key = self.to_list_storage_value(key_value, key_ty);
                let stored_value = self.to_list_storage_value(map_value, value_ty);
                self.builder
                    .ins()
                    .call(fref, &[map_v, stored_key, stored_value]);
                self.emit_arc_register_edge_if_managed(key_ty, map_v, key_value)?;
                self.emit_arc_register_edge_if_managed(value_ty, map_v, map_value)?;
                Ok(self.builder.ins().iconst(types::I8, 0))
            }
            "ori_map_get" => {
                if args.len() != 2 {
                    return Err("ori_map_get expects map and key".to_string());
                }
                let key_value = self.emit_expr_for_expected(&args[1].value, key_ty)?;
                let stored_key = self.to_list_storage_value(key_value, key_ty);
                let call = self.builder.ins().call(fref, &[map_v, stored_key]);
                let stored_value = self.builder.inst_results(call)[0];
                Ok(self.from_list_storage_value(stored_value, value_ty))
            }
            "ori_map_try_get" | "ori_map_try_remove" => {
                if args.len() != 2 {
                    return Err(format!("{name} expects map and key"));
                }
                let key_value = self.emit_expr_for_expected(&args[1].value, key_ty)?;
                let stored_key = self.to_list_storage_value(key_value, key_ty);
                let call = self.builder.ins().call(fref, &[map_v, stored_key]);
                Ok(self.builder.inst_results(call)[0])
            }
            "ori_map_contains" => {
                if args.len() != 2 {
                    return Err("ori_map_contains expects map and key".to_string());
                }
                let key_value = self.emit_expr_for_expected(&args[1].value, key_ty)?;
                let stored_key = self.to_list_storage_value(key_value, key_ty);
                let call = self.builder.ins().call(fref, &[map_v, stored_key]);
                Ok(self.builder.inst_results(call)[0])
            }
            "ori_map_remove" => {
                if args.len() != 2 {
                    return Err("ori_map_remove expects map and key".to_string());
                }
                let key_value = self.emit_expr_for_expected(&args[1].value, key_ty)?;
                let stored_key = self.to_list_storage_value(key_value, key_ty);
                self.builder.ins().call(fref, &[map_v, stored_key]);
                Ok(self.builder.ins().iconst(types::I8, 0))
            }
            _ => Err(native_codegen_unsupported(format!(
                "map runtime call `{name}`"
            ))),
        }
    }

    fn emit_hash_table_runtime_call(
        &mut self,
        name: &str,
        args: &[HirArg],
    ) -> Result<ir::Value, String> {
        let Some(first_arg) = args.first() else {
            return Err(format!(
                "hash_table runtime call `{name}` expects a table argument"
            ));
        };
        let Ty::Opaque {
            kind: OpaqueTy::HashTable,
            args: table_args,
        } = &first_arg.value.ty
        else {
            return Err(format!(
                "hash_table runtime call `{name}` received `{}`",
                first_arg.value.ty.display()
            ));
        };
        if table_args.len() != 2 {
            return Err(format!(
                "hash_table runtime call `{name}` received malformed table type"
            ));
        }
        let key_ty = &table_args[0];
        let value_ty = &table_args[1];
        let table_v = self.emit_expr(&first_arg.value)?;
        let runtime_name = match (name, key_ty) {
            ("ori_hash_table_set", Ty::String) => "ori_hash_table_set_string",
            ("ori_hash_table_get", Ty::String) => "ori_hash_table_get_string",
            ("ori_hash_table_remove", Ty::String) => "ori_hash_table_remove_string",
            ("ori_hash_table_contains", Ty::String) => "ori_hash_table_contains_string",
            _ => name,
        };
        let fref = *self
            .func_refs
            .get(runtime_name)
            .ok_or_else(|| format!("missing runtime function `{runtime_name}`"))?;
        match name {
            "ori_hash_table_set" => {
                if args.len() != 3 {
                    return Err("ori_hash_table_set expects table, key, and value".to_string());
                }
                let key_value = self.emit_expr_for_expected(&args[1].value, key_ty)?;
                let table_value = self.emit_expr_for_expected(&args[2].value, value_ty)?;
                let stored_key = self.to_list_storage_value(key_value, key_ty);
                let stored_value = self.to_list_storage_value(table_value, value_ty);
                self.builder
                    .ins()
                    .call(fref, &[table_v, stored_key, stored_value]);
                self.emit_arc_register_edge_if_managed(key_ty, table_v, key_value)?;
                self.emit_arc_register_edge_if_managed(value_ty, table_v, table_value)?;
                Ok(self.builder.ins().iconst(types::I8, 0))
            }
            "ori_hash_table_get" | "ori_hash_table_remove" => {
                if args.len() != 2 {
                    return Err(format!("{name} expects table and key"));
                }
                let key_value = self.emit_expr_for_expected(&args[1].value, key_ty)?;
                let stored_key = self.to_list_storage_value(key_value, key_ty);
                let call = self.builder.ins().call(fref, &[table_v, stored_key]);
                Ok(self.builder.inst_results(call)[0])
            }
            "ori_hash_table_contains" => {
                if args.len() != 2 {
                    return Err("ori_hash_table_contains expects table and key".to_string());
                }
                let key_value = self.emit_expr_for_expected(&args[1].value, key_ty)?;
                let stored_key = self.to_list_storage_value(key_value, key_ty);
                let call = self.builder.ins().call(fref, &[table_v, stored_key]);
                Ok(self.builder.inst_results(call)[0])
            }
            _ => Err(native_codegen_unsupported(format!(
                "hash_table runtime call `{name}`"
            ))),
        }
    }

    fn emit_linked_list_find_runtime_call(
        &mut self,
        name: &str,
        args: &[HirArg],
    ) -> Result<ir::Value, String> {
        if args.len() != 2 {
            return Err(format!("{name} expects list and value"));
        }
        let elem_ty = match &args[0].value.ty {
            Ty::Opaque {
                kind: OpaqueTy::LinkedList | OpaqueTy::DoublyLinkedList,
                args,
            } => args.first().cloned().unwrap_or(Ty::Infer(0)),
            other => {
                return Err(format!(
                    "linked-list find runtime call `{name}` received `{}`",
                    other.display()
                ))
            }
        };
        let runtime_name = match (name, &elem_ty) {
            ("ori_linked_list_find", Ty::String) => "ori_linked_list_find_string",
            ("ori_doubly_linked_list_find", Ty::String) => "ori_doubly_linked_list_find_string",
            _ => name,
        };
        let fref = *self
            .func_refs
            .get(runtime_name)
            .ok_or_else(|| format!("missing runtime function `{runtime_name}`"))?;
        let list = self.emit_expr(&args[0].value)?;
        let value = self.emit_expr_for_expected(&args[1].value, &elem_ty)?;
        let stored = self.to_list_storage_value(value, &elem_ty);
        let call = self.builder.ins().call(fref, &[list, stored]);
        Ok(self.builder.inst_results(call)[0])
    }

    fn emit_graph_runtime_call(
        &mut self,
        name: &str,
        args: &[HirArg],
    ) -> Result<ir::Value, String> {
        let Some(first_arg) = args.first() else {
            return Err(format!(
                "graph runtime call `{name}` expects a graph argument"
            ));
        };
        let Ty::Opaque {
            kind: OpaqueTy::Graph,
            args: graph_args,
        } = &first_arg.value.ty
        else {
            return Err(format!(
                "graph runtime call `{name}` received `{}`",
                first_arg.value.ty.display()
            ));
        };
        let node_ty = graph_args.first().cloned().unwrap_or(Ty::Infer(0));
        let runtime_name = match (name, &node_ty) {
            ("ori_graph_add_node", Ty::String) => "ori_graph_add_node_string",
            ("ori_graph_remove_node", Ty::String) => "ori_graph_remove_node_string",
            ("ori_graph_add_edge", Ty::String) => "ori_graph_add_edge_string",
            ("ori_graph_add_weighted_edge", Ty::String) => "ori_graph_add_weighted_edge_string",
            ("ori_graph_remove_edge", Ty::String) => "ori_graph_remove_edge_string",
            ("ori_graph_has_node", Ty::String) => "ori_graph_has_node_string",
            ("ori_graph_has_edge", Ty::String) => "ori_graph_has_edge_string",
            ("ori_graph_edge_weight", Ty::String) => "ori_graph_edge_weight_string",
            ("ori_graph_neighbors", Ty::String) => "ori_graph_neighbors_string",
            ("ori_graph_bfs", Ty::String) => "ori_graph_bfs_string",
            ("ori_graph_dfs", Ty::String) => "ori_graph_dfs_string",
            ("ori_graph_shortest_path", Ty::String) => "ori_graph_shortest_path_string",
            ("ori_graph_shortest_weighted_path", Ty::String) => {
                "ori_graph_shortest_weighted_path_string"
            }
            _ => name,
        };
        let fref = *self
            .func_refs
            .get(runtime_name)
            .ok_or_else(|| format!("missing runtime function `{runtime_name}`"))?;
        let graph = self.emit_expr(&first_arg.value)?;
        match name {
            "ori_graph_add_node" => {
                if args.len() != 2 {
                    return Err("ori_graph_add_node expects graph and node".to_string());
                }
                let node = self.emit_expr_for_expected(&args[1].value, &node_ty)?;
                let stored = self.to_list_storage_value(node, &node_ty);
                self.builder.ins().call(fref, &[graph, stored]);
                Ok(self.builder.ins().iconst(types::I8, 0))
            }
            "ori_graph_remove_node"
            | "ori_graph_has_node"
            | "ori_graph_neighbors"
            | "ori_graph_bfs"
            | "ori_graph_dfs" => {
                if args.len() != 2 {
                    return Err(format!("{name} expects graph and node"));
                }
                let node = self.emit_expr_for_expected(&args[1].value, &node_ty)?;
                let stored = self.to_list_storage_value(node, &node_ty);
                let call = self.builder.ins().call(fref, &[graph, stored]);
                let res = self.builder.inst_results(call);
                Ok(res
                    .first()
                    .copied()
                    .unwrap_or_else(|| self.builder.ins().iconst(types::I8, 0)))
            }
            "ori_graph_add_edge" => {
                if args.len() != 3 {
                    return Err("ori_graph_add_edge expects graph, from, and to".to_string());
                }
                let from = self.emit_expr_for_expected(&args[1].value, &node_ty)?;
                let to = self.emit_expr_for_expected(&args[2].value, &node_ty)?;
                let stored_from = self.to_list_storage_value(from, &node_ty);
                let stored_to = self.to_list_storage_value(to, &node_ty);
                self.builder
                    .ins()
                    .call(fref, &[graph, stored_from, stored_to]);
                Ok(self.builder.ins().iconst(types::I8, 0))
            }
            "ori_graph_add_weighted_edge" => {
                if args.len() != 4 {
                    return Err(
                        "ori_graph_add_weighted_edge expects graph, from, to, and weight"
                            .to_string(),
                    );
                }
                let from = self.emit_expr_for_expected(&args[1].value, &node_ty)?;
                let to = self.emit_expr_for_expected(&args[2].value, &node_ty)?;
                let weight = self.emit_expr_for_expected(&args[3].value, &Ty::Int)?;
                let stored_from = self.to_list_storage_value(from, &node_ty);
                let stored_to = self.to_list_storage_value(to, &node_ty);
                self.builder
                    .ins()
                    .call(fref, &[graph, stored_from, stored_to, weight]);
                Ok(self.builder.ins().iconst(types::I8, 0))
            }
            "ori_graph_remove_edge"
            | "ori_graph_has_edge"
            | "ori_graph_edge_weight"
            | "ori_graph_shortest_path"
            | "ori_graph_shortest_weighted_path" => {
                if args.len() != 3 {
                    return Err(format!("{name} expects graph, from, and to"));
                }
                let from = self.emit_expr_for_expected(&args[1].value, &node_ty)?;
                let to = self.emit_expr_for_expected(&args[2].value, &node_ty)?;
                let stored_from = self.to_list_storage_value(from, &node_ty);
                let stored_to = self.to_list_storage_value(to, &node_ty);
                let call = self
                    .builder
                    .ins()
                    .call(fref, &[graph, stored_from, stored_to]);
                let res = self.builder.inst_results(call);
                Ok(res
                    .first()
                    .copied()
                    .unwrap_or_else(|| self.builder.ins().iconst(types::I8, 0)))
            }
            _ => Err(native_codegen_unsupported(format!(
                "graph runtime call `{name}`"
            ))),
        }
    }

    fn emit_set_runtime_call(&mut self, name: &str, args: &[HirArg]) -> Result<ir::Value, String> {
        if name == "ori_set_from_list" {
            if args.len() != 1 {
                return Err("ori_set_from_list expects one source list".to_string());
            }
            let elem_ty = match &args[0].value.ty {
                Ty::List(elem) => elem.as_ref(),
                other => {
                    return Err(format!(
                        "ori_set_from_list expects list input, got `{}`",
                        other.display()
                    ))
                }
            };
            let runtime_name = if matches!(elem_ty, Ty::String) {
                "ori_set_from_list_string"
            } else {
                name
            };
            let fref = *self
                .func_refs
                .get(runtime_name)
                .ok_or_else(|| format!("missing runtime function `{runtime_name}`"))?;
            let source = self.emit_expr(&args[0].value)?;
            let call = self.builder.ins().call(fref, &[source]);
            return Ok(self.builder.inst_results(call)[0]);
        }
        let Some(first_arg) = args.first() else {
            return Err(format!("set runtime call `{name}` expects a set argument"));
        };
        let Ty::Set(elem_ty) = &first_arg.value.ty else {
            return Err(format!(
                "set runtime call `{name}` received `{}`",
                first_arg.value.ty.display()
            ));
        };
        let elem_ty = elem_ty.as_ref();
        let set_v = self.emit_expr(&first_arg.value)?;
        let runtime_name = match (name, elem_ty) {
            ("ori_set_add", Ty::String) => "ori_set_add_string",
            ("ori_set_contains", Ty::String) => "ori_set_contains_string",
            ("ori_set_remove", Ty::String) => "ori_set_remove_string",
            ("ori_set_try_remove", Ty::String) => "ori_set_try_remove_string",
            ("ori_set_from_list", Ty::String) => "ori_set_from_list_string",
            _ => name,
        };
        let fref = *self
            .func_refs
            .get(runtime_name)
            .ok_or_else(|| format!("missing runtime function `{runtime_name}`"))?;
        match name {
            "ori_set_add" => {
                if args.len() != 2 {
                    return Err("ori_set_add expects set and value".to_string());
                }
                let value = self.emit_expr_for_expected(&args[1].value, elem_ty)?;
                let stored = self.to_list_storage_value(value, elem_ty);
                self.builder.ins().call(fref, &[set_v, stored]);
                self.emit_arc_register_edge_if_managed(elem_ty, set_v, value)?;
                Ok(self.builder.ins().iconst(types::I8, 0))
            }
            "ori_set_contains" => {
                if args.len() != 2 {
                    return Err("ori_set_contains expects set and value".to_string());
                }
                let value = self.emit_expr_for_expected(&args[1].value, elem_ty)?;
                let stored = self.to_list_storage_value(value, elem_ty);
                let call = self.builder.ins().call(fref, &[set_v, stored]);
                Ok(self.builder.inst_results(call)[0])
            }
            "ori_set_remove" | "ori_set_try_remove" => {
                if args.len() != 2 {
                    return Err(format!("{name} expects set and value"));
                }
                let value = self.emit_expr_for_expected(&args[1].value, elem_ty)?;
                let stored = self.to_list_storage_value(value, elem_ty);
                let call = self.builder.ins().call(fref, &[set_v, stored]);
                if name == "ori_set_try_remove" {
                    Ok(self.builder.inst_results(call)[0])
                } else {
                    Ok(self.builder.ins().iconst(types::I8, 0))
                }
            }
            _ => Err(native_codegen_unsupported(format!(
                "set runtime call `{name}`"
            ))),
        }
    }

    fn emit_tree_runtime_call(&mut self, name: &str, args: &[HirArg]) -> Result<ir::Value, String> {
        let node_id_ty = Ty::Opaque {
            kind: OpaqueTy::NodeId,
            args: vec![],
        };
        let tree_elem_ty = |arg: &HirArg| match &arg.value.ty {
            Ty::Opaque {
                kind: OpaqueTy::Tree,
                args,
            } => args.first().cloned(),
            _ => None,
        };
        let elem_ty = args.first().and_then(tree_elem_ty).unwrap_or(Ty::Infer(0));
        let runtime_name = match (name, &elem_ty) {
            ("ori_tree_find", Ty::String) => "ori_tree_find_string",
            _ => name,
        };
        let fref = *self
            .func_refs
            .get(runtime_name)
            .ok_or_else(|| format!("missing runtime function `{runtime_name}`"))?;
        match name {
            "ori_tree_new" => {
                if args.len() != 1 {
                    return Err("ori_tree_new expects one root value".to_string());
                }
                let elem_ty = args[0].value.ty.clone();
                let value = self.emit_expr(&args[0].value)?;
                let stored = self.to_list_storage_value(value, &elem_ty);
                let call = self.builder.ins().call(fref, &[stored]);
                let tree = self.builder.inst_results(call)[0];
                Ok(tree)
            }
            "ori_tree_add_child" => {
                if args.len() != 3 {
                    return Err("ori_tree_add_child expects tree, parent, and value".to_string());
                }
                let tree = self.emit_expr(&args[0].value)?;
                let parent = self.emit_expr_for_expected(&args[1].value, &node_id_ty)?;
                let value = self.emit_expr_for_expected(&args[2].value, &elem_ty)?;
                let stored = self.to_list_storage_value(value, &elem_ty);
                let call = self.builder.ins().call(fref, &[tree, parent, stored]);
                let node = self.builder.inst_results(call)[0];
                Ok(node)
            }
            "ori_tree_value" => {
                if args.len() != 2 {
                    return Err("ori_tree_value expects tree and node".to_string());
                }
                let tree = self.emit_expr(&args[0].value)?;
                let node = self.emit_expr_for_expected(&args[1].value, &node_id_ty)?;
                let call = self.builder.ins().call(fref, &[tree, node]);
                let stored = self.builder.inst_results(call)[0];
                Ok(self.from_list_storage_value(stored, &elem_ty))
            }
            "ori_tree_set_value" => {
                if args.len() != 3 {
                    return Err("ori_tree_set_value expects tree, node, and value".to_string());
                }
                let tree = self.emit_expr(&args[0].value)?;
                let node = self.emit_expr_for_expected(&args[1].value, &node_id_ty)?;
                let value = self.emit_expr_for_expected(&args[2].value, &elem_ty)?;
                let stored = self.to_list_storage_value(value, &elem_ty);
                let call = self.builder.ins().call(fref, &[tree, node, stored]);
                Ok(self.builder.inst_results(call)[0])
            }
            "ori_tree_find" => {
                if args.len() != 2 {
                    return Err("ori_tree_find expects tree and value".to_string());
                }
                let tree = self.emit_expr(&args[0].value)?;
                let value = self.emit_expr_for_expected(&args[1].value, &elem_ty)?;
                let stored = self.to_list_storage_value(value, &elem_ty);
                let call = self.builder.ins().call(fref, &[tree, stored]);
                Ok(self.builder.inst_results(call)[0])
            }
            "ori_tree_root"
            | "ori_tree_len"
            | "ori_tree_pre_order"
            | "ori_tree_post_order"
            | "ori_tree_breadth_first"
            | "ori_tree_clone" => {
                if args.len() != 1 {
                    return Err(format!("{name} expects one tree argument"));
                }
                let tree = self.emit_expr(&args[0].value)?;
                let call = self.builder.ins().call(fref, &[tree]);
                let res = self.builder.inst_results(call);
                Ok(res
                    .first()
                    .copied()
                    .unwrap_or_else(|| self.builder.ins().iconst(types::I8, 0)))
            }
            "ori_tree_children"
            | "ori_tree_parent"
            | "ori_tree_remove_subtree"
            | "ori_tree_depth"
            | "ori_tree_try_value"
            | "ori_tree_contains_node"
            | "ori_tree_clone_subtree" => {
                if args.len() != 2 {
                    return Err(format!("{name} expects tree and node"));
                }
                let tree = self.emit_expr(&args[0].value)?;
                let node = self.emit_expr_for_expected(&args[1].value, &node_id_ty)?;
                let call = self.builder.ins().call(fref, &[tree, node]);
                let res = self.builder.inst_results(call);
                Ok(res
                    .first()
                    .copied()
                    .unwrap_or_else(|| self.builder.ins().iconst(types::I8, 0)))
            }
            "ori_tree_move_subtree" => {
                if args.len() != 3 {
                    return Err("ori_tree_move_subtree expects tree, node, and parent".to_string());
                }
                let tree = self.emit_expr(&args[0].value)?;
                let node = self.emit_expr_for_expected(&args[1].value, &node_id_ty)?;
                let parent = self.emit_expr_for_expected(&args[2].value, &node_id_ty)?;
                let call = self.builder.ins().call(fref, &[tree, node, parent]);
                Ok(self.builder.inst_results(call)[0])
            }
            _ => Err(native_codegen_unsupported(format!(
                "tree runtime call `{name}`"
            ))),
        }
    }

    fn emit_heap_runtime_call(
        &mut self,
        name: &str,
        args: &[HirArg],
        result_ty: &Ty,
    ) -> Result<ir::Value, String> {
        let heap_elem_ty = |ty: &Ty| match ty {
            Ty::Opaque {
                kind: OpaqueTy::Heap,
                args,
            } => args.first().cloned(),
            _ => None,
        };
        match name {
            "ori_heap_new" => {
                if !args.is_empty() {
                    return Err("ori_heap_new expects no public arguments".to_string());
                }
                let elem_ty = heap_elem_ty(result_ty).unwrap_or(Ty::Int);
                let runtime_name = match &elem_ty {
                    Ty::String => "ori_heap_new_string",
                    Ty::Named(def_id, _) => {
                        let compare = SmolStr::new("compare");
                        let Some(compare_name) =
                            self.trait_method_func_name_for_type(*def_id, &compare)
                        else {
                            return Err(format!(
                                "heap element `{}` has no Comparable.compare implementation",
                                elem_ty.display()
                            ));
                        };
                        let compare_ref =
                            *self.func_refs.get(compare_name.as_str()).ok_or_else(|| {
                                format!("missing function reference `{compare_name}`")
                            })?;
                        let compare_ptr = self.builder.ins().func_addr(self.ptr_ty, compare_ref);
                        let new_ref =
                            *self.func_refs.get("ori_heap_new_custom").ok_or_else(|| {
                                "missing runtime function `ori_heap_new_custom`".to_string()
                            })?;
                        let call = self.builder.ins().call(new_ref, &[compare_ptr]);
                        return Ok(self.builder.inst_results(call)[0]);
                    }
                    _ => "ori_heap_new",
                };
                let fref = *self
                    .func_refs
                    .get(runtime_name)
                    .ok_or_else(|| format!("missing runtime function `{runtime_name}`"))?;
                let call = self.builder.ins().call(fref, &[]);
                Ok(self.builder.inst_results(call)[0])
            }
            "ori_heap_push" => {
                if args.len() != 2 {
                    return Err("ori_heap_push expects heap and value".to_string());
                }
                let elem_ty = match heap_elem_ty(&args[0].value.ty) {
                    Some(ty) if !ty.contains_infer() => ty,
                    _ => args[1].value.ty.clone(),
                };
                let heap = self.emit_expr(&args[0].value)?;
                let value = self.emit_expr_for_expected(&args[1].value, &elem_ty)?;
                let stored = self.to_list_storage_value(value, &elem_ty);
                match &elem_ty {
                    Ty::String => {
                        let fref =
                            *self.func_refs.get("ori_heap_push_string").ok_or_else(|| {
                                "missing runtime function `ori_heap_push_string`".to_string()
                            })?;
                        self.builder.ins().call(fref, &[heap, stored]);
                    }
                    Ty::Named(def_id, _) => {
                        let compare = SmolStr::new("compare");
                        let Some(compare_name) =
                            self.trait_method_func_name_for_type(*def_id, &compare)
                        else {
                            return Err(format!(
                                "heap element `{}` has no Comparable.compare implementation",
                                elem_ty.display()
                            ));
                        };
                        let compare_ref =
                            *self.func_refs.get(compare_name.as_str()).ok_or_else(|| {
                                format!("missing function reference `{compare_name}`")
                            })?;
                        let compare_ptr = self.builder.ins().func_addr(self.ptr_ty, compare_ref);
                        let fref =
                            *self.func_refs.get("ori_heap_push_custom").ok_or_else(|| {
                                "missing runtime function `ori_heap_push_custom`".to_string()
                            })?;
                        self.builder.ins().call(fref, &[heap, stored, compare_ptr]);
                    }
                    _ => {
                        let fref = *self
                            .func_refs
                            .get(name)
                            .ok_or_else(|| format!("missing runtime function `{name}`"))?;
                        self.builder.ins().call(fref, &[heap, stored]);
                    }
                }
                self.emit_arc_register_edge_if_managed(&elem_ty, heap, value)?;
                Ok(self.builder.ins().iconst(types::I8, 0))
            }
            "ori_heap_from_list" => {
                if args.len() != 1 {
                    return Err("ori_heap_from_list expects one source list".to_string());
                }
                let elem_ty = heap_elem_ty(result_ty)
                    .or_else(|| match &args[0].value.ty {
                        Ty::List(elem) => Some(*elem.clone()),
                        _ => None,
                    })
                    .unwrap_or(Ty::Int);
                let source = self.emit_expr(&args[0].value)?;
                let runtime_name = match &elem_ty {
                    Ty::String => "ori_heap_from_list_string",
                    Ty::Named(def_id, _) => {
                        let compare = SmolStr::new("compare");
                        let Some(compare_name) =
                            self.trait_method_func_name_for_type(*def_id, &compare)
                        else {
                            return Err(format!(
                                "heap element `{}` has no Comparable.compare implementation",
                                elem_ty.display()
                            ));
                        };
                        let compare_ref =
                            *self.func_refs.get(compare_name.as_str()).ok_or_else(|| {
                                format!("missing function reference `{compare_name}`")
                            })?;
                        let compare_ptr = self.builder.ins().func_addr(self.ptr_ty, compare_ref);
                        let fref =
                            *self
                                .func_refs
                                .get("ori_heap_from_list_custom")
                                .ok_or_else(|| {
                                    "missing runtime function `ori_heap_from_list_custom`"
                                        .to_string()
                                })?;
                        let call = self.builder.ins().call(fref, &[source, compare_ptr]);
                        return Ok(self.builder.inst_results(call)[0]);
                    }
                    _ => "ori_heap_from_list",
                };
                let fref = *self
                    .func_refs
                    .get(runtime_name)
                    .ok_or_else(|| format!("missing runtime function `{runtime_name}`"))?;
                let call = self.builder.ins().call(fref, &[source]);
                Ok(self.builder.inst_results(call)[0])
            }
            "ori_heap_remove" => {
                if args.len() != 2 {
                    return Err("ori_heap_remove expects heap and value".to_string());
                }
                let elem_ty = match heap_elem_ty(&args[0].value.ty) {
                    Some(ty) if !ty.contains_infer() => ty,
                    _ => args[1].value.ty.clone(),
                };
                let heap = self.emit_expr(&args[0].value)?;
                let value = self.emit_expr_for_expected(&args[1].value, &elem_ty)?;
                let stored = self.to_list_storage_value(value, &elem_ty);
                let call = match &elem_ty {
                    Ty::String => {
                        let fref =
                            *self
                                .func_refs
                                .get("ori_heap_remove_string")
                                .ok_or_else(|| {
                                    "missing runtime function `ori_heap_remove_string`".to_string()
                                })?;
                        self.builder.ins().call(fref, &[heap, stored])
                    }
                    Ty::Named(def_id, _) => {
                        let compare = SmolStr::new("compare");
                        let Some(compare_name) =
                            self.trait_method_func_name_for_type(*def_id, &compare)
                        else {
                            return Err(format!(
                                "heap element `{}` has no Comparable.compare implementation",
                                elem_ty.display()
                            ));
                        };
                        let compare_ref =
                            *self.func_refs.get(compare_name.as_str()).ok_or_else(|| {
                                format!("missing function reference `{compare_name}`")
                            })?;
                        let compare_ptr = self.builder.ins().func_addr(self.ptr_ty, compare_ref);
                        let fref =
                            *self
                                .func_refs
                                .get("ori_heap_remove_custom")
                                .ok_or_else(|| {
                                    "missing runtime function `ori_heap_remove_custom`".to_string()
                                })?;
                        self.builder.ins().call(fref, &[heap, stored, compare_ptr])
                    }
                    _ => {
                        let fref = *self
                            .func_refs
                            .get(name)
                            .ok_or_else(|| format!("missing runtime function `{name}`"))?;
                        self.builder.ins().call(fref, &[heap, stored])
                    }
                };
                Ok(self.builder.inst_results(call)[0])
            }
            "ori_heap_merge" => {
                if args.len() != 2 {
                    return Err("ori_heap_merge expects two heaps".to_string());
                }
                let left = self.emit_expr(&args[0].value)?;
                let right = self.emit_expr(&args[1].value)?;
                let fref = *self
                    .func_refs
                    .get(name)
                    .ok_or_else(|| format!("missing runtime function `{name}`"))?;
                let call = self.builder.ins().call(fref, &[left, right]);
                Ok(self.builder.inst_results(call)[0])
            }
            "ori_heap_pop"
            | "ori_heap_peek"
            | "ori_heap_len"
            | "ori_heap_is_empty"
            | "ori_heap_clear"
            | "ori_heap_clone"
            | "ori_heap_to_list"
            | "ori_heap_into_sorted_list" => {
                if args.len() != 1 {
                    return Err(format!("{name} expects one heap argument"));
                }
                let heap = self.emit_expr(&args[0].value)?;
                let fref = *self
                    .func_refs
                    .get(name)
                    .ok_or_else(|| format!("missing runtime function `{name}`"))?;
                let call = self.builder.ins().call(fref, &[heap]);
                let res = self.builder.inst_results(call);
                Ok(res
                    .first()
                    .copied()
                    .unwrap_or_else(|| self.builder.ins().iconst(types::I8, 0)))
            }
            _ => Err(native_codegen_unsupported(format!(
                "heap runtime call `{name}`"
            ))),
        }
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

    fn emit_short_circuit_binary(
        &mut self,
        op: BinaryOp,
        lhs: &HirExpr,
        rhs: &HirExpr,
    ) -> Result<ir::Value, String> {
        let lhs_value = self.emit_expr(lhs)?;
        let rhs_block = self.builder.create_block();
        let merge_block = self.builder.create_block();
        self.builder.append_block_param(merge_block, types::I8);

        let skip_value = match op {
            BinaryOp::And => self.builder.ins().iconst(types::I8, 0),
            BinaryOp::Or => self.builder.ins().iconst(types::I8, 1),
            _ => unreachable!("short-circuit only handles logical operators"),
        };
        let skip_args = [BlockArg::Value(skip_value)];

        match op {
            BinaryOp::And => {
                self.builder
                    .ins()
                    .brif(lhs_value, rhs_block, &[], merge_block, &skip_args);
            }
            BinaryOp::Or => {
                self.builder
                    .ins()
                    .brif(lhs_value, merge_block, &skip_args, rhs_block, &[]);
            }
            _ => unreachable!("short-circuit only handles logical operators"),
        }

        self.builder.seal_block(rhs_block);
        self.builder.switch_to_block(rhs_block);
        self.terminated = false;
        let rhs_value = self.emit_expr(rhs)?;
        if !self.terminated {
            let rhs_args = [BlockArg::Value(rhs_value)];
            self.builder.ins().jump(merge_block, &rhs_args);
        }

        self.builder.seal_block(merge_block);
        self.builder.switch_to_block(merge_block);
        self.terminated = false;
        Ok(self.builder.block_params(merge_block)[0])
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
                if matches!(ty, Ty::String) {
                    let concat_ref = *self.func_refs.get("ori_string_concat").ok_or_else(|| {
                        "missing runtime function `ori_string_concat`".to_string()
                    })?;
                    let call = self.builder.ins().call(concat_ref, &[lv, rv]);
                    self.builder.inst_results(call)[0]
                } else if matches!(ty, Ty::Bytes) {
                    let concat_ref = *self
                        .func_refs
                        .get("ori_bytes_concat")
                        .ok_or_else(|| "missing runtime function `ori_bytes_concat`".to_string())?;
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

/// Native linker facade used by the Cranelift backend.
///
/// The default path uses `rustc` as the native linker driver. This keeps the
/// route independent from a C compiler driver while still letting the Rust
/// toolchain provide the platform-specific CRT and linker configuration.
#[derive(Debug, Clone)]
pub struct NativeLinker {
    strategy: NativeLinkerStrategy,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NativeLinkOptions {
    pub raw_diagnostics: bool,
}

#[derive(Debug, Clone)]
enum NativeLinkerStrategy {
    RustcDriver {
        command: PathBuf,
        linker_override: Option<PathBuf>,
    },
    RawNativeCommand {
        command: PathBuf,
    },
}

impl NativeLinker {
    pub fn discover() -> Result<Self, String> {
        if let Ok(command) = std::env::var("ORI_NATIVE_LINKER") {
            let command = command.trim();
            if command.is_empty() {
                return Err("ORI_NATIVE_LINKER is set but empty".to_string());
            }
            return Ok(Self {
                strategy: NativeLinkerStrategy::RawNativeCommand {
                    command: PathBuf::from(command),
                },
            });
        }

        let command = std::env::var("ORI_RUSTC")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("rustc"));
        let linker_override = rust_lld_override(&command)?;
        Ok(Self {
            strategy: NativeLinkerStrategy::RustcDriver {
                command,
                linker_override,
            },
        })
    }

    pub fn link(
        &self,
        obj_path: &Path,
        exe_path: &Path,
        extra_libs: &[PathBuf],
        options: NativeLinkOptions,
    ) -> Result<(), String> {
        match &self.strategy {
            NativeLinkerStrategy::RustcDriver {
                command,
                linker_override,
            } => link_with_rustc_driver(
                command,
                linker_override.as_deref(),
                obj_path,
                exe_path,
                extra_libs,
                options,
            ),
            NativeLinkerStrategy::RawNativeCommand { command } => {
                link_with_raw_native_command(command, obj_path, exe_path, extra_libs, options)
            }
        }
    }
}

/// Link `obj_path` into an executable at `exe_path`.
/// `extra_libs`: additional static libraries to link, usually `ori-runtime`.
pub fn link(obj_path: &Path, exe_path: &Path, extra_libs: &[PathBuf]) -> Result<(), String> {
    link_with_options(obj_path, exe_path, extra_libs, NativeLinkOptions::default())
}

pub fn link_with_options(
    obj_path: &Path,
    exe_path: &Path,
    extra_libs: &[PathBuf],
    options: NativeLinkOptions,
) -> Result<(), String> {
    NativeLinker::discover()?.link(obj_path, exe_path, extra_libs, options)
}

const NATIVE_LINKER_MISSING: &str = "native.linker_missing";
const NATIVE_LINK_FAILED: &str = "native.link_failed";
const NATIVE_RUNTIME_SYMBOL_MISSING: &str = "native.runtime_symbol_missing";

fn link_with_rustc_driver(
    command: &Path,
    linker_override: Option<&Path>,
    obj_path: &Path,
    exe_path: &Path,
    extra_libs: &[PathBuf],
    options: NativeLinkOptions,
) -> Result<(), String> {
    static NEXT_LINK_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    let id = NEXT_LINK_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let shim = std::env::temp_dir().join(format!("ori_link_shim_{}_{}.rs", std::process::id(), id));
    std::fs::write(&shim, "#![no_main]\n")
        .map_err(|e| format!("failed to write native linker shim {}: {e}", shim.display()))?;

    let mut cmd = std::process::Command::new(command);
    cmd.arg("--edition=2021")
        .arg("--crate-name")
        .arg(format!("ori_link_shim_{id}"))
        .arg(&shim)
        .arg("-o")
        .arg(exe_path)
        .arg("-C")
        .arg(format!("link-arg={}", obj_path.display()));

    if let Some(linker) = linker_override {
        cmd.arg("-C").arg(format!("linker={}", linker.display()));
    }

    for lib in extra_libs {
        cmd.arg("-C").arg(format!("link-arg={}", lib.display()));
    }

    let output = cmd.output().map_err(|e| {
        format!(
            "{NATIVE_LINKER_MISSING}: could not invoke native linker driver `{}`: {e}",
            command.display()
        )
    });
    let _ = std::fs::remove_file(&shim);
    let output = output?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format_native_link_failure(
            "driver",
            command,
            output.status,
            &output.stdout,
            &output.stderr,
            options,
        ))
    }
}

fn rust_lld_override(rustc: &Path) -> Result<Option<PathBuf>, String> {
    if std::env::var("ORI_USE_RUST_LLD").is_err() {
        return Ok(None);
    }

    if let Ok(path) = std::env::var("ORI_RUST_LLD") {
        let path = path.trim();
        if path.is_empty() {
            return Err("ORI_RUST_LLD is set but empty".to_string());
        }
        return Ok(Some(PathBuf::from(path)));
    }

    if let Some(path) = discover_rust_lld_from_rustc(rustc) {
        return Ok(Some(path));
    }

    Ok(Some(PathBuf::from(if cfg!(windows) {
        "rust-lld.exe"
    } else {
        "rust-lld"
    })))
}

fn discover_rust_lld_from_rustc(rustc: &Path) -> Option<PathBuf> {
    let sysroot = std::process::Command::new(rustc)
        .args(["--print", "sysroot"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())?;
    let host = rustc_host_triple(rustc)?;
    let exe = if cfg!(windows) {
        "rust-lld.exe"
    } else {
        "rust-lld"
    };
    let candidate = PathBuf::from(sysroot)
        .join("lib")
        .join("rustlib")
        .join(host)
        .join("bin")
        .join(exe);
    candidate.is_file().then_some(candidate)
}

fn rustc_host_triple(rustc: &Path) -> Option<String> {
    let output = std::process::Command::new(rustc).arg("-vV").output().ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .find_map(|line| {
            line.strip_prefix("host:")
                .map(|host| host.trim().to_string())
        })
}

fn link_with_raw_native_command(
    command: &Path,
    obj_path: &Path,
    exe_path: &Path,
    extra_libs: &[PathBuf],
    options: NativeLinkOptions,
) -> Result<(), String> {
    let mut cmd = std::process::Command::new(command);
    if cfg!(windows) {
        cmd.arg("/NOLOGO")
            .arg(format!("/OUT:{}", exe_path.display()))
            .arg(obj_path);
    } else {
        cmd.arg("-o").arg(exe_path).arg(obj_path);
    }
    for lib in extra_libs {
        cmd.arg(lib);
    }

    let output = cmd.output().map_err(|e| {
        format!(
            "{NATIVE_LINKER_MISSING}: could not invoke native linker `{}` from ORI_NATIVE_LINKER: {e}",
            command.display()
        )
    })?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format_native_link_failure(
            "",
            command,
            output.status,
            &output.stdout,
            &output.stderr,
            options,
        ))
    }
}

fn format_native_link_failure(
    kind: &str,
    command: &Path,
    status: impl std::fmt::Display,
    stdout: &[u8],
    stderr: &[u8],
    options: NativeLinkOptions,
) -> String {
    let stdout_text = String::from_utf8_lossy(stdout);
    let stderr_text = String::from_utf8_lossy(stderr);
    let stdout_trimmed = stdout_text.trim();
    let stderr_trimmed = stderr_text.trim();
    let label = if kind.trim().is_empty() {
        "native linker".to_string()
    } else {
        format!("native linker {kind}")
    };
    let missing_symbol = looks_like_missing_native_symbol(stdout_trimmed)
        || looks_like_missing_native_symbol(stderr_trimmed);
    let mut message = String::new();
    if missing_symbol {
        message.push_str(
            &format!("{NATIVE_RUNTIME_SYMBOL_MISSING}: native link failed because at least one native symbol was not resolved.\n"),
        );
        message.push_str(
            "Check whether the packaged ori-runtime was staged for the same compiler version, target and ABI, and whether the runtime exports every symbol used by the native backend.\n",
        );
    } else {
        message.push_str(&format!("{NATIVE_LINK_FAILED}: native linker failed.\n"));
    }
    message.push_str(&format!(
        "{label} `{}` failed with status {status}",
        command.display()
    ));
    if let Some(first_error) = first_non_empty_linker_line(stderr_trimmed, stdout_trimmed) {
        message.push_str(&format!("\nfirst linker message: {first_error}"));
    }
    if options.raw_diagnostics {
        message.push_str(&format!(
            "\nstdout:\n{}\nstderr:\n{}",
            stdout_trimmed, stderr_trimmed
        ));
    } else {
        message.push_str("\nuse `ori compile --native-raw` to print full linker stdout/stderr");
    }
    message
}

fn first_non_empty_linker_line<'a>(stderr: &'a str, stdout: &'a str) -> Option<&'a str> {
    stderr
        .lines()
        .chain(stdout.lines())
        .map(str::trim)
        .find(|line| !line.is_empty())
}

fn looks_like_missing_native_symbol(text: &str) -> bool {
    let text = text.to_ascii_lowercase();
    [
        "unresolved external symbol",
        "undefined reference",
        "undefined symbol",
        "symbol(s) not found",
        "unresolved symbol",
        "lnk2001",
        "lnk2019",
    ]
    .iter()
    .any(|needle| text.contains(needle))
}

#[cfg(test)]
mod tests;
