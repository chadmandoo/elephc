//! Purpose:
//! Declarative eval registry entry for `ptr_write_string`.
//!
//! Called from:
//! - `crate::interpreter::builtins::raw_memory::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the raw-memory helper.

eval_builtin! {
    name: "ptr_write_string",
    area: RawMemory,
    params: [pointer, string],
    direct: RawMemory,
    values: RawMemory,
}
