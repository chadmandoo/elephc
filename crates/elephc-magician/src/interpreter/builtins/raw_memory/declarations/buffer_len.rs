//! Purpose:
//! Declarative eval registry entry for `buffer_len`.
//!
//! Called from:
//! - `crate::interpreter::builtins::raw_memory::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the raw-memory helper.

eval_builtin! {
    name: "buffer_len",
    area: RawMemory,
    params: [buffer],
    direct: RawMemory,
    values: RawMemory,
}
