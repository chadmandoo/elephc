//! Purpose:
//! Home of the PHP `hash_algos` builtin: declaration, type-check hook, and lowering.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook), and the EIR
//!   backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - A check hook is required because `builtin!`'s `returns:` field cannot express an
//!   array return type inline; the hook returns `PhpType::Array(Box::new(PhpType::Str))`.
//! - No bridge library is required (pure compile-time name list, no crypto).
//! - Arity (0 args) is validated by the registry.

use crate::builtins::spec::BuiltinCheckCtx;
use crate::errors::CompileError;
use crate::types::PhpType;

builtin! {
    name: "hash_algos",
    area: String,
    params: [],
    returns: Mixed,
    check: check,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::HashAlgos,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Returns an array of supported hashing algorithm names.",
    php_manual: "https://www.php.net/manual/en/function.hash-algos.php",
}

/// Returns `PhpType::Array(Box::new(PhpType::Str))` for a `hash_algos` call.
///
/// A check hook is required because the `builtin!` macro cannot express array return
/// types inline. No bridge library is required. Arity (0 args) is pre-validated by
/// the registry before this hook fires.
fn check(_cx: &mut BuiltinCheckCtx) -> Result<PhpType, CompileError> {
    Ok(PhpType::Array(Box::new(PhpType::Str)))
}
