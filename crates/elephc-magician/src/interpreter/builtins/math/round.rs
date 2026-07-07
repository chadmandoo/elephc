//! Purpose:
//! Declarative eval registry entry for `round`.
//!
//! Called from:
//! - `crate::interpreter::builtins::math`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the existing numeric rounding hook.

use super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "round",
    area: Math,
    params: [num, precision = EvalBuiltinDefaultValue::Int(0)],
    direct: Round,
    values: Round,
}
