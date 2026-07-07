//! Purpose:
//! Declarative eval registry entry for `atan2`.
//!
//! Called from:
//! - `crate::interpreter::builtins::math`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the existing paired float math hook.

eval_builtin! {
    name: "atan2",
    area: Math,
    params: [y, x],
    direct: FloatPair,
    values: FloatPair,
}
