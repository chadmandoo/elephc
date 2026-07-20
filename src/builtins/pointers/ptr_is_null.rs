//! Purpose:
//! Home of the PHP `ptr_is_null` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `check` validates that the argument is a pointer type and returns `PhpType::Bool`.
//! - `lower` is a thin wrapper over the shared `pointers::lower_ptr_is_null` emitter.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "ptr_is_null",
    area: Pointers,
    params: [pointer: Mixed],
    returns: Bool,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::PtrIsNull,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Returns true if the pointer is null.",
    extension: true,
}

/// Validates that the argument is a pointer type and returns `PhpType::Bool`.
///
/// The registry's `check_arity` handles arity enforcement (exactly 1 argument).
pub(crate) fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    let ty = cx.checker.infer_type(&cx.args[0], cx.env)?;
    cx.checker.ensure_pointer_type(&ty, cx.span, "ptr_is_null()")?;
    Ok(PhpType::Bool)
}
