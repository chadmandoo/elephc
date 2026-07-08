//! Purpose:
//! Declarative eval registry entry for `hash_copy`.
//!
//! Called from:
//! - `crate::interpreter::builtins::strings::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the incremental hash-context hook.

eval_builtin! {
    name: "hash_copy",
    area: String,
    params: [context],
    direct: HashContext,
    values: HashContext,
}
