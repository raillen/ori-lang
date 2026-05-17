// ori-hir  real module exports, implementation pending

pub mod hir;
pub mod lower;
pub mod monomorph;

pub use hir::*;
pub use lower::{insert_default_arguments, lower};
pub use monomorph::monomorphize_generics;
