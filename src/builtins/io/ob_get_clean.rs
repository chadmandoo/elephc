//! Purpose:
//! Home of the PHP `ob_get_clean` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook when present),
//!   and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - Captures the contents, then pops and discards the buffer.
//! - `check` returns `Union(Str, False)`: the captured contents, or `false` when
//! -   no output buffer is active.
//! - `returns_fresh_storage` marks both result branches as caller-owned fresh boxes.
//! - `lower` is a thin wrapper over `output_buffering::lower_ob_get_clean`.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "ob_get_clean",
    area: Io,
    params: [],
    returns: Mixed,
    returns_fresh_storage: true,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::ObGetClean,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Gets the current buffer contents and deletes the current output buffer.",
    php_manual: "function.ob-get-clean",
}

/// Returns `Union(Str, False)`: the buffered bytes on success, `false` when no
/// output buffer is active.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    Ok(cx.checker.normalize_union_type(vec![PhpType::Str, PhpType::False]))
}
