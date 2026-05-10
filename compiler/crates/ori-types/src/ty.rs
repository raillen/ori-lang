use smol_str::SmolStr;
use crate::def::DefId;

/// The canonical type representation used throughout the type checker.
///
/// Unlike `ori_ast::ty::Type` (which mirrors source syntax), `Ty` uses
/// resolved `DefId`s so comparisons are O(1) for named types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    // ── Primitives ────────────────────────────────────────────────────────────
    Bool,
    Int, Int8, Int16, Int32, Int64,
    U8,  U16,  U32,  U64,
    Float, Float32, Float64,
    String,
    Bytes,
    Void,

    /// Type of expressions that never return (break, continue, panic, return).
    Never,

    /// Sentinel — emitted after a type error so checking can continue.
    Error,

    // ── Built-in generic types ────────────────────────────────────────────────
    Optional(Box<Ty>),
    Result(Box<Ty>, Box<Ty>),
    List(Box<Ty>),
    Map(Box<Ty>, Box<Ty>),
    Set(Box<Ty>),
    Range(Box<Ty>),
    Lazy(Box<Ty>),

    /// `any<Trait>` — dynamic dispatch; trait identified by `DefId`.
    Any(DefId),

    /// `tuple<A, B, …>` — always 2 or more elements.
    Tuple(Vec<Ty>),

    /// `func(T, U) -> R`
    Func { params: Vec<Ty>, ret: Box<Ty> },

    // ── User-defined types ────────────────────────────────────────────────────
    /// A named type (struct or enum) with optional generic arguments.
    Named(DefId, Vec<Ty>),

    // ── Generic type parameters ───────────────────────────────────────────────
    /// A generic type parameter inside a declaration: `T` in `func f<T>`.
    Param { index: u32, name: SmolStr },

    /// An unsolved inference variable (used during type inference).
    Infer(u32),
}

impl Ty {
    pub fn is_error(&self) -> bool { matches!(self, Ty::Error) }
    pub fn is_never(&self) -> bool { matches!(self, Ty::Never) }
    pub fn is_numeric(&self) -> bool {
        matches!(self, Ty::Int | Ty::Int8 | Ty::Int16 | Ty::Int32 | Ty::Int64
                      | Ty::U8 | Ty::U16 | Ty::U32 | Ty::U64
                      | Ty::Float | Ty::Float32 | Ty::Float64)
    }
    pub fn is_integer(&self) -> bool {
        matches!(self, Ty::Int | Ty::Int8 | Ty::Int16 | Ty::Int32 | Ty::Int64
                      | Ty::U8 | Ty::U16 | Ty::U32 | Ty::U64)
    }
    pub fn is_float(&self) -> bool {
        matches!(self, Ty::Float | Ty::Float32 | Ty::Float64)
    }

    /// Returns `true` if this type or any contained type is an inference variable.
    pub fn contains_infer(&self) -> bool {
        match self {
            Ty::Infer(_)       => true,
            Ty::Optional(t) | Ty::List(t) | Ty::Set(t) | Ty::Range(t) | Ty::Lazy(t) => t.contains_infer(),
            Ty::Any(_)         => false,
            Ty::Result(a, b) | Ty::Map(a, b) => a.contains_infer() || b.contains_infer(),
            Ty::Tuple(ts)      => ts.iter().any(|t| t.contains_infer()),
            Ty::Func { params, ret } => params.iter().any(|p| p.contains_infer()) || ret.contains_infer(),
            Ty::Named(_, args) => args.iter().any(|a| a.contains_infer()),
            _                  => false,
        }
    }

    /// `Never` and `Infer` (at any depth) are subtypes of everything (v1 — full inference pending).
    pub fn is_assignable_to(&self, other: &Ty) -> bool {
        if self == other    { return true; }
        if self.is_error()  { return true; }
        if self.is_never()  { return true; }
        if other.is_error() { return true; }
        // If either side contains an unresolved Infer, skip the check in v1
        if self.contains_infer() || other.contains_infer() { return true; }
        false
    }

    /// Human-readable display name for diagnostics.
    pub fn display(&self) -> std::string::String {
        match self {
            Ty::Bool    => "bool".into(),
            Ty::Int     => "int".into(),
            Ty::Int8    => "int8".into(),
            Ty::Int16   => "int16".into(),
            Ty::Int32   => "int32".into(),
            Ty::Int64   => "int64".into(),
            Ty::U8      => "u8".into(),
            Ty::U16     => "u16".into(),
            Ty::U32     => "u32".into(),
            Ty::U64     => "u64".into(),
            Ty::Float   => "float".into(),
            Ty::Float32 => "float32".into(),
            Ty::Float64 => "float64".into(),
            Ty::String  => "string".into(),
            Ty::Bytes   => "bytes".into(),
            Ty::Void    => "void".into(),
            Ty::Never   => "never".into(),
            Ty::Error   => "<error>".into(),
            Ty::Optional(t)     => format!("optional<{}>", t.display()),
            Ty::Result(ok, err) => format!("result<{}, {}>", ok.display(), err.display()),
            Ty::List(t)         => format!("list<{}>", t.display()),
            Ty::Map(k, v)       => format!("map<{}, {}>", k.display(), v.display()),
            Ty::Set(t)          => format!("set<{}>", t.display()),
            Ty::Range(t)        => format!("range<{}>", t.display()),
            Ty::Lazy(t)         => format!("lazy<{}>", t.display()),
            Ty::Any(d)          => format!("any<{:?}>", d),
            Ty::Tuple(ts) => {
                let inner = ts.iter().map(|t| t.display()).collect::<Vec<_>>().join(", ");
                format!("tuple<{}>", inner)
            }
            Ty::Func { params, ret } => {
                let ps = params.iter().map(|p| p.display()).collect::<Vec<_>>().join(", ");
                format!("func({}) -> {}", ps, ret.display())
            }
            Ty::Named(id, args) => {
                if args.is_empty() {
                    format!("<def {:?}>", id)
                } else {
                    let as_ = args.iter().map(|a| a.display()).collect::<Vec<_>>().join(", ");
                    format!("<def {:?}><{}>", id, as_)
                }
            }
            Ty::Param { name, .. } => name.to_string(),
            Ty::Infer(id)          => format!("_#{}", id),
        }
    }
}
