//! Purpose:
//! Home of the PHP `hash_update` builtin: single-source registry declaration and semantic target.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through `crate::builtins::registry`.
//!
//! Key details:
//! - Arity (exactly 2 args) is validated by the registry's `check_arity` before the hook fires.


builtin! {
    name: "hash_update",
    area: String,
    params: [context: Mixed, data: Str],
    returns: Bool,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::HashUpdate,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Pumps data into an active incremental hashing context.",
    php_manual: "https://www.php.net/manual/en/function.hash-update.php",
}
