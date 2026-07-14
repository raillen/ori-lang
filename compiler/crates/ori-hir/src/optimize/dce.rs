//! Dead binding elimination for pure, unused `const` bindings.

use std::collections::HashSet;

use smol_str::SmolStr;

use crate::hir::*;

pub(super) fn dce_module(module: &mut HirModule) {
    for f in &mut module.funcs {
        dce_block(&mut f.body);
    }
}

fn dce_block(block: &mut HirBlock) {
    for stmt in &mut block.stmts {
        dce_stmt_nested(stmt);
    }

    let mut used = HashSet::<SmolStr>::new();
    for stmt in &block.stmts {
        collect_stmt_uses(stmt, &mut used);
    }

    block.stmts.retain(|stmt| match stmt {
        HirStmt::Let {
            name,
            value,
            mutable,
            ..
        } => {
            if *mutable {
                return true;
            }
            if used.contains(name) {
                return true;
            }
            expr_may_effect(value)
        }
        _ => true,
    });
}

fn dce_stmt_nested(stmt: &mut HirStmt) {
    match stmt {
        HirStmt::If {
            then,
            else_ifs,
            else_,
            ..
        } => {
            dce_block(then);
            for (_, b) in else_ifs {
                dce_block(b);
            }
            if let Some(b) = else_ {
                dce_block(b);
            }
        }
        HirStmt::While { body, .. }
        | HirStmt::For { body, .. }
        | HirStmt::Loop { body, .. }
        | HirStmt::Repeat { body, .. }
        | HirStmt::WhileSome { body, .. } => dce_block(body),
        HirStmt::IfSome { then, else_, .. } => {
            dce_block(then);
            if let Some(b) = else_ {
                dce_block(b);
            }
        }
        HirStmt::Match { arms, .. } => {
            for arm in arms {
                let mut nested = HirBlock {
                    stmts: std::mem::take(&mut arm.body),
                    span: arm.span,
                };
                dce_block(&mut nested);
                arm.body = nested.stmts;
            }
        }
        _ => {}
    }
}

fn collect_stmt_uses(stmt: &HirStmt, used: &mut HashSet<SmolStr>) {
    match stmt {
        HirStmt::Let { value, .. } => collect_expr_uses(value, used),
        HirStmt::Assign { lvalue, value, .. } => {
            collect_lvalue_uses(lvalue, used);
            collect_expr_uses(value, used);
        }
        HirStmt::Return(Some(e), _) | HirStmt::Expr(e) => collect_expr_uses(e, used),
        HirStmt::Return(None, _) | HirStmt::Break(_) | HirStmt::Continue(_) => {}
        HirStmt::If {
            cond,
            then,
            else_ifs,
            else_,
            ..
        } => {
            collect_expr_uses(cond, used);
            collect_block_uses(then, used);
            for (c, b) in else_ifs {
                collect_expr_uses(c, used);
                collect_block_uses(b, used);
            }
            if let Some(b) = else_ {
                collect_block_uses(b, used);
            }
        }
        HirStmt::While { cond, body, .. } => {
            collect_expr_uses(cond, used);
            collect_block_uses(body, used);
        }
        HirStmt::For {
            iterable, body, ..
        } => {
            collect_expr_uses(iterable, used);
            collect_block_uses(body, used);
        }
        HirStmt::Loop { body, .. } => collect_block_uses(body, used),
        HirStmt::Repeat { count, body, .. } => {
            collect_expr_uses(count, used);
            collect_block_uses(body, used);
        }
        HirStmt::Match {
            scrutinee, arms, ..
        } => {
            collect_expr_uses(scrutinee, used);
            for arm in arms {
                for s in &arm.body {
                    collect_stmt_uses(s, used);
                }
            }
        }
        HirStmt::IfSome {
            value, then, else_, ..
        } => {
            collect_expr_uses(value, used);
            collect_block_uses(then, used);
            if let Some(b) = else_ {
                collect_block_uses(b, used);
            }
        }
        HirStmt::WhileSome { value, body, .. } => {
            collect_expr_uses(value, used);
            collect_block_uses(body, used);
        }
        HirStmt::Using { value, .. } => collect_expr_uses(value, used),
        HirStmt::Check { condition, .. } => collect_expr_uses(condition, used),
    }
}

fn collect_block_uses(block: &HirBlock, used: &mut HashSet<SmolStr>) {
    for s in &block.stmts {
        collect_stmt_uses(s, used);
    }
}

fn collect_lvalue_uses(lv: &HirLValue, used: &mut HashSet<SmolStr>) {
    match lv {
        HirLValue::Var(name) => {
            used.insert(name.clone());
        }
        HirLValue::Field { base, .. } => collect_lvalue_uses(base, used),
        HirLValue::Index { base, index } => {
            collect_lvalue_uses(base, used);
            collect_expr_uses(index, used);
        }
    }
}

fn collect_expr_uses(expr: &HirExpr, used: &mut HashSet<SmolStr>) {
    match &expr.kind {
        HirExprKind::Var(name) => {
            used.insert(name.clone());
        }
        HirExprKind::Binary { lhs, rhs, .. } => {
            collect_expr_uses(lhs, used);
            collect_expr_uses(rhs, used);
        }
        HirExprKind::Unary { operand, .. }
        | HirExprKind::Field { object: operand, .. }
        | HirExprKind::TupleIndex { object: operand, .. }
        | HirExprKind::Some_(operand)
        | HirExprKind::Ok_(operand)
        | HirExprKind::Err_(operand)
        | HirExprKind::Propagate(operand)
        | HirExprKind::Await(operand)
        | HirExprKind::IsCheck { value: operand, .. } => collect_expr_uses(operand, used),
        HirExprKind::Index { object, index } => {
            collect_expr_uses(object, used);
            collect_expr_uses(index, used);
        }
        HirExprKind::Call { callee, args } => {
            collect_expr_uses(callee, used);
            for a in args {
                collect_expr_uses(&a.value, used);
            }
        }
        HirExprKind::MethodCall { receiver, args, .. } => {
            collect_expr_uses(receiver, used);
            for a in args {
                collect_expr_uses(a, used);
            }
        }
        HirExprKind::IfExpr { cond, then, else_ } => {
            collect_expr_uses(cond, used);
            collect_expr_uses(then, used);
            collect_expr_uses(else_, used);
        }
        HirExprKind::StructLit { fields, .. } | HirExprKind::EnumVariant { fields, .. } => {
            for (_, e) in fields {
                collect_expr_uses(e, used);
            }
        }
        HirExprKind::ListLit { elements, .. }
        | HirExprKind::TupleLit(elements)
        | HirExprKind::SetLit { elements, .. } => {
            for e in elements {
                collect_expr_uses(e, used);
            }
        }
        HirExprKind::ListSpreadLit { elements, .. } => {
            for el in elements {
                collect_expr_uses(&el.value, used);
            }
        }
        HirExprKind::MapLit { entries, .. } => {
            for (k, v) in entries {
                collect_expr_uses(k, used);
                collect_expr_uses(v, used);
            }
        }
        HirExprKind::Range { start, end } => {
            collect_expr_uses(start, used);
            collect_expr_uses(end, used);
        }
        HirExprKind::StructUpdate { base, updates, .. } => {
            collect_expr_uses(base, used);
            for (_, e) in updates {
                collect_expr_uses(e, used);
            }
        }
        HirExprKind::InterpolatedStr(parts) => {
            for p in parts {
                if let HirStrPart::Expr(e) = p {
                    collect_expr_uses(e, used);
                }
            }
        }
        _ => {}
    }
}

fn expr_may_effect(expr: &HirExpr) -> bool {
    match &expr.kind {
        HirExprKind::Call { .. }
        | HirExprKind::MethodCall { .. }
        | HirExprKind::Await(_)
        | HirExprKind::Propagate(_) => true,
        HirExprKind::Binary { lhs, rhs, .. } => expr_may_effect(lhs) || expr_may_effect(rhs),
        HirExprKind::Unary { operand, .. }
        | HirExprKind::Field { object: operand, .. }
        | HirExprKind::Some_(operand)
        | HirExprKind::Ok_(operand)
        | HirExprKind::Err_(operand) => expr_may_effect(operand),
        HirExprKind::Index { object, index } => {
            expr_may_effect(object) || expr_may_effect(index)
        }
        HirExprKind::IfExpr { cond, then, else_ } => {
            expr_may_effect(cond) || expr_may_effect(then) || expr_may_effect(else_)
        }
        HirExprKind::ListLit { elements, .. } | HirExprKind::TupleLit(elements) => {
            elements.iter().any(expr_may_effect)
        }
        HirExprKind::StructLit { fields, .. } => fields.iter().any(|(_, e)| expr_may_effect(e)),
        _ => false,
    }
}
