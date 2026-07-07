//! Purpose:
//! Declarative eval registry entry for `log`.
//!
//! Called from:
//! - `crate::interpreter::builtins::math`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the existing logarithm hook.

use super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "log",
    area: Math,
    params: [num, base = EvalBuiltinDefaultValue::Float(std::f64::consts::E)],
    direct: Log,
    values: Log,
}
