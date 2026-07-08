//! Purpose:
//! Declarative eval registry entry for `ptr_sizeof`.
//!
//! Called from:
//! - `crate::interpreter::builtins::raw_memory::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the raw-memory helper.

eval_builtin! {
    name: "ptr_sizeof",
    area: RawMemory,
    params: [r#type],
    direct: RawMemory,
    values: RawMemory,
}
