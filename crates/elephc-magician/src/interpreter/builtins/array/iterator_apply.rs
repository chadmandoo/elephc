//! Purpose:
//! Declarative eval registry entry for `iterator_apply`.
//!
//! Called from:
//! - `crate::interpreter::builtins::array`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the non-mutating array hook.

use super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "iterator_apply",
    area: Array,
    params: [iterator, callback, args = EvalBuiltinDefaultValue::Null],
    direct: Array,
    values: Array,
}
