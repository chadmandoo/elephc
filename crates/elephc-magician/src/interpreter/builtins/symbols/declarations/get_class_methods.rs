//! Purpose:
//! Declarative eval registry entry for `get_class_methods`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the OOP introspection helper.

eval_builtin! {
    name: "get_class_methods",
    area: Symbols,
    params: [object_or_class],
    direct: Symbols,
    values: Symbols,
}
