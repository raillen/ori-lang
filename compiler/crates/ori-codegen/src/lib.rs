// ori-codegen  real lib.rs, implementation provided

pub mod c_backend;
pub mod native_backend;

pub use c_backend::CCodegen;
pub use native_backend::{
    emit_native, emit_native_with_options, jit::run_jit, link, link_with_options,
    NativeEmitOptions, NativeLinkOptions, NativeLinker,
};

/// Generate C source code from a `HirModule` (debug / fallback backend).
pub fn emit_c(module: &ori_hir::HirModule) -> Result<String, String> {
    CCodegen::new().generate(module)
}
