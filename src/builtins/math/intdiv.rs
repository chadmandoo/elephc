//! Purpose:
//! Home of the PHP `intdiv` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `intdiv` is a pure-data builtin whose return type
//!   (`Int`) is fully determined by its declaration.


builtin! {
    name: "intdiv",
    area: Math,
    params: [num1: Int, num2: Int],
    returns: Int,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Intdiv,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Integer division.",
    php_manual: "https://www.php.net/manual/en/function.intdiv.php",
}
