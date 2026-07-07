//! Purpose:
//! Declarative eval registry entry for `getdate`.
//!
//! Called from:
//! - `crate::interpreter::builtins::time::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the local calendar hook.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "getdate",
    area: Time,
    params: [timestamp = EvalBuiltinDefaultValue::Null],
    direct: Time,
    values: Time,
}
