//! Purpose:
//! Declarative eval registry entry for `lstat`.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the stat-array helper.

eval_builtin! {
    name: "lstat",
    area: Filesystem,
    params: [filename],
    direct: Filesystem,
    values: Filesystem,
}
