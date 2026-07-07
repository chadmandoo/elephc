//! Purpose:
//! Declarative eval registry entry for `localtime`.
//!
//! Called from:
//! - `crate::interpreter::builtins::time::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the local calendar hook.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "localtime",
    area: Time,
    params: [
        timestamp = EvalBuiltinDefaultValue::Null,
        associative = EvalBuiltinDefaultValue::Bool(false),
    ],
    direct: Time,
    values: Time,
}
