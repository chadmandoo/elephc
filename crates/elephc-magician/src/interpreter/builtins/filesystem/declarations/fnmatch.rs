//! Purpose:
//! Declarative eval registry entry for `fnmatch`.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the fnmatch helper.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "fnmatch",
    area: Filesystem,
    params: [pattern, filename, flags = EvalBuiltinDefaultValue::Int(0)],
    direct: Filesystem,
    values: Filesystem,
}
