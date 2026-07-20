//! Purpose:
//! Home of the PHP `ob_clean` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook when present),
//!   and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - Truncates the top buffer without popping it.
//! - Pure-data builtin: returns `Bool` (`false` when no output buffer is active).
//! - `lower` is a thin wrapper over `output_buffering::lower_ob_clean`.


builtin! {
    name: "ob_clean",
    area: Io,
    params: [],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::ObClean,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Cleans (erases) the contents of the active output buffer.",
    php_manual: "function.ob-clean",
}
