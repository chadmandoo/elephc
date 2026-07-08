//! Purpose:
//! Declarative eval registry entry for `define`.
//!
//! Called from:
//! - `crate::interpreter::builtins::core::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the dynamic-constant hook.

eval_builtin! {
    name: "define",
    area: Core,
    params: [constant_name, value],
    direct: Core,
    values: Core,
}
