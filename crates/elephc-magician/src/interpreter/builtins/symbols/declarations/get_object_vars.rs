//! Purpose:
//! Declarative eval registry entry for `get_object_vars`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the OOP introspection helper.

eval_builtin! {
    name: "get_object_vars",
    area: Symbols,
    params: [object],
    direct: Symbols,
    values: Symbols,
}
