//! Purpose:
//! Declarative eval registry entry for `mktime`.
//!
//! Called from:
//! - `crate::interpreter::builtins::time::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the local mktime hook.

eval_builtin! {
    name: "mktime",
    area: Time,
    params: [hour, minute, second, month, day, year],
    direct: Time,
    values: Time,
}
