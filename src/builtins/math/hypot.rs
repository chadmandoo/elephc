//! Purpose:
//! Home of the PHP `hypot` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through
//!   `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `hypot` is a pure-data builtin whose return type
//!   (`Float`) is fully determined by its declaration.


builtin! {
    name: "hypot",
    area: Math,
    params: [x: Float, y: Float],
    returns: Float,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::Hypot,
            crate::builtins::semantics::BuiltinTargetStrategy::EirGraph,
    ),
    summary: "Calculates the length of the hypotenuse of a right-angle triangle.",
    php_manual: "https://www.php.net/manual/en/function.hypot.php",
}
