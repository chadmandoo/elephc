//! Purpose:
//! Declarative eval registry entry for `deg2rad`.
//!
//! Called from:
//! - `crate::interpreter::builtins::math`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the existing unary float math hook.

eval_builtin! {
    name: "deg2rad",
    area: Math,
    params: [num],
    direct: FloatUnary,
    values: FloatUnary,
}
