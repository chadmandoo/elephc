//! Purpose:
//! Home of the PHP `array_multisort` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - The golden signature is `fixed(["array1","array2"])` with `ref_params = [true, true]`:
//!   exactly 2 by-ref params. The `ref` markers are mandatory for in-place mutation.
//! - `check` requires BOTH arguments are indexed `Array(_)` types, returning Bool.
//! - `lower` is a thin wrapper over the shared `arrays::lower_array_multisort` emitter.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "array_multisort",
    area: Array,
    params: [ref array1: Mixed, ref array2: Mixed],
    returns: Bool,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::ArrayMultisort,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Sorts multiple arrays or multi-dimensional arrays.",
    php_manual: "https://www.php.net/manual/en/function.array-multisort.php",
}

/// Validates argument types for an `array_multisort` call.
///
/// Requires both arguments be indexed arrays (`PhpType::Array(_)`). Arity (exactly 2) is
/// pre-validated by the registry. Returns `Ok(PhpType::Bool)` on success.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    let ty1 = cx.checker.infer_type(&cx.args[0], cx.env)?;
    let ty2 = cx.checker.infer_type(&cx.args[1], cx.env)?;
    if !matches!(ty1, PhpType::Array(_)) || !matches!(ty2, PhpType::Array(_)) {
        return Err(CompileError::new(cx.span, "array_multisort() arguments must be indexed arrays"));
    }
    Ok(PhpType::Bool)
}
