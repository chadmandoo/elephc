//! Purpose:
//! Declarative eval registry entry for `max`.
//!
//! Called from:
//! - `crate::interpreter::builtins::math`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the existing variadic comparison hook.

eval_builtin! {
    name: "max",
    area: Math,
    params: [value],
    variadic: values,
    direct: MinMax,
    values: MinMax,
}
