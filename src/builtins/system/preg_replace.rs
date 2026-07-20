//! Purpose:
//! Home of the PHP `preg_replace` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - Pure-data builtin: return type (`Str`) is fully determined by the declaration.
//! - `lower` is a thin wrapper over `regex::lower_preg_replace` in the EIR backend.


builtin! {
    name: "preg_replace",
    area: System,
    params: [pattern: Str, replacement: Str, subject: Str],
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::PregReplace,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Performs a regular expression search and replace.",
}
