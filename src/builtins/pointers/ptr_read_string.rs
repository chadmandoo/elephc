//! Purpose:
//! Home of the PHP `ptr_read_string` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `check` validates that the first argument is a pointer and the second is an integer
//!   length, and returns `PhpType::Str`.
//! - `lower` is a thin wrapper over the shared `pointers::lower_ptr_read_string` emitter.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "ptr_read_string",
    area: Pointers,
    params: [pointer: Mixed, length: Mixed],
    returns: Str,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::PtrReadString,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Copies raw bytes from a pointer into a PHP string of the given length.",
    extension: true,
}

/// Validates pointer and integer length arguments and returns `PhpType::Str`.
///
/// The registry's `check_arity` handles arity enforcement (exactly 2 arguments).
pub(crate) fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    let ptr_ty = cx.checker.infer_type(&cx.args[0], cx.env)?;
    cx.checker.ensure_pointer_type(&ptr_ty, cx.span, "ptr_read_string()")?;
    let len_ty = cx.checker.infer_type(&cx.args[1], cx.env)?;
    if len_ty != PhpType::Int {
        return Err(CompileError::new(
            cx.span,
            "ptr_read_string() length must be int",
        ));
    }
    Ok(PhpType::Str)
}
