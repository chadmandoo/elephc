//! Purpose:
//! Declarative eval registry entry for `is_subclass_of`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the class-relation predicate helper.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "is_subclass_of",
    area: Symbols,
    params: [object_or_class, r#class, allow_string = EvalBuiltinDefaultValue::Bool(true)],
    direct: Symbols,
    values: Symbols,
}
