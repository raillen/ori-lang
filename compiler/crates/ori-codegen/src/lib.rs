// ori-codegen  real lib.rs, implementation provided

pub mod c_backend;
pub mod native_backend;

pub use c_backend::CCodegen;
pub use native_backend::{emit_native, link};

/// Generate C source code from a `HirModule` (debug / fallback backend).
pub fn emit_c(module: &ori_hir::HirModule) -> String {
    CCodegen::new().generate(module)
}
