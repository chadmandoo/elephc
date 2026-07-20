//! Purpose:
//! Home of the PHP `stream_set_timeout` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `check` validates that the first argument is a stream resource before returning `Bool`.
//! - `microseconds` is optional (defaults to 0). Arguments are pre-inferred by the registry.
//! - `lower` is a thin wrapper over `io::lower_stream_set_timeout` in the EIR backend.

use crate::builtins::spec::{BuiltinCheckCtx, DefaultSpec};
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "stream_set_timeout",
    area: Io,
    params: [stream: Mixed, seconds: Int, microseconds: Int = DefaultSpec::Int(0)],
    returns: Bool,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::StreamSetTimeout,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Sets timeout period on a stream.",
    php_manual: "function.stream-set-timeout",
}

/// Validates the stream resource argument and returns `Bool`.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    crate::types::checker::builtins::io::common::ensure_stream_resource(
        cx.checker,
        cx.name,
        &cx.args[0],
        cx.env,
    )?;
    Ok(PhpType::Bool)
}
