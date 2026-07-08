//! Purpose:
//! Declarative eval registry entry for `array_fill_keys`.
//!
//! Called from:
//! - `crate::interpreter::builtins::array`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the non-mutating array hook.

eval_builtin! {
    name: "array_fill_keys",
    area: Array,
    params: [keys, value],
    direct: Array,
    values: Array,
}
