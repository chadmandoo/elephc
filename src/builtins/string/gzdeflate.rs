//! Purpose:
//! Home of the PHP `gzdeflate` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `check` records the zlib bridge requirement via `require_builtin_library("z")`
//!   so the linker pulls in the raw-DEFLATE implementation.
//! - Returns a raw string; unlike the inflate variant it never fails with false.
//! - Argument types are inferred by the common registry dispatch path before the hook fires.

use crate::builtins::spec::{BuiltinCheckCtx, DefaultSpec};
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "gzdeflate",
    area: String,
    params: [data: Str, level: Int = DefaultSpec::Int(-1)],
    returns: Str,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Gzdeflate,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Deflate a string using the DEFLATE data format.",
    php_manual: "https://www.php.net/manual/en/function.gzdeflate.php",
}

/// Returns `PhpType::Str` for a `gzdeflate` call and records the zlib bridge requirement.
///
/// `require_builtin_library("z")` ensures the linker pulls in the zlib implementation.
/// Argument types are inferred by the common registry dispatch path before this hook fires;
/// arity (1–2 args) is pre-validated by the registry.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    cx.checker.require_builtin_library("z");
    Ok(PhpType::Str)
}
