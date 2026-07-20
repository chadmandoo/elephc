//! Purpose:
//! Home of the PHP `stream_filter_remove` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `check` validates that the argument is a stream resource before returning `Bool`.
//! - Arguments are pre-inferred by the registry before the hook runs; the hook does NOT
//!   re-infer them.
//! - `lower` is a thin wrapper over `io::lower_stream_filter_remove` in the EIR backend.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "stream_filter_remove",
    area: Io,
    params: [stream_filter: Mixed],
    returns: Bool,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::StreamFilterRemove,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Removes a filter from a stream.",
    php_manual: "function.stream-filter-remove",
}

/// Validates the argument is a stream resource and returns `Bool`.
///
/// Arguments are pre-inferred by the registry; this hook only validates the resource constraint.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    crate::types::checker::builtins::io::common::ensure_stream_resource(
        cx.checker,
        cx.name,
        &cx.args[0],
        cx.env,
    )?;
    Ok(PhpType::Bool)
}
