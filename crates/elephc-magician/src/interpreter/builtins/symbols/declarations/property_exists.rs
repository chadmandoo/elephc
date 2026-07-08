//! Purpose:
//! Declarative eval registry entry for `property_exists`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the OOP member-existence helper.

eval_builtin! {
    name: "property_exists",
    area: Symbols,
    params: [object_or_class, property],
    direct: Symbols,
    values: Symbols,
}
