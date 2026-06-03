//! Purpose:
//! IR-consuming assembly backend. Produces functionally equivalent ASM to
//! `src/codegen/` while reading from an EIR `Module` instead of an AST.
//!
//! Called from:
//! - `crate::pipeline::compile()` when the `--ir-backend` flag is set.
//!
//! Key details:
//! - Phase 04: 1:1 lowering, no optimization, no register allocation.
//! - Phase 06 adds linear-scan register allocation.
//! - Phase 09 replaces `src/codegen/` as the default backend.

#[allow(dead_code)]
pub mod value_placement;

use std::error::Error;
use std::fmt;

use crate::ir::Module;

/// Error returned by the Phase 04 IR backend while a required lowering path is missing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodegenIrError {
    message: String,
}

impl CodegenIrError {
    /// Creates an error for the current Phase 04 scaffold before instruction lowering exists.
    fn phase04_not_implemented() -> Self {
        Self {
            message: "EIR backend is not implemented yet; use the default AST backend".to_string(),
        }
    }
}

impl fmt::Display for CodegenIrError {
    /// Formats the backend error for CLI diagnostics.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for CodegenIrError {}

/// Result type returned by IR backend entry points.
pub type Result<T> = std::result::Result<T, CodegenIrError>;

/// Generates user-code assembly from a lowered EIR module.
///
/// The Phase 04 backend is currently scaffolded but not yet able to emit code.
/// Later commits replace this explicit error with the function/block/instruction
/// driver while keeping the public entry point stable for the pipeline.
pub fn generate_user_asm_from_ir(
    _module: &Module,
    _gc_stats: bool,
    _heap_debug: bool,
) -> Result<String> {
    Err(CodegenIrError::phase04_not_implemented())
}
