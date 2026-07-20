//! Purpose:
//! Home of the PHP `ob_clean` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through `crate::builtins::registry`.
//!
//! Key details:
//! - Truncates the top buffer without popping it.
//! - Pure-data builtin: returns `Bool` (`false` when no output buffer is active).


builtin! {
    name: "ob_clean",
    area: Io,
    params: [],
    returns: Bool,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::ObClean,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Cleans (erases) the contents of the active output buffer.",
    php_manual: "function.ob-clean",
}
