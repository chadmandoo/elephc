//! Purpose:
//! Home of the PHP `date_default_timezone_set` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `date_default_timezone_set` is a pure-data builtin
//!   whose return type (`Bool`) is fully determined by its declaration.


builtin! {
    name: "date_default_timezone_set",
    area: System,
    params: [timezoneId: Str],
    returns: Bool,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::DateDefaultTimezoneSet,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Sets the default timezone.",
}
