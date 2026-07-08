//! Purpose:
//! Declarative eval registry entry for `array_map`.
//!
//! Called from:
//! - `crate::interpreter::builtins::array`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the non-mutating array hook.

eval_builtin! {
    name: "array_map",
    area: Array,
    params: [callback, array],
    variadic: arrays,
    direct: Array,
    values: Array,
}
