//! Purpose:
//! Home of the PHP `json_last_error_msg` builtin: its declaration and semantic metadata.
//!
//! Called from:
//! - Checker, EIR, optimizer, ownership, and callable consumers through
//!   `crate::builtins::registry`.
//!
//! Key details:
//! - No `check` hook is needed: `json_last_error_msg` takes no arguments and
//!   always returns `Str`. The registry common path enforces arity.


builtin! {
    name: "json_last_error_msg",
    area: System,
    params: [],
    returns: Str,
    semantics: crate::builtins::semantics::runtime_target_semantics(
            crate::ir::BuiltinRuntimeTarget::JsonLastErrorMsg,
            crate::builtins::semantics::BuiltinTargetStrategy::Conditional,
    ),
    summary: "Returns the error string of the last json_encode() or json_decode() call.",
}
