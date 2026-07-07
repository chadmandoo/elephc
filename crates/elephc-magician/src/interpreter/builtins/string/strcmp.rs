//! Purpose:
//! Declarative eval registry entry for `strcmp`.
//!
//! Called from:
//! - `crate::interpreter::builtins::string`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the string-compare hook.

eval_builtin! {
    name: "strcmp",
    area: String,
    params: [string1, string2],
    direct: StringCompare,
    values: StringCompare,
}
