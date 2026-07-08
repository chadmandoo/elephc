//! Purpose:
//! Declarative eval registry entry for `printf`.
//!
//! Called from:
//! - `crate::interpreter::builtins::formatting::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the formatting hook.

eval_builtin! {
    name: "printf",
    area: Formatting,
    params: [format],
    variadic: values,
    direct: Formatting,
    values: Formatting,
}
