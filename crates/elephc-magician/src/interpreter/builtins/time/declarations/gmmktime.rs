//! Purpose:
//! Declarative eval registry entry for `gmmktime`.
//!
//! Called from:
//! - `crate::interpreter::builtins::time::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the UTC mktime hook.

eval_builtin! {
    name: "gmmktime",
    area: Time,
    params: [hour, minute, second, month, day, year],
    direct: Time,
    values: Time,
}
