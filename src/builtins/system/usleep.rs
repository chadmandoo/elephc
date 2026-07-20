//! Purpose:
//! Home of the PHP `usleep` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `usleep` is a pure-data builtin whose return type
//!   (`Void`) is fully determined by its declaration.


builtin! {
    name: "usleep",
    area: System,
    params: [microseconds: Int],
    returns: Void,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Usleep,
            crate::builtins::semantics::BuiltinTargetStrategy::RuntimeCall,
    ),
    summary: "Delays execution for a number of microseconds.",
}
