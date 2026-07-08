//! Purpose:
//! Declarative eval registry entry for `hash_algos`.
//!
//! Called from:
//! - `crate::interpreter::builtins::strings::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the static hash algorithm list helper.

eval_builtin! {
    name: "hash_algos",
    area: String,
    params: [],
    direct: HashAlgos,
    values: HashAlgos,
}
