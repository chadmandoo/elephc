//! Purpose:
//! Home of the PHP `getprotobyname` builtin: its single-source registry declaration and semantic target.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through `crate::builtins::registry`.
//!
//! Key details:
//! - `check` returns `Union(Int, Bool)` reflecting PHP's false-on-failure return.
//! - `returns: Mixed` is used because the union cannot be expressed through the scalar field.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "getprotobyname",
    area: Io,
    params: [protocol: Str],
    returns: Mixed,
    check: check,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::Getprotobyname,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Gets the protocol number associated with the given protocol name.",
    php_manual: "function.getprotobyname",
}

/// Returns `Union(Int, Bool)` reflecting PHP's false-on-failure return.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    Ok(cx.checker.normalize_union_type(vec![PhpType::Int, PhpType::False]))
}
