//! Purpose:
//! Home of the PHP `readdir` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `check` validates the `dir_handle` argument is a stream resource and returns
//!   `Union(Str, Bool)` to reflect PHP's false-on-failure pattern.
//! - `returns: Mixed` is used because the union cannot be expressed through the scalar
//!   `returns:` field. Arguments are pre-inferred by the registry before the hook runs.
//! - `lower` is a thin wrapper over `io::lower_readdir` in the EIR backend.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "readdir",
    area: Io,
    params: [dir_handle: Mixed],
    returns: Mixed,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Readdir,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Read entry from directory handle.",
    php_manual: "function.readdir",
}

/// Validates the directory handle is a stream resource and returns `Union(Str, Bool)`.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    crate::types::checker::builtins::io::common::ensure_stream_resource(
        cx.checker,
        cx.name,
        &cx.args[0],
        cx.env,
    )?;
    Ok(cx.checker.normalize_union_type(vec![
        PhpType::Str,
        PhpType::Bool,
    ]))
}
