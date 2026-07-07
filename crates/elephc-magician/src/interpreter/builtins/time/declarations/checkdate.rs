//! Purpose:
//! Declarative eval registry entry for `checkdate`.
//!
//! Called from:
//! - `crate::interpreter::builtins::time::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the calendar hook.

eval_builtin! {
    name: "checkdate",
    area: Time,
    params: [month, day, year],
    direct: Time,
    values: Time,
}
