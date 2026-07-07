//! Purpose:
//! Declarative eval registry entry for `sqrt`.
//!
//! Called from:
//! - `crate::interpreter::builtins::math`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the existing square-root hook.

eval_builtin! {
    name: "sqrt",
    area: Math,
    params: [num],
    direct: Sqrt,
    values: Sqrt,
}
