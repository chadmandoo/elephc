//! Purpose:
//! Declarative eval registry entry for `substr`.
//!
//! Called from:
//! - `crate::interpreter::builtins::string`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the substring hook.

use super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "substr",
    area: String,
    params: [string, offset, length = EvalBuiltinDefaultValue::Null],
    direct: Substr,
    values: Substr,
}
