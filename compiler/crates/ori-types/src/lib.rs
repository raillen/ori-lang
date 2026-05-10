// ori-types

pub mod check;
pub mod def;
pub mod lower;
pub mod resolve;
pub mod ty;

pub use check::Checker;
pub use def::{Def, DefId, DefKind, DefMap};
pub use lower::lower_type;
pub use resolve::{resolve, FuncSig, ResolvedModule};
pub use ty::Ty;
