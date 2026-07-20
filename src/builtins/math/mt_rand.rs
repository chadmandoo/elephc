//! Purpose:
//! Home of the PHP `mt_rand` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `min_args: 0` allows 0-arg calls (returns a raw random u32) in addition to
//!   the 2-arg range form.
//! - A `check` hook rejects exactly 1 argument, matching PHP's "0 or 2 arguments" rule.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "mt_rand",
    area: Math,
    params: [min: Int, max: Int],
    min_args: 0,
    returns: Int,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::MtRand,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Generate a random value via the Mersenne Twister Random Number Generator.",
    php_manual: "https://www.php.net/manual/en/function.mt-rand.php",
}

/// Rejects exactly 1 argument, matching PHP's "0 or 2 arguments" arity rule.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    if cx.args.len() == 1 {
        return Err(CompileError::new(cx.span, "mt_rand() takes 0 or 2 arguments"));
    }
    Ok(PhpType::Int)
}
