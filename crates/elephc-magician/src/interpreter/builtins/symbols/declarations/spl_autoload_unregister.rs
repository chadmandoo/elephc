//! Purpose:
//! Declarative eval registry entry for `spl_autoload_unregister`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the SPL autoload registration stub.

eval_builtin! {
    name: "spl_autoload_unregister",
    area: Symbols,
    params: [callback],
    direct: Symbols,
    values: Symbols,
}
