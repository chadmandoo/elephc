//! Purpose:
//! Declarative eval registry entry for `spl_object_hash`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the SPL object identity helper.

eval_builtin! {
    name: "spl_object_hash",
    area: Symbols,
    params: [object],
    direct: Symbols,
    values: Symbols,
}
