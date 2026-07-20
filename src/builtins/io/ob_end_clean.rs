//! Purpose:
//! Home of the PHP `ob_end_clean` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook when present),
//!   and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - Discards the top buffer and pops the stack.
//! - Pure-data builtin: returns `Bool` (`false` when no output buffer is active).
//! - `lower` is a thin wrapper over `output_buffering::lower_ob_end_clean`.


builtin! {
    name: "ob_end_clean",
    area: Io,
    params: [],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::ObEndClean,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Cleans (erases) the contents of the active output buffer and turns it off.",
    php_manual: "function.ob-end-clean",
}
