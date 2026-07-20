//! Purpose:
//! Home of the PHP `readlink` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `check` returns `normalize_union_type([Str, Bool])` reflecting PHP behaviour
//!   where `readlink` returns the symlink target or `false` on failure. A check hook
//!   is required because the union return cannot be expressed through the scalar
//!   `returns:` field.
//! - `lower` is a thin wrapper over `io::lower_readlink` in the EIR backend.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "readlink",
    area: Io,
    params: [path: Str],
    returns: Mixed,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Readlink,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Returns the target of a symbolic link.",
    php_manual: "function.readlink",
}

/// Returns `Union(Str, Bool)` reflecting the link target on success or `false` on failure.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    cx.checker.infer_type(&cx.args[0], cx.env)?;
    Ok(cx.checker.normalize_union_type(vec![PhpType::Str, PhpType::False]))
}
