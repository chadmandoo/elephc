//! Purpose:
//! Home of the PHP `stream_wrapper_register` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `check` validates that the class argument names a declared class and returns `Bool`.
//! - Arguments are pre-inferred by the registry before the hook runs; the hook does NOT
//!   re-infer them.
//! - `lower` is a thin wrapper over `io::lower_stream_wrapper_register` in the EIR backend.

use crate::builtins::spec::{BuiltinCheckCtx, DefaultSpec};
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "stream_wrapper_register",
    area: Io,
    params: [protocol: Str, class: Str, flags: Int = DefaultSpec::Int(0)],
    returns: Bool,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::StreamWrapperRegister,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Registers a URL wrapper implemented as a PHP class.",
    php_manual: "function.stream-wrapper-register",
}

/// Validates the class argument names a declared class and returns `Bool`.
///
/// Arguments are pre-inferred by the registry; this hook validates the class
/// registration using the shared `validate_registered_stream_class` helper.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    crate::builtins::io::stream_support::validate_registered_stream_class(
        cx.checker,
        cx.name,
        &cx.args[1],
        cx.span,
    )?;
    Ok(PhpType::Bool)
}
