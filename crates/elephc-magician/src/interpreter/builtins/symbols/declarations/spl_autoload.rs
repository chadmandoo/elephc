//! Purpose:
//! Declarative eval registry entry for `spl_autoload`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the SPL autoload stub.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "spl_autoload",
    area: Symbols,
    params: [class, file_extensions = EvalBuiltinDefaultValue::Null],
    direct: Symbols,
    values: Symbols,
}
