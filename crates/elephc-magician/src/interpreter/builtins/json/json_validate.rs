//! Purpose:
//! Declarative eval registry entry for `json_validate`.
//!
//! Called from:
//! - `crate::interpreter::builtins::json`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the JSON validation hook.

use super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "json_validate",
    area: Json,
    params: [
        json,
        depth = EvalBuiltinDefaultValue::Int(512),
        flags = EvalBuiltinDefaultValue::Int(0),
    ],
    direct: Json,
    values: Json,
}
