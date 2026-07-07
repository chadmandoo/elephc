//! Purpose:
//! Declarative eval registry entry for `json_encode`.
//!
//! Called from:
//! - `crate::interpreter::builtins::json`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the JSON encode hook.

use super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "json_encode",
    area: Json,
    params: [
        value,
        flags = EvalBuiltinDefaultValue::Int(0),
        depth = EvalBuiltinDefaultValue::Int(512),
    ],
    direct: Json,
    values: Json,
}
