//! Purpose:
//! Declarative eval registry entry for `intdiv`.
//!
//! Called from:
//! - `crate::interpreter::builtins::math`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the existing integer-division hook.

eval_builtin! {
    name: "intdiv",
    area: Math,
    params: [num1, num2],
    direct: Intdiv,
    values: Intdiv,
}
