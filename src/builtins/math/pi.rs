//! Purpose:
//! Home of the PHP `pi` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `pi` is a pure-data builtin whose return type
//!   (`Float`) is fully determined by its declaration. It takes no arguments.


builtin! {
    name: "pi",
    area: Math,
    params: [],
    returns: Float,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Pi,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Gets value of pi.",
    php_manual: "https://www.php.net/manual/en/function.pi.php",
}
