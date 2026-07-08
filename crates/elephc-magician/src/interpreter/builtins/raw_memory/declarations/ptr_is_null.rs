//! Purpose:
//! Declarative eval registry entry for `ptr_is_null`.
//!
//! Called from:
//! - `crate::interpreter::builtins::raw_memory::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the raw-memory helper.

eval_builtin! {
    name: "ptr_is_null",
    area: RawMemory,
    params: [pointer],
    direct: RawMemory,
    values: RawMemory,
}
