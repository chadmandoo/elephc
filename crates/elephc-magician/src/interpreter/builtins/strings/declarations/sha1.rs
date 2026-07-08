//! Purpose:
//! Declarative eval registry entry for `sha1`.
//!
//! Called from:
//! - `crate::interpreter::builtins::strings::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the one-shot hash hook.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "sha1",
    area: String,
    params: [string, binary = EvalBuiltinDefaultValue::Bool(false)],
    direct: HashOneShot,
    values: HashOneShot,
}
