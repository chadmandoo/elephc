//! Purpose:
//! Declarative eval registry entry for `file_exists`.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the file-probe helper.

eval_builtin! {
    name: "file_exists",
    area: Filesystem,
    params: [filename],
    direct: Filesystem,
    values: Filesystem,
}
