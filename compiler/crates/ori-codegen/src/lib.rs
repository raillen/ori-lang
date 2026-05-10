// ori-codegen  real lib.rs, implementation provided

pub mod c_backend;

pub use c_backend::CCodegen;

/// Generate C source code from a `HirModule`.
pub fn emit_c(module: &ori_hir::HirModule) -> String {
    CCodegen::new().generate(module)
}
