//! Purpose:
//! Home of the PHP `ob_get_length` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook when present),
//!   and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `check` returns `Union(Int, False)`: the buffered byte count, or `false` when
//! -   no output buffer is active (runtime -1 sentinel boxed by the lowering).
//! - `lower` is a thin wrapper over `output_buffering::lower_ob_get_length`.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "ob_get_length",
    area: Io,
    params: [],
    returns: Mixed,
    returns_fresh_storage: true,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::ObGetLength,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Returns the length of the output buffer.",
    php_manual: "function.ob-get-length",
}

/// Returns `Union(Int, False)`: the buffered byte count, or `false` when no
/// output buffer is active.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    Ok(cx.checker.normalize_union_type(vec![PhpType::Int, PhpType::False]))
}
