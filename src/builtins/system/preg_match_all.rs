//! Purpose:
//! Home of the PHP `preg_match_all` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - Pure-data builtin: return type (`Int`) is fully determined by the declaration.
//! - `lower` is a thin wrapper over `regex::lower_preg_match_all` in the EIR backend.


builtin! {
    name: "preg_match_all",
    area: System,
    params: [pattern: Str, subject: Str],
    returns: Int,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::PregMatchAll,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Performs a global regular expression match and returns the number of matches.",
}
