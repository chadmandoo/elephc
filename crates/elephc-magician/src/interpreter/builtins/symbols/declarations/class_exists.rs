//! Purpose:
//! Declarative eval registry entry for `class_exists`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the class-existence probe.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "class_exists",
    area: Symbols,
    params: [r#class, autoload = EvalBuiltinDefaultValue::Bool(true)],
    direct: Symbols,
    values: Symbols,
}
