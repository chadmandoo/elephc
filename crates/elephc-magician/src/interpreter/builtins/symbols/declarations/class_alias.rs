//! Purpose:
//! Declarative eval registry entry for `class_alias`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the symbol dispatch adapter.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "class_alias",
    area: Symbols,
    params: [r#class, alias, autoload = EvalBuiltinDefaultValue::Bool(true)],
    direct: Symbols,
    values: Symbols,
}
