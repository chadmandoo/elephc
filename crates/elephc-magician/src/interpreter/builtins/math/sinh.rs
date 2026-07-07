//! Purpose:
//! Declarative eval registry entry for `sinh`.
//!
//! Called from:
//! - `crate::interpreter::builtins::math`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the existing unary float math hook.

eval_builtin! {
    name: "sinh",
    area: Math,
    params: [num],
    direct: FloatUnary,
    values: FloatUnary,
}
