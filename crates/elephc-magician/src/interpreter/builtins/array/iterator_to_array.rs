//! Purpose:
//! Declarative eval registry entry for `iterator_to_array`.
//!
//! Called from:
//! - `crate::interpreter::builtins::array`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the non-mutating array hook.

use super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "iterator_to_array",
    area: Array,
    params: [iterator, preserve_keys = EvalBuiltinDefaultValue::Bool(true)],
    direct: Array,
    values: Array,
}
