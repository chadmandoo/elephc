//! Purpose:
//! Home of the PHP `ob_get_level` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration), the type checker (check hook when present),
//!   and the EIR backend (lower hook), all via `crate::builtins::registry`.
//!
//! Key details:
//! - Pure-data builtin: return type (`Int`, the nesting depth, 0 = no buffering)
//! -   is fully determined by the declaration.
//! - `lower` is a thin wrapper over `output_buffering::lower_ob_get_level`.


builtin! {
    name: "ob_get_level",
    area: Io,
    params: [],
    returns: Int,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::ObGetLevel,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Returns the nesting level of the output buffering mechanism.",
    php_manual: "function.ob-get-level",
}
