//! Purpose:
//! Declarative eval registry entry for `pi`.
//!
//! Called from:
//! - `crate::interpreter::builtins::math`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the existing constant hook.

eval_builtin! {
    name: "pi",
    area: Math,
    params: [],
    direct: Pi,
    values: Pi,
}
