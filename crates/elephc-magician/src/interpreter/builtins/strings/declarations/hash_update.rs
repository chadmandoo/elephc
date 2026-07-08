//! Purpose:
//! Declarative eval registry entry for `hash_update`.
//!
//! Called from:
//! - `crate::interpreter::builtins::strings::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the incremental hash-context hook.

eval_builtin! {
    name: "hash_update",
    area: String,
    params: [context, data],
    direct: HashContext,
    values: HashContext,
}
