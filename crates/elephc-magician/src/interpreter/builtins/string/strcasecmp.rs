//! Purpose:
//! Declarative eval registry entry for `strcasecmp`.
//!
//! Called from:
//! - `crate::interpreter::builtins::string`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the string-compare hook.

eval_builtin! {
    name: "strcasecmp",
    area: String,
    params: [string1, string2],
    direct: StringCompare,
    values: StringCompare,
}
