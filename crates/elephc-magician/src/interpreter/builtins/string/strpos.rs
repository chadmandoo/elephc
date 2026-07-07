//! Purpose:
//! Declarative eval registry entry for `strpos`.
//!
//! Called from:
//! - `crate::interpreter::builtins::string`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the string-position hook.

use super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "strpos",
    area: String,
    params: [haystack, needle, offset = EvalBuiltinDefaultValue::Int(0)],
    direct: StringPosition,
    values: StringPosition,
}
