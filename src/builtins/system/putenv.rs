//! Purpose:
//! Home of the PHP `putenv` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through
//!   `crate::builtins::registry`.
//!
//! Key details:
//! - Pure-data builtin: return type (`Bool`) is fully determined by the declaration.


builtin! {
    name: "putenv",
    area: System,
    params: [assignment: Str],
    returns: Bool,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::Putenv,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Sets an environment variable.",
}
