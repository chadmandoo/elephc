//! Purpose:
//! Home of the PHP `random_int` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `random_int` is a pure-data builtin returning `Int`.


builtin! {
    name: "random_int",
    area: Math,
    params: [min: Int, max: Int],
    returns: Int,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::RandomInt,
            crate::builtins::semantics::BuiltinTargetStrategy::EirGraph,
    ),
    summary: "Get a cryptographically secure, uniformly selected integer.",
    php_manual: "https://www.php.net/manual/en/function.random-int.php",
}
