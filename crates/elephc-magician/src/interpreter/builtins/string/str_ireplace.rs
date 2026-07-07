//! Purpose:
//! Declarative eval registry entry for `str_ireplace`.
//!
//! Called from:
//! - `crate::interpreter::builtins::string`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the string-replace hook.

use super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "str_ireplace",
    area: String,
    params: [search, replace, subject, count = EvalBuiltinDefaultValue::Null],
    direct: StrReplace,
    values: StrReplace,
}
