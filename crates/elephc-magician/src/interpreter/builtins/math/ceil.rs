//! Purpose:
//! Declarative eval registry entry for `ceil`.
//!
//! Called from:
//! - `crate::interpreter::builtins::math`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the existing numeric rounding hook.

eval_builtin! {
    name: "ceil",
    area: Math,
    params: [num],
    direct: Ceil,
    values: Ceil,
}
