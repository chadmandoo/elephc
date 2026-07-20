//! Purpose:
//! Home of the PHP `glob` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `check` returns `Array<Str>` (the matched pathnames). A check hook is required
//!   because the array return type cannot be expressed through the scalar `returns:`
//!   field.
//! - `lower` is a thin wrapper over `io::lower_glob` in the EIR backend.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "glob",
    area: Io,
    params: [pattern: Str],
    returns: Mixed,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Glob,
            crate::builtins::semantics::BuiltinTargetStrategy::RuntimeCall,
    ),
    summary: "Finds pathnames matching a pattern.",
    php_manual: "function.glob",
}

/// Returns `Array<Str>` reflecting that `glob` yields the matched pathnames.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    cx.checker.infer_type(&cx.args[0], cx.env)?;
    Ok(PhpType::Array(Box::new(PhpType::Str)))
}
