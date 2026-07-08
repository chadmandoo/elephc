//! Purpose:
//! Declarative eval registry entry for `ptr`.
//!
//! Called from:
//! - `crate::interpreter::builtins::raw_memory::declarations`.
//!
//! Key details:
//! - Eval keeps `ptr(...)` unsupported because by-value cells do not expose raw
//!   lvalue storage addresses safely.

eval_builtin! {
    name: "ptr",
    area: RawMemory,
    params: [value],
    direct: RawMemory,
    values: RawMemory,
}
