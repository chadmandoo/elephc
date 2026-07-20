//! Purpose:
//! Home of the PHP `date` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `date` is a pure-data builtin whose return type
//!   (`Str`) is fully determined by its declaration. The `timestamp` parameter
//!   is optional and defaults to `null` (current time).

use crate::builtins::spec::DefaultSpec;

builtin! {
    name: "date",
    area: System,
    params: [format: Str, timestamp: Int = DefaultSpec::Null],
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Date,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Formats a local time/date.",
}
