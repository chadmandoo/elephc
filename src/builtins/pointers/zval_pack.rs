//! Purpose:
//! Home of the PHP `zval_pack` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `check` accepts any value and returns `PhpType::Pointer(None)`.
//! - `lower` boxes the value as Mixed and dispatches to the shared zval runtime bridge.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "zval_pack",
    area: Pointers,
    params: [value: Mixed],
    returns: Mixed,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::ZvalPack,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Packs an elephc runtime value into a heap-allocated PHP zval pointer.",
    extension: true,
}

/// Accepts any value and returns an untyped raw pointer to the allocated zval.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    cx.checker.infer_type(&cx.args[0], cx.env)?;
    Ok(PhpType::Pointer(None))
}
