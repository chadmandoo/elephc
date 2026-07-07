//! Purpose:
//! Declarative eval registry entry for `nl2br`.
//!
//! Called from:
//! - `crate::interpreter::builtins::string`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the newline-to-break hook.

use super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "nl2br",
    area: String,
    params: [string, use_xhtml = EvalBuiltinDefaultValue::Bool(true)],
    direct: Nl2br,
    values: Nl2br,
}
