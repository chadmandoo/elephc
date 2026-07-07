//! Purpose:
//! Declarative eval registry entry for `wordwrap`.
//!
//! Called from:
//! - `crate::interpreter::builtins::string`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the wordwrap hook.

use super::super::spec::EvalBuiltinDefaultValue;

eval_builtin! {
    name: "wordwrap",
    area: String,
    params: [
        string,
        width = EvalBuiltinDefaultValue::Int(75),
        r#break = EvalBuiltinDefaultValue::String("\n"),
        cut_long_words = EvalBuiltinDefaultValue::Bool(false),
    ],
    direct: Wordwrap,
    values: Wordwrap,
}
