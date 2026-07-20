//! Purpose:
//! Home of the PHP `putenv` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - Pure-data builtin: return type (`Bool`) is fully determined by the declaration.
//! - `lower` is a thin wrapper over `system::lower_putenv` in the EIR backend.


builtin! {
    name: "putenv",
    area: System,
    params: [assignment: Str],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Putenv,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Sets an environment variable.",
}
