//! Purpose:
//! Declarative eval registry entry for `defined`.
//!
//! Called from:
//! - `crate::interpreter::builtins::core::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the dynamic-constant hook.

eval_builtin! {
    name: "defined",
    area: Core,
    params: [constant_name],
    direct: Core,
    values: Core,
}
