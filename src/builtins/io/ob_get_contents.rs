//! Purpose:
//! Home of the PHP `ob_get_contents` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook when present),
//!   and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - Read-only query: the buffer stays active and untouched.
//! - `check` returns `Union(Str, False)`: the captured contents, or `false` when
//! -   no output buffer is active.
//! - `returns_fresh_storage` marks both result branches as caller-owned fresh boxes.
//! - `lower` is a thin wrapper over `output_buffering::lower_ob_get_contents`.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "ob_get_contents",
    area: Io,
    params: [],
    returns: Mixed,
    returns_fresh_storage: true,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::ObGetContents,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Returns the contents of the output buffer.",
    php_manual: "function.ob-get-contents",
}

/// Returns `Union(Str, False)`: the buffered bytes on success, `false` when no
/// output buffer is active.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    Ok(cx.checker.normalize_union_type(vec![PhpType::Str, PhpType::False]))
}
