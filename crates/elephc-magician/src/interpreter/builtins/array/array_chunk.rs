//! Purpose:
//! Declarative eval registry entry for `array_chunk`.
//!
//! Called from:
//! - `crate::interpreter::builtins::array`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the non-mutating array hook.

eval_builtin! {
    name: "array_chunk",
    area: Array,
    params: [array, length],
    direct: Array,
    values: Array,
}
