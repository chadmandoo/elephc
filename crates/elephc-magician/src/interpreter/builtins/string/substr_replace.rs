//! Purpose:
//! Declarative eval registry entry for `substr_replace`.
//!
//! Called from:
//! - `crate::interpreter::builtins::string`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the substring-replace hook.

use super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "substr_replace",
    area: String,
    params: [string, replace, offset, length = EvalBuiltinDefaultValue::Null],
    direct: SubstrReplace,
    values: SubstrReplace,
}
