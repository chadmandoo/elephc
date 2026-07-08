//! Purpose:
//! Declarative eval registry entry for `trait_exists`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the class-like existence probe.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "trait_exists",
    area: Symbols,
    params: [r#trait, autoload = EvalBuiltinDefaultValue::Bool(true)],
    direct: Symbols,
    values: Symbols,
}
