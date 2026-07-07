//! Purpose:
//! Declarative eval registry entry for `pathinfo`.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the pathinfo helper.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "pathinfo",
    area: Filesystem,
    params: [path, flags = EvalBuiltinDefaultValue::Int(15)],
    direct: Filesystem,
    values: Filesystem,
}
