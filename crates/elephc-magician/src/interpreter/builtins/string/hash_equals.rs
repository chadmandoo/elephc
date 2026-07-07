//! Purpose:
//! Declarative eval registry entry for `hash_equals`.
//!
//! Called from:
//! - `crate::interpreter::builtins::string`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the constant-time byte compare hook.

eval_builtin! {
    name: "hash_equals",
    area: String,
    params: [known_string, user_string],
    direct: HashEquals,
    values: HashEquals,
}
