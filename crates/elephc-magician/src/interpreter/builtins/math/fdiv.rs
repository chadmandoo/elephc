//! Purpose:
//! Declarative eval registry entry for `fdiv`.
//!
//! Called from:
//! - `crate::interpreter::builtins::math`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the existing binary float math hook.

eval_builtin! {
    name: "fdiv",
    area: Math,
    params: [num1, num2],
    direct: FloatBinary,
    values: FloatBinary,
}
