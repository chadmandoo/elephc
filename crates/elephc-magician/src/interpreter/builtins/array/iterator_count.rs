//! Purpose:
//! Declarative eval registry entry for `iterator_count`.
//!
//! Called from:
//! - `crate::interpreter::builtins::array`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the non-mutating array hook.

eval_builtin! {
    name: "iterator_count",
    area: Array,
    params: [iterator],
    direct: Array,
    values: Array,
}
