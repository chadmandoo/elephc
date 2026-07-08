//! Purpose:
//! Declarative eval registry entry for `get_class`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the class-name introspection helper.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "get_class",
    area: Symbols,
    params: [object = EvalBuiltinDefaultValue::Null],
    direct: Symbols,
    values: Symbols,
}
