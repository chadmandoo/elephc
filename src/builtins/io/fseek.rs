//! Purpose:
//! Home of the PHP `fseek` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `check` calls `ensure_stream_resource` on the stream argument for validation and
//!   returns `Int`, matching PHP's `0` success / `-1` failure contract. Arguments are
//!   pre-inferred by the registry before the hook runs.
//! - `lower` is a thin wrapper over `io::lower_fseek` in the EIR backend.

use crate::builtins::spec::{BuiltinCheckCtx, DefaultSpec};
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "fseek",
    area: Io,
    params: [stream: Mixed, offset: Int, whence: Int = DefaultSpec::Int(0)],
    returns: Int,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Fseek,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Seeks on a file pointer.",
    php_manual: "function.fseek",
}

/// Validates the stream argument and returns `Int` for the seek result.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    crate::types::checker::builtins::io::common::ensure_stream_resource(
        cx.checker,
        cx.name,
        &cx.args[0],
        cx.env,
    )?;
    Ok(PhpType::Int)
}
