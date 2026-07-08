//! Purpose:
//! Declarative eval registry entry for `hash`.
//!
//! Called from:
//! - `crate::interpreter::builtins::strings::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the one-shot hash hook.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "hash",
    area: String,
    params: [algo, data, binary = EvalBuiltinDefaultValue::Bool(false)],
    direct: HashOneShot,
    values: HashOneShot,
}
