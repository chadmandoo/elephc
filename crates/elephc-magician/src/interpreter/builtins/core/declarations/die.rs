//! Purpose:
//! Declarative eval registry entry for `die`.
//!
//! Called from:
//! - `crate::interpreter::builtins::core::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the process-control hook.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "die",
    area: Core,
    params: [status = EvalBuiltinDefaultValue::Int(0)],
    direct: Core,
    values: Core,
}
