//! Purpose:
//! Declarative eval registry entry for `microtime`.
//!
//! Called from:
//! - `crate::interpreter::builtins::time::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the wall-clock hook.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "microtime",
    area: Time,
    params: [as_float = EvalBuiltinDefaultValue::Bool(false)],
    direct: Time,
    values: Time,
}
