//! Purpose:
//! Home of the PHP `hrtime` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - The builtin registry (declaration) and the EIR backend (lower hook),
//!   both via `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `hrtime` is a pure-data builtin whose return type
//!   (`Mixed`) is fully determined by its declaration. The `as_number` parameter
//!   is optional and defaults to `false`.

use crate::builtins::spec::DefaultSpec;

builtin! {
    name: "hrtime",
    area: System,
    params: [as_number: Bool = DefaultSpec::Bool(false)],
    returns: Mixed,
    semantics: crate::builtins::semantics::backend_target_adapter(
            crate::ir::BuiltinRuntimeTarget::Hrtime,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Returns the current high-resolution time.",
}
