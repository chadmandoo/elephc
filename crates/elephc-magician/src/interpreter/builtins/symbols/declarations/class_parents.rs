//! Purpose:
//! Declarative eval registry entry for `class_parents`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the class-relation metadata helper.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "class_parents",
    area: Symbols,
    params: [object_or_class, autoload = EvalBuiltinDefaultValue::Bool(true)],
    direct: Symbols,
    values: Symbols,
}
