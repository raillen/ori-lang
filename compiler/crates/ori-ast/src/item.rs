use crate::common::{Attr, Name, QualifiedName, TypeParams, Visibility, WhereClause};
use crate::expr::Expr;
use crate::stmt::Block;
use crate::ty::Type;
use ori_diagnostics::Span;

// ── Source file ───────────────────────────────────────────────────────────────

/// The root node: one `.orl` source file.
#[derive(Debug, Clone, PartialEq)]
pub struct SourceFile {
    pub namespace: NamespaceDecl,
    pub imports: Vec<ImportDecl>,
    pub items: Vec<ItemWithAttrs>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamespaceDecl {
    pub name: QualifiedName,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportDecl {
    pub visibility: Visibility,
    pub path: QualifiedName,
    pub alias: Option<Name>,
    pub selected: Vec<ImportItem>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportItem {
    pub name: Name,
    pub alias: Option<Name>,
    pub span: Span,
}

// ── Top-level items ───────────────────────────────────────────────────────────

/// A top-level declaration optionally preceded by attributes.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemWithAttrs {
    pub attrs: Vec<Attr>,
    pub item: Item,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Func(FuncDecl),
    Struct(StructDecl),
    Enum(EnumDecl),
    Trait(TraitDecl),
    Implement(ImplementDecl),
    Alias(AliasDecl),
    Const(TopConst),
    Var(TopVar),
    Extern(ExternBlock),
}

impl Item {
    pub fn span(&self) -> Span {
        match self {
            Item::Func(f) => f.span,
            Item::Struct(s) => s.span,
            Item::Enum(e) => e.span,
            Item::Trait(t) => t.span,
            Item::Implement(i) => i.span,
            Item::Alias(a) => a.span,
            Item::Const(c) => c.span,
            Item::Var(v) => v.span,
            Item::Extern(e) => e.span,
        }
    }
}

// ── Functions ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct FuncDecl {
    pub visibility: Visibility,
    pub is_async: bool,
    pub is_mut: bool,
    pub name: Name,
    pub type_params: TypeParams,
    pub params: Vec<Param>,
    pub return_ty: Option<Type>,
    pub where_clause: Option<WhereClause>,
    pub body: Block,
    pub span: Span,
}

/// A `func` declaration with no body (used inside trait declarations).
#[derive(Debug, Clone, PartialEq)]
pub struct FuncSignature {
    pub visibility: Visibility,
    pub is_async: bool,
    pub is_mut: bool,
    pub name: Name,
    pub type_params: TypeParams,
    pub params: Vec<Param>,
    pub return_ty: Option<Type>,
    pub where_clause: Option<WhereClause>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: Name,
    pub ty: Type,
    pub kind: ParamKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParamKind {
    /// Plain required parameter.
    Required,
    /// `name: Type = default_expr`
    Default(Box<Expr>),
    /// `name: Type if it > 0`
    Contract(Box<Expr>),
    /// `name: Type = default_expr if it > 0`
    DefaultAndContract(Box<Expr>, Box<Expr>),
    /// `name: Type...`  (must be the last parameter)
    Variadic,
}

// ── Structs ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct StructDecl {
    pub visibility: Visibility,
    pub name: Name,
    pub type_params: TypeParams,
    pub where_clause: Option<WhereClause>,
    pub fields: Vec<StructField>,
    pub methods: Vec<FuncDecl>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    pub name: Name,
    pub ty: Type,
    /// `if it > 0` value contract; `None` = no contract.
    pub contract: Option<Box<Expr>>,
    pub span: Span,
}

// ── Enums ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDecl {
    pub visibility: Visibility,
    pub name: Name,
    pub type_params: TypeParams,
    pub variants: Vec<EnumVariant>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub name: Name,
    /// Empty = unit variant; non-empty = named-field variant.
    pub fields: Vec<NamedField>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedField {
    pub name: Name,
    pub ty: Type,
    pub span: Span,
}

// ── Traits ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct TraitDecl {
    pub visibility: Visibility,
    pub name: Name,
    pub type_params: TypeParams,
    pub where_clause: Option<WhereClause>,
    pub members: Vec<TraitMember>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TraitMember {
    /// A required method — has signature but no body.
    Required(FuncSignature),
    /// A default method — has both signature and body.
    Default(FuncDecl),
    /// An associated type declaration.
    Type(Name),
}

// ── Implement ─────────────────────────────────────────────────────────────────

/// `implement<T> Trait for Type where T is Bound … end`
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementDecl {
    pub type_params: TypeParams,
    pub trait_name: QualifiedName,
    pub for_type: QualifiedName,
    pub where_clause: Option<WhereClause>,
    pub methods: Vec<FuncDecl>,
    pub associated_types: Vec<(Name, Type)>,
    pub span: Span,
}

// ── Alias ─────────────────────────────────────────────────────────────────────

/// `alias Name<T> = Type`
#[derive(Debug, Clone, PartialEq)]
pub struct AliasDecl {
    pub visibility: Visibility,
    pub name: Name,
    pub type_params: TypeParams,
    pub ty: Type,
    pub span: Span,
}

// ── Top-level const / var ─────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct TopConst {
    pub visibility: Visibility,
    pub name: Name,
    pub ty: Type,
    pub value: Box<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TopVar {
    pub visibility: Visibility,
    pub name: Name,
    pub ty: Type,
    pub value: Box<Expr>,
    pub span: Span,
}

// ── Extern ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct ExternBlock {
    pub abi: AbiLabel,
    pub members: Vec<ExternMember>,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbiLabel {
    C,
    Host,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExternMember {
    Func {
        visibility: Visibility,
        name: Name,
        params: Vec<Param>,
        return_ty: Option<Type>,
        span: Span,
    },
    Var {
        visibility: Visibility,
        name: Name,
        ty: Type,
        span: Span,
    },
}
