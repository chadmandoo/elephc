//! Purpose:
//! Declarative eval registry entry for `json_last_error`.
//!
//! Called from:
//! - `crate::interpreter::builtins::json`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the JSON error-state hook.

eval_builtin! {
    name: "json_last_error",
    area: Json,
    params: [],
    direct: Json,
    values: Json,
}
