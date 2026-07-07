//! Purpose:
//! Declarative eval registry entry for `gmdate`.
//!
//! Called from:
//! - `crate::interpreter::builtins::time::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the UTC date-format hook.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "gmdate",
    area: Time,
    params: [format, timestamp = EvalBuiltinDefaultValue::Null],
    direct: Time,
    values: Time,
}
