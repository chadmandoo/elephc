//! Purpose:
//! Declarative eval registry entry for `array_intersect_key`.
//!
//! Called from:
//! - `crate::interpreter::builtins::array`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the non-mutating array hook.

eval_builtin! {
    name: "array_intersect_key",
    area: Array,
    params: [array],
    variadic: arrays,
    direct: Array,
    values: Array,
}
