//! Purpose:
//! Declarative eval registry entry for `class_get_attributes`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the class-attribute metadata helper.

eval_builtin! {
    name: "class_get_attributes",
    area: Symbols,
    params: [class_name],
    direct: Symbols,
    values: Symbols,
}
