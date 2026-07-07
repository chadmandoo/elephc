//! Purpose:
//! Declarative eval registry entry for `log10`.
//!
//! Called from:
//! - `crate::interpreter::builtins::math`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the existing unary float math hook.

eval_builtin! {
    name: "log10",
    area: Math,
    params: [num],
    direct: FloatUnary,
    values: FloatUnary,
}
