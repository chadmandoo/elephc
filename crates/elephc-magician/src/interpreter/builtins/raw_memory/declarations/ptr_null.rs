//! Purpose:
//! Declarative eval registry entry for `ptr_null`.
//!
//! Called from:
//! - `crate::interpreter::builtins::raw_memory::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the raw-memory helper.

eval_builtin! {
    name: "ptr_null",
    area: RawMemory,
    params: [],
    direct: RawMemory,
    values: RawMemory,
}
