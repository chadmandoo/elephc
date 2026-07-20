//! Purpose:
//! Home of the PHP `system` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - Pure-data builtin: return type (`Str`) is fully determined by the declaration.
//! - `lower` is a thin wrapper over `system::lower_system` in the EIR backend.


builtin! {
    name: "system",
    area: System,
    params: [command: Str],
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::System,
            crate::builtins::semantics::BuiltinTargetStrategy::RuntimeCall,
    ),
    summary: "Executes an external program and displays the output.",
}
