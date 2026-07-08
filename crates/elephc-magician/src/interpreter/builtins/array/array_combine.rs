//! Purpose:
//! Declarative eval registry entry for `array_combine`.
//!
//! Called from:
//! - `crate::interpreter::builtins::array`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the non-mutating array hook.

eval_builtin! {
    name: "array_combine",
    area: Array,
    params: [keys, values],
    direct: Array,
    values: Array,
}
