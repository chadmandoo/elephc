//! Purpose:
//! Declarative eval registry entry for `usleep`.
//!
//! Called from:
//! - `crate::interpreter::builtins::time::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the microsecond sleep hook.

eval_builtin! {
    name: "usleep",
    area: Time,
    params: [microseconds],
    direct: Time,
    values: Time,
}
