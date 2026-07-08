//! Purpose:
//! Declarative eval registry entry for `hash_final`.
//!
//! Called from:
//! - `crate::interpreter::builtins::strings::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the incremental hash-context hook.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "hash_final",
    area: String,
    params: [context, binary = EvalBuiltinDefaultValue::Bool(false)],
    direct: HashContext,
    values: HashContext,
}
