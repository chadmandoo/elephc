//! Purpose:
//! Declarative eval registry entry for `interface_exists`.
//!
//! Called from:
//! - `crate::interpreter::builtins::symbols::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the interface-existence probe.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "interface_exists",
    area: Symbols,
    params: [interface, autoload = EvalBuiltinDefaultValue::Bool(true)],
    direct: Symbols,
    values: Symbols,
}
