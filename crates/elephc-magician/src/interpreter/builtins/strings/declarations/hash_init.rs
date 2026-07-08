//! Purpose:
//! Declarative eval registry entry for `hash_init`.
//!
//! Called from:
//! - `crate::interpreter::builtins::strings::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the incremental hash-context hook.
//! - Optional HMAC parameters remain metadata-only for current eval behavior.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "hash_init",
    area: String,
    params: [
        algo,
        flags = EvalBuiltinDefaultValue::Int(0),
        key = EvalBuiltinDefaultValue::String(""),
    ],
    direct: HashContext,
    values: HashContext,
}
