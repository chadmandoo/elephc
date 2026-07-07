//! Purpose:
//! Declarative eval registry entry for `json_last_error_msg`.
//!
//! Called from:
//! - `crate::interpreter::builtins::json`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the JSON error-message hook.

eval_builtin! {
    name: "json_last_error_msg",
    area: Json,
    params: [],
    direct: Json,
    values: Json,
}
