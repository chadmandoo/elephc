//! Purpose:
//! Home of the PHP `gzinflate` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `check` records the zlib bridge requirement via `require_builtin_library("z")` and
//!   returns the `string|false` union (false on decompression failure).
//! - A check hook is required both for the library requirement and to express the
//!   union return type that the `builtin!` macro cannot encode inline.
//! - Argument types are inferred by the common registry dispatch path before the hook fires.

use crate::builtins::spec::{BuiltinCheckCtx, DefaultSpec};
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "gzinflate",
    area: String,
    params: [data: Str, max_length: Int = DefaultSpec::Int(0)],
    returns: Mixed,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Gzinflate,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Inflate a deflated string.",
    php_manual: "https://www.php.net/manual/en/function.gzinflate.php",
}

/// Returns `PhpType::Union([Str, Bool])` for a `gzinflate` call and records the zlib bridge requirement.
///
/// `require_builtin_library("z")` ensures the linker pulls in the zlib implementation.
/// The union return (string on success, false on decompression error) cannot be expressed
/// inline in the `builtin!` macro so a check hook is required.
/// Argument types are inferred by the common registry dispatch path before this hook fires;
/// arity (1–2 args) is pre-validated by the registry.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    cx.checker.require_builtin_library("z");
    Ok(PhpType::Union(vec![PhpType::Str, PhpType::False]))
}
