//! Purpose:
//! Declarative eval registry entry for `get_declared_interfaces`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the declared-symbols helper.

eval_builtin! {
    name: "get_declared_interfaces",
    area: Symbols,
    params: [],
    direct: Symbols,
    values: Symbols,
}
