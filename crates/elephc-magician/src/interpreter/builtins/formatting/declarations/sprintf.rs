//! Purpose:
//! Declarative eval registry entry for `sprintf`.
//!
//! Called from:
//! - `crate::interpreter::builtins::formatting::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the formatting hook.

eval_builtin! {
    name: "sprintf",
    area: Formatting,
    params: [format],
    variadic: values,
    direct: Formatting,
    values: Formatting,
}
