//! Purpose:
//! Home of the PHP `mktime` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through
//!   `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `mktime` is a pure-data builtin whose return type
//!   (`Int`) is fully determined by its declaration.


builtin! {
    name: "mktime",
    area: System,
    params: [hour: Int, minute: Int, second: Int, month: Int, day: Int, year: Int],
    returns: Int,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::Mktime,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Returns the Unix timestamp for a date.",
}
