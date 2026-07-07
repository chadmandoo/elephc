//! Purpose:
//! Declarative eval registry entry for `file_get_contents`.
//!
//! Called from:
//! - `crate::interpreter::builtins::filesystem::declarations`.
//!
//! Key details:
//! - Runtime behavior stays delegated to the one-shot file read helper.

eval_builtin! {
    name: "file_get_contents",
    area: Filesystem,
    params: [filename],
    direct: Filesystem,
    values: Filesystem,
}
