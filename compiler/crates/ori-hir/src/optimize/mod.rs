//! HIR mid-end optimisations (LANG-PERF-2).
//!
//! Semantics-preserving rewrites between monomorphization and native lower.
//! Default level is safe under FREEZE-1; Aggressive may enable more rewrites.

mod const_fold;
mod dce;
mod pipeline;

pub use pipeline::{optimize_module, OptLevel};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::*;
    use ori_diagnostics::Span;
    use ori_types::{DefId, Ty};
    use smol_str::SmolStr;

    fn span() -> Span {
        Span::DUMMY
    }

    fn int_lit(n: i64) -> HirExpr {
        HirExpr {
            kind: HirExprKind::IntLit(n),
            ty: Ty::Int,
            span: span(),
        }
    }

    fn bin(op: ori_ast::expr::BinaryOp, lhs: HirExpr, rhs: HirExpr) -> HirExpr {
        HirExpr {
            kind: HirExprKind::Binary {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            ty: Ty::Int,
            span: span(),
        }
    }

    #[test]
    fn const_fold_adds_literals() {
        let mut module = HirModule {
            namespace: SmolStr::new("app"),
            structs: vec![],
            enums: vec![],
            traits: vec![],
            trait_impls: vec![],
            funcs: vec![HirFunc {
                def_id: DefId(1),
                name: SmolStr::new("main"),
                params: vec![],
                return_ty: Ty::Void,
                body: HirBlock {
                    stmts: vec![
                        HirStmt::Let {
                            name: SmolStr::new("x"),
                            ty: Ty::Int,
                            mutable: false,
                            value: bin(
                                ori_ast::expr::BinaryOp::Add,
                                int_lit(2),
                                int_lit(3),
                            ),
                            span: span(),
                        },
                        HirStmt::Return(
                            Some(HirExpr {
                                kind: HirExprKind::Var(SmolStr::new("x")),
                                ty: Ty::Int,
                                span: span(),
                            }),
                            span(),
                        ),
                    ],
                    span: span(),
                },
                closure_captures: vec![],
                is_public: true,
                is_async: false,
                is_mut: false,
                span: span(),
            }],
            consts: vec![],
            externs: vec![],
        };
        optimize_module(&mut module, OptLevel::Default);
        match &module.funcs[0].body.stmts[0] {
            HirStmt::Let { value, .. } => match value.kind {
                HirExprKind::IntLit(5) => {}
                ref other => panic!("expected IntLit(5), got {other:?}"),
            },
            other => panic!("expected Let, got {other:?}"),
        }
    }
}
