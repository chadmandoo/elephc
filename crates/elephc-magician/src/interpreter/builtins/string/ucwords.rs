//! Purpose:
//! Declarative eval registry entry for `ucwords`.
//!
//! Called from:
//! - `crate::interpreter::builtins::string`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the ucwords hook.

use super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "ucwords",
    area: String,
    params: [string, separators = EvalBuiltinDefaultValue::Bytes(b" \t\r\n\x0c\x0b")],
    direct: Ucwords,
    values: Ucwords,
}
