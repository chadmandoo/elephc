//! Purpose:
//! Declarative eval registry entry for `sscanf`.
//!
//! Called from:
//! - `crate::interpreter::builtins::formatting::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the formatting hook.

eval_builtin! {
    name: "sscanf",
    area: Formatting,
    params: [string, format],
    variadic: vars,
    direct: Formatting,
    values: Formatting,
}
