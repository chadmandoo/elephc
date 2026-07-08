//! Purpose:
//! Declarative eval registry entry for `gzinflate`.
//!
//! Called from:
//! - `crate::interpreter::builtins::strings::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the gzip/zlib hook.

use super::super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "gzinflate",
    area: String,
    params: [data, max_length = EvalBuiltinDefaultValue::Int(0)],
    direct: Gzip,
    values: Gzip,
}
