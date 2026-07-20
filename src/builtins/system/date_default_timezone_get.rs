//! Purpose:
//! Home of the PHP `date_default_timezone_get` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `date_default_timezone_get` is a pure-data builtin
//!   whose return type (`Str`) is fully determined by its declaration.


builtin! {
    name: "date_default_timezone_get",
    area: System,
    params: [],
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::DateDefaultTimezoneGet,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Gets the default timezone.",
}
