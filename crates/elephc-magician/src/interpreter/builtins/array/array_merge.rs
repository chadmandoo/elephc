//! Purpose:
//! Declarative eval registry entry for `array_merge`.
//!
//! Called from:
//! - `crate::interpreter::builtins::array`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the non-mutating array hook.

eval_builtin! {
    name: "array_merge",
    area: Array,
    params: [],
    variadic: arrays,
    direct: Array,
    values: Array,
}
