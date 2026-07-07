//! Purpose:
//! Declarative eval registry entry for `str_starts_with`.
//!
//! Called from:
//! - `crate::interpreter::builtins::string`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the string-search predicate hook.

eval_builtin! {
    name: "str_starts_with",
    area: String,
    params: [haystack, needle],
    direct: StringSearch,
    values: StringSearch,
}
