//! Purpose:
//! Home of the PHP `ob_flush` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook when present),
//!   and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - Flushes the top buffer to the parent sink without popping it.
//! - Pure-data builtin: returns `Bool` (`false` when no output buffer is active).
//! - `lower` is a thin wrapper over `output_buffering::lower_ob_flush`.


builtin! {
    name: "ob_flush",
    area: Io,
    params: [],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::ObFlush,
            crate::builtins::semantics::BuiltinTargetStrategy::RuntimeCall,
    ),
    summary: "Flushes (sends) the contents of the active output buffer.",
    php_manual: "function.ob-flush",
}
