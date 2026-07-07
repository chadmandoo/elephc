//! Purpose:
//! Declarative eval registry entry for `json_decode`.
//!
//! Called from:
//! - `crate::interpreter::builtins::json`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the JSON decode hook.

use super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "json_decode",
    area: Json,
    params: [
        json,
        associative = EvalBuiltinDefaultValue::Null,
        depth = EvalBuiltinDefaultValue::Int(512),
        flags = EvalBuiltinDefaultValue::Int(0),
    ],
    direct: Json,
    values: Json,
}
