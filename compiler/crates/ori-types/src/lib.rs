// ori-types

pub mod check;
pub mod def;
pub mod lower;
pub mod resolve;
pub mod ty;

pub use check::Checker;
pub use def::{Def, DefId, DefKind, DefMap};
pub use lower::{lower_type, lower_type_with_aliases};
pub use resolve::{
    resolve, EnumSig, FuncSig, ImplMethodSig, ImplSig, ReExport, ResolvedModule, StructSig,
    TraitMethodSig, TraitSig, TypeAliasSig, ValueSig, WhereConstraintSig,
};
pub use ty::{expand_ty_aliases, normalize_ty_aliases, substitute_ty_params, Ty};
