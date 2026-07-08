//! Purpose:
//! Declarative eval registry entry for `array_column`.
//!
//! Called from:
//! - `crate::interpreter::builtins::array`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the non-mutating array hook.

eval_builtin! {
    name: "array_column",
    area: Array,
    params: [array, column_key],
    direct: Array,
    values: Array,
}
