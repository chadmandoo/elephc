//! Purpose:
//! Home of the internal `__elephc_phar_get_signature_hash` PHAR intrinsic: its declaration,
//! type-check hook, and lowering. Compiler-synthesized; not PHP-visible.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `internal: true` keeps it out of PHP-visible builtin name sets and
//!   `function_exists()`; it is reachable only through compiler-generated PHAR bodies.
//! - The `check` hook links the `elephc_phar` bridge library (a mandatory side effect);
//!   argument inference is handled by the registry common path, so the hook does not
//!   call `infer_type`.
//! - `lower` is a thin wrapper over `io::lower_elephc_phar_get_signature_hash` in the EIR backend.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "__elephc_phar_get_signature_hash",
    area: Io,
    params: [path: Str],
    returns: Str,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::ElephcPharGetSignatureHash,
            crate::builtins::semantics::BuiltinTargetStrategy::RuntimeCall,
    ),
    summary: "Returns the PHAR signature hash bytes.",
    internal: true,
}

/// Links the `elephc_phar` bridge and returns the intrinsic's `Str` result type.
/// Argument inference is performed by the registry common path before this hook runs.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    cx.checker.require_builtin_library("elephc_phar");
    Ok(PhpType::Str)
}
