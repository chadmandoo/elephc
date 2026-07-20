//! Purpose:
//! Home of the PHP `passthru` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - Pure-data builtin: return type (`Void`) is fully determined by the declaration.
//! - `lower` is a thin wrapper over `system::lower_passthru` in the EIR backend.


builtin! {
    name: "passthru",
    area: System,
    params: [command: Str],
    returns: Void,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Passthru,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Executes an external program and passes its output directly.",
}
