//! Purpose:
//! Home of the PHP `scandir` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `check` returns `Array<Str>` (the directory entries). A check hook is required
//!   because the array return type cannot be expressed through the scalar `returns:`
//!   field.
//! - `lower` is a thin wrapper over `io::lower_scandir` in the EIR backend.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "scandir",
    area: Io,
    params: [directory: Str],
    returns: Mixed,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Scandir,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Lists files and directories inside the specified path.",
    php_manual: "function.scandir",
}

/// Returns `Array<Str>` reflecting that `scandir` yields directory entry names.
fn check(cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    cx.checker.infer_type(&cx.args[0], cx.env)?;
    Ok(PhpType::Array(Box::new(PhpType::Str)))
}
