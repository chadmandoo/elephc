//! Purpose:
//! Declarative eval registry entry for `get_parent_class`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the parent-class introspection helper.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "get_parent_class",
    area: Symbols,
    params: [object_or_class = EvalBuiltinDefaultValue::Null],
    direct: Symbols,
    values: Symbols,
}
