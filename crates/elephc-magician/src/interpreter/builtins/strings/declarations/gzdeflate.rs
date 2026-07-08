//! Purpose:
//! Declarative eval registry entry for `gzdeflate`.
//!
//! Called from:
//! - `crate::interpreter::builtins::strings::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the gzip/zlib hook.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "gzdeflate",
    area: String,
    params: [data, level = EvalBuiltinDefaultValue::Int(-1)],
    direct: Gzip,
    values: Gzip,
}
