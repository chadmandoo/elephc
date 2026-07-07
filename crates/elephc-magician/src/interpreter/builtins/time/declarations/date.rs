//! Purpose:
//! Declarative eval registry entry for `date`.
//!
//! Called from:
//! - `crate::interpreter::builtins::time::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the date-format hook.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "date",
    area: Time,
    params: [format, timestamp = EvalBuiltinDefaultValue::Null],
    direct: Time,
    values: Time,
}
