//! Purpose:
//! Declarative eval registry entry for `clamp`.
//!
//! Called from:
//! - `crate::interpreter::builtins::math`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the existing clamp hook.

eval_builtin! {
    name: "clamp",
    area: Math,
    params: [value, min, max],
    direct: Clamp,
    values: Clamp,
}
