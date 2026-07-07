//! Purpose:
//! Declarative eval registry entry for `dirname`.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the path helper.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "dirname",
    area: Filesystem,
    params: [path, levels = EvalBuiltinDefaultValue::Int(1)],
    direct: Filesystem,
    values: Filesystem,
}
