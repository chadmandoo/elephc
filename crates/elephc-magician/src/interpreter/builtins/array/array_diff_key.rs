//! Purpose:
//! Declarative eval registry entry for `array_diff_key`.
//!
//! Called from:
//! - `crate::interpreter::builtins::array`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the non-mutating array hook.

eval_builtin! {
    name: "array_diff_key",
    area: Array,
    params: [array],
    variadic: arrays,
    direct: Array,
    values: Array,
}
