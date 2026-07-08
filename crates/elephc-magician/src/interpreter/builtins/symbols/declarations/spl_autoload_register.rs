//! Purpose:
//! Declarative eval registry entry for `spl_autoload_register`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the SPL autoload registration stub.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "spl_autoload_register",
    area: Symbols,
    params: [
        callback = EvalBuiltinDefaultValue::Null,
        throw = EvalBuiltinDefaultValue::Bool(true),
        prepend = EvalBuiltinDefaultValue::Bool(false),
    ],
    direct: Symbols,
    values: Symbols,
}
