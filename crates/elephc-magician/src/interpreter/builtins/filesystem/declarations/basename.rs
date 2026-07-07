//! Purpose:
//! Declarative eval registry entry for `basename`.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the path helper.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "basename",
    area: Filesystem,
    params: [path, suffix = EvalBuiltinDefaultValue::String("")],
    direct: Filesystem,
    values: Filesystem,
}
