//! Purpose:
//! Declarative eval registry entry for `array_filter`.
//!
//! Called from:
//! - `crate::interpreter::builtins::array`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the non-mutating array hook.

use super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "array_filter",
    area: Array,
    params: [
        array,
        callback = EvalBuiltinDefaultValue::Null,
        mode = EvalBuiltinDefaultValue::Int(0),
    ],
    direct: Array,
    values: Array,
}
