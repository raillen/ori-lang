use smol_str::SmolStr;
use ori_diagnostics::Span;
use ori_ast::expr::{BinaryOp, UnaryOp};
use ori_types::{DefId, Ty};

// ── Module ────────────────────────────────────────────────────────────────────

/// One source file lowered to HIR.
#[derive(Debug, Clone)]
pub struct HirModule {
    pub namespace: SmolStr,
    pub structs:   Vec<HirStruct>,
    pub enums:     Vec<HirEnum>,
    pub funcs:     Vec<HirFunc>,
    pub consts:    Vec<HirConst>,
}

// ── Declarations ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct HirStruct {
    pub def_id:    DefId,
    pub name:      SmolStr,
    pub fields:    Vec<HirField>,
    pub is_public: bool,
    pub span:      Span,
}

#[derive(Debug, Clone)]
pub struct HirField {
    pub name: SmolStr,
    pub ty:   Ty,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct HirEnum {
    pub def_id:    DefId,
    pub name:      SmolStr,
    pub variants:  Vec<HirVariant>,
    pub is_public: bool,
    pub span:      Span,
}

#[derive(Debug, Clone)]
pub struct HirVariant {
    pub name:   SmolStr,
    pub fields: Vec<HirField>,
    pub span:   Span,
}

#[derive(Debug, Clone)]
pub struct HirFunc {
    pub def_id:    DefId,
    pub name:      SmolStr,
    pub params:    Vec<HirParam>,
    pub return_ty: Ty,
    pub body:      HirBlock,
    pub is_public: bool,
    pub is_mut:    bool,
    pub span:      Span,
}

#[derive(Debug, Clone)]
pub struct HirParam {
    pub name: SmolStr,
    pub ty:   Ty,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct HirConst {
    pub def_id:    DefId,
    pub name:      SmolStr,
    pub ty:        Ty,
    pub value:     HirExpr,
    pub is_public: bool,
    pub span:      Span,
}

// ── Statements ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct HirBlock {
    pub stmts: Vec<HirStmt>,
    pub span:  Span,
}

#[derive(Debug, Clone)]
pub enum HirStmt {
    /// `const name: ty = value` or `var name: ty = value`
    Let { name: SmolStr, ty: Ty, mutable: bool, value: HirExpr, span: Span },
    /// `lvalue = value` or `lvalue += value` etc.
    Assign { lvalue: HirLValue, value: HirExpr, span: Span },
    Return(Option<HirExpr>, Span),
    Break(Span),
    Continue(Span),
    Expr(HirExpr),
    If {
        cond:      HirExpr,
        then:      HirBlock,
        else_ifs:  Vec<(HirExpr, HirBlock)>,
        else_:     Option<HirBlock>,
        span:      Span,
    },
    While { cond: HirExpr, body: HirBlock, span: Span },
    For   { binding: SmolStr, elem_ty: Ty, iterable: HirExpr, body: HirBlock, span: Span },
    Loop  { body: HirBlock, span: Span },
    Match { scrutinee: HirExpr, arms: Vec<HirArm>, span: Span },
}

#[derive(Debug, Clone)]
pub struct HirArm {
    pub pattern: HirPattern,
    pub body:    Vec<HirStmt>,
    pub span:    Span,
}

#[derive(Debug, Clone)]
pub enum HirPattern {
    Wildcard,
    Binding(SmolStr, Ty),
    BoolLit(bool),
    IntLit(i64),
    StrLit(SmolStr),
    None_,
    Some_(Box<HirPattern>),
    Ok_(Box<HirPattern>),
    Err_(Box<HirPattern>),
    Variant { def_id: DefId, variant: SmolStr, fields: Vec<(SmolStr, HirPattern)> },
    Tuple(Vec<HirPattern>),
}

#[derive(Debug, Clone)]
pub enum HirLValue {
    Var(SmolStr),
    Field { base: Box<HirLValue>, field: SmolStr },
    Index { base: Box<HirLValue>, index: Box<HirExpr> },
}

// ── Expressions ───────────────────────────────────────────────────────────────

/// A type-annotated expression.
#[derive(Debug, Clone)]
pub struct HirExpr {
    pub kind: HirExprKind,
    pub ty:   Ty,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum HirExprKind {
    // Literals
    BoolLit(bool),
    IntLit(i64),
    FloatLit(f64),
    StrLit(SmolStr),
    InterpolatedStr(Vec<HirStrPart>),
    BytesLit(Vec<u8>),
    Unit,

    // Variables and paths
    Var(SmolStr),
    GlobalConst(DefId),

    // Operators
    Binary { op: BinaryOp, lhs: Box<HirExpr>, rhs: Box<HirExpr> },
    Unary  { op: UnaryOp,  operand: Box<HirExpr> },

    // Compound access
    Field  { object: Box<HirExpr>, field: SmolStr },
    Index  { object: Box<HirExpr>, index: Box<HirExpr> },
    TupleIndex { object: Box<HirExpr>, index: u32 },

    // Calls
    Call       { callee: Box<HirExpr>, args: Vec<HirExpr> },
    MethodCall { receiver: Box<HirExpr>, method: SmolStr, args: Vec<HirExpr> },

    // Construction
    StructLit { def_id: DefId, fields: Vec<(SmolStr, HirExpr)> },
    ListLit   { elem_ty: Ty, elements: Vec<HirExpr> },
    TupleLit  (Vec<HirExpr>),

    // Built-in wrappers
    Some_(Box<HirExpr>),
    None_,
    Ok_(Box<HirExpr>),
    Err_(Box<HirExpr>),

    // `expr?` desugared: propagate None or Err upward
    Propagate(Box<HirExpr>),

    // `if cond then a else b`
    IfExpr { cond: Box<HirExpr>, then: Box<HirExpr>, else_: Box<HirExpr> },

    // Range `a..b`
    Range { start: Box<HirExpr>, end: Box<HirExpr> },

    // Closure (kept opaque for now)
    Closure,
}

#[derive(Debug, Clone)]
pub enum HirStrPart {
    Literal(SmolStr),
    Expr(HirExpr),
}
