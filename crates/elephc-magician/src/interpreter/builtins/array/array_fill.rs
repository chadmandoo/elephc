//! Purpose:
//! Declarative eval registry entry for `array_fill`.
//!
//! Called from:
//! - `crate::interpreter::builtins::array`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the non-mutating array hook.

eval_builtin! {
    name: "array_fill",
    area: Array,
    params: [start_index, count, value],
    direct: Array,
    values: Array,
}
