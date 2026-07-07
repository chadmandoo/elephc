//! Purpose:
//! Declarative eval registry entry for `pow`.
//!
//! Called from:
//! - `crate::interpreter::builtins::math`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the existing exponentiation hook.

eval_builtin! {
    name: "pow",
    area: Math,
    params: [num, exponent],
    direct: Pow,
    values: Pow,
}
