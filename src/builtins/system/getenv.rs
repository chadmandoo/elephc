//! Purpose:
//! Home of the PHP `getenv` builtin: its single-source registry declaration and semantic target.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through `crate::builtins::registry`.
//!
//! Key details:
//! - `check` returns `Union(Str, Bool)` to reflect PHP's behaviour where `getenv`
//!   returns the value string on success or `false` if the variable is unset.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "getenv",
    area: System,
    params: [name: Str],
    returns: Mixed,
    check: check,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::Getenv,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Gets the value of an environment variable.",
}

/// Returns `Union(Str, Bool)` reflecting that `getenv` can return a string or `false`.
///
/// Infers the argument type to trigger type-environment side effects before returning
/// the normalized union type.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    cx.checker.infer_type(&cx.args[0], cx.env)?;
    Ok(cx.checker.normalize_union_type(vec![PhpType::Str, PhpType::False]))
}
