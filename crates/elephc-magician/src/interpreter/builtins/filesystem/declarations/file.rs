//! Purpose:
//! Declarative eval registry entry for `file`.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the file-lines helper.

eval_builtin! {
    name: "file",
    area: Filesystem,
    params: [filename],
    direct: Filesystem,
    values: Filesystem,
}
