//! Purpose:
//! Declarative eval registry entry for `isset`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Direct calls stay source-sensitive so operands are checked without normal reads.

eval_builtin! {
    name: "isset",
    area: Symbols,
    params: [var],
    variadic: vars,
    direct: Symbols,
    values: Symbols,
}
