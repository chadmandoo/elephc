//! Purpose:
//! Home of the PHP `spl_autoload_functions` builtin: its declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - A `check` hook is required because the return type `Array<Mixed>` cannot be
//!   expressed as a plain `TypeSpec` ident in the `builtin!` macro.
//! - The function takes no arguments; arity is enforced by the registry.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "spl_autoload_functions",
    area: Spl,
    params: [],
    returns: Mixed,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::SplAutoloadFunctions,
            crate::builtins::semantics::BuiltinTargetStrategy::EirGraph,
    ),
    summary: "Return all registered __autoload() functions.",
    php_manual: "https://www.php.net/manual/en/function.spl-autoload-functions.php",
}

/// Returns `Array<Mixed>` as the precise return type for `spl_autoload_functions()`.
fn check(_cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    Ok(PhpType::Array(Box::new(PhpType::Mixed)))
}
