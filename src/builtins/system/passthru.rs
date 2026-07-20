//! Purpose:
//! Home of the PHP `passthru` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through
//!   `crate::builtins::registry`.
//!
//! Key details:
//! - Pure-data builtin: return type (`Void`) is fully determined by the declaration.


builtin! {
    name: "passthru",
    area: System,
    params: [command: Str],
    returns: Void,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::Passthru,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Executes an external program and passes its output directly.",
}
