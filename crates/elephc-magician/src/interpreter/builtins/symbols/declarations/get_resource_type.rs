//! Purpose:
//! Declarative eval registry entry for `get_resource_type`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the resource introspection helper.

eval_builtin! {
    name: "get_resource_type",
    area: Symbols,
    params: [resource],
    direct: Symbols,
    values: Symbols,
}
