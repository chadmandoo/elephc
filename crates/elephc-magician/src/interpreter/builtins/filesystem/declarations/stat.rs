//! Purpose:
//! Declarative eval registry entry for `stat`.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the stat-array helper.

eval_builtin! {
    name: "stat",
    area: Filesystem,
    params: [filename],
    direct: Filesystem,
    values: Filesystem,
}
