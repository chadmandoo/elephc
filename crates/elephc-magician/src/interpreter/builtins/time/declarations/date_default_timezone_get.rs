//! Purpose:
//! Declarative eval registry entry for `date_default_timezone_get`.
//!
//! Called from:
//! - `crate::interpreter::builtins::time::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to eval context timezone state.

eval_builtin! {
    name: "date_default_timezone_get",
    area: Time,
    params: [],
    direct: Time,
    values: Time,
}
