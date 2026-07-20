//! Purpose:
//! Home of the PHP `shell_exec` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - Pure-data builtin: return type (`Str`) is fully determined by the declaration.
//! - `lower` is a thin wrapper over `system::lower_shell_exec` in the EIR backend.


builtin! {
    name: "shell_exec",
    area: System,
    params: [command: Str],
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::ShellExec,
            crate::builtins::semantics::BuiltinTargetStrategy::EirGraph,
    ),
    summary: "Executes a command via the shell and returns the complete output as a string.",
}
