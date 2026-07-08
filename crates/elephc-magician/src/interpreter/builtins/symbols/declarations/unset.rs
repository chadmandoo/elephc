//! Purpose:
//! Declarative eval registry entry for `unset`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Direct calls stay source-sensitive so writable operands can be removed.

eval_builtin! {
    name: "unset",
    area: Symbols,
    params: [var],
    variadic: vars,
    direct: Symbols,
    values: Symbols,
}
