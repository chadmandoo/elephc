//! Purpose:
//! Declarative eval registry entry for `hypot`.
//!
//! Called from:
//! - `crate::interpreter::builtins::math`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the existing paired float math hook.

eval_builtin! {
    name: "hypot",
    area: Math,
    params: [x, y],
    direct: FloatPair,
    values: FloatPair,
}
