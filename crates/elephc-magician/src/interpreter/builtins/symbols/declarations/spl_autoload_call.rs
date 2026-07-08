//! Purpose:
//! Declarative eval registry entry for `spl_autoload_call`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the SPL autoload stub.

eval_builtin! {
    name: "spl_autoload_call",
    area: Symbols,
    params: [class],
    direct: Symbols,
    values: Symbols,
}
