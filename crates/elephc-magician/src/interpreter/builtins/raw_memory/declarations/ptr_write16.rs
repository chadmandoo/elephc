//! Purpose:
//! Declarative eval registry entry for `ptr_write16`.
//!
//! Called from:
//! - `crate::interpreter::builtins::raw_memory::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the raw-memory helper.

eval_builtin! {
    name: "ptr_write16",
    area: RawMemory,
    params: [pointer, value],
    direct: RawMemory,
    values: RawMemory,
}
