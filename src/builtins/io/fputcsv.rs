//! Purpose:
//! Home of the PHP `fputcsv` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `check` validates the `stream` argument is a stream resource and returns `Int`.
//! - Arguments are pre-inferred by the registry before the hook runs.
//! - `lower` is a thin wrapper over `io::lower_fputcsv` in the EIR backend.

use crate::builtins::spec::{BuiltinCheckCtx, DefaultSpec};
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "fputcsv",
    area: Io,
    params: [
        stream: Mixed,
        fields: Mixed,
        separator: Str = DefaultSpec::Str(","),
        enclosure: Str = DefaultSpec::Str("\"")
    ],
    returns: Int,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Fputcsv,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Format line as CSV and write to file pointer.",
    php_manual: "function.fputcsv",
}

/// Validates the stream argument is a stream resource and returns `Int`.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    crate::types::checker::builtins::io::common::ensure_stream_resource(
        cx.checker,
        cx.name,
        &cx.args[0],
        cx.env,
    )?;
    Ok(PhpType::Int)
}
