//! Purpose:
//! Declarative eval registry entry for `strstr`.
//!
//! Called from:
//! - `crate::interpreter::builtins::string`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the strstr hook.

use super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "strstr",
    area: String,
    params: [haystack, needle, before_needle = EvalBuiltinDefaultValue::Bool(false)],
    direct: Strstr,
    values: Strstr,
}
