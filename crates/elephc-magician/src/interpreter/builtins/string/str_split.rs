//! Purpose:
//! Declarative eval registry entry for `str_split`.
//!
//! Called from:
//! - `crate::interpreter::builtins::string`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the string-split hook.

use super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "str_split",
    area: String,
    params: [string, length = EvalBuiltinDefaultValue::Int(1)],
    direct: StrSplit,
    values: StrSplit,
}
