//! Purpose:
//! Home of the PHP `localtime` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `localtime` is a pure-data builtin whose return type
//!   (`Mixed`) is fully determined by its declaration. Both parameters are optional:
//!   `timestamp` defaults to -1 (current time) and `associative` defaults to `false`.

use crate::builtins::spec::DefaultSpec;

builtin! {
    name: "localtime",
    area: System,
    params: [timestamp: Int = DefaultSpec::Int(-1), associative: Bool = DefaultSpec::Bool(false)],
    returns: Mixed,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Localtime,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Returns the local time.",
}
