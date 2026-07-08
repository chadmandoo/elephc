//! Purpose:
//! Declarative eval registry entry for `buffer_new`.
//!
//! Called from:
//! - `crate::interpreter::builtins::raw_memory::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the raw-memory helper.

eval_builtin! {
    name: "buffer_new",
    area: RawMemory,
    params: [length],
    direct: RawMemory,
    values: RawMemory,
}
