//! Purpose:
//! Home of the PHP `gmdate` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `gmdate` is a pure-data builtin whose return type
//!   (`Str`) is fully determined by its declaration. The `timestamp` parameter
//!   is optional and defaults to `null` (current time).

use crate::builtins::spec::DefaultSpec;

builtin! {
    name: "gmdate",
    area: System,
    params: [format: Str, timestamp: Int = DefaultSpec::Null],
    returns: Str,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Gmdate,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Formats a GMT/UTC date and time.",
}
