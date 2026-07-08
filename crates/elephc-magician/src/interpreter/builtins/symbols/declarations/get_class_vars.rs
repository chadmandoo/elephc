//! Purpose:
//! Declarative eval registry entry for `get_class_vars`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the OOP introspection helper.

eval_builtin! {
    name: "get_class_vars",
    area: Symbols,
    params: [r#class],
    direct: Symbols,
    values: Symbols,
}
