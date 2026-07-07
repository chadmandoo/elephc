//! Purpose:
//! Declarative eval registry entry for `str_pad`.
//!
//! Called from:
//! - `crate::interpreter::builtins::string`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the string-pad hook.

use super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "str_pad",
    area: String,
    params: [
        string,
        length,
        pad_string = EvalBuiltinDefaultValue::String(" "),
        pad_type = EvalBuiltinDefaultValue::Int(1),
    ],
    direct: StrPad,
    values: StrPad,
}
