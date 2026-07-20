//! Purpose:
//! Home of the PHP `ptr_null` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `check` takes no arguments and returns `PhpType::Pointer(None)`.
//! - `lower` is a thin wrapper over the shared `pointers::lower_ptr_null` emitter.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "ptr_null",
    area: Pointers,
    params: [],
    arity_error: "ptr_null() takes 0 arguments",
    returns: Mixed,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::PtrNull,
            crate::builtins::semantics::BuiltinTargetStrategy::EirGraph,
    ),
    summary: "Returns a null raw pointer.",
    extension: true,
}

/// Returns `PhpType::Pointer(None)` unconditionally (no arguments to validate).
///
/// The registry's `check_arity` handles arity enforcement (exactly 0 arguments).
fn check(_cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    Ok(PhpType::Pointer(None))
}
