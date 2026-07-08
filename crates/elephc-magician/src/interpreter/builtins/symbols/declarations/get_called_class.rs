//! Purpose:
//! Declarative eval registry entry for `get_called_class`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the current-class scope helper.

eval_builtin! {
    name: "get_called_class",
    area: Symbols,
    params: [],
    direct: Symbols,
    values: Symbols,
}
