//! Purpose:
//! Declarative eval registry entry for `method_exists`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the OOP member-existence helper.

eval_builtin! {
    name: "method_exists",
    area: Symbols,
    params: [object_or_class, method],
    direct: Symbols,
    values: Symbols,
}
