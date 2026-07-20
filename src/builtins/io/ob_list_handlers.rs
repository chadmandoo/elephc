//! Purpose:
//! Home of the PHP `ob_list_handlers` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook when present),
//!   and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - `check` returns `Array<Str>` (the macro cannot express array returns inline):
//! -   one "default output handler" entry per active buffer level.
//! - `lower` is a thin wrapper over `output_buffering::lower_ob_list_handlers`.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "ob_list_handlers",
    area: Io,
    params: [],
    returns: Mixed,
    returns_fresh_storage: true,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::ObListHandlers,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Lists all output handlers in use.",
    php_manual: "function.ob-list-handlers",
}

/// Returns `Array<Str>`: one "default output handler" name per active buffer level.
fn check(_cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    Ok(PhpType::Array(Box::new(PhpType::Str)))
}
