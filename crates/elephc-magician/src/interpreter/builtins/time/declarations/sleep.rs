//! Purpose:
//! Declarative eval registry entry for `sleep`.
//!
//! Called from:
//! - `crate::interpreter::builtins::time::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the sleep hook.

eval_builtin! {
    name: "sleep",
    area: Time,
    params: [seconds],
    direct: Time,
    values: Time,
}
