//! Purpose:
//! Declarative eval registry entry for `vprintf`.
//!
//! Called from:
//! - `crate::interpreter::builtins::formatting::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the formatting hook.

eval_builtin! {
    name: "vprintf",
    area: Formatting,
    params: [format, values],
    direct: Formatting,
    values: Formatting,
}
