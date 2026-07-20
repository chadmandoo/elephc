//! Purpose:
//! Home of the PHP `checkdate` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `checkdate` is a pure-data builtin whose return type
//!   (`Bool`) is fully determined by its declaration.


builtin! {
    name: "checkdate",
    area: System,
    params: [month: Int, day: Int, year: Int],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Checkdate,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Validates a Gregorian date.",
}
