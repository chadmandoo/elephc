//! Purpose:
//! Declarative eval registry entry for `ptr_offset`.
//!
//! Called from:
//! - `crate::interpreter::builtins::raw_memory::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the raw-memory helper.

eval_builtin! {
    name: "ptr_offset",
    area: RawMemory,
    params: [pointer, offset],
    direct: RawMemory,
    values: RawMemory,
}
