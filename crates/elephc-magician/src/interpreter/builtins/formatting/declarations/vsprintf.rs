//! Purpose:
//! Declarative eval registry entry for `vsprintf`.
//!
//! Called from:
//! - `crate::interpreter::builtins::formatting::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the formatting hook.

eval_builtin! {
    name: "vsprintf",
    area: Formatting,
    params: [format, values],
    direct: Formatting,
    values: Formatting,
}
