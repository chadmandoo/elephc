//! Purpose:
//! Declarative eval registry entry for `call_user_func`.
//!
//! Called from:
//! - `crate::interpreter::builtins::core::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the callable dispatch hook.

eval_builtin! {
    name: "call_user_func",
    area: Core,
    params: [callback],
    variadic: args,
    direct: Core,
    values: Core,
}
