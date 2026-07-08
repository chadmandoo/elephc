//! Purpose:
//! Declarative eval registry entry for `empty`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Direct calls stay source-sensitive so missing variables are not evaluated normally.

eval_builtin! {
    name: "empty",
    area: Symbols,
    params: [value],
    direct: Symbols,
    values: Symbols,
}
