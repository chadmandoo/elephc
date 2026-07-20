//! Purpose:
//! Home of the PHP `pow` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `pow` is a pure-data builtin whose return type
//!   (`Float`) is fully determined by its declaration.


builtin! {
    name: "pow",
    area: Math,
    params: [num: Mixed, exponent: Mixed],
    returns: Float,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Pow,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Exponential expression.",
    php_manual: "https://www.php.net/manual/en/function.pow.php",
}
