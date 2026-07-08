//! Purpose:
//! Declarative eval registry entry for `function_exists`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the builtin/function probe helper.

eval_builtin! {
    name: "function_exists",
    area: Symbols,
    params: [function],
    direct: Symbols,
    values: Symbols,
}
