//! Purpose:
//! Declarative eval registry entry for `time`.
//!
//! Called from:
//! - `crate::interpreter::builtins::time::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the wall-clock hook.

eval_builtin! {
    name: "time",
    area: Time,
    params: [],
    direct: Time,
    values: Time,
}
