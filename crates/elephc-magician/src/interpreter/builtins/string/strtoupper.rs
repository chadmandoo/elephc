//! Purpose:
//! Declarative eval registry entry for `strtoupper`.
//!
//! Called from:
//! - `crate::interpreter::builtins::string`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the string-case hook.

eval_builtin! {
    name: "strtoupper",
    area: String,
    params: [string],
    direct: StringCase,
    values: StringCase,
}
