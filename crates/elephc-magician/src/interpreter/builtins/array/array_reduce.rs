//! Purpose:
//! Declarative eval registry entry for `array_reduce`.
//!
//! Called from:
//! - `crate::interpreter::builtins::array`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the non-mutating array hook.

use super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "array_reduce",
    area: Array,
    params: [array, callback, initial = EvalBuiltinDefaultValue::Null],
    direct: Array,
    values: Array,
}
