//! Purpose:
//! Home of the PHP `readline` builtin: its single-source registry declaration and semantic target.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through `crate::builtins::registry`.
//!
//! Key details:
//! - `check` returns `Union(Str, Bool)` to match PHP's false-on-failure pattern for
//!   end-of-input. The `prompt` argument is optional and pre-inferred by the registry.
//! - `arity_error` is overridden to "readline() takes 0 or 1 arguments" because the
//!   registry's default message for min0/max1 ("takes at most 1 argument") does not
//!   match the legacy error text.
//! - `returns: Mixed` is used because the union cannot be expressed through the scalar
//!   `returns:` field.

use crate::builtins::spec::{BuiltinCheckCtx, DefaultSpec};
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "readline",
    area: Io,
    params: [prompt: Str = DefaultSpec::Null],
    arity_error: "readline() takes 0 or 1 arguments",
    returns: Mixed,
    check: check,
    semantics: crate::builtins::semantics::runtime_fn_semantics(
        crate::ir::RuntimeFnId::Readline,
    ),
    summary: "Reads a line from the user's terminal.",
    php_manual: "function.readline",
}

/// Returns `Union(Str, Bool)` for the readline result (false on end-of-input).
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    Ok(cx.checker.normalize_union_type(vec![
        PhpType::Str,
        PhpType::Bool,
    ]))
}
