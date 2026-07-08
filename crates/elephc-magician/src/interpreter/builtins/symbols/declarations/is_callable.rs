//! Purpose:
//! Declarative eval registry entry for `is_callable`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Direct and dynamic-ref paths preserve `$callable_name` writeback elsewhere.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "is_callable",
    area: Symbols,
    params: [
        value,
        syntax_only = EvalBuiltinDefaultValue::Bool(false),
        callable_name: by_ref = EvalBuiltinDefaultValue::Null
    ],
    by_ref: [callable_name],
    direct: Symbols,
    values: Symbols,
}
