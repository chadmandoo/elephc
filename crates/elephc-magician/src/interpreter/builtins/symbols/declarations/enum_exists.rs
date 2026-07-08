//! Purpose:
//! Declarative eval registry entry for `enum_exists`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the class-like existence probe.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "enum_exists",
    area: Symbols,
    params: [r#enum, autoload = EvalBuiltinDefaultValue::Bool(true)],
    direct: Symbols,
    values: Symbols,
}
