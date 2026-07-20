//! Purpose:
//! Home of the PHP `fdiv` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `fdiv` is a pure-data builtin whose return type
//!   (`Float`) is fully determined by its declaration.


builtin! {
    name: "fdiv",
    area: Math,
    params: [num1: Float, num2: Float],
    returns: Float,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Fdiv,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Divides two numbers, according to IEEE 754.",
    php_manual: "https://www.php.net/manual/en/function.fdiv.php",
}
