//! Purpose:
//! Declarative eval registry entry for `class_attribute_names`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the class-attribute metadata helper.

eval_builtin! {
    name: "class_attribute_names",
    area: Symbols,
    params: [class_name],
    direct: Symbols,
    values: Symbols,
}
