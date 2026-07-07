//! Purpose:
//! Declarative eval registry entry for `strtotime`.
//!
//! Called from:
//! - `crate::interpreter::builtins::time::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the date parser hook.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "strtotime",
    area: Time,
    params: [datetime, baseTimestamp = EvalBuiltinDefaultValue::Null],
    direct: Time,
    values: Time,
}
