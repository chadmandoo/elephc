//! Purpose:
//! Declarative eval registry entry for `hrtime`.
//!
//! Called from:
//! - `crate::interpreter::builtins::time::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the high-resolution clock hook.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "hrtime",
    area: Time,
    params: [as_number = EvalBuiltinDefaultValue::Bool(false)],
    direct: Time,
    values: Time,
}
