//! Purpose:
//! Declarative eval registry entry for `spl_autoload_extensions`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to eval-local autoload extension state.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "spl_autoload_extensions",
    area: Symbols,
    params: [file_extensions = EvalBuiltinDefaultValue::Null],
    direct: Symbols,
    values: Symbols,
}
