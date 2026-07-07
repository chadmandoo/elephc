//! Purpose:
//! Declarative eval registry entry for `floor`.
//!
//! Called from:
//! - `crate::interpreter::builtins::math`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the existing numeric rounding hook.

eval_builtin! {
    name: "floor",
    area: Math,
    params: [num],
    direct: Floor,
    values: Floor,
}
