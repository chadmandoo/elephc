//! Purpose:
//! Declarative eval registry entry for `chop`.
//!
//! Called from:
//! - `crate::interpreter::builtins::string`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the trim-family hook.

use super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "chop",
    area: String,
    params: [string, characters = EvalBuiltinDefaultValue::Bytes(b" \n\r\t\x0b\x0c\0")],
    direct: TrimLike,
    values: TrimLike,
}
