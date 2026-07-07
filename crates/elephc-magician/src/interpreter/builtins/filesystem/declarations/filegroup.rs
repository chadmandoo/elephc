//! Purpose:
//! Declarative eval registry entry for `filegroup`.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the scalar stat helper.

eval_builtin! {
    name: "filegroup",
    area: Filesystem,
    params: [filename],
    direct: Filesystem,
    values: Filesystem,
}
