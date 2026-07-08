//! Purpose:
//! Declarative eval registry entry for `buffer_free`.
//!
//! Called from:
//! - `crate::interpreter::builtins::raw_memory::declarations`.
//!
//! Key details:
//! - Direct calls stay source-sensitive so a local buffer variable can be nulled.

eval_builtin! {
    name: "buffer_free",
    area: RawMemory,
    params: [buffer],
    direct: RawMemory,
    values: RawMemory,
}
