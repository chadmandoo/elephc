//! Purpose:
//! Home of the PHP `fseek` builtin: its single-source registry declaration and semantic target.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through `crate::builtins::registry`.
//!
//! Key details:
//! - `check` calls `ensure_stream_resource` on the stream argument for validation and
//!   returns `Int`, matching PHP's `0` success / `-1` failure contract. Arguments are
//!   pre-inferred by the registry before the hook runs.

use crate::builtins::spec::{BuiltinCheckCtx, DefaultSpec};
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "fseek",
    area: Io,
    params: [stream: Mixed, offset: Int, whence: Int = DefaultSpec::Int(0)],
    returns: Int,
    check: check,
    semantics: crate::builtins::semantics::runtime_fn_semantics(
        crate::ir::RuntimeFnId::Fseek,
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
