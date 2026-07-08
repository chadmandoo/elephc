//! Purpose:
//! Declarative eval registry entry for `ptr_read32`.
//!
//! Called from:
//! - `crate::interpreter::builtins::raw_memory::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the raw-memory helper.

eval_builtin! {
    name: "ptr_read32",
    area: RawMemory,
    params: [pointer],
    direct: RawMemory,
    values: RawMemory,
}
