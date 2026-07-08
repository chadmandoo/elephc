//! Purpose:
//! Declarative eval registry entry for `call_user_func_array`.
//!
//! Called from:
//! - `crate::interpreter::builtins::core::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the callable dispatch hook.

eval_builtin! {
    name: "call_user_func_array",
    area: Core,
    params: [callback, args],
    direct: Core,
    values: Core,
}
