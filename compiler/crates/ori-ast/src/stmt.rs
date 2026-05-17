use crate::common::Name;
use crate::expr::Expr;
use crate::pattern::Pattern;
use crate::ty::Type;
use ori_diagnostics::Span;
use smol_str::SmolStr;

/// A sequence of statements closed by `end`.
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

/// Every statement that can appear inside a function body.
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Const(LocalConst),
    Var(LocalVar),
    Assign(AssignStmt),
    CompoundAssign(CompoundAssignStmt),
    Return(ReturnStmt),
    Break(Span),
    Continue(Span),
    If(IfStmt),
    IfSome(IfSomeStmt),
    While(WhileStmt),
    WhileSome(WhileSomeStmt),
    For(ForStmt),
    Repeat(RepeatStmt),
    Loop(LoopStmt),
    Match(MatchStmt),
    Using(UsingStmt),
    Check(CheckStmt),
    /// An expression used as a statement (return value discarded).
    Expr(Box<Expr>),
}

// ── Bindings ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct LocalConst {
    pub name: Name,
    pub ty: Type,
    pub value: Box<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalVar {
    pub name: Name,
    pub ty: Type,
    pub value: Box<Expr>,
    pub span: Span,
}

// ── Assignment ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct AssignStmt {
    pub lvalue: LValue,
    pub value: Box<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompoundAssignStmt {
    pub lvalue: LValue,
    pub op: CompoundOp,
    pub value: Box<Expr>,
    pub span: Span,
}

/// An assignable location: a variable, field, or index.
#[derive(Debug, Clone, PartialEq)]
pub enum LValue {
    Ident(Name),
    Field {
        base: Box<LValue>,
        field: Name,
        span: Span,
    },
    Index {
        base: Box<LValue>,
        index: Box<Expr>,
        span: Span,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompoundOp {
    Add,
    Sub,
    Mul,
    Div,
}

// ── Control flow ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt {
    pub value: Option<Box<Expr>>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt {
    pub condition: Box<Expr>,
    pub then_block: Block,
    /// `else if cond … end` chains.
    pub else_ifs: Vec<(Box<Expr>, Block)>,
    pub else_block: Option<Block>,
    pub span: Span,
}

/// `if some(binding) = expr … end`
#[derive(Debug, Clone, PartialEq)]
pub struct IfSomeStmt {
    pub binding: Name,
    pub value: Box<Expr>,
    pub then_block: Block,
    pub else_block: Option<Block>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStmt {
    pub condition: Box<Expr>,
    pub body: Block,
    pub span: Span,
}

/// `while some(binding) = expr … end`
#[derive(Debug, Clone, PartialEq)]
pub struct WhileSomeStmt {
    pub binding: Name,
    pub value: Box<Expr>,
    pub body: Block,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForStmt {
    pub binding: Name,
    pub second_binding: Option<Name>,
    pub iterable: Box<Expr>,
    pub body: Block,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RepeatStmt {
    pub count: Box<Expr>,
    pub body: Block,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoopStmt {
    pub body: Block,
    pub span: Span,
}

// ── Match ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct MatchStmt {
    pub scrutinee: Box<Expr>,
    pub cases: Vec<MatchCase>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchCase {
    /// `case pattern [if guard]: stmts`
    Pattern {
        pattern: Pattern,
        guard: Option<Box<Expr>>,
        body: Vec<Stmt>,
        span: Span,
    },
    /// `case else: stmts`
    Else { body: Vec<Stmt>, span: Span },
}

// ── Resource cleanup ──────────────────────────────────────────────────────────

/// `using name: Type = expr`
#[derive(Debug, Clone, PartialEq)]
pub struct UsingStmt {
    pub name: Name,
    pub ty: Type,
    pub value: Box<Expr>,
    pub span: Span,
}

// ── Assertion ─────────────────────────────────────────────────────────────────

/// `check expr` or `check expr, "message"`
#[derive(Debug, Clone, PartialEq)]
pub struct CheckStmt {
    pub condition: Box<Expr>,
    pub message: Option<SmolStr>,
    pub span: Span,
}
