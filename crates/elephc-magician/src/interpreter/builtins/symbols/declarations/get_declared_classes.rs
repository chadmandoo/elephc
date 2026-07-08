//! Purpose:
//! Declarative eval registry entry for `get_declared_classes`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the declared-symbols helper.

eval_builtin! {
    name: "get_declared_classes",
    area: Symbols,
    params: [],
    direct: Symbols,
    values: Symbols,
}
