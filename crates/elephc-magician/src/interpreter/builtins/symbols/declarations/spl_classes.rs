//! Purpose:
//! Declarative eval registry entry for `spl_classes`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the SPL classes helper.

eval_builtin! {
    name: "spl_classes",
    area: Symbols,
    params: [],
    direct: Symbols,
    values: Symbols,
}
