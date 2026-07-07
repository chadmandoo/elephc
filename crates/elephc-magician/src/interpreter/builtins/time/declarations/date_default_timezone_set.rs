//! Purpose:
//! Declarative eval registry entry for `date_default_timezone_set`.
//!
//! Called from:
//! - `crate::interpreter::builtins::time::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to eval context timezone state.

eval_builtin! {
    name: "date_default_timezone_set",
    area: Time,
    params: [timezoneId],
    direct: Time,
    values: Time,
}
